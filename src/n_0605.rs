pub struct Solution;

impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        let mut flower = flowerbed;
        let mut n = n;
        flower.insert(0,0);
        flower.push(0);

        let mut i = 1;
        while i != flower.len()-1{
            if flower[i] == 0 && flower[i-1] == 0 && flower[i+1]==0{
                flower[i]=1;
                n -= 1;
            }
            i+=1;
        }

        n <= 0
    }
}

#[cfg(test)]
mod tests{
    use super::Solution;

    #[test]
    fn test(){
        assert_eq!(Solution::can_place_flowers(vec![1,0,0,0,1], 1),true);
    }
}