// Определение структуры узла односвязного списка
pub struct Node<T> {
    pub data: T,
    pub next: Option<Box<Node<T>>>,
}

// Определение структуры односвязного списка
pub struct LinkedList<T> {
    pub head: Option<Box<Node<T>>>,
}

impl<T> LinkedList<T> {
    // Создание нового пустого списка
    pub fn new() -> Self {
        LinkedList { head: None }
    }

    // Добавление элемента в начало списка
    pub fn push(&mut self, data: T) {
        let new_node = Box::new(Node {
            data,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    // Разворот списка
    pub fn reverse(&mut self) {
        let mut prev = None;
        let mut current = self.head.take();

        while let Some(mut node) = current.take() {
            let next = node.next.take();
            node.next = prev.take();
            prev = Some(node);
            current = next;
        }

        self.head = prev;
    }
}
