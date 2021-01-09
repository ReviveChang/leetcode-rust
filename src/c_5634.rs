pub struct Solution;

impl Solution {
    pub fn maximum_gain(s: String, x: i32, y: i32) -> i32 {
        let mut avaab = 0;
        let mut avaba = 0;
        let mut acca = 0;
        let mut accb = 0;
        let mut a=0;
        let mut b=0;
        let mut ans = 0;
        
        for ch in s.chars(){
            match &ch {
                'a' => {
                    a+=1;
                    acca += 1;
                    if accb>0 { avaba +=1; accb-=1;}
                },
                'b' =>{
                    b+=1;
                    accb += 1;
                    if acca>0 { avaab +=1; acca-=1;}
                },
                _ =>{
                    if avaab+avaba == 0 {a=0;b=0;acca=0;accb=0;avaab=0;avaba=0;continue;}
                    if x>y{
                        ans += avaab*x+(a.min(b)-avaab)*y;
                    }else{
                        ans += avaba*y+(a.min(b)-avaba)*x;
                    }
                    a=0;b=0;acca=0;accb=0;avaab=0;avaba=0;
                }
            }
        }
        if x>y{
            ans += avaab*x+(a.min(b)-avaab)*y;
        }else{
            ans += avaba*y+(a.min(b)-avaba)*x;
        }
        ans
    }
}