use std::collections::HashMap;

pub struct Solution;
impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let mut res:Vec<i32> = Vec::new();
        let ps = p.len();
        let mut std = HashMap::new();
        for ch in p.chars(){
            let n = std.entry(ch).or_insert(0);
            *n+=1;
        }
        let mut count = [0;26];
        let mut cvec:Vec<char> = Vec::new();
        cvec.push('a');
        count[0] = 1;
        //println!("std:{:?}",std);
        for (i,ch) in s.char_indices(){
            if !std.contains_key(&ch) {continue;}
            if cvec.len() != ps{
                cvec.push(ch);
                count[ch as usize - 'a' as usize] += 1; 
                continue;
            }
            cvec.push(ch);
            count[ch as usize - 'a' as usize] += 1;
            count[cvec[0] as usize - 'a' as usize] -= 1;
            cvec.remove(0);
            
            let mut flag = true;
            for (&key,&val) in std.iter(){
                if count[key as usize - 'a' as usize] != val { flag = false; break;}
            }

            if flag {res.push(i as i32-ps as i32 +1);}
            
            //println!("index:{},char:{},\ncvec:{:?},\ncount:{:?}",i,ch,cvec,count);
            //println!("Res:{:?}",res);
        }
        res
    }
}

#[cfg(test)]
mod tests{
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::find_anagrams("cbaebabacd".to_string(), "abc".to_string()),vec![0,6]);
    }
}