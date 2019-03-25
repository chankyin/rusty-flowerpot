use crate::sort::{SortAlgo, Sortable};

/// Complexity: O(n^2)
pub struct BubbleSort;

impl SortAlgo for BubbleSort {
    fn sort<T: Sortable>(mut vec: Vec<T>) -> Vec<T> {
        for i in (1..vec.len()).rev() {
            for j in 1..=i {
                if vec[j - 1] > vec[j] {
                    vec.swap(j - 1, j);
                }
            }
        }
        vec
    }
}

