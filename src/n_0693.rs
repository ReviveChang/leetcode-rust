pub struct Solution;

impl Solution {
    pub fn has_alternating_bits(n: i32) -> bool {
        if n == 0 {return false;}
        let mut max_bit = 0;
        let mut t = n;
        while t != 1 {
            t>>=1;
            max_bit+=1;
        }
        n + (n>>1) + 1== 1<<max_bit+1 
        
    }
}

#[cfg(test)]
mod tests{
    use super::Solution;

    #[test]
    fn test(){
        assert_eq!(Solution::has_alternating_bits(10),true);
    }
}