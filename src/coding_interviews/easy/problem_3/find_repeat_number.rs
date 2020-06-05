use std::collections::HashSet;

pub fn find_repeat_number(nums: Vec<i32>) -> i32 {
    let mut set: HashSet<i32> = HashSet::new();

    for elem in &nums {
        if set.contains(&elem) {
            return *elem;
        }
        set.insert(*elem);
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find_repeat_number() {
        let case = vec![2, 3, 1, 0, 2, 5, 3];
        assert_eq!(2, find_repeat_number(case))
    }
}
