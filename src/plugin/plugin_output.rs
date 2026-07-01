use crate::utils::{ScriptItem, ScriptItemType, ScriptResult};
use xxhash_rust::xxh3::Xxh3;

pub fn hash_output(result: &ScriptResult) -> u64 {
    let mut hasher = Xxh3::new();
    hash_str(&mut hasher, &result.title);
    hash_items(&mut hasher, &result.items);
    hasher.digest()
}

pub fn notification_hash(items: &[ScriptItem]) -> Option<u64> {
    if items.is_empty() {
        return None;
    }
    let mut hasher = Xxh3::new();
    hash_items(&mut hasher, items);
    Some(hasher.digest())
}

fn hash_items(hasher: &mut Xxh3, items: &[ScriptItem]) {
    for_each_item(items, &mut |item| {
        let Some(item) = item else {
            hasher.update(&[0]);
            return;
        };
        hasher.update(&[item_type_id(&item.item_type)]);
        if let Some(title) = &item.title {
            hash_str(hasher, title);
        }
        if let Some(path) = &item.path {
            hash_str(hasher, path);
        }
        hasher.update(&[0]);
    });
}

fn hash_str(hasher: &mut Xxh3, value: &str) {
    hasher.update(&value.len().to_ne_bytes());
    hasher.update(value.as_bytes());
}

fn item_type_id(item_type: &ScriptItemType) -> u8 {
    match item_type {
        ScriptItemType::Link => 1,
        ScriptItemType::Submenu => 2,
        ScriptItemType::Divider => 3,
        ScriptItemType::Text => 4,
    }
}

fn for_each_item<F: FnMut(Option<&ScriptItem>)>(items: &[ScriptItem], f: &mut F) {
    for item in items {
        f(Some(item));
        if let Some(children) = &item.items {
            for_each_item(children, f);
        }
    }
    f(None);
}
