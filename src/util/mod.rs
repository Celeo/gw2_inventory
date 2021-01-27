use crate::{
    cache::Cache,
    models::{FullItem, Inventory},
};
use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use std::collections::HashMap;

pub fn process_inventories(
    inventories: &HashMap<String, Inventory>,
    cache: &Cache,
) -> Vec<FullItem> {
    let mut items = Vec::new();

    for (character, inventory) in inventories {
        let slots = inventory.all_content();

        // combine like items
        let mut dedup = Vec::new();
        for slot in slots {
            let mut found = false;
            for already in dedup.iter_mut() {
                if slot.same_item(already) {
                    already.count += slot.count;
                    found = true;
                    break;
                }
            }
            if !found {
                dedup.push(slot);
            }
        }

        // convert to 'FullItem'
        for item in dedup {
            let info = cache.lookup(&item.id);
            items.push(FullItem::new(&item, info, character));
        }
    }

    // sort alphabetically
    items.sort_by_key(|item| item.name.clone());

    items
}

pub fn filter<'a>(items: &'a [FullItem], user_input: &str) -> Vec<&'a FullItem> {
    let matcher = SkimMatcherV2::default();
    items
        .iter()
        .filter(|&item| matcher.fuzzy_match(&item.name, user_input).is_some())
        .collect()
}

pub fn paginate(items: &[FullItem], page: usize, page_size: u16) -> Vec<&FullItem> {
    let page_size = page_size as usize;
    items
        .iter()
        .skip(page * page_size)
        .take(page_size)
        .collect()
}
