use crate::models::FullItem;

// TODO improve
// note that I also may want to just switch to a table; have to see
fn for_display(item: &FullItem) -> String {
    format!("{} (x{})", item.name, item.count)
}

pub fn filter(items: &[FullItem], page: usize, page_size: u16) -> Vec<String> {
    // variable setup
    let page_size = page_size as usize;
    let mut ret = Vec::new();

    // TODO filter by user input
    // ...

    // pagination
    for item in items.iter().skip(page * page_size).take(page_size) {
        ret.push(for_display(&item));
    }
    ret
}
