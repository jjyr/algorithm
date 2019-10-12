pub mod merge_sort;
pub mod red_black_tree;

pub fn is_sorted<E: std::cmp::Ord>(l: &[E]) -> bool {
    if l.len() == 1 {
        return true;
    }
    let mut p = &l[0];
    // each item should greater than previous one
    for i in &l[1..] {
        if i < p {
            return false;
        }
        p = i;
    }
    true
}
