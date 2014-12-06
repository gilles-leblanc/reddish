pub enum EnumLinkedList {
    Node(u32, Box<EnumLinkedList>),
    Nil
}

impl EnumLinkedList {
    fn append(&mut self, element: u32) {
        match *self {
            Node(_, ref mut next) => next.append(element),
            Nil => *self = Node(element, box Nil),
        }
    }

    fn next(&self) -> &EnumLinkedList {
        match *self {
            Node(_, next) => &*next,
            Nil => &Nil,
        }
    }
}


#[test]
fn next_returns_next_element() {
    let list = Node(1, box Node(2, box Nil));

    match list.next() {
        Node(2, _) => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn append_node_to_list() {
    let list = Node(1, box Node(2, box Nil));
    list.append(4);


}
