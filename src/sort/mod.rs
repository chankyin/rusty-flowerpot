/// Arrays that can be sorted must only contain Sortable elements.
///
/// Obviously, only elements that can be compared (PartialOrd) can be sorted.
/// Elements must also support Copy so that non-in-place sorting algorithms can copy references efficiently.
///
/// To improve performance, use `&T` instead of T if T is not a simple value like `i32` or `usize`.
pub trait Sortable: PartialOrd + Copy {}

impl Sortable for i32 {}

pub trait SortAlgo {
    fn sort<T: Sortable>(vec: Vec<T>) -> Vec<T>;
}

pub mod bubble;
pub mod insertion;
pub mod selection;
pub mod merge;
pub mod quick;
pub mod heap;

#[cfg(test)]
pub mod test {
    use crate::sort::bubble::BubbleSort;
    use crate::sort::insertion::InsertionSort;
    use crate::sort::merge::MergeSort;
    use crate::sort::quick::QuickSort;
    use crate::sort::selection::SelectionSort;
    use crate::sort::SortAlgo;

    macro_rules! assert_vec_eq {
        ($actual: expr, $expect: expr) => {
            assert!($actual.eq($expect), format!("Expected {:?}, got {:?}", $expect, $actual));
        };
    }

    fn test<T: SortAlgo>() {
        assert_vec_eq!(T::sort(vec![5, 3, 1, 2, 4]), &vec![1, 2, 3, 4, 5]);
    }

    #[test]
    pub fn bubble() { test::<BubbleSort>() }

    #[test]
    pub fn insertion() { test::<InsertionSort>() }

    #[test]
    pub fn selection() { test::<SelectionSort>() }

    #[test]
    pub fn merge() { test::<MergeSort>() }

    #[test]
    pub fn quick() { test::<QuickSort>() }
}
