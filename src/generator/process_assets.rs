use std::{
    collections::HashMap,
    fs::{self},
};

use chrono::{DateTime, Utc};
use handlebars::Handlebars;
use serde::Serialize;
use serde_json::json;
use walkdir::WalkDir;

const ASSETS_DIRECTORY: &str = "./src/assets";
const TARGET_DIRECTORY: &str = "./dist";

#[derive(Serialize)]
struct PostMetadata {
    path: String,
    title: String,
    desc: Option<String>,
    date: String,
    content: String,
}

pub fn process_assets() -> anyhow::Result<()> {
    let mut handlebars = Handlebars::new();
    let mut posts_for_index: Vec<PostMetadata> = vec![];

    handlebars.register_template_file("layout", "./src/templates/layout.hbs")?;
    handlebars.register_template_file("blog-index", "./src/templates/blog-index.hbs")?;
    handlebars.register_partial("indent", "{{{content}}}")?; // this is weird, but it works https://github.com/sunng87/handlebars-rust/issues/691

    for entry in WalkDir::new(ASSETS_DIRECTORY)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.path().extension().is_some_and(|v| v == "md") {
            let markdown = fs::read_to_string(entry.path()).expect("file to be readable");
            let (meta, meat) = markdown
                .split_once("-->")
                .unwrap_or_else(|| ("", &markdown));
            let mut metadata = parse_metadata(meta);

            let content = markdown::to_html_with_options(meat, &markdown::Options::gfm())
                .map_err(|e| anyhow::anyhow!(e))?;

            // populate post metadata for building the index
            if entry
                .path()
                .parent()
                .is_some_and(|parent| parent.to_string_lossy().contains("blog"))
            {
                posts_for_index.push(PostMetadata {
                    path: entry
                        .path()
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
            }

            let file_sub_path = entry.path().strip_prefix(ASSETS_DIRECTORY)?;
            let dist_mirror = format!(
                "{TARGET_DIRECTORY}/{}",
                file_sub_path.parent().unwrap().to_str().unwrap()
            );

            fs::create_dir_all(&dist_mirror)?;

            metadata.entry("content".into()).or_insert_with(|| content);
            // metadata default
            metadata
                .entry("title".into())
                .or_insert_with(|| "heffree.dev".into());

            if entry
                .path()
                .to_string_lossy()
                .contains(&format!("{ASSETS_DIRECTORY}/index.md"))
            {
                metadata
                    .entry("home".into())
                    .or_insert_with(|| "exists".into());
            }

            // first pass to apply metadata and insert content
            let html = handlebars.render("layout", &metadata)?;
            // second pass to populate content values
            let html_2 = handlebars.render_template(&html, &json!({"prof_years": 8}))?;

            fs::write(
                format!(
                    "{dist_mirror}/{}.html",
                    entry.path().file_stem().unwrap().to_string_lossy()
                ),
                html_2,
            )?;
        }

        if entry.path().extension().is_some_and(|v| {
            ["css", "html", "pdf", "svg", "jpg", "png", "webm"]
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

/// Parses a collection of string slices by separating them into key-value pairs separated by a ":"
fn parse_metadata(metadata: &str) -> HashMap<String, String> {
    let mut map: HashMap<String, String> = HashMap::new();
    for line in metadata.lines() {
        if line.contains(":") {
            let (key, value) = line.split_once(":").expect("contains to work right?");
            map.entry(key.to_string())
                .or_insert_with(|| value.trim().to_string());
        }
    }
    map
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
