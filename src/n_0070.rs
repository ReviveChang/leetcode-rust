pub struct Solution;

impl Solution {
    pub fn mat_mul(mat:[i32;4], res:[i32;4]) -> [i32;4]{
        [mat[0]*res[0]+mat[1]*res[2],mat[0]*res[1]+mat[1]*res[3],
                mat[2]*res[0]+mat[3]*res[2],mat[2]*res[1]+mat[3]*res[3]]
    }
    pub fn climb_stairs(n: i32) -> i32 {
        let mut mat = [1,1,1,0];
        let mut res = [1,0,0,1];
        let mut n = n-1;
        while n>0{
            if n&1 == 1{
                res = Solution::mat_mul(res, mat);
                if n==1 {break;}
            }
            
            let temp2 = Solution::mat_mul(mat, mat);
            mat = temp2;
            n = n >> 1;
            //println!("{},{:?}",n,mat)
        }
        //println!("{}",res[0]+res[1]);
        res[0]+res[1]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::climb_stairs(43),701408733);
    }
}
