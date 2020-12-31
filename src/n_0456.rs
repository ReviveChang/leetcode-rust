pub struct Solution;

impl Solution {
    pub fn find132pattern(nums: Vec<i32>) -> bool {
        if nums.len() < 3 { return false; }
        let mut min = vec![0;nums.len()];
        let mut min_v = nums[0];
        let mut dec:Vec<i32> = Vec::new();
        
        let mut mi = 0;
        while mi != nums.len(){
            if nums[mi] < min_v {
                min_v = nums[mi];
            }
            min[mi] = min_v;
            mi += 1;
        }
        
        let mut di = nums.len() -1 ;
        loop {
            match dec.last(){
                Some(&i) => {
                    if i >= nums[di] {dec.push(nums[di]);}
                    else if i <= min[di-1] {dec.pop();continue;}
                    else { return true; }
                },
                None => dec.push(nums[di]),
            }

            if di == 1 {break;}
            di-=1;
        }
        false
    }
}

#[cfg(test)]
mod tests{
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::find132pattern(vec![3, 1, 4, 2]),true);
    }
}