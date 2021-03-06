use EnumLinkedList::{Node, Nil};

pub enum EnumLinkedList {
    Node(u32, Box<EnumLinkedList>),
    Nil
}

impl EnumLinkedList {
    fn prepend(self, element: u32) -> EnumLinkedList {
        Node(element, box self)
    }

    fn append(&mut self, element: u32) {
        match *self {
            Node(_, ref mut next) => next.append(element),
            Nil => *self = Node(element, box Nil),
        }
    }

    fn next(&self) -> &EnumLinkedList {
        match *self {
            Node(_, ref next) => &**next,
            Nil => self,
        }
    }
}

#[test]
fn prepend_adds_element_to_front_of_list() {
    let list = Node(1, box Nil);
    let new_list = list.prepend(0);

    match new_list {
        Node(0, _) => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn next_returns_next_element() {
    let mut list = Node(1, box Node(2, box Nil));

    match *list.next() {
        Node(2, _) => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn append_node_to_list() {
    let mut list = Node(1, box Node(2, box Nil));
    list.append(4);

    match *list.next().next() {
        Node(4, _) => assert!(true),
        _ => assert!(false),
    }
}
