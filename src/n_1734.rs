struct Solution;

impl Solution {
    pub fn decode(encoded: Vec<i32>) -> Vec<i32> {
        let tot = encoded.iter().fold(0,|acc, &i| acc ^ i);
        let odd = encoded.iter()
            .enumerate()
            .filter(|(idx, &i)| idx & 1 == 1)
            .map(|(idx, &i)| i)
            .fold(0, |acc, i| acc ^ i);
            
        let mut first = tot ^ odd;
        let mut ans = vec![first];
        for &i in encoded.iter(){
            first = i ^ first;
            ans.push(first);
        }
        ans
    }
}

#[test]
fn test_1734() {
    assert_eq!(Solution::decode(vec![6,5,4,6]),vec![2,4,1,5,3]);
}