use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    h.into_iter().fold(BTreeMap::new(),|mut acc, (&val, keys)|
              {acc.extend(keys.iter().map(|&key| (key.to_ascii_lowercase(), val)));acc})
}
