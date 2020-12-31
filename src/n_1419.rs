pub struct Solution;

impl Solution {
    pub fn min_number_of_frogs(croak_of_frogs: String) -> i32 {
        let mut croak = [0, 0, 0, 0, 0];
        let mut res = 0;
        for ch in croak_of_frogs.chars() {
            match ch {
                'c' => {
                    croak[0] += 1;
                    let summary = croak.iter().sum::<i32>();
                    if  summary > res { res = summary;}
                },
                'r' => {
                    if croak[0] == 0 {
                        return -1;
                    }
                    croak[0] -= 1;
                    croak[1] += 1;
                }
                'o' => {
                    if croak[1] == 0 {
                        return -1;
                    }
                    croak[1] -= 1;
                    croak[2] += 1;
                }
                'a' => {
                    if croak[2] == 0 {
                        return -1;
                    }
                    croak[2] -= 1;
                    croak[3] += 1;
                }
                'k' => {
                    if croak[3] == 0 {
                        return -1;
                    }
                    croak[3] -= 1;
                },
                _ => return -1,
            }
        }
        if croak.iter().sum::<i32>() == 0 {
            res
        } else {
            -1
        }
    }
}
