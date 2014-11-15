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
    pub fn append(&mut self, node: LinkedListNode) {
        // find end of current list
            // while ! None
        match self.head {
            None => self.head = Some(box node),
            _ => println!("duh"), // don't know what to do here yet
        }
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

#[test]
fn append_node_to_empty_list_list_contains_at_least_one_node() {
    let mut linked_list = LinkedList::new();
    let node = LinkedListNode { value: 0, next: None };

    linked_list.append(node);
    assert!(linked_list.head.is_some())
}

