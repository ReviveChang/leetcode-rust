pub struct Solution;

impl Solution {
    pub fn count_squares(matrix: Vec<Vec<i32>>) -> i32 {
        let mut matrix = matrix;
        let mut res = 0;

        let mut i = 0;
        println!("{},{}",matrix.len(),matrix[0].len());
        while i != matrix.len() {
            let mut j = 0;
            while j != matrix[0].len() {

                if i !=0 &&j !=0 && matrix[i][j] != 0 {
                    matrix[i][j] += matrix[i-1][j-1].min(matrix[i][j-1]).min(matrix[i-1][j]);
                }
                res += matrix[i][j];
                println!("{},{},{:?}",i,j,matrix);
                j+=1;
            }
            println!("{},{},{:?}",i,j,matrix);
            i+=1;
        }
        res
    }
}

#[cfg(test)]
mod tests{
    use super::Solution;

    #[test]
    fn testing(){
        assert_eq!(Solution::count_squares(vec![vec![0,1,1,1],vec![1,1,1,1],vec![0,1,1,1]]),15);
    }
}