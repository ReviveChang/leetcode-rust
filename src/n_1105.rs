pub struct Solution;

impl Solution {
    pub fn min_height_shelves(books: Vec<Vec<i32>>, shelf_width: i32) -> i32 {
        let mut dp = vec![0; books.len()];
        dp[0] = books[0][1];

        let mut ind = 1;
        while ind != books.len() {
            let mut cur = ind - 1;
            let mut cur_wid = books[ind][0];
            let mut lahe = books[ind][1];
            let mut mindp = lahe + dp[ind - 1];
            loop {
                if cur_wid + books[cur][0] > shelf_width {
                    break;
                }
                cur_wid += books[cur][0];
                lahe = lahe.max(books[cur][1]);
                if cur == 0 {
                    mindp = if lahe < mindp { lahe } else { mindp };
                } else {
                    mindp = if lahe + dp[cur - 1] < mindp {
                        lahe + dp[cur - 1]
                    } else {
                        mindp
                    };
                }
                if cur == 0 {
                    break;
                }
                cur -= 1;
            }
            dp[ind] = mindp;
            //println!("{:?}", dp);
            ind += 1;
        }
        if let Some(&i) = dp.last() {
            i
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn testing() {
        assert_eq!(
            Solution::min_height_shelves(
                vec![
                    vec![1, 1],
                    vec![2, 3],
                    vec![2, 3],
                    vec![1, 1],
                    vec![1, 1],
                    vec![1, 1],
                    vec![1, 2]
                ],
                4
            ),
            6
        );
    }
}
