use std::collections::BTreeSet;

pub fn _unique_items<T>(items: impl Iterator<Item = T>) -> Vec<String>
where
    T: AsRef<str>,
{
    let mut set = BTreeSet::new();
    items
        .filter_map(|id| {
            let id = id.as_ref().trim();
            if id.is_empty() {
                None
            } else {
                Some(id.to_string())
            }
        })
        .for_each(|id| {
            set.insert(id);
        });

    set.into_iter().collect()
}

pub fn unique_items<I, T>(items: I) -> Vec<String>
where
    I: Iterator<Item = T>,
    T: AsRef<str>,
{
    let mut items = items
        .filter_map(|id| {
            let id = id.as_ref().trim();
            if id.is_empty() {
                None
            } else {
                Some(id.to_string())
            }
        })
        .collect::<Vec<_>>();

    items.sort();
    items.dedup();

    items
}

/// Example usage
pub fn main() {
    let product_ids = vec![
        "abc123".to_string(),
        "  ".to_string(),
        "def456".to_string(),
        "abc123".to_string(),
        "ghi789".to_string(),
        "ghi789".to_string(),
        "   def456".to_string(),
    ];

    let unique_ids = unique_items(product_ids.into_iter());
    assert_eq!(unique_ids, vec!["abc123", "def456", "ghi789"]);
}
