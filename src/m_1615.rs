pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn master_mind(solution: String, guess: String) -> Vec<i32> {
        let mut m = HashMap::new();
        let mut ans = vec![0, 0];
        for ch in solution.chars() {
            let it = m.entry(ch).or_insert(0);
            *it += 1;
        }

        let mut itt = solution.chars();
        for ch in guess.chars() {
            let ce = itt.next();
            match m.get_mut(&ch) {
                Some(t) => {
                    if *t != 0 {
                        ans[1] += 1;
                        *t -= 1;
                    }
                    if ch == ce.unwrap() {
                        ans[1] -= 1;
                        ans[0] += 1;
                    }
                }
                None => {}
            }
        }
        ans
    }
}
