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
pub struct GraphNode {
    pub dst: u32,
    pub next: *mut GraphNode,
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Graph {
    pub V: u32,
    pub adj: *mut *mut GraphNode,
}
impl Graph {
    pub unsafe fn push(&self, mut src: u32, mut dst: u32) {
        (*self.adj.offset((src) as isize)) = (Box::leak(Box::new(GraphNode {
            dst: dst,
            next: (*self.adj.offset((src) as isize)),
        })) as *mut GraphNode);
        (*self.adj.offset((dst) as isize)) = (Box::leak(Box::new(GraphNode {
            dst: src,
            next: (*self.adj.offset((dst) as isize)),
        })) as *mut GraphNode);
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Partial {
    pub p: *mut i32,
}
impl Partial {
    pub unsafe fn Partial1(mut q: *mut i32) -> Self {
        let mut this = Self { p: q };
        this
    }
}
impl Default for Partial {
    fn default() -> Self {
        Partial {
            p: std::ptr::null_mut(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Declared {}
impl Declared {}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct S {
    pub i: i32,
    pub d: *mut Declared,
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut g: Graph = Graph {
        V: 5_u32,
        adj: std::ptr::null_mut(),
    };
    let mut arr: [i32; 3] = [3, 1, 4];
    let mut it: Partial = Partial::Partial1({ arr.as_mut_ptr() });
    if ((it.p) != (arr.as_mut_ptr())) {
        return 1;
    }
    let mut def: Partial = <Partial>::default();
    if !((def.p).is_null()) {
        return 1;
    }
    let mut s: S = S {
        i: 7,
        d: std::ptr::null_mut(),
    };
    if ((s.i) != (7)) || (!((s.d).is_null())) {
        return 1;
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
