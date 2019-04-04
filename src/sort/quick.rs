use crate::sort::{SortAlgo, Sortable};
use std::mem::swap;

pub struct QuickSort;

impl SortAlgo for QuickSort {
    fn sort<T: Sortable>(mut vec: Vec<T>) -> Vec<T> {
        let length = vec.len();
        quick_sort(&mut vec, 0, length);
        vec
    }
}

fn quick_sort<T: Sortable>(vec: &mut Vec<T>, from: usize, to: usize) {
    if from >= to { return; }

    let mut i = 0;
    for j in from..=to - 2 {
        if vec[j] <= vec[to - 1] {
            vec.swap(j, i + 1);
            i += 1;
        }
    }
    vec.swap(i + 1, to - 1);

    quick_sort(vec, from, i);
    quick_sort(vec, i + 2, to);
}
