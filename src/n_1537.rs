pub struct Solution;
impl Solution {
    pub fn minimum_time_required(jobs: Vec<i32>, k: i32) -> i32 {
        let n = jobs.len() as i32;
        if (k>=n) {return *jobs.iter().max().unwrap();}
        let mut worker = vec![0;k as usize];
        let mut jobs = jobs;
        jobs.sort_by(|a,b| b.cmp(a));
        for job in jobs{
            *worker.iter_mut().min().unwrap()+=job;
        }
        *worker.iter().max().unwrap()
    }
}