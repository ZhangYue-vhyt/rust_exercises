pub fn is_valid(s: String) -> bool {
    let mut stack = vec![];
    for ch in s.chars() {
            if let '(' | '[' | '{' = ch {
                stack.push(ch);
            } else {
                if let (Some('('), ')') | (Some('['), ']') | (Some('{'), '}') = (stack.pop(), ch) {
                    ()
                } else {
                    return false;
                }
            }
    }

    stack.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_valid() {
        assert_eq!(true, is_valid("()".to_string()));
        assert_eq!(true, is_valid("()[]{}".to_string()));
        assert_eq!(false, is_valid("(]".to_string()));
        assert_eq!(false, is_valid("([)]".to_string()));
        assert_eq!(false, is_valid("[".to_string()));
    }
}
