pub enum EnumLinkedList {
    Node(u32, Box<EnumLinkedList>),
    Nil
}


