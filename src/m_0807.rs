pub struct Solution;
impl Solution {
    pub fn permutation(s: String) -> Vec<String> {
        fn trav(
            ans:&mut Vec<String>,
            s:&String,level:usize,
            state:&mut Vec<bool>,
            cur:&mut String){
            if level == s.len() { ans.push(cur.clone()); return; }
            for (i,ch) in s.char_indices(){
                if state[i] {
                    state[i] = false;
                    cur.push(ch);
                    trav(ans,s,level+1,state,cur);
                    cur.pop();
                    state[i] = true;
                }
            }
        }
        let mut ans:Vec<String> = Vec::new();
        let mut state = vec![true;s.len()];
        let mut cur = String::new();
        trav(&mut ans,&s,0,&mut state,&mut cur);

        ans
    }
}