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
        return (unsafe {
            let _node: *mut node_t = (*node).left;
            let _value: i32 = value;
            find_0(_node, _value)
        });
    } else if ((value) > ((*node).value)) && (!(((*node).right).is_null())) {
        return (unsafe {
            let _node: *mut node_t = (*node).right;
            let _value: i32 = value;
            find_0(_node, _value)
        });
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
        (*node).left = (unsafe {
            let _node: *mut node_t = (*node).left;
            let _value: i32 = value;
            insert_1(_node, _value)
        });
    } else if ((value) > ((*node).value)) {
        (*node).right = (unsafe {
            let _node: *mut node_t = (*node).right;
            let _value: i32 = value;
            insert_1(_node, _value)
        });
    }
    return node;
}
pub unsafe fn del_2(mut node: *mut node_t) {
    if !(((*node).left).is_null()) {
        (unsafe {
            let _node: *mut node_t = (*node).left;
            del_2(_node)
        });
    }
    if !(((*node).right).is_null()) {
        (unsafe {
            let _node: *mut node_t = (*node).right;
            del_2(_node)
        });
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
    root = (unsafe {
        let _node: *mut node_t = root;
        let _value: i32 = 1;
        insert_1(_node, _value)
    });
    root = (unsafe {
        let _node: *mut node_t = root;
        let _value: i32 = 2;
        insert_1(_node, _value)
    });
    root = (unsafe {
        let _node: *mut node_t = root;
        let _value: i32 = 3;
        insert_1(_node, _value)
    });
    root = (unsafe {
        let _node: *mut node_t = root;
        let _value: i32 = 4;
        insert_1(_node, _value)
    });
    let mut out: bool = (((((((*(unsafe {
        let _node: *mut node_t = root;
        let _value: i32 = 0;
        find_0(_node, _value)
    }))
    .value)
        == (0))
        && (((*(unsafe {
            let _node: *mut node_t = root;
            let _value: i32 = 1;
            find_0(_node, _value)
        }))
        .value)
            == (1)))
        && (((*(unsafe {
            let _node: *mut node_t = root;
            let _value: i32 = 2;
            find_0(_node, _value)
        }))
        .value)
            == (2)))
        && (((*(unsafe {
            let _node: *mut node_t = root;
            let _value: i32 = 3;
            find_0(_node, _value)
        }))
        .value)
            == (3)))
        && (((*(unsafe {
            let _node: *mut node_t = root;
            let _value: i32 = 4;
            find_0(_node, _value)
        }))
        .value)
            == (4)))
        && ((unsafe {
            let _node: *mut node_t = root;
            let _value: i32 = 5;
            find_0(_node, _value)
        })
        .is_null());
    (unsafe {
        let _node: *mut node_t = root;
        del_2(_node)
    });
    return (out as i32);
}
