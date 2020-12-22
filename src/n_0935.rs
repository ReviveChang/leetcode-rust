pub struct Solution;

impl Solution {
    pub fn knight_dialer(n: i32) -> i32 {
        if n == 1 {return 10;}
        if n == 0 {return 0;}
        let md = 1000000007;
        let mut res:[i64;4] = [1,2,2,4];

        let mut ind = 0;
        while ind != n-1 {
            let tmp = res;
            res[0] = tmp[1];
            res[1] = (tmp[0] * 2 + tmp[3])%md;
            res[2] = tmp[3];
            res[3] = (tmp[1] *2 + tmp[2]*2 )%md;
            ind+=1;
            //println!("{:?}",res);
        }
        ((res[0]+res[1]+res[2]+res[3])%md) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::knight_dialer(500),84202957);
    }
}