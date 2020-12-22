pub struct Solution;

impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        if n < 3 { return 0; }

        let mut res = 0;
        let mut cur = 2;
        while cur != n{
            if Solution::is_prime(cur) { res += 1;}
            cur += 1;
        }
        return res;
    }
    pub fn is_prime(x:i32) -> bool {
        let mut cur = 2;
        while x >= cur*cur {
            if x%cur == 0 { return false; }
            cur += 1;
        }
        return true;
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::count_primes(10), 4);
    }
}
