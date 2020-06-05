trait Bubble {
    fn bubble_sort(&mut self);
}

impl<T: PartialOrd> Bubble for [T] {
    fn bubble_sort(&mut self) {
        let mut sorted = self.len() - 1;
        while sorted > 0 {
            for i in 0..sorted {
                if self[i] > self[i + 1] {
                    self.swap(i, i + 1);
                }
            }
            sorted -= 1;
        }
    }
}

// # Unit Tests
// Unit tests should be placed in the same file with the test functions.
#[cfg(test)]
mod tests {
    use super::Bubble;
    #[test]
    fn bubble_sort_test() {
        let mut list = vec![3, 2, 1, 4, 6, 5, 7];
        list.bubble_sort();
        assert_ne!(list, vec![3, 2, 1, 4, 6, 4, 7]);
        assert_eq!(list, vec![1, 2, 3, 4, 5, 6, 7]);
    }
}
