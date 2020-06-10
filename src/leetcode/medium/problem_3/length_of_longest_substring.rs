use std::cmp::max;
use std::collections::HashMap;

pub fn length_of_longest_substring(s: String) -> i32 {
    // HashMap:::new() is quicker than HashMap::with_capacity(s.len()).
    let mut hashmap: HashMap<char, usize> = HashMap::new();
    let mut result = 0;
    let mut start_position: usize = 0;
    for (i, ch) in s.char_indices() {
        if let Some(&index) = hashmap.get(&ch) {
            result = max(result, i - start_position);
            if start_position <= index {
                start_position = index + 1;
            }
        }
        hashmap.insert(ch, i);
    }
    max(result, s.len() - start_position) as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_length_of_longest_substring() {
        let case_1 = "abcabcbb".to_string();
        assert_eq!(3, length_of_longest_substring(case_1));
        let case_2 = "bbbbb".to_string();
        assert_eq!(1, length_of_longest_substring(case_2));
        let case_3 = "pwwkew".to_string();
        assert_eq!(3, length_of_longest_substring(case_3));
        let case_4 = "aab".to_string();
        assert_eq!(2, length_of_longest_substring(case_4));
        let case_5 = "dvdf".to_string();
        assert_eq!(3, length_of_longest_substring(case_5));
        let case_6 = "abba".to_string();
        assert_eq!(2, length_of_longest_substring(case_6));
    }
}
