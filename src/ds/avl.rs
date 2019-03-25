pub trait AvlTree<K, V> {
    fn insert(key: K, value: V);
}

pub struct AvlNode<'tree, K, V> {
    parent: &'tree AvlNode<'tree, K, V>,
    left: &'tree AvlNode<'tree, K, V>,
    right: &'tree AvlNode<'tree, K, V>,
}
