pub struct BinaryNode<'l, T> {
    left: Option<Box<BinaryNode<'l, T>>>,
    right: Option<Box<BinaryNode<'l, T>>>,
}
