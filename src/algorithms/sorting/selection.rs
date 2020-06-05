trait Selection {
    fn selection_sort(&mut self);
}

impl<T: PartialOrd> Selection for [T] {
    fn selection_sort(&mut self) {
        for i in 0..self.len() {
            let mut min_index = i;
            for j in i..self.len() {
                if self[j] < self[min_index] {
                    min_index = j;
                }
            }
            self.swap(i, min_index);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Selection;
    #[test]
    fn selection_sort_test() {
        let mut list = vec![3, 2, 1, 4, 6, 5, 7];
        list.selection_sort();
        assert_ne!(list, vec![3, 2, 1, 4, 6, 4, 7]);
        assert_eq!(list, vec![1, 2, 3, 4, 5, 6, 7]);
    }
}
