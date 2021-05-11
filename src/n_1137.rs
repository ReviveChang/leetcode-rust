pub struct Solution;

impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        if n<3{
            return match n{
                0 => 0,
                _ => 1,
            }
        }

        let mut cas = vec![0,1,1];
        let mut time = 0;
        while time != n-3 {
            let res:i32 = cas.iter().sum();
            cas.remove(0);
            cas.push(res);
            time +=1;
        }
        cas[2]
    }
}