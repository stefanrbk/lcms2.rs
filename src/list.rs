pub enum List<T> {
    Empty,
    More(Box<Node<T>>),
}

pub struct Node<T> {
    elem: T,
    next: List<T>,
}
