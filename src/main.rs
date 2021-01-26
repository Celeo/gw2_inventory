use anyhow::Result;
use std::path::Path;

mod api;
mod models;
// use api::Api;
mod cache;
// use cache::Cache;
mod screen;

fn main() -> Result<()> {
    if Path::new(".env").exists() {
        dotenv::dotenv().expect("Could not load from .env file");
    }
    pretty_env_logger::init();

    // let api = Api::new(&env::var("API_KEY")?)?;
    // let mut cache = Cache::new();
    // cache.populate_cache(&api)?;
    // let inventories = api.get_all_inventories()?;

    screen::run()?;

    Ok(())
}
