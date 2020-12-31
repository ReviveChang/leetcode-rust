pub struct Solution;

impl Solution {
    pub fn count_prime_set_bits(l: i32, r: i32) -> i32 {
        let mut res = 0;
        let dict = vec![2, 3, 5, 7, 11, 13, 17, 19];
        for i in l..r+1 {
            let n = i.count_ones();
            if dict.contains(&n) {res += 1;}
        }
        res
    }
}

#[cfg(test)]
mod tests{
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::count_prime_set_bits(6, 10),4);
    }
}