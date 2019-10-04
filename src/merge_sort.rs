//! Merge sort has two steps:
//! merge_sort: merge list from bottom to up: log(n) steps
//! merge: merge two sub lists: n steps
//! The total time complex is nlog(n)
//!
//! Merge sort is stable sorting, use an extra aux list in space complex
//! In case all elements is reversed, merge sort still has nlog(n) complexity

use std::cmp::{min, Ord};
use std::fmt::Debug;

pub fn merge_sort<E: Ord + Clone + Debug>(l: &mut [E]) {
    if l.len() == 1 {
        return;
    }
    // copy a auxiliary list do merge
    let mut aux = l.to_owned();
    // merge from bottom to up
    let mut chunk_size = 1;
    while chunk_size < l.len() {
        let mut i = 0;
        while i < l.len() {
            merge(
                l,
                &mut aux,
                i,
                i + chunk_size,
                min(i + chunk_size * 2, l.len()),
            );
            i += chunk_size * 2;
        }
        chunk_size *= 2;
    }
}

fn merge<E: Ord + Clone + Debug>(l: &mut [E], aux: &mut [E], lo: usize, mid: usize, hi: usize) {
    let mut j = lo;
    let mut k = mid;

    // update aux list, then do merge
    aux[lo..hi].clone_from_slice(&l[lo..hi]);

    for i in lo..hi {
        if j >= mid {
            l[i] = aux[k].clone();
            k += 1;
        } else if k >= hi {
            l[i] = aux[j].clone();
            j += 1;
        } else if aux[j] <= aux[k] {
            l[i] = aux[j].clone();
            j += 1;
        } else {
            l[i] = aux[k].clone();
            k += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::is_sorted;
    #[test]
    fn try_merge_sort() {
        let mut list = vec![1, 5, 12, 3, 5, 7, 9, 1, 2, 3, 4, 5];
        merge_sort(&mut list);
        assert!(is_sorted(&list));
    }
}
