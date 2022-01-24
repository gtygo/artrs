use artrs::node::{Node, Type};


#[test]
fn test_lock_new() {
    let mut n = Node::new(Type::N4);
    assert_eq!(*n.version.get_mut(),0);
}


/*

use crate::node::{Leaf, Type};
use crate::node::Node;
use crate::tree::Tree;
use std::str::from_utf8;

mod node;
mod tree;

fn main() {
    let k = "ssssssss";
    let v = "bbbbbbb";

    let s = Leaf::new(k.as_bytes(), v.as_bytes());

    let t = s.n_type;
    let kk = from_utf8(s.key.as_ref()).unwrap();
    let vv = from_utf8(s.value.as_ref()).unwrap();

    // let tree = Tree { i: 2};
    println!("k={},v={}", k, v);
    println!("k={},v={}", kk, vv);
    println!("t={}", t);

    println!("new node !!!!");

    let mut  n = Node::new(Type::N4);

    println!("wait unlock  : {}", *n.version.get_mut());
    n.wait_unlock();
    println!("info= {}", *n.version.get_mut());

    println!("info= {:?}", n.prefix);
}


 */