extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
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
    let mut tree: Option<Box<node_t>> = Some(Box::new(node_t {
        left: Default::default(),
        right: Default::default(),
        value: 0,
    }));
    let mut n1: Option<Box<node_t>> = Some(Box::new(node_t {
        left: Default::default(),
        right: Default::default(),
        value: 1,
    }));
    let mut n2: Option<Box<node_t>> = Some(Box::new(node_t {
        left: Default::default(),
        right: Default::default(),
        value: 2,
    }));
    let mut n3: Option<Box<node_t>> = Some(Box::new(node_t {
        left: Default::default(),
        right: Default::default(),
        value: 3,
    }));
    let mut n4: Option<Box<node_t>> = Some(Box::new(node_t {
        left: Default::default(),
        right: Default::default(),
        value: 4,
    }));
    let mut ptr1: *mut node_t = (&mut (*tree.as_deref_mut().unwrap()) as *mut node_t);
    ptr1 = (unsafe {
        let _node: *mut node_t = ptr1;
        let _new_node: *mut node_t = (&mut (*n1.as_deref_mut().unwrap()) as *mut node_t);
        insert_1(_node, _new_node)
    });
    ptr1 = (unsafe {
        let _node: *mut node_t = ptr1;
        let _new_node: *mut node_t = (&mut (*n2.as_deref_mut().unwrap()) as *mut node_t);
        insert_1(_node, _new_node)
    });
    ptr1 = (unsafe {
        let _node: *mut node_t = ptr1;
        let _new_node: *mut node_t = (&mut (*n3.as_deref_mut().unwrap()) as *mut node_t);
        insert_1(_node, _new_node)
    });
    ptr1 = (unsafe {
        let _node: *mut node_t = ptr1;
        let _new_node: *mut node_t = (&mut (*n4.as_deref_mut().unwrap()) as *mut node_t);
        insert_1(_node, _new_node)
    });
    return (((((((((*(unsafe {
        let _node: *mut node_t = ptr1;
        let _value: i32 = 0;
        find_0(_node, _value)
    }))
    .value)
        == (0))
        && (((*(unsafe {
            let _node: *mut node_t = ptr1;
            let _value: i32 = 1;
            find_0(_node, _value)
        }))
        .value)
            == (1)))
        && (((*(unsafe {
            let _node: *mut node_t = ptr1;
            let _value: i32 = 2;
            find_0(_node, _value)
        }))
        .value)
            == (2)))
        && (((*(unsafe {
            let _node: *mut node_t = ptr1;
            let _value: i32 = 3;
            find_0(_node, _value)
        }))
        .value)
            == (3)))
        && (((*(unsafe {
            let _node: *mut node_t = ptr1;
            let _value: i32 = 4;
            find_0(_node, _value)
        }))
        .value)
            == (4)))
        && ((unsafe {
            let _node: *mut node_t = ptr1;
            let _value: i32 = 5;
            find_0(_node, _value)
        })
        .is_null())) as i32);
}
