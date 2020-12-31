pub struct Solution;

impl Solution {
    pub fn is_robot_bounded(instructions: String) -> bool {
        let mut dir = 0;
        let mut coor = [0,0];

        for ch in instructions.chars() {
            match ch {
                'G' => match dir {
                    0 => coor[1] += 1,
                    1 => coor[0] += 1,
                    2 => coor[1] -= 1,
                    3 => coor[0] -= 1,
                    _ => {},
                },
                'L' => match dir {
                    0 => dir = 3,
                    _ => dir -=1,
                },
                'R' => match dir {
                    3 => dir = 0,
                    _ => dir +=1,
                }
                _ =>{},
            }
        }
        if dir == 0 && (coor[0]!=0||coor[1]!=0) { false} else {true}
    }
}