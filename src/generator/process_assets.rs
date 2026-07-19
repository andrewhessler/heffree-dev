use std::{
    collections::HashMap,
    fs::{self},
    path::Path,
};

use chrono::{DateTime, Utc};
use handlebars::Handlebars;
use markdown::{Constructs, ParseOptions};
use regex::{Captures, Regex};
use serde::Serialize;
use serde_json::json;
use syntect::{highlighting::ThemeSet, html::highlighted_html_for_string, parsing::SyntaxSet};
use walkdir::WalkDir;

const ASSETS_DIRECTORY: &str = "./src/assets";
const TARGET_DIRECTORY: &str = "./dist";
const BUCKET_PREFIX: &str = "https://heffree-dev.us-ord-10.linodeobjects.com/";

#[derive(Serialize)]
struct PostMetadata {
    path: String,
    title: String,
    tags: Vec<String>,
    desc: Option<String>,
    date: String,
    content: String,
}

pub fn process_assets() -> anyhow::Result<()> {
    let mut handlebars = Handlebars::new();
    let mut ss = SyntaxSet::load_defaults_newlines().into_builder();
    ss.add_from_folder("./src/syntaxes/", true)?;
    let ss = ss.build();

    let mut posts_for_index: Vec<PostMetadata> = vec![];

    handlebars.register_template_file("layout", "./src/templates/layout.hbs")?;
    handlebars.register_template_file("blog-index", "./src/templates/blog-index.hbs")?;
    handlebars.register_partial("indent", "{{{content}}}")?; // this is weird, but it works https://github.com/sunng87/handlebars-rust/issues/691

    for entry in WalkDir::new(ASSETS_DIRECTORY)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.path().extension().is_some_and(|v| v == "md")
            && let Some(post_metadata) =
                process_md(entry.path(), &handlebars, &ss).expect("md is processable")
        {
            posts_for_index.push(post_metadata);
        }

        // move the actual "assets" and existing html to target directory
        if entry.path().extension().is_some_and(|v| {
            ["css", "js", "html", "pdf", "svg", "jpg", "png", "webm"]
                .iter()
                .any(|c| *c == v.to_str().unwrap())
        }) {
            let file_sub_path = entry.path().strip_prefix(ASSETS_DIRECTORY)?;
            let dist_mirror_parent = format!(
                "{TARGET_DIRECTORY}/{}",
                file_sub_path.parent().unwrap().to_str().unwrap()
            );
            let dist_mirror = format!("{TARGET_DIRECTORY}/{}", file_sub_path.to_str().unwrap());

            fs::create_dir_all(&dist_mirror_parent)?;
            fs::copy(entry.path().to_str().unwrap(), dist_mirror)?;
        }
    }

    // sort posts by date desc
    posts_for_index.sort_by(|p1, p2| p2.date.cmp(&p1.date));

    let blikidex = gen_blikidex(&posts_for_index, handlebars);
    let atom_feed = gen_rss(&posts_for_index);
    fs::write(format!("{TARGET_DIRECTORY}/blog/index.html"), blikidex)?;
    fs::write(format!("{TARGET_DIRECTORY}/atom.xml"), atom_feed)?;
    Ok(())
}

/// Writes rendered markdown file to an html file, returns PostMetadata if markdown was a blog post
fn process_md(
    file_path: &Path,
    handlebars: &Handlebars,
    ss: &SyntaxSet,
) -> anyhow::Result<Option<PostMetadata>> {
    let mut post_metadata = None;
    let mut sidenote_map: HashMap<String, String> = HashMap::new();

    let markdown = fs::read_to_string(file_path).expect("file to be readable");
    let mut iter = markdown.splitn(3, "---\n").skip(1);

    let archetype = iter.next().unwrap_or("");
    let meat = iter
        .next()
        .unwrap_or_else(|| panic!("{file_path:?} content should exist"));
    let (mut metadata, tags) = parse_metadata(archetype);

    // gather sidenote content
    let re = Regex::new(r"\[\^(.*)\]:(.*)").expect("sidenote content regex should be valid");
    let meat = re.replace_all(meat, |caps: &regex::Captures| {
        let id = &caps[1];
        let text = &caps[2];
        sidenote_map
            .entry(id.to_string())
            .or_insert_with(|| text.trim().to_string());
        println!("{sidenote_map:?}");
        ""
    });

    // replace @@img tags
    let re = Regex::new(r"@@img:([^\s]*)").expect("img bucket regex should be valid");
    let meat = re.replace_all(&meat, |caps: &regex::Captures| {
        let name = &caps[1];
        let alt_text = name.replace("_", "");
        format!(r"![{alt_text}]({BUCKET_PREFIX}{name})")
    });

    let markdown_options = markdown::Options {
        parse: ParseOptions {
            constructs: Constructs {
                gfm_footnote_definition: false,
                ..Default::default()
            },
            ..Default::default()
        },
        ..markdown::Options::gfm()
    };
    let content =
        markdown::to_html_with_options(&meat, &markdown_options).map_err(|e| anyhow::anyhow!(e))?;

    // populate post metadata for building the index
    if file_path
        .parent()
        .is_some_and(|parent| parent.to_string_lossy().contains("blog"))
    {
        post_metadata = Some(PostMetadata {
            tags: tags.clone(),
            path: file_path
                .file_stem()
                .expect("file stem")
                .to_string_lossy()
                .to_string(),
            desc: metadata.get("desc").cloned(),
            title: metadata
                .get("title")
                .expect("title to exist for post")
                .to_string(),
            date: metadata
                .get("date")
                .expect("date to exist for post")
                .to_string(),
            content: content.clone(),
        });
        metadata
            .entry("post".into())
            .or_insert_with(|| "true".to_string());
    }

    let file_sub_path = file_path.strip_prefix(ASSETS_DIRECTORY)?;
    let dist_mirror = format!(
        "{TARGET_DIRECTORY}/{}",
        file_sub_path.parent().unwrap().to_str().unwrap()
    );

    fs::create_dir_all(&dist_mirror)?;

    metadata.entry("content".into()).or_insert_with(|| content);
    // metadata default
    metadata
        .entry("title".into())
        .or_insert_with(|| "heffree.dev".to_string());

    if file_path
        .to_string_lossy()
        .contains(&format!("{ASSETS_DIRECTORY}/index.md"))
    {
        metadata
            .entry("home".into())
            .or_insert_with(|| "exists".to_string());
    }

    // first pass to apply metadata and insert content
    let mut json_metadata: HashMap<String, serde_json::Value> = metadata
        .into_iter()
        .map(|(key, val)| (key, json!(val)))
        .collect();
    json_metadata.insert("tags".to_string(), json!(tags));
    let html = handlebars.render("layout", &json_metadata)?;
    // second pass to populate content values
    let html = handlebars.render_template(&html, &json!({"prof_years": 8}))?;

    // replace inline footnotes
    let re = Regex::new(r"\[\^([^\]]*)\]").expect("inline footnote regex should be healthy");
    let mut count = 0;
    let html = re.replace_all(&html, |cap: &regex::Captures| {
        let id = &cap[1];
        let text = sidenote_map
            .get(id)
            .unwrap_or_else(|| panic!("{id} footnote content should exist in map"));
        count += 1;
        let count_string = count.to_string().trim().to_owned();
        format!(
            r#"<label for="{id}" class="sidenote-number">{count_string}</label><input type="checkbox" id={id} class="margin-toggle" /><span class="sidenote"><label for="{id}" class="sidenote-number">{count_string}</label>{text}</span>"#
        )
    });

    let highlighted_html = highlight_code_blocks(&html, ss)?;

    fs::write(
        format!(
            "{dist_mirror}/{}.html",
            file_path.file_stem().unwrap().to_string_lossy()
        ),
        highlighted_html,
    )?;

    Ok(post_metadata)
}

fn highlight_code_blocks(html: &str, ss: &SyntaxSet) -> anyhow::Result<String> {
    let theme = ThemeSet::get_theme("./src/themes/one-dark.tmTheme")?;
    let re = Regex::new(r#"(?s)<pre><code class="language-(\w+)">(.*?)</code></pre>"#)?;

    let result = re.replace_all(html, |caps: &Captures| {
        let lang = &caps[1];
        let code = html_escape::decode_html_entities(&caps[2]);

        let syntax = ss.find_syntax_by_token(lang).unwrap();
        let highlighted = highlighted_html_for_string(&code, ss, syntax, &theme).unwrap();
        highlighted.replace("<pre style=\"", "<pre class=\"code-block\" style=\"")
    });

    Ok(result.to_string())
}

/// Takes a string, e.g.
/// ```
/// key1: value1
/// key2: value2
/// ```
/// and parses into a HashMap, also handles parsing tags into a vec
fn parse_metadata(metadata: &str) -> (HashMap<String, String>, Vec<String>) {
    let mut map: HashMap<String, String> = HashMap::new();
    let mut tags: Vec<String> = vec![];
    for line in metadata.lines() {
        if line.contains(":") {
            let (key, value) = line.split_once(":").expect("contains to work right?");
            if key == "tags" {
                tags = value
                    .trim()
                    .replace("[", "")
                    .replace("]", "")
                    .split(",")
                    .map(|val| val.trim().to_string().replace("\"", ""))
                    .collect();
            } else {
                map.entry(key.to_string())
                    .or_insert_with(|| value.trim().to_string());
            }
        }
    }
    (map, tags)
}

fn gen_blikidex(posts: &[PostMetadata], handlebars: Handlebars) -> String {
    let content = handlebars
        .render(
            "blog-index",
            &json!({
                "posts": posts,
            }),
        )
        .expect("index to render");

    handlebars
        .render(
            "layout",
            &json!({
                "content": content,
                "title": "Blikidex"
            }),
        )
        .expect("index to render in layout")
}

fn gen_rss(posts: &[PostMetadata]) -> String {
    let entries: Vec<String> = posts
        .iter()
        .map(|post| {
            let title = post.title.clone();
            let url = format!("https://heffree.dev/blog/{}{}", post.path.clone(), ".html");
            let pub_date = DateTime::parse_from_str(
                &format!("{} 00:00:00 +0000", post.date),
                "%Y-%m-%d %H:%M:%S %z",
            )
            .expect("date to be parseable")
            .to_rfc3339();
            let desc = post.desc.clone().unwrap_or("".to_string());
            let content = post.content.clone();

            format!(
                r#"<entry>
                <title>{title}</title>
                <id>{url}</id>
                <link href="{url}" />
                <author>
                <name>Andrew Hessler</name>
                </author>
                <published>{pub_date}</published>
                <updated>{pub_date}</updated>
                <summary>{desc}</summary>
                <content type="html"><![CDATA[{content}]]></content>
                </entry>"#
            )
        })
        .collect();

    let entries_string = entries.join("");

    let now = Utc::now().to_rfc3339();

    let base_url = "https://heffree.dev";
    let rss = format!(
        r#"<?xml version="1.0" encoding="utf-8"?>
        <feed xmlns="http://www.w3.org/2005/Atom">
        <title>Andrew Hessler</title>
        <link href="{base_url}/" />
        <link href="{base_url}/feed.atom" rel="self" />
        <updated>{now}</updated>
        <language>en</language>
        <author>
        <name>Andrew Hessler</name>
        </author>
        {entries_string}
        </feed>"#
    );

    rss.to_string()
}
