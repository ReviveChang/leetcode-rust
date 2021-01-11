pub struct Solution;

impl Solution {
    pub fn minimum_abs_difference(arr: Vec<i32>) -> Vec<Vec<i32>> {
        let mut arr = arr;
        arr.sort_unstable();

        let mut min_abs_diff = std::i32::MAX;
        let mut ans = Vec::new();
        let mut iter = arr.iter().peekable();
        while let Some(&i) = iter.next(){
            if let Some(&&j) = iter.peek(){
                if j-i > min_abs_diff {
                    min_abs_diff = j-i;
                    ans.clear();
                    ans.push(vec![i,j]);
                }else if j-i == min_abs_diff{
                    ans.push(vec![i,j]);
                }
            }
        }
        ans
    }
}