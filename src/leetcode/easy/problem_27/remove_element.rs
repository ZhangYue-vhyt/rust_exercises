pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let (mut i, mut j) = (0, (nums.len() - 1) as i32);
    loop {
        while i <= j && nums[j as usize] == val {
            j -= 1;
        }
        while i < j && nums[i as usize] != val {
            i += 1;
        }
        if i < j {
            nums.swap(i as usize, j as usize);
        } else {
            break;
        }
    }

    j + 1
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_remove_element() {
        assert_eq!(0, remove_element(&mut vec![], 0));
        assert_eq!(1, remove_element(&mut vec![1], 0));
        assert_eq!(0, remove_element(&mut vec![1], 1));
        assert_eq!(2, remove_element(&mut vec![3, 2, 2, 3], 3));
        assert_eq!(1, remove_element(&mut vec![1, 1, 1, 1, 2, 1, 1, 1, 1], 1));
    }
}
