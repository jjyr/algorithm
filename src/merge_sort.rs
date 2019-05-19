pub fn merge_sort<E: std::cmp::Ord>(mut list: Vec<E>) -> Vec<E> {
    if list.len() == 1 {
        return list;
    }
    let b = list.split_off(list.len() / 2);
    merge(merge_sort(list), merge_sort(b))
}

fn merge<E: std::cmp::Ord>(mut a: Vec<E>, mut b: Vec<E>) -> Vec<E> {
    if a.is_empty() {
        return b;
    } else if b.is_empty() {
        return a;
    }

    if a[0] <= b[0] {
        let tail = a.split_off(1);
        a.extend(merge(tail, b));
        a
    } else {
        let tail = b.split_off(1);
        b.extend(merge(a, tail));
        b
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn try_merge_sort() {
        let elems = vec![1, 5, 12, 3, 5, 7, 9, 1, 2, 3, 4, 5];
        let sorted = merge_sort(elems);
        println!("sorted: {:?}", sorted);
    }
}
