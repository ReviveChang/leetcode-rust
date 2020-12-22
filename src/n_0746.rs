pub struct Solution;

impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let mut dp = vec![0; cost.len() + 1];

        let mut index = 2;
        while index != dp.len() {
            let first = dp[index - 2] + cost[index - 2];
            let second = dp[index - 1] + cost[index - 1];
            dp[index] = if first > second { second } else { first };
            index += 1;
        }
        return match dp.last() {
            Some(i) => *i,
            None => 0,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::min_cost_climbing_stairs(vec![10, 15, 20]), 15);
        assert_eq!(
            Solution::min_cost_climbing_stairs(vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1]),
            6
        );
    }
}
