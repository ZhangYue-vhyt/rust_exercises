pub fn is_palindrome(x: i32) -> bool {
    if x.is_negative() {
        return false;
    };

    let mut y = x;
    let mut reverse_number: i32 = 0;

    while y > 0 {
        reverse_number = reverse_number * 10 + y % 10;
        y /= 10;
    }

    x == reverse_number
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_palindrome_number() {
        assert!(is_palindrome(121));
        assert!(!is_palindrome(-121));
        assert!(!is_palindrome(10));
    }
}
