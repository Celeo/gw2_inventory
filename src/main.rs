use anyhow::Result;
use dialoguer::{theme::ColorfulTheme, MultiSelect};
use log::{debug, info};
use std::{env, iter, path::Path};

mod api;
mod models;
use api::Api;
mod cache;
use cache::Cache;
mod filtering;
mod screen;

fn select_characters(characters: &[String]) -> Result<Vec<&str>> {
    let theme = ColorfulTheme::default();
    info!("Select which character(s) to search");
    info!("Use the arrow keys to move, the spacebar to toggle, and enter to submit\n");
    let chosen = MultiSelect::with_theme(&theme)
        .with_prompt("Select values")
        .items(&characters)
        .defaults(
            &iter::repeat(false) // TODO change to 'true'
                .take(characters.len())
                .collect::<Vec<_>>(),
        )
        .interact()?;
    Ok(characters
        .iter()
        .enumerate()
        .filter(|(index, _)| chosen.contains(index))
        .map(|(_, name)| name.as_str())
        .collect::<Vec<_>>())
}

fn main() -> Result<()> {
    if Path::new(".env").exists() {
        dotenv::dotenv().expect("Could not load from .env file");
    }
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "gw2_inventory=info");
    }
    pretty_env_logger::init();
    debug!("Starting up");

    let api = Api::new(&env::var("API_KEY")?)?;
    let mut cache = Cache::new();
    info!("Getting item data");
    cache.populate_cache(&api)?;
    info!("Getting your characters");
    let characters = api.get_token_character_names()?;

    debug!("Selecting characeters");
    let selected = select_characters(&characters)?;
    if selected.is_empty() {
        info!("No characters selected; exiting");
        return Ok(());
    }

    info!("Getting inventories");
    let inventories = api.get_inventories(&selected)?;

    debug!("Launching TUI");
    screen::run(inventories, cache)?;

    Ok(())
}
