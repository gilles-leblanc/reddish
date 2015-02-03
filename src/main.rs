#![feature(box_syntax)]

extern crate reddish_lib;

use reddish_lib::enumlinkedlist::EnumLinkedList::{Node, Nil};

fn main() {
    let list = Node(1, box Node(2, box Node(3, box Nil)));

    match list {
        Node(first_value, _) => println!("First value {}", first_value),
        Nil => println!("Nil value")
    }

    match list {
        Node(first_value, box ref second_node) => {
            match second_node {
                &Node(second_value, _) => println!("First value {0}, Second value {1}", first_value, second_value),
                &Nil => println!("Nil value")
            }
        }
        _ => println!("Can't match")
    }

    let dsec = match list {
        Node(_, box ref second_node) => {
            match second_node {
                &Node(second_value, _) => Some(second_value),
                &Nil => None
            }
        },
        Nil => None
    };

    println!("Second value {0}", dsec.unwrap());
}
