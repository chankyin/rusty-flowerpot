use crate::sort::{Sortable, SortAlgo};

pub struct HeapSort;

impl SortAlgo for HeapSort {
    fn sort<T: Sortable>(vec: Vec<T>) -> Vec<T> {
        vec
    }
}

struct BinaryTree<T: Sortable> {
    array: Vec<T>,
}

struct BinaryNode<'tree, T: Sortable> {
    id: usize,
    tree: &'tree Vec<T>,
}

impl<'tree, T: Sortable> BinaryNode<'tree, T> {
    fn parent(&self) -> Option<BinaryNode<'tree, T>> {
        match self.id {
            0 => None,
            _ => Some(BinaryNode {
                id: (self.id - 1) / 2,
                tree: self.tree,
            }),
        }
    }
    fn children(&self) -> (Option<T>, Option<T>) {
        let tree = &self.tree;
        let mut ret = (None, None);
        let (ref mut first, ref mut second) = ret;
        let next = self.id * 2 + 1;
        if next < self.tree.len() { *first = Some(BinaryNode { tree, id: next }); }
        if next + 1 < self.tree.len() { *second = Some(BinaryNode { tree, id: next + 1 }); }
        return ret;
    }
}
