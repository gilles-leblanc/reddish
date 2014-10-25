extern crate reddish_lib;

use reddish_lib::linkedlist::{Node, Nil};

fn main() {
    let list = Node(1, box Node(2, box Node(3, box Nil)));

    match list {
        Node(first_value, _) => println!("First value {}", first_value),
        Nil => println!("Nil value")
    }
}
