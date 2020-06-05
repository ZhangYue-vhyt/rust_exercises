pub fn reverse(x: i32) -> i32 {
    let mut y = x;
    let mut result: i32 = 0;

    while y != 0 {
        if let Some(n) = result
            .checked_mul(10)
            .and_then(|product| product.checked_add(y % 10))
        {
            result = n;
            y = y / 10;
        } else {
            return 0;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse() {
        assert_eq!(321, reverse(123));
        assert_eq!(-321, reverse(-123));
        assert_eq!(21, reverse(120));
        assert_eq!(0, reverse(1234567899));
        assert_eq!(0, reverse(i32::MIN));
    }
}
