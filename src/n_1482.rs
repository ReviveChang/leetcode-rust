struct Solution;

impl Solution {
    pub fn min_days(bloom_day: Vec<i32>, m: i32, k: i32) -> i32 {
        fn check(bloom_day:&Vec<i32>, thr: i32, m:i32, k:i32) -> bool {
            let arr:Vec<bool> = bloom_day.iter().map(|&i| i <= thr).collect();
            let cur = arr.split(|&b| b==false).fold(0,|acc, sp| acc+(sp.len() as i32 / k));

            cur >= m
        }
        let n = bloom_day.len() as i32;
        if m*k > n {
            return -1;
        }
        
        let mut bloom = bloom_day.clone();
        bloom.sort_unstable();

        let mut lo = 0;
        let mut hi = n-1;

        while lo < hi {
            let mi = lo + (hi-lo) / 2;
            if !check(&bloom_day, bloom[mi as usize],m,k) {
                lo = mi + 1;
            }else{
                hi = mi;
            }
        }
        bloom[lo as usize]
    }
}