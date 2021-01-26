use crate::{
    cache::Cache,
    models::{InventorySlot, ItemInfo},
};

// TODO improvements
fn for_display(slot: &InventorySlot, info: &ItemInfo) -> String {
    format!("{} (x{})", info.name, slot.count)
}

// TODO filtering and pagination
// TODO combine all items of the same type per character into a single "stack"
pub fn filter(slots: &[InventorySlot], cache: &Cache, page: usize, page_size: u16) -> Vec<String> {
    let page_size = page_size as usize;
    let mut ret = Vec::new();

    for slot in slots.iter().skip(page * page_size).take(page_size) {
        let info = cache.lookup(&slot.id);
        ret.push(for_display(&slot, info));
    }
    ret
}
