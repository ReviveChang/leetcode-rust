pub struct Solution;

use std::collections::{HashMap};

impl Solution {
    pub fn find_longest_subarray(array: Vec<String>) -> Vec<String> {
        let mut sum = 0i32;
        let mut map = HashMap::new();
        map.insert(0i32,-1i32);
        let mut max = 0;
        let mut loc = (0usize,0usize);
        for (i,st) in array.iter().enumerate(){
            if st.chars().next().unwrap().is_ascii_digit() {sum+=1;} else {sum-=1;}
            let cur = map.entry(sum).or_insert(i as i32);
            if i as i32 - *cur > max{
                max = i as i32 - *cur;
                loc = ((*cur+1) as usize,i+1);
            }
        }
        array[loc.0 .. loc.1].to_vec()
    }
}