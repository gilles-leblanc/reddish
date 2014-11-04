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
        // find end of current list
            // while ! None
        //let current: LinkedListNode;
        //current = head;

        // append at end
    }
}

#[test]
fn new_list_is_empty() {
    let linked_list = LinkedList::new();
    assert!(linked_list.head.is_none())
}
