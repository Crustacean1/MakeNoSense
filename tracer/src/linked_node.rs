/*struct LinkedNode<T> {
    value: Box<T>,
    next: Option<Box<LinkedNode<T>>>,
}

impl<T> LinkedNode<T> {
    pub fn new(value: T) -> Box<LinkedNode<T>> {
        Box::new(LinkedNode {
            value: Box::new(value),
            next: None,
        })
    }

    pub fn push(&mut self, node: Box<LinkedNode<T>>) {
        let mut node = node;
        let next_node = match self.next {
            Some(mut node) => {
                node.next = Some(node);
            }
            _ => {}
        };
    }

    pub fn next(&mut self) -> Option<Box<LinkedNode<T>>> {
        self.next
    }
}*/
