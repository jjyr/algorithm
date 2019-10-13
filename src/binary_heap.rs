pub struct BinaryHeap<T>(Vec<T>);

impl<T: Default + Ord> BinaryHeap<T> {
    pub fn new() -> Self {
        // to make the index start from 1 we initialize vector with one unused item
        // NOTE we can also archieve this by convert index before access vector.
        BinaryHeap(vec![T::default()])
    }

    pub fn push(&mut self, t: T) {
        self.0.push(t);
        self.bubble(self.0.len() - 1);
    }

    pub fn remove_max(&mut self) -> T {
        assert!(self.0.len() > 1);
        let last_i = self.0.len() - 1;
        self.0.swap(1, last_i);
        let t = self.0.remove(last_i);
        self.sink(1);
        t
    }

    pub fn size(&self) -> usize {
        self.0.len() - 1
    }

    /// re-balance with i from bottom to up.
    fn bubble(&mut self, mut i: usize) {
        while (i / 2) > 0 && self.0[i] > self.0[i / 2] {
            self.0.swap(i, i / 2);
            i = i / 2;
        }
    }

    /// re-balance with i from up to bottom.
    fn sink(&mut self, mut i: usize) {
        while (i * 2) < self.0.len() {
            // left child position
            let mut j = i * 2;
            // find greater child
            if (j + 1) < self.0.len() && self.0[j + 1] > self.0[j] {
                j += 1;
            }
            if self.0[i] >= self.0[j] {
                break;
            }
            self.0.swap(i, j);
            i = j;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::is_sorted;
    #[test]
    fn try_heap_sort() {
        let mut bheap = BinaryHeap::new();
        let list = vec![1, 5, 12, 3, 5, 7, 9, 1, 2, 3, 4, 5];
        // push to heap
        for &i in &list {
            bheap.push(i);
        }
        // remove max from heap
        let mut sorted_list = (0..list.len())
            .map(|_| bheap.remove_max())
            .collect::<Vec<_>>();
        // reverse list to make list sorted from min to max
        sorted_list.reverse();
        assert!(is_sorted(&sorted_list));
    }
}
