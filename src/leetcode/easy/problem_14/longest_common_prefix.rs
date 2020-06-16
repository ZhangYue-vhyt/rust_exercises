use std::cmp::min;

pub fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.is_empty() {
        return String::default();
    }
    let common: String = String::from(&strs[0]);
    let mut end = common.len();

    for s in strs.iter() {
        for i in 0..common.len() {
            match (common.chars().nth(i), s.chars().nth(i)) {
                (Some(a), Some(b)) => {
                    if a != b {
                        end = min(end, i);
                    }
                }
                _ => end = min(end, i),
            }
        }
    }

    String::from(&common[..end])
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_longest_common_prefix() {
        let case_1 = vec![
            "flower".to_string(),
            "flow".to_string(),
            "flight".to_string(),
        ];
        let case_2 = vec!["dog".to_string(), "racecar".to_string(), "car".to_string()];
        let case_3 = vec!["aa".to_string(), "a".to_string()];
        assert_eq!("fl", longest_common_prefix(case_1));
        assert_eq!("", longest_common_prefix(case_2));
        assert_eq!("a", longest_common_prefix(case_3));
    }
}
