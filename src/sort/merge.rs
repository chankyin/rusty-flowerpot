use crate::sort::{SortAlgo, Sortable};

pub struct MergeSort;

impl SortAlgo for MergeSort {
    fn sort<T: Sortable>(mut vec: Vec<T>) -> Vec<T> {
        let len = vec.len();
        merge_sort(&mut vec, 0, len);
        vec
    }
}

fn merge_sort<T: Sortable>(vec: &mut Vec<T>, from: usize, to: usize) {
    if to - from <= 1 { return; }

    let mid = (from + to) / 2;
    merge_sort(vec, from, mid);
    merge_sort(vec, mid, to);

    let merged = merge_sorted(&vec[from..mid], &vec[mid..to]);
    vec.splice(from..to, merged);
}


/// Time complexity: O(v1.len() + v2.len())
/// Memory complexity: O(v1.len() + v2.len())
pub fn merge_sorted<T: Sortable>(v1: &[T], v2: &[T]) -> Vec<T> {
    let mut ret: Vec<T> = Vec::with_capacity(v1.len() + v2.len());
    let mut i = v1.iter();
    let mut j = v2.iter();
    let mut next_i = i.next();
    let mut next_j = j.next();
    loop {
        match (next_i, next_j) {
            (None, None) => break,
            (None, Some(value)) => {
                ret.push(*value);
                next_j = j.next();
            }
            (Some(value), None) => {
                ret.push(*value);
                next_i = i.next();
            }
            (Some(a), Some(b)) if a < b => {
                ret.push(*a);
                next_i = i.next();
            }
            (Some(_), Some(value)) => {
                ret.push(*value);
                next_j = j.next();
            }
        }
    }
    ret
}
