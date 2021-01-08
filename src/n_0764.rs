pub struct Solution;

impl Solution {
    pub fn order_of_largest_plus_sign(n: i32, mines: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut dp = vec![vec![vec![1; 4]; n]; n];
        for mine in mines {
            let (r, c) = (mine[0] as usize, mine[1] as usize);
            for ix in 0..4 {
                dp[r][c][ix] = 0;
            }
        }

        let mut i = 1;
        while i != n {
            let mut j = 1;
            while j != n {
                if dp[i][j][0] != 0 {
                    dp[i][j][0] = 1 + dp[i][j - 1][0];
                }
                if dp[i][j][1] != 0 {
                    dp[i][j][1] = 1 + dp[i - 1][j][1];
                }
                if dp[i][n - j - 1][2] != 0 {
                    dp[i][n - j - 1][2] = 1 + dp[i][n - j][2];
                }
                if dp[n - i - 1][j][3] != 0 {
                    dp[n - i - 1][j][3] = 1 + dp[n - i][j][3];
                }
                j += 1;
            }
            i += 1;
        }

        let mut ans = 0;
        let mut i = 0;
        while i != n {
            let mut j = 0;
            while j != n {
                let k = dp[i][j].iter().min().unwrap();
                ans = ans.max(*k);
                j += 1;
            }
            i += 1;
        }
        ans
    }
}
