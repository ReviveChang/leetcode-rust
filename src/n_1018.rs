pub struct Solution;

impl Solution {
    pub fn prefixes_div_by5(a: Vec<i32>) -> Vec<bool> {
        let mut suff = 0;
        let mut ans = Vec::new();
        for b in a{
            suff = suff *2 +b;
            suff %= 5;
            if suff == 0{
                ans.push(true);
            }else {ans.push(false);}
        }
        ans
    }
}