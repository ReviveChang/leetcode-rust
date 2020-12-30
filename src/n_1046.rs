use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut heap= BinaryHeap::from(stones);
        
        loop{
            let a = heap.pop();
            let b = heap.pop();

            match (a,b) {
                (None,_) => return 0,
                (Some(i),None) => return i,
                (Some(i),Some(j)) => heap.push((i-j).abs()),
            }
        }
    }
}

#[cfg(test)]
mod tests{
    use super::Solution;

    #[test]
    fn test(){
        assert_eq!(Solution::last_stone_weight(vec![2,7,4,1,8,1]),1);
    }
}