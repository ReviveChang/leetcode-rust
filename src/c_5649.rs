pub struct Solution;


impl Solution {
    pub fn decode(encoded: Vec<i32>, first: i32) -> Vec<i32> {
        let mut cur = first;
        let mut ans = vec![first];
        for n in encoded{
            ans.push(cur ^ n);
            cur = cur ^n;
        }
        ans
    }
}