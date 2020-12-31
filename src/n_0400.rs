pub struct Solution;

impl Solution {
    pub fn find_nth_digit(n: i32) -> i32 {
        let n = n-1;
        let mut dic = [0;9];
        let mut div = [1;9];
        let mut i = 1;
        while i != 9{
            dic[i] = 9*10i32.pow((i-1) as u32 )*i as i32 + dic[i-1];
            div[i] = 10i32.pow(i as u32);
            i+=1;
        }
        let mut prefix = 0;
        while prefix !=8 && n>dic[prefix+1] {prefix+=1;}
        let suffix = n - dic[prefix];

        let num = suffix / (prefix+1) as i32;
        let ch_ind = (suffix % (prefix+1) as i32) as i32;

        let num = num + div[prefix];

        let word:Vec<char> = num.to_string().chars().collect();
        
        //println!("{:?}, ind:{}",word,ch_ind);
        word[ch_ind as usize].to_digit(10).unwrap() as i32
    }
}

#[cfg(test)]

mod tests{
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::find_nth_digit(11),0);
    }
}