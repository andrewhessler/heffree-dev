use std::{
    collections::HashMap,
    fs::{self},
};

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
    date: String,
}

pub fn process_assets() -> anyhow::Result<()> {
    let mut handlebars = Handlebars::new();
    let mut posts_for_index: Vec<PostMetadata> = vec![];

    handlebars.register_template_file("layout", "./src/templates/layout.hbs")?;
    handlebars.register_template_file("bliki-index", "./src/templates/bliki-index.hbs")?;
    handlebars.register_partial("indent", "{{{content}}}")?; // this is weird, but it works https://github.com/sunng87/handlebars-rust/issues/691

    for entry in WalkDir::new(format!("{ASSETS_DIRECTORY}"))
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.path().extension().is_some_and(|v| v == "md") {
            let markdown = fs::read_to_string(entry.path()).expect("file to be readable");
            let (meta, meat) = markdown
                .split_once("-->")
                .unwrap_or_else(|| ("", &markdown));
            let mut metadata = parse_metadata(meta);

            // populate post metadata for building the index
            if entry
                .path()
                .parent()
                .is_some_and(|parent| parent.to_string_lossy().contains("bliki"))
            {
                posts_for_index.push(PostMetadata {
                    path: entry
                        .path()
                        .file_stem()
                        .expect("file stem")
                        .to_string_lossy()
                        .to_string(),
                    title: metadata
                        .get("title")
                        .expect("title to exist for post")
                        .to_string(),
                    date: metadata
                        .get("date")
                        .expect("date to exist for post")
                        .to_string(),
                });
            }

            let content = markdown::to_html_with_options(meat, &markdown::Options::gfm())
                .map_err(|e| anyhow::anyhow!(e))?;

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
            ["css", "html", "pdf", "svg"]
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

    let bliki_index = gen_bliki_index(posts_for_index, handlebars);
    fs::write(format!("{TARGET_DIRECTORY}/bliki/index.html"), bliki_index)?;
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

fn gen_bliki_index(mut posts: Vec<PostMetadata>, handlebars: Handlebars) -> String {
    posts.sort_by(|p1, p2| p2.date.cmp(&p1.date));
    let content = handlebars
        .render(
            "bliki-index",
            &json!({
                "posts": posts,
            }),
        )
        .expect("index to render");

    let index = handlebars
        .render(
            "layout",
            &json!({
                "content": content,
            }),
        )
        .expect("index to render in layout");

    index
}
