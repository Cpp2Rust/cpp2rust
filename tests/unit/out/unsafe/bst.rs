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
    if ((value) < ((*node).value)) && (!(((*node).left).is_null())) {
        return (unsafe { find_0((*node).left, value) });
    } else if ((value) > ((*node).value)) && (!(((*node).right).is_null())) {
        return (unsafe { find_0((*node).right, value) });
    } else if ((value) == ((*node).value)) {
        return node;
    }
    return std::ptr::null_mut();
}
pub unsafe fn insert_1(mut node: *mut node_t, mut new_node: *mut node_t) -> *mut node_t {
    if (node).is_null() {
        return new_node;
    }
    if (((*new_node).value) < ((*node).value)) {
        (*node).left = (unsafe { insert_1((*node).left, new_node) });
    } else if (((*new_node).value) > ((*node).value)) {
        (*node).right = (unsafe { insert_1((*node).right, new_node) });
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
        left: std::ptr::null_mut(),
        right: std::ptr::null_mut(),
        value: 0,
    }));
    let mut n1: Option<Box<node_t>> = Some(Box::new(node_t {
        left: std::ptr::null_mut(),
        right: std::ptr::null_mut(),
        value: 1,
    }));
    let mut n2: Option<Box<node_t>> = Some(Box::new(node_t {
        left: std::ptr::null_mut(),
        right: std::ptr::null_mut(),
        value: 2,
    }));
    let mut n3: Option<Box<node_t>> = Some(Box::new(node_t {
        left: std::ptr::null_mut(),
        right: std::ptr::null_mut(),
        value: 3,
    }));
    let mut n4: Option<Box<node_t>> = Some(Box::new(node_t {
        left: std::ptr::null_mut(),
        right: std::ptr::null_mut(),
        value: 4,
    }));
    let mut ptr1: *mut node_t = (&mut (*tree.as_deref_mut().unwrap()) as *mut node_t);
    ptr1 = (unsafe { insert_1(ptr1, (&mut (*n1.as_deref_mut().unwrap()) as *mut node_t)) });
    ptr1 = (unsafe { insert_1(ptr1, (&mut (*n2.as_deref_mut().unwrap()) as *mut node_t)) });
    ptr1 = (unsafe { insert_1(ptr1, (&mut (*n3.as_deref_mut().unwrap()) as *mut node_t)) });
    ptr1 = (unsafe { insert_1(ptr1, (&mut (*n4.as_deref_mut().unwrap()) as *mut node_t)) });
    return (((((((((*(unsafe { find_0(ptr1, 0) })).value) == (0))
        && (((*(unsafe { find_0(ptr1, 1) })).value) == (1)))
        && (((*(unsafe { find_0(ptr1, 2) })).value) == (2)))
        && (((*(unsafe { find_0(ptr1, 3) })).value) == (3)))
        && (((*(unsafe { find_0(ptr1, 4) })).value) == (4)))
        && ((unsafe { find_0(ptr1, 5) }).is_null())) as i32);
}
