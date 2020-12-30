pub struct Solution;

impl Solution {
    pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
        let mut ivec = vec![0;101];

        for i in &nums{
            ivec[*i as usize]+=1;
        }
        
        println!("{:?}",ivec.get(0..11).unwrap());
        let mut ind = 1;
        let mut tmp =ivec[0];
        ivec[0] = 0;
        while ind != ivec.len(){     
            let tmpb = ivec[ind]; 
            ivec[ind] = ivec[ind-1]+tmp;
            tmp = tmpb;
            ind+=1;         
        }

        let mut res:Vec<i32> = Vec::new();
        for i in &nums{
            res.push(ivec[*i as usize]);
        }
        res
    }
}

#[cfg(test)]
mod tests{
    use super::Solution;

    #[test]
    fn test(){
        assert_eq!(Solution::smaller_numbers_than_current(vec![5,0,10,0,10,6]),vec![2,0,4,0,4,3]);
    }
}