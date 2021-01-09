pub struct Solution;
impl Solution {
    pub fn total_money(n: i32) -> i32 {
        let i = n / 7;
        let m = n % 7;

        i*21+7*(i+1)*i/2 + (2*i+m+1)*m/2
    }

}