pub enum Link<T> {
    Empty,
    More(Box<Node<T>>),
}

pub struct Node<T> {
    elem: T,
    next: Link<T>,
}
