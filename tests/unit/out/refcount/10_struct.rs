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
impl ByteRepr for GraphNode {}
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
impl ByteRepr for Graph {}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let g: Value<Graph> = Rc::new(RefCell::new(Graph {
        V: Rc::new(RefCell::new(5_u32)),
        adj: Rc::new(RefCell::new(Default::default())),
    }));
    return 0;
}
