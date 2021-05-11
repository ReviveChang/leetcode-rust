pub struct Solution;

impl Solution {
    pub fn find_numbers(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        for i in nums{
            if i.to_string().len() &1 == 0 {
                ans +=1;
            }
        }
        ans
    }
}