pub struct Solution;

// TLE

pub fn construct_distanced_sequence(n: i32) -> Vec<i32> {
    fn trav(flag:&mut bool,states:&mut Vec<bool>,cur:&mut Vec<i32>,level:usize,n:usize,res:&mut Vec<i32>) {
        let mut i =n - 1;
        if !*flag {return;}
        loop{
            if i == 0 && states[i]{
                cur[level] = 1;
                states[i] = false;
            }else if states[i] && cur[level+i+1]==0{
                cur[level] = i as i32 +1;
                cur[level+i+1] = i as i32 +1;
                states[i] = false;
            }else { i-=1; continue;}

            if level == 2*n-1 {
                *res = cur.clone();
                *flag = false;
            }

            trav(flag,states,cur,level+1,n,res);
            if i==0 {break;}
            i-=1;
        }
        
    }
    let mut states = vec![true;2*n as usize-1];
    let mut cur = vec![0;2*n as usize -1];
    let mut flag = false;
    let mut res:Vec<i32> = Vec::new();

    trav(&mut flag,&mut states,&mut cur,0,n as usize,&mut res);
    res
}