pub struct Solution;

impl Solution {
    pub fn is_number(s: String) -> bool {
        enum STATE {
            Start,
            Sign,
            Digit,
            Decimal,
            DecimalNoInt,
            PostDigit,
            Exp,
            ExpSign,
            ExpDigit,
            End,
            Illigal,
        }

        fn state_change(state: STATE, ch: char) -> STATE {
            match state {
                STATE::Start => match ch {
                    ' ' => STATE::Start,
                    '-' | '+' => STATE::Sign,
                    '0'..='9' => STATE::Digit,
                    '.' => STATE::DecimalNoInt,
                    _ => STATE::Illigal,
                },
                STATE::Sign => match ch {
                    '0'..='9' => STATE::Digit,
                    '.' => STATE::DecimalNoInt,
                    _ => STATE::Illigal,
                },
                STATE::Digit => match ch {
                    '0'..='9' => STATE::Digit,
                    '.' => STATE::Decimal,
                    'e' | 'E' => STATE::Exp,
                    ' ' => STATE::End,
                    _ => STATE::Illigal,
                },
                STATE::Decimal => match ch {
                    '0'..='9' => STATE::PostDigit,
                    'e' | 'E' => STATE::Exp,
                    ' ' => STATE::End,
                    _ => STATE::Illigal,
                },
                STATE::DecimalNoInt => match ch {
                    '0'..='9' => STATE::PostDigit,
                    _ => STATE::Illigal,
                },
                STATE::PostDigit => match ch {
                    '0'..='9' => STATE::PostDigit,
                    'e' | 'E' => STATE::Exp,
                    ' ' => STATE::End,
                    _ => STATE::Illigal,
                },
                STATE::Exp => match ch {
                    '-' | '+' => STATE::ExpSign,
                    '0'..='9' => STATE::ExpDigit,
                    _ => STATE::Illigal,
                },
                STATE::ExpSign => match ch {
                    '0'..='9' => STATE::ExpDigit,
                    _ => STATE::Illigal,
                },
                STATE::ExpDigit => match ch {
                    '0'..='9' => STATE::ExpDigit,
                    ' ' => STATE::End,
                    _ => STATE::Illigal,
                },
                STATE::End => match ch {
                    ' ' => STATE::End,
                    _ => STATE::Illigal,
                },
                STATE::Illigal => STATE::Illigal,
            }
        }

        let mut chars = s.chars();
        let mut state = STATE::Start;
        loop {
            let cur = chars.next();
            if let Some(i) = cur {
                let ch = i;
                state = state_change(state, ch);
            } else {
                break;
            }
            
        }
        match state {
            STATE::Digit | STATE::Decimal | STATE::PostDigit | STATE::ExpDigit | STATE::End => {
                true
            }
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::is_number(" .2E1   ".to_string()), true);
    }
}
