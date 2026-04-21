extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[derive(Copy, Clone, Default)]
pub struct node_t {
    pub left: *mut node_t,
    pub right: *mut node_t,
    pub value: i32,
}
pub unsafe fn find_0(mut node: *mut node_t, mut value: i32) -> *mut node_t {
    if (((value) < ((*node).value)) && (!(((*node).left).is_null()))) {
        return (unsafe {
            let _node: *mut node_t = (*node).left;
            let _value: i32 = value;
            find_0(_node, _value)
        });
    } else if (((value) > ((*node).value)) && (!(((*node).right).is_null()))) {
        return (unsafe {
            let _node: *mut node_t = (*node).right;
            let _value: i32 = value;
            find_0(_node, _value)
        });
    } else if ((value) == ((*node).value)) {
        return node;
    }
    return Default::default();
}
pub unsafe fn insert_1(mut node: *mut node_t, mut new_node: *mut node_t) -> *mut node_t {
    if (node).is_null() {
        return new_node;
    }
    if (((*new_node).value) < ((*node).value)) {
        (*node).left = (unsafe {
            let _node: *mut node_t = (*node).left;
            let _new_node: *mut node_t = new_node;
            insert_1(_node, _new_node)
        });
    } else if (((*new_node).value) > ((*node).value)) {
        (*node).right = (unsafe {
            let _node: *mut node_t = (*node).right;
            let _new_node: *mut node_t = new_node;
            insert_1(_node, _new_node)
        });
    }
    return node;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let N: i32 = 25000;
    let mut tree: *mut node_t = (Box::leak(Box::new(node_t {
        left: Default::default(),
        right: Default::default(),
        value: 0,
    })) as *mut node_t);
    let mut i: i32 = 0;
    'loop_: while ((i) < (N)) {
        (unsafe {
            let _node: *mut node_t = tree;
            let _new_node: *mut node_t = (Box::leak(Box::new(node_t {
                left: Default::default(),
                right: Default::default(),
                value: i,
            })) as *mut node_t);
            insert_1(_node, _new_node)
        });
        i.prefix_inc();
    }
    let mut i: i32 = 0;
    'loop_: while ((i) < (N)) {
        (unsafe {
            let _node: *mut node_t = tree;
            let _value: i32 = i;
            find_0(_node, _value)
        });
        i.prefix_inc();
    }
    return 0;
}
