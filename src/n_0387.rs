pub struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut map = HashMap::new();

        for ch in s.chars() {
            let count = map.entry(ch).or_insert(0);
            *count += 1;
        }

        for (i,ch) in s.char_indices(){
            if *map.get(&ch).unwrap() == 1 { return i as i32;}
        }
        return -1;
    }
}