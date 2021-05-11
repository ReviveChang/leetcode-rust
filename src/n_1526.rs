pub struct Solution;

impl Solution {
    pub fn min_number_operations(target: Vec<i32>) -> i32 {
        let mut ans = target[0];
        let n = target.len();
        let mut i = 1;
        while i != n{
            ans += (target[i]-target[i-1]).max(0);
            i+=1;
        }
        return ans;
    }
}