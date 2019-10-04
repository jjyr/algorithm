//! Merge sort has two steps:
//! sort_inner: recurrently split the list to smaller parts: log(n)O
//! merge: merge two sub lists from bottom to up: log(n)O
//!
//! Merge sort is stable sorting, use an extra aux list in space complex.
//! The total time complex is 2log(n)O -> log(n)O
//! In case all elements is reversed, merge sort is still log(n)O

pub fn merge_sort<E: std::cmp::Ord + Clone>(l: &mut [E]) {
    if l.len() == 1 {
        return;
    }
    // copy a auxiliary list do merge
    let mut aux = l.to_owned();
    sort_inner(l, &mut aux, 0, l.len());
}

fn sort_inner<E: std::cmp::Ord + Clone>(l: &mut [E], aux: &mut [E], lo: usize, hi: usize) {
    // sort aux list first, then merge back to l
    if hi <= (lo + 1) {
        return;
    }
    let mid = (lo + hi) / 2;
    sort_inner(aux, l, lo, mid);
    sort_inner(aux, l, mid, hi);
    // cut off if aux is already merged
    if aux[mid - 1] < aux[mid] {
        l[lo..hi].clone_from_slice(&aux[lo..hi]);
        return;
    }
    merge(l, aux, lo, mid, hi);
}

fn merge<E: std::cmp::Ord + Clone>(l: &mut [E], aux: &mut [E], lo: usize, mid: usize, hi: usize) {
    let mut j = lo;
    let mut k = mid;

    // update aux list, then do merge

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
