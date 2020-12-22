pub struct Solution;

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let str = "99".to_string() + &s;
        let mut dp = vec![1; str.len()];
        //println!("{}",str);
        let mut pre = '0';
        for (index, cur) in str.char_indices() {
            if index < 2 {
                continue;
            }
            let mut val = 0;
            if cur > '0' && cur <= '9' {
                val += 1;
            }
            if pre == '1' || (pre == '2' && (cur >= '0' && cur <= '6')) {
                val += 2;
            }

            match val {
                1 => {
                    dp[index] = dp[index - 1];
                }
                2 => {
                    dp[index] = dp[index - 2];
                }
                3 => {
                    dp[index] = dp[index - 1] + dp[index - 2];
                }
                _ => {
                    return 0;
                }
            }
            pre = cur;
            //println!("pre:{},cur:{},index:{},val:{},dp:{}",pre,cur,index,val,dp[index]);
        }
        match dp.last() {
            Some(&i) => i,
            None => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::num_decodings("2101".to_string()), 1);
    }
}
