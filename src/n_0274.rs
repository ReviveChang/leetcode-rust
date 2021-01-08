pub struct Solution;
use std::cmp::Reverse;

use std::collections::BinaryHeap;
impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let mut heap:BinaryHeap<_> = citations.into_iter().map(Reverse).collect();

        loop{
            if let Some(Reverse(i)) = heap.pop(){
                let len = heap.len() as i32 +1;
                if i>=len{
                    return len;
                }
            }else{
                return 0;
            }
        }
    }
}