pub struct Solution;

impl Solution {
    pub fn restore_matrix(row_sum: Vec<i32>, col_sum: Vec<i32>) -> Vec<Vec<i32>> {
        let mut row_sum = row_sum;
        let mut col_sum = col_sum;
        let (row_size, col_size) = (row_sum.len(), col_sum.len());
        let mut res = vec![vec![0; col_size]; row_size];
        let mut i = 0;
        while i != row_size {
            let mut j = 0;
            while j != col_size {
                let cur = row_sum[i].min(col_sum[j]);
                row_sum[i] -= cur;
                col_sum[j] -= cur;
                res[i][j] = cur;
                j+=1;
            }
            i += 1;
        }
        res
    }
}
