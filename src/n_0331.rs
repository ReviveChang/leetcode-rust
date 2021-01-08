pub struct Solution;

impl Solution {
    pub fn is_valid_serialization(preorder: String) -> bool {
        let mut slots = 1;
        let mut mt = false;

        for ch in preorder.chars(){
            
            match ch{
                ','=>{
                    if slots == 0 { return false; }
                    mt = false;
                }
                '#' => slots-=1,
                _ => {
                    if mt==false {slots += 1;}
                    mt = true;
                }

            }
            
        }
        slots == 0
    }
}