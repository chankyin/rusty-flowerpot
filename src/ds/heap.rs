use crate::ds::binary_tree::BinaryNode;

pub struct Heap<'l, T>{
    root: Option<Box<BinaryNode<'l, T>>>,
}
