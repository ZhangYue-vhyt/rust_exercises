trait Merge {
    fn merge_sort(&mut self);
}

impl<T: PartialOrd + Copy> Merge for [T] {
    fn merge_sort(&mut self) {
        if self.len() > 1 {
            let mid = self.len() / 2;
            let list_1 = &mut self[mid..];
            list_1.merge_sort();
            let list_2 = &mut self[..mid];
            list_2.merge_sort();

            let temp = merge(&self[mid..], &self[..mid]);

            self.copy_from_slice(&temp);
        }
    }
}

fn merge<T: PartialOrd + Copy>(list_1: &[T], list_2: &[T]) -> Vec<T> {
    let (mut i, mut j) = (0, 0);
    let mut result: Vec<T> = vec![];
    while i < list_1.len() && j < list_2.len() {
        if list_1[i] < list_2[j] {
            result.push(list_1[i]);
            i += 1;
        } else {
            result.push(list_2[j]);
            j += 1;
        }
    }

    while i < list_1.len() {
        result.push(list_1[i]);
        i += 1;
    }

    while j < list_2.len() {
        result.push(list_2[j]);
        j += 1;
    }

    result
}

// # Unit Tests
// Unit tests should be placed in the same file with the test functions.
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn merge_sort_test() {
        let mut list = vec![3, 2, 1, 4, 6, 5, 7];
        list.merge_sort();
        println!("{:?}", list);
        assert_ne!(list, vec![3, 2, 1, 4, 6, 4, 7]);
        assert_eq!(list, vec![1, 2, 3, 4, 5, 6, 7]);
    }
}
