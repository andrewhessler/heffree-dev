use std::fs;

use walkdir::WalkDir;

const ASSETS_DIRECTORY: &str = "./src/assets";
const TARGET_DIRECTORY: &str = "./dist";

fn main() -> anyhow::Result<()> {
    for entry in WalkDir::new(format!("{ASSETS_DIRECTORY}"))
        .into_iter()
        .filter_map(|e| e.ok())
    {
        println!("{:?}", entry.path());
        if entry.path().extension().is_some_and(|v| v == "md") {
            let markdown = fs::read_to_string(entry.path()).expect("file to be readable");
            let html = markdown::to_html_with_options(&markdown, &markdown::Options::gfm())
                .map_err(|e| anyhow::anyhow!(e))?;

            let file_sub_path = entry.path().strip_prefix(ASSETS_DIRECTORY)?;
            let dist_mirror = format!(
                "{TARGET_DIRECTORY}/{}",
                file_sub_path.parent().unwrap().to_str().unwrap()
            );

            fs::create_dir_all(&dist_mirror)?;
            fs::write(
                format!(
                    "{dist_mirror}/{}.html",
                    entry.path().file_stem().unwrap().to_string_lossy()
                ),
                html,
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
    Ok(())
}
