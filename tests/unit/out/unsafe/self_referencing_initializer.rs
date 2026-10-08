extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct list_head {
    pub next: *mut list_head,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct node {
    pub value: i32,
    pub value_ptr: *mut i32,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct node_with_defaults {
    pub next: *mut node_with_defaults,
    pub value: i32,
}
impl Default for node_with_defaults {
    fn default() -> Self {
        node_with_defaults {
            next: std::ptr::null_mut(),
            value: 3,
        }
    }
}
pub unsafe fn init_0(mut l: *mut list_head) -> *mut list_head {
    (*l).next = std::ptr::null_mut();
    return l;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut list: list_head = <list_head>::default();
    list = list_head {
        next: (&mut list as *mut list_head),
    };
    assert!(((list.next) == (&mut list as *mut list_head)));
    let mut n: node = <node>::default();
    n = node {
        value: 42,
        value_ptr: (&mut n.value as *mut i32),
    };
    assert!(((n.value_ptr) == (&mut n.value as *mut i32)));
    (*n.value_ptr) = 7;
    assert!(((n.value) == (7)));
    let mut arr: [list_head; 2] = [<list_head>::default(); 2];
    arr = [
        list_head {
            next: (&mut arr[(1) as usize] as *mut list_head),
        },
        list_head {
            next: (&mut arr[(0) as usize] as *mut list_head),
        },
    ];
    assert!(((arr[(0) as usize].next) == (&mut arr[(1) as usize] as *mut list_head)));
    assert!(((arr[(1) as usize].next) == (&mut arr[(0) as usize] as *mut list_head)));
    let mut d: node_with_defaults = <node_with_defaults>::default();
    d = node_with_defaults {
        next: (&mut d as *mut node_with_defaults),
        value: 3,
    };
    assert!(((d.next) == (&mut d as *mut node_with_defaults)));
    assert!(((d.value) == (3)));
    let mut called: list_head = <list_head>::default();
    called = list_head {
        next: (unsafe { init_0((&mut called as *mut list_head)) }),
    };
    assert!(((called.next) == (&mut called as *mut list_head)));
    let mut size: usize = ::std::mem::size_of::<usize>();
    assert!(((size) == (::std::mem::size_of::<usize>())));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
