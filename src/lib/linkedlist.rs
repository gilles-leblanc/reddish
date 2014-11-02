pub enum EnumLinkedList {
    Node(u32, Box<EnumLinkedList>),
    Nil
}

pub struct LinkedListNode {
    value: u32,
    next: Option<Box<LinkedListNode>>
}

pub struct LinkedList {
    head: Option<Box<LinkedListNode>>
}

impl LinkedList {
    pub fn new() -> LinkedList {
        LinkedList {
            head: None
        }
    }

    /// Append a node after the last node of the linked list.
    pub fn append(node: LinkedListNode) {

    }
}

#[test]
fn new_list_is_empty() {
    let linked_list = LinkedList::new();
    let head = linked_list.head;

    match linked_list.head {
        None => assert!(true),
        _ => assert!(false)
    }
}
