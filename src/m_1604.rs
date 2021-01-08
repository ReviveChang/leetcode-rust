pub struct Solution;

impl Solution {
    pub fn tictactoe(board: Vec<String>) -> String {
        let mut pending = " ";
        let size = board.len();
        let mut space = false;

        for str in &board {
            let pre = str.get(0..1).unwrap();
            let mut flag = false;
            if pre == " " {space = true;continue;}
            let mut ind = 0;
            while ind != size{
                let ch = str.get(ind..ind+1).unwrap();
                if ch != pre {flag = true;}
                if ch == " " {space = true;}
                if flag && space {break;}
                ind +=1 ;
            }
            if flag {continue;}
            if pending == " " {pending = pre;}
            if pending != pre {return String::from("Draw")}
        }

        let mut index = 0;
        while index != size {
            let mut flag = false;
            let pre = board[0].get(index..index+1).unwrap();
            //println!("{}",pre);
            if pre == " " {index += 1;continue;}
            for str in &board{
                let ch = str.get(index..index+1).unwrap();
                if ch != pre {flag = true;}
                if ch == " " {space = true;}
                if flag && space {break;}
            }
            if flag {index += 1;continue;}
            if pending == " " {pending = pre;}
            if pending != pre {return String::from("Draw")}
            index += 1;
        }

        
        let prel = board[0].get(0..1).unwrap();
        let prer = board[0].get(size-1..size).unwrap();
        if prel != " "{
            let mut diag = 0;
            let mut flag = false;
            while diag != size {
                if board[diag].get(diag..diag+1).unwrap() != prel {flag = true;}
                diag+=1;
            }
            if !flag {
                if pending == " " {pending = prel;}
                if pending != prel {return String::from("Draw")}
            }
        }

        if prer != " "{
            let mut diag = 0;
            let mut flag = false;
            while diag != size {
                if board[diag].get(size-diag-1..size-diag).unwrap() != prer {flag = true;break;}
                diag += 1;
            }
            if !flag {
                if pending == " " {pending = prer;}
                if pending != prer {return String::from("Draw")}
            }
        }

        if pending == " " {
            if space {String::from("Pending")}
            else{String::from("Draw")}
        }
        else {
            pending.to_string()
        }
    }
}

#[cfg(test)]
mod tests{
    use super::Solution;

    #[test]
    fn test(){
        assert_eq!(Solution::tictactoe(vec!["O X".to_string()," XO".to_string(),"X O".to_string()]),"X".to_string());
    }
}