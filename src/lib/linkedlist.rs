pub enum LinkedList {
    Node(u32, Box<LinkedList>),
    Nil
}
