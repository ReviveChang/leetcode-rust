pub struct Solution;
use std::collections::HashMap;

impl Solution {
    pub fn get_folder_names(names: Vec<String>) -> Vec<String> {
        let mut d = HashMap::new();
        names.into_iter().map(|name| {
            let mut s = name.clone();
            while d.contains_key(&s) {
                s = format!("{}({})", name, d[&name]);
                *d.get_mut(&name).unwrap() += 1;
            }
            d.insert(s.clone(), 1);
            s
        }).collect()
    }
}