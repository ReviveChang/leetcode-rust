pub struct Solution;
impl Solution {
    pub fn which_is_min(n1:i32,n2:i32,n3:i32) ->usize{
        if n1<=n2 && n1<=n3 {0}
        else if n2<=n1 && n2<= n3 {1}
        else {2}
    }
    pub fn nth_ugly_number(n: i32) -> i32 {
        let val = [2,3,5];
        let mut point = [0,0,0];
        
        let mut ivec = vec![1];

        while ivec.len() != n as usize{
            let ind = Solution::which_is_min(val[0]*ivec[point[0]], val[1]*ivec[point[1]], val[2]*ivec[point[2]]);
            //println!("In {} {} {}, Min: {}",ivec[point[0]],ivec[point[1]],ivec[point[2]],ivec[point[ind]]);
            if let Some(&i) = ivec.last() {
                let mu = val[ind]*ivec[point[ind]];
                if  mu > i {ivec.push(mu);}
                point[ind]+=1;
            }else{}
            //println!("{:?}",ivec);
        }
        if let Some(&i) = ivec.last(){
            i
        }else{0}
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::nth_ugly_number(1690),2123366400);
    }
}