pub struct Solution;

impl Solution {
    pub fn erase_overlap_intervals(intervals: Vec<Vec<i32>>) -> i32 {
        let mut res = 0;
        let mut limit:Option<i32> = None;
        let mut intervals = intervals;
        intervals.sort_by(|a,b| a[1].cmp(&b[1]));
        //println!("{:?}",intervals);
        for intv in intervals{
            if let Some(i) = limit {
                if intv[0] >= i {
                    limit = Some(intv[1]);
                }else{
                    res += 1;
                }
            }else{
                limit = Some(intv[1]);
                continue;
            }
        }
        res
    }
}

#[cfg(test)]
mod tests{
    use super::Solution;

    #[test]
    fn test(){
        assert_eq!(Solution::erase_overlap_intervals(vec![ vec![1,2], vec![2,3], vec![3,4], vec![1,3] ]),1);
    }
}