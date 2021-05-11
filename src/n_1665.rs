pub struct Solution;

impl Solution {
    pub fn minimum_effort(tasks: Vec<Vec<i32>>) -> i32 {
        let mut tasks = tasks;
        tasks.sort_by_key(|x| x[0]-x[1]);
        tasks.into_iter().fold(0,|acc,x| x[1].max(acc+x[0]))
    }
}