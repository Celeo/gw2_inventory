use crate::models::*;
use anyhow::Result;
use log::debug;
use reqwest::{blocking::Client, header};
use std::collections::HashMap;

const API_ROOT: &str = "https://api.guildwars2.com/v2/";

pub struct Api {
    client: Client,
}

impl Api {
    pub fn new(api_key: &str) -> Result<Self> {
        let headers = {
            let mut headers = header::HeaderMap::new();
            headers.insert(
                header::AUTHORIZATION,
                header::HeaderValue::from_str(&format!("Bearer {}", api_key))?,
            );
            headers
        };
        Ok(Self {
            client: Client::builder().default_headers(headers).build()?,
        })
    }

    fn get_character_names(&self) -> Result<Vec<String>> {
        debug!("Getting character names");
        let resp = self.client.get(&format!("{}characters", API_ROOT)).send()?;
        debug!("Response code: {}", resp.status());
        let data = resp.json()?;
        Ok(data)
    }

    fn get_character_inventory(&self, name: &str) -> Result<Inventory> {
        debug!("Getting character inventory for: {}", name);
        let resp = self
            .client
            .get(&format!("{}characters/{}/inventory", API_ROOT, name))
            .send()?;
        debug!("Response code: {}", resp.status());
        let inventory = resp.json()?;
        Ok(inventory)
    }

    pub fn get_all_inventories(&self) -> Result<HashMap<String, Inventory>> {
        let mut inventories = HashMap::new();
        let names = self.get_character_names()?;
        for name in &names {
            inventories.insert(name.to_owned(), self.get_character_inventory(&name)?);
        }
        Ok(inventories)
    }

    pub fn get_all_game_item_ids(&self) -> Result<Vec<u64>> {
        debug!("Getting all game item ids");
        let resp = self.client.get(&format!("{}/items", API_ROOT)).send()?;
        debug!("Response code: {}", resp.status());
        let ids = resp.json()?;
        Ok(ids)
    }

    pub fn get_items_info(&self, ids: Vec<u64>) -> Result<Vec<ItemInfo>> {
        debug!("Getting item info for {} item ids", ids.len());
        let mut data = Vec::new();

        let mut processed = 0;
        loop {
            debug!(
                "Getting batch {} of {} of items from the API",
                processed / 200 + 1,
                ids.len() / 200
            );
            let ids_batch: Vec<_> = ids.iter().skip(processed).take(200).collect();
            processed += 200;
            let resp = self
                .client
                .get(&format!(
                    "{}/items?ids={}",
                    API_ROOT,
                    ids_batch
                        .iter()
                        .map(|n| n.to_string())
                        .collect::<Vec<_>>()
                        .join(",")
                ))
                .send()?;
            debug!("Response code: {}", resp.status());
            let mut segment: Vec<_> = resp.json()?;
            data.append(&mut segment);
            if processed >= ids.len() {
                break;
            }
        }

        Ok(data)
    }
}
