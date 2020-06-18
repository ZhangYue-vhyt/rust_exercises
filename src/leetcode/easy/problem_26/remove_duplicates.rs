pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }
    let mut result = 0;
    for i in 0..nums.len() {
        if result < i && nums[result] < nums[i] {
            result += 1;
            nums.swap(result, i);
        }
    }
    (result + 1) as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_remove_duplicates() {
        assert_eq!(0, remove_duplicates(&mut vec![]));
        assert_eq!(1, remove_duplicates(&mut vec![1]));
        assert_eq!(1, remove_duplicates(&mut vec![1, 1]));
        assert_eq!(2, remove_duplicates(&mut vec![1, 2]));
    }
}
