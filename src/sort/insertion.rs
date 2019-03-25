use crate::sort::{SortAlgo, Sortable};

pub struct InsertionSort;

/// Complexity: O(n^2)
impl SortAlgo for InsertionSort {
    fn sort<T: Sortable>(vec: Vec<T>) -> Vec<T> {
        let mut list = Vec::new();
        for item in vec {
            let pos = binary_search_lower_bound(&list, &item);
            list.insert(pos, item);
        }
        return list;
    }
}

/// Returns x such that for all `i < x`, `self[i] < search`
pub fn binary_search_lower_bound<T: PartialOrd>(vec: &Vec<T>, search: &T) -> usize {
    if vec.is_empty() { return 0; }
    let mut a = 0;
    let mut b = vec.len();
    while b - a > 1 {
        let mid = (a + b) / 2;
        if vec[mid] >= *search {
            b = mid;
        } else {
            a = mid;
        }
    }
    if vec[a] >= *search { a } else { b }
}

#[cfg(test)]
mod test {
    use crate::sort::insertion::binary_search_lower_bound;

    #[test]
    pub fn test_binary_search_lower_bound() {
        assert_eq!(binary_search_lower_bound(&vec![], &0), 0);
        assert_eq!(binary_search_lower_bound(&vec![0], &0), 0);
        assert_eq!(binary_search_lower_bound(&vec![1], &0), 0);
        assert_eq!(binary_search_lower_bound(&vec![0], &1), 1);
        let data = vec![0, 0, 0, 1, 1, 1, 2, 2];
        assert_eq!(binary_search_lower_bound(&data, &0), 0);
        assert_eq!(binary_search_lower_bound(&data, &1), 3);
        assert_eq!(binary_search_lower_bound(&data, &2), 6);
    }
}
