pub struct Solution;

impl Solution {
    pub fn max_rotate_function(a: Vec<i32>) -> i32 {
        let sum:i32 = a.iter().sum();
        let n = a.len();
        let mut func = a.iter().enumerate().fold(0,|acc,(i,t)| acc+i as i32*t);
        let mut ans = func;

        for &i in a.iter().rev(){
            func += sum - n as i32 * i;
            if func > ans {ans=func;}
        }
        ans
    }
}