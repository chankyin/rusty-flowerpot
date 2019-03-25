use crate::sort::{SortAlgo, Sortable};

pub struct SelectionSort;

impl SortAlgo for SelectionSort {
    fn sort<T: Sortable>(mut vec: Vec<T>) -> Vec<T> {
        for i in 0..vec.len() {
            let mut min = i;
            for j in i + 1..vec.len() {
                if vec[min] > vec[j] { min = j; }
            }
            if i != min { vec.swap(i, min); }
        }
        vec
    }
}
