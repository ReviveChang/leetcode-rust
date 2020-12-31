
pub struct Solution;

use std::collections::HashSet;


impl Solution {
    pub fn repeated_n_times(a: Vec<i32>) -> i32 {
        let mut set = HashSet::new();
        for i in a{
            if !set.insert(i) {return i;} 
        }
        1
    }
}

#[cfg(test)]
mod tests{
    use super::Solution;

    #[test]
    fn test(){
        assert_eq!(Solution::repeated_n_times(vec![1,2,3,3]),3);
    }
}