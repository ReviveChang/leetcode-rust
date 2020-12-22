pub struct Solution;

impl Solution {
    pub fn num_ways(n: i32) -> i32 {
        let n = n as usize;
        if n<=1 {return 1;}
        let mut dp = vec![1;n+1];

        let mut ind = 2;
        while ind!=n+1{
            dp[ind] = (dp[ind-1]+dp[ind-2])%1000000007;
            ind += 1;
        }
        if let Some(&i)=dp.last(){
            i
        }else{1}
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::num_ways(92),720754435);
    }
}