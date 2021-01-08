pub struct Solution;

use std::collections::BinaryHeap;

impl Solution {
    pub fn maximum_units(box_types: Vec<Vec<i32>>, truck_size: i32) -> i32 {
        let mut prp = BinaryHeap::new();
        for v in box_types{
            for _i in 0..v[0]{
                prp.push(v[1]);
            }
        }

        let mut res = 0;
        for _t in 0..truck_size{
            res += prp.pop().unwrap_or(0);
        }

        res
    }
}

#[cfg(test)]
mod tests{
    use super::Solution;

    #[test]
    fn test(){
        assert_eq!(Solution::maximum_units(vec![vec![1,3],vec![2,2],vec![3,1]], 4),8);
    }
}