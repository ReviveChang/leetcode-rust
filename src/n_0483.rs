pub struct Solution;

impl Solution {
    pub fn smallest_good_base(n: String) -> String {
        let num = n.parse::<i64>().unwrap();
        let mut up = 59;
        while up != 1{
            let pn = (num as f64).powf(1f64/up as f64);
            //println!("{}",pn);
            let k = pn.floor() as i64;
            if k ==1 {up-=1; continue;}
            //println!("{} vs {}", (k.pow(up+1)-1)/(k-1), num);
            if (k-1)*num+1 == k.pow(up+1) { return k.to_string();}
            up-=1;
        }

        return (num-1).to_string();
    }
}

#[cfg(test)]

mod tests{
    use super::Solution;
    #[test]
    fn test(){
        assert_eq!(Solution::smallest_good_base("13".to_string()),"3".to_string());
    }
}