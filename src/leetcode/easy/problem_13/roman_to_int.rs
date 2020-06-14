pub fn roman_to_int(s: String) -> i32 {
    let mut result = 0;
    let chars: Vec<char> = s.chars().collect();
    let mut i = 0;
    while i < s.len() {
        match (chars.get(i), chars.get(i + 1)) {
            (Some('I'), Some('V')) => {
                result += 4;
                i += 1;
            }
            (Some('I'), Some('X')) => {
                result += 9;
                i += 1;
            }
            (Some('I'), _) => result += 1,
            (Some('V'), _) => result += 5,
            (Some('X'), Some('L')) => {
                result += 40;
                i += 1;
            }
            (Some('X'), Some('C')) => {
                result += 90;
                i += 1;
            }
            (Some('X'), _) => result += 10,
            (Some('L'), _) => result += 50,
            (Some('C'), Some('D')) => {
                result += 400;
                i += 1;
            }
            (Some('C'), Some('M')) => {
                result += 900;
                i += 1;
            }
            (Some('C'), _) => result += 100,
            (Some('D'), _) => result += 500,
            (Some('M'), _) => result += 1000,
            _ => (),
        }
        i += 1;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_roman_to_int() {
        let case_1 = "III".to_string();
        let case_2 = "IV".to_string();
        let case_3 = "IX".to_string();
        let case_4 = "LVIII".to_string();
        let case_5 = "MCMXCIV".to_string();
        assert_eq!(3, roman_to_int(case_1));
        assert_eq!(4, roman_to_int(case_2));
        assert_eq!(9, roman_to_int(case_3));
        assert_eq!(58, roman_to_int(case_4));
        assert_eq!(1994, roman_to_int(case_5));
    }
}
