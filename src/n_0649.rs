pub struct Solution;
use std::collections::VecDeque;
impl Solution {
    pub fn predict_party_victory(senate: String) -> String {
        let mut radiants = VecDeque::new();
        let mut dires = VecDeque::new();
        let length = senate.len();

        for (i,ch) in senate.char_indices(){
            if ch == 'R' {
                radiants.push_back(i);
            }else{
                dires.push_back(i);
            }
        }
        loop{
            match (radiants.pop_front(),dires.pop_front()) {
                (Some(r),Some(d)) =>{
                    if r<d {
                        radiants.push_back(r+length);    
                    }else{
                        dires.push_back(d+length);
                    }
                },
                (Some(_r),_) =>{
                    return "Radiant".to_string();
                },
                (_,Some(_d)) =>{
                    return "Dire".to_string();
                },
                (_,_)=>{},
            }
            
        }
    }
}

#[cfg(test)]
mod tests{
    use super::Solution;

    #[test]
    fn test(){
        assert_eq!(Solution::predict_party_victory("RDD".to_string()),"Dire".to_string());
    }
}