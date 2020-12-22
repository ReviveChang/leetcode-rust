pub struct Solution;

impl Solution {
    pub fn expressive_words(s: String, words: Vec<String>) -> i32 {
        let mut vref: Vec<i32> = Vec::new();
        let mut cref: Vec<char> = Vec::new();
        let mut res = 0;
        let mut prev = ' ';
        //println!("{}",s);
        for ch in s.chars() {
            if ch != prev {
                cref.push(ch);
                vref.push(1);
            } else {
                match vref.last_mut() {
                    Some(p) => *p += 1,
                    _ => {}
                }
            }
            prev = ch;
        }
        //println!("{:?}\n{:?}", vref, cref);
        for word in words {
            let mut cur_ind = 0;
            let mut cur_val = 1;
            let mut prev = ' ';
            let mut flagval = 0;

            //println!("{}",word);
            for (i, c) in word.char_indices() {
                
                if c != prev {
                    if i != 0 {
                        //println!("{} vs {}", cur_val, vref[cur_ind]);
                        if (vref[cur_ind] != cur_val && vref[cur_ind] < 3) || cur_val > vref[cur_ind]{
                            flagval = -1;
                            break;
                        }
                        
                        cur_ind += 1;
                    }

                    if cur_ind == vref.len() {
                        flagval = -2;
                        break;
                    }
                    if cref[cur_ind] != c {
                        flagval = -3;
                        break;
                    }
                    cur_val = 1;
                } else {
                    cur_val += 1;
                }
                prev = c;
            }
            if flagval < 0 {
                //println!("Fail at {},{}", cur_ind, flagval);
                continue;
            }
            //println!("{} vs {}", cur_val, vref[cur_ind]);
            if cur_ind == vref.len() - 1
                && (vref[cur_ind] == cur_val || vref[cur_ind] >= 3) && cur_val <= vref[cur_ind]
            {
                res += 1;
            //println!("Success");
            } else {
                //println!("Fail at {},-4", cur_ind);
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(
            Solution::expressive_words(
                "dddiiiinnssssssoooo".to_string(),
                vec![
                    "dinnssoo".to_string(),
                    "ddinso".to_string(),
                    "ddiinnso".to_string(),
                    "ddiinnssoo".to_string(),
                    "ddiinso".to_string(),
                    "dinsoo".to_string(),
                    "ddiinsso".to_string(),
                    "dinssoo".to_string(),
                    "dinso".to_string()
                ]
            ),
            3
        );
    }
}
