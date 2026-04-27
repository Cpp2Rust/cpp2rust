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
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut g: Graph = Graph {
        V: 5_u32,
        adj: Default::default(),
    };
    return 0;
}
