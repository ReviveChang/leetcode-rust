pub struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let size = prices.len();
        match size {
            0 | 1 => return 0,
            2 => return 0i32.max(prices[1]-prices[0]),
            3 => return 0i32.max(prices[1]-prices[0]).max(prices[2]-prices[0]).max(prices[2]-prices[1]),
            _ => {},
        }
        let mut state_ind = 1;
        let mut dp = vec![vec![0;size];5];

        dp[1][0] = -prices[0];
        dp[2][1] = prices[1]-prices[0];
        dp[3][2] = prices[1]-prices[0]-prices[2];
        dp[4][3] = prices[3]-prices[2]+prices[1]-prices[0];

        while state_ind != 5{
            let mut ind = state_ind ;
            while ind != size {
                dp[state_ind][ind] = match state_ind%2 {
                    0 => dp[state_ind][ind-1].max(dp[state_ind-1][ind-1]+prices[ind]),
                    _ => dp[state_ind][ind-1].max(dp[state_ind-1][ind-1]-prices[ind]),
                };
                ind += 1;
            }
            state_ind += 1;
        }

        dp[2][size-1].max(dp[4][size-1]).max(0i32)
    }
}
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::max_profit(vec![3,3,5,0,0,3,1,4]),6);
    }
}