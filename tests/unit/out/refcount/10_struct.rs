extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct GraphNode {
    pub dst: Value<u32>,
    pub next: Value<Ptr<GraphNode>>,
}
impl Clone for GraphNode {
    fn clone(&self) -> Self {
        let mut this = Self {
            dst: Rc::new(RefCell::new((*self.dst.borrow()))),
            next: Rc::new(RefCell::new((*self.next.borrow()).clone())),
        };
        this
    }
}
impl ByteRepr for GraphNode {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.dst.borrow()).to_bytes(&mut buf[0..4]);
        (*self.next.borrow()).to_bytes(&mut buf[8..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            dst: Rc::new(RefCell::new(<u32>::from_bytes(&buf[0..4]))),
            next: Rc::new(RefCell::new(<Ptr<GraphNode>>::from_bytes(&buf[8..16]))),
        }
    }
}
#[derive(Default)]
pub struct Graph {
    pub V: Value<u32>,
    pub adj: Value<Ptr<Ptr<GraphNode>>>,
}
impl Graph {
    pub fn push(&self, src: u32, dst: u32) {
        let src: Value<u32> = Rc::new(RefCell::new(src));
        let dst: Value<u32> = Rc::new(RefCell::new(dst));
        let __rhs = Ptr::alloc(GraphNode {
            dst: Rc::new(RefCell::new((*dst.borrow()))),
            next: Rc::new(RefCell::new(
                ((*self.adj.borrow()).offset((*src.borrow()) as isize).read()).clone(),
            )),
        });
        (*self.adj.borrow())
            .offset((*src.borrow()) as isize)
            .write(__rhs);
        let __rhs = Ptr::alloc(GraphNode {
            dst: Rc::new(RefCell::new((*src.borrow()))),
            next: Rc::new(RefCell::new(
                ((*self.adj.borrow()).offset((*dst.borrow()) as isize).read()).clone(),
            )),
        });
        (*self.adj.borrow())
            .offset((*dst.borrow()) as isize)
            .write(__rhs);
    }
}
impl Clone for Graph {
    fn clone(&self) -> Self {
        let mut this = Self {
            V: Rc::new(RefCell::new((*self.V.borrow()))),
            adj: Rc::new(RefCell::new((*self.adj.borrow()).clone())),
        };
        this
    }
}
impl ByteRepr for Graph {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.V.borrow()).to_bytes(&mut buf[0..4]);
        (*self.adj.borrow()).to_bytes(&mut buf[8..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            V: Rc::new(RefCell::new(<u32>::from_bytes(&buf[0..4]))),
            adj: Rc::new(RefCell::new(<Ptr<Ptr<GraphNode>>>::from_bytes(&buf[8..16]))),
        }
    }
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let g: Value<Graph> = Rc::new(RefCell::new(Graph {
        V: Rc::new(RefCell::new(5_u32)),
        adj: Rc::new(RefCell::new(Ptr::<Ptr<GraphNode>>::null())),
    }));
    return 0;
}
