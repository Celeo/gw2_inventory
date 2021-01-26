use crate::{api::Api, models::ItemInfo};
use anyhow::Result;
use log::debug;
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

fn cache_file_path() -> PathBuf {
    Path::join(
        &home::home_dir().expect("Could not determine home directory"),
        ".gw2_inventory_cache.json",
    )
}

pub struct Cache {
    items: HashMap<u64, ItemInfo>,
}

impl Cache {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }

    fn load_from_file(&mut self) -> Result<()> {
        debug!("Loading cache from file");
        let content = fs::read_to_string(cache_file_path())?;
        let data: HashMap<u64, ItemInfo> = serde_json::from_str(&content)?;
        self.items = data;
        debug!(
            "Loaded {} items and their info from the cache file",
            self.items.len()
        );
        Ok(())
    }

    fn do_populate(&mut self, api: &Api) -> Result<()> {
        debug!("Getting all item info from the API");
        let all_ids = api.get_all_game_item_ids()?;
        let all_info = api.get_items_info(all_ids)?;
        let mut data = HashMap::new();
        for info in all_info {
            data.insert(info.id, info);
        }
        self.items = data;
        debug!(
            "Loaded {} items and their info from the API",
            self.items.len()
        );
        debug!("Writing items to the cache file");
        fs::write(
            cache_file_path(),
            serde_json::to_string_pretty(&self.items)?,
        )?;
        debug!("Wrote cache file");
        Ok(())
    }

    pub fn populate_cache(&mut self, api: &Api) -> Result<()> {
        if cache_file_path().exists() {
            debug!("Cache file already exists");
            self.load_from_file()?;
            return Ok(());
        }
        debug!("Cache file does not exist");
        self.do_populate(api)?;
        Ok(())
    }

    pub fn lookup(&self, item: &u64) -> &ItemInfo {
        self.items
            .get(item)
            .expect("Could not locate item information")
    }
}
