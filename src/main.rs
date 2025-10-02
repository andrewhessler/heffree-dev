use crate::generator::process_assets;

mod generator;

fn main() -> anyhow::Result<()> {
    process_assets::process_assets()
}
