use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct InventorySlot {
    pub id: u64,
    pub count: u8,
    pub binding: Option<String>,
    pub bound_to: Option<String>,
    #[serde(default)]
    pub infusions: Vec<u64>,
    #[serde(default)]
    pub upgrades: Vec<u64>,
    pub skin: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct Bag {
    pub id: u64,
    pub size: u64,
    pub inventory: Vec<Option<InventorySlot>>,
}

#[derive(Debug, Deserialize)]
pub struct Inventory {
    pub bags: Vec<Bag>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ItemInfo {
    pub id: u64,
    pub name: String,
    pub description: Option<String>,
    #[serde(rename = "type")]
    pub item_type: String,
    pub level: u8,
    pub rarity: String,
    pub icon: Option<String>,
}
