use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize)]
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

#[derive(Clone, Debug, Deserialize)]
pub struct Bag {
    pub id: u64,
    pub size: u64,
    pub inventory: Vec<Option<InventorySlot>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Inventory {
    pub bags: Vec<Bag>,
}

impl Inventory {
    pub fn all_content(&self) -> Vec<InventorySlot> {
        self.bags
            .iter()
            .flat_map(|bag| bag.inventory.clone()) // find a way to do this without 'clone'
            .filter_map(|slot| slot)
            .collect()
    }
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

// TODO improvements
pub fn for_display(slot: &InventorySlot, info: &ItemInfo) -> String {
    format!("{} (x{})", info.name, slot.count)
}
