use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut result: HashMap<i32, i32> = HashMap::new();

    for i in 0..nums.len() {
        let complement = target - nums[i];
        if result.contains_key(&complement) {
            return vec![result[&complement], i as i32];
        }
        result.insert(nums[i], i as i32);
    }
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_two_sum() {
        let input = vec![2, 7, 11, 15];
        let target = 9;
        let expected = vec![0, 1];
        let actual = two_sum(input, target);
        assert!(expected.eq(&actual));
    }
}
