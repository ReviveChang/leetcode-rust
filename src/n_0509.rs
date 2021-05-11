pub struct Solution;

impl Solution {
    pub fn fib(n: i32) -> i32 {
        if n<3{
            return match n{
                0 => 0,
                _ => 1,
            }
        }

        let mut cas = vec![1,1];
        let mut time = 0;
        while time != n-2 {
            let res:i32 = cas[0]+cas[1];
            cas.swap(0,1);
            cas[1]=res;
            time +=1;
        }
        cas[1]
    }
}