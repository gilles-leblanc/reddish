pub struct LinkedList {
    head: Option<Box<LinkedListNode>>
}

impl LinkedList {
    pub fn new() -> LinkedList {
        LinkedList {
            head: None
        }
    }

    pub fn append(&mut self, new_node: LinkedListNode) {
        match self.head {
            Some(ref mut node) => node.append(new_node),
            None => self.head = Some(box new_node),
        }
    }

    pub fn prepend_value(&mut self, value: u32) {
        let head = self.head.take();
        self.head = Some(box LinkedListNode { next: head, value: value });
    }
}


pub struct LinkedListNode {
    value: u32,
    next: Option<Box<LinkedListNode>>
}

impl LinkedListNode {
    pub fn append(&mut self, new_node: LinkedListNode) {
        match self.next {
            Some(ref mut n) => n.append(new_node),
            None => self.next = Some(box new_node),
        }
    }
}


#[test]
fn new_list_is_empty() {
    let linked_list = LinkedList::new();
    assert!(linked_list.head.is_none())
}

#[test]
fn append_node_to_empty_list_contains_at_least_one_node() {
    let mut linked_list = LinkedList::new();
    let node = LinkedListNode { value: 0, next: None };

    linked_list.append(node);
    assert!(linked_list.head.is_some())
}

#[test]
fn append_node_to_empty_list_contains_node() {
    let mut linked_list = LinkedList::new();
    let node = LinkedListNode { value: 10, next: None };

    linked_list.append(node);
    let head = linked_list.head;

    match head {
        Some(ref node) => assert!(node.value == 10),
        None => assert!(false),
    }
}

#[test]
fn append_two_nodes() {
    let mut linked_list = LinkedList::new();
    let first_node = LinkedListNode { value: 10, next: None };
    let second_node = LinkedListNode { value: 20, next: None };

    linked_list.append(first_node);
    linked_list.append(second_node);
    let head = linked_list.head;

    match head {
        Some(ref node) => {
            match node.next {
                Some(ref second) => assert!(second.value == 20),
                None => assert!(false),
            }
        },
        None => assert!(false),
    }
}
