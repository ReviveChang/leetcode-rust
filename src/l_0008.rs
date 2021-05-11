pub struct Solution;

impl Solution { 
    pub fn get_trigger_time(increase: Vec<Vec<i32>>, requirements: Vec<Vec<i32>>) -> Vec<i32> {
        let max = increase.len();
        let mut inc = vec![Vec::new();3];
        for v in &increase{
            for i in 0..3{
                let hi =inc[i].last().unwrap_or(&0)+v[i];
                //println!("{}",v[i]);
                inc[i].push(hi);
            }  
        }
        //println!("{:?}\n{:?}\n{:?}",inc[0],inc[1],inc[2]);
        
        let mut res:Vec<i32> = Vec::new();

        for require in requirements{
            let mut loc = vec![0i32;3];
            let mut flag = false;
            for i in 0..3 {
                if require[i] <= 0 {
                    loc[i] = -1;
                    continue;
                }
                loc[i] = match inc[i].binary_search(&(require[i])) {
                    Ok(e) => e as i32,
                    Err(e) => {
                        if e==max {flag = true;}
                        e as i32
                    }
                };
                while loc[i]!=1 && loc[i]!=max as i32 && inc[i][loc[i] as usize]==inc[i][loc[i] as usize -1]{
                    loc[i] -= 1;
                }
            }
            //println!("{:?}",loc);
            let lmax = loc[0].max(loc[1]).max(loc[2]);
            //println!("{}",lmax);
            if flag {res.push(-1);}
            else {res.push(lmax as i32+1);}
        }
        res
    }
}
