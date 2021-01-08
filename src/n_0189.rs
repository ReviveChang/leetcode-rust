pub struct Solution;

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let l = nums.len();
        nums.rotate_right(k as usize % l)
    }
}