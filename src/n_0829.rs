pub struct Solution;
// Todo
impl Solution {
    pub fn consecutive_numbers_sum(n: i32) -> i32 {
        let mut res = 1;
        for i in 2..(2f64*n as f64).sqrt() as i32+1{
            if (n - (i*(i-1))/2)%i == 0 {res += 1;} 
        }
        res
    }
}