pub enum EnumLinkedList {
    Node(u32, Box<EnumLinkedList>),
    Nil
}

pub struct LinkedListNode {
    value: u32,
    next: Option<Box<LinkedListNode>>
}
//impl Iterator<u32> for LinkedList {
//    fn next(&mut self) -> Option<u32> {
//        if !self
