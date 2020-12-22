pub struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut dp = vec![vec![0;prices.len()];2];
        
        dp[0][0] = 0;
        dp[1][0] = -prices[0];
        let mut index = 1;
        while index != prices.len() {
            dp[0][index] = std::cmp::max(dp[0][index-1],dp[1][index-1]+prices[index]);
            dp[1][index] = std::cmp::max(dp[1][index-1],dp[0][index-1]-prices[index]);
            //println!("{},{}",dp[0][index],dp[1][index]);
            index += 1;
        }

        dp[0][index-1]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::max_profit(vec![7,1,5,3,6,4]),7);
    }
}