pub struct Solution;

impl Solution {
    pub fn max_depth(s: String) -> i32 {
        let (mut cur,mut max) = (0,0);
        for ch in s.chars(){
            match ch {
                '(' => cur += 1,
                ')' => cur -= 1,
                _ => {},
            }
            if cur > max { max = cur;}
        }
        max
    }
}

#[cfg(test)]
mod tests{
    use super::Solution;

    #[test]
    fn test(){
        assert_eq!(Solution::max_depth("(1+(2*3)+((8)/4))+1".to_string()),3);
    }
}