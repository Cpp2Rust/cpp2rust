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
pub unsafe fn insert_1(mut node: *mut node_t, mut value: i32) -> *mut node_t {
    if (node).is_null() {
        return (Box::leak(Box::new(node_t {
            left: std::ptr::null_mut(),
            right: std::ptr::null_mut(),
            value: value,
        })) as *mut node_t);
    }
    if ((value) < ((*node).value)) {
        (*node).left = (unsafe { insert_1((*node).left, value) });
    } else if ((value) > ((*node).value)) {
        (*node).right = (unsafe { insert_1((*node).right, value) });
    }
    return node;
}
pub unsafe fn del_2(mut node: *mut node_t) {
    if !(((*node).left).is_null()) {
        (unsafe { del_2((*node).left) });
    }
    if !(((*node).right).is_null()) {
        (unsafe { del_2((*node).right) });
    }
    ::std::mem::drop(Box::from_raw(node));
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut root: *mut node_t = (Box::leak(Box::new(node_t {
        left: std::ptr::null_mut(),
        right: std::ptr::null_mut(),
        value: 0,
    })) as *mut node_t);
    root = (unsafe { insert_1(root, 1) });
    root = (unsafe { insert_1(root, 2) });
    root = (unsafe { insert_1(root, 3) });
    root = (unsafe { insert_1(root, 4) });
    let mut out: bool = (((((((*(unsafe { find_0(root, 0) })).value) == (0))
        && (((*(unsafe { find_0(root, 1) })).value) == (1)))
        && (((*(unsafe { find_0(root, 2) })).value) == (2)))
        && (((*(unsafe { find_0(root, 3) })).value) == (3)))
        && (((*(unsafe { find_0(root, 4) })).value) == (4)))
        && ((unsafe { find_0(root, 5) }).is_null());
    (unsafe { del_2(root) });
    return (out as i32);
}
