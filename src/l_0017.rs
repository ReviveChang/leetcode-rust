pub struct Solution;

impl Solution {
    pub fn calculate(s: String) -> i32 {
        1 << (s.len() as i32) 
    }
}


#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::calculate("ABAABBBA".to_string()), 256);
    }
}