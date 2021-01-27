use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct InventorySlot {
    pub id: u64,
    pub count: u64,
    pub binding: Option<String>,
    pub bound_to: Option<String>,
    #[serde(default)]
    pub infusions: Vec<u64>,
    #[serde(default)]
    pub upgrades: Vec<u64>,
    pub skin: Option<u64>,
}

impl InventorySlot {
    pub fn same_item(&self, other: &InventorySlot) -> bool {
        if self == other {
            return true;
        }
        self.id == other.id
            && self.binding == other.binding
            && self.bound_to == other.bound_to
            && self.infusions == other.infusions
            && self.upgrades == other.upgrades
            && self.skin == other.skin
    }
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

#[derive(Debug)]
pub struct FullItem {
    pub id: u64,
    pub count: u64,
    pub name: String,
    pub description: Option<String>,
    pub item_type: String,
    pub character: String,
}

impl FullItem {
    pub fn new(slot: &InventorySlot, info: &ItemInfo, character: &str) -> Self {
        Self {
            id: info.id,
            count: slot.count,
            name: info.name.clone(),
            description: info.description.clone(),
            item_type: info.item_type.clone(),
            character: character.to_string(),
        }
    }
}
