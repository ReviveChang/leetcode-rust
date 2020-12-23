pub struct Solution;

impl Solution {
    pub fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let mut res:Vec<i32> = Vec::new();
        let mut col:usize = 0;
        let mut row:usize;
        while col != mat[0].len(){
            row = 0;
            while row != mat.len(){
                if res.contains(&(row as i32)) {row+= 1; continue;}
                if mat[row][col] == 0 {
                    //println!("col:{}, row:{} HIT!",col,row);
                    res.push(row as i32);
                    if res.len() == k as usize {return res;}
                }
                row+=1;
            }

            col+=1;
        }
        row = 0;
        while row != mat.len(){
            if res.contains(&(row as i32)) {row += 1; continue;}
            res.push(row as i32);
            if res.len() == k as usize {return res;}
            row +=1;
        }
        return res;
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::k_weakest_rows(vec![vec![1,1,0,0,0],
            vec![1,1,1,1,0],vec![1,0,0,0,0],vec![1,1,0,0,0],vec![1,1,1,1,1]],3)
            ,vec![2,0,3]);
    }
}
