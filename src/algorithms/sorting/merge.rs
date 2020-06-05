trait Merge {
    fn merge_sort(&mut self);
}

impl<T: PartialOrd + Copy> Merge for [T] {
    fn merge_sort(&mut self) {
        if self.len() > 1 {
            let pivot = self.len() / 2;
            let list_1 = &mut self[pivot..];
            list_1.merge_sort();
            let list_2 = &mut self[..pivot];
            list_2.merge_sort();

            let mut temp: Vec<T> = vec![];

            merge(&self[pivot..], &self[..pivot], &mut temp);

            self.copy_from_slice(&temp);
        }
    }
}

fn merge<T: PartialOrd + Copy>(list_1: &[T], list_2: &[T], ret: &mut Vec<T>) {
    let (mut i, mut j) = (0, 0);
    while i < list_1.len() && j < list_2.len() {
        if list_1[i] < list_2[j] {
            ret.push(list_1[i]);
            i += 1;
        } else {
            ret.push(list_2[j]);
            j += 1;
        }
    }

    while i < list_1.len() {
        ret.push(list_1[i]);
        i += 1;
    }

    while j < list_2.len() {
        ret.push(list_2[j]);
        j += 1;
    }
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
        assert_ne!(list, vec![3, 2, 1, 4, 6, 4, 7]);
        assert_eq!(list, vec![1, 2, 3, 4, 5, 6, 7]);
    }
}
