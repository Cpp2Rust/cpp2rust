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
        let __this: Value<GraphNode> = Rc::new(RefCell::new(Self {
            dst: Rc::new(RefCell::new((*self.dst.borrow()))),
            next: Rc::new(RefCell::new((*self.next.borrow()).clone())),
        }));
        let this: Ptr<GraphNode> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
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
impl Clone for Graph {
    fn clone(&self) -> Self {
        let __this: Value<Graph> = Rc::new(RefCell::new(Self {
            V: Rc::new(RefCell::new((*self.V.borrow()))),
            adj: Rc::new(RefCell::new((*self.adj.borrow()).clone())),
        }));
        let this: Ptr<Graph> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
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
#[derive(Default)]
pub struct Partial {
    pub p: Value<Ptr<i32>>,
}
impl Partial {
    pub fn new_1(q: Ptr<i32>) -> Self {
        let q: Value<Ptr<i32>> = Rc::new(RefCell::new(q));
        let __this: Value<Partial> = Rc::new(RefCell::new(Self {
            p: Rc::new(RefCell::new((*q.borrow()).clone())),
        }));
        let this: Ptr<Partial> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for Partial {
    fn clone(&self) -> Self {
        let __this: Value<Partial> = Rc::new(RefCell::new(Self {
            p: Rc::new(RefCell::new((*self.p.borrow()).clone())),
        }));
        let this: Ptr<Partial> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for Partial {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.p.borrow()).to_bytes(&mut buf[0..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            p: Rc::new(RefCell::new(<Ptr<i32>>::from_bytes(&buf[0..8]))),
        }
    }
}
#[derive(Clone, ByteRepr, Default)]
pub struct Declared {}
impl Declared {}
#[derive(Default)]
pub struct S {
    pub i: Value<i32>,
    pub d: Value<Ptr<Declared>>,
}
impl Clone for S {
    fn clone(&self) -> Self {
        let __this: Value<S> = Rc::new(RefCell::new(Self {
            i: Rc::new(RefCell::new((*self.i.borrow()))),
            d: Rc::new(RefCell::new((*self.d.borrow()).clone())),
        }));
        let this: Ptr<S> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for S {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.i.borrow()).to_bytes(&mut buf[0..4]);
        (*self.d.borrow()).to_bytes(&mut buf[8..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            i: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
            d: Rc::new(RefCell::new(<Ptr<Declared>>::from_bytes(&buf[8..16]))),
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let g: Value<Graph> = Rc::new(RefCell::new(Graph {
        V: Rc::new(RefCell::new(5_u32)),
        adj: Rc::new(RefCell::new(Ptr::<Ptr<GraphNode>>::null())),
    }));
    let arr: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([3, 1, 4])));
    let it: Value<Partial> = Rc::new(RefCell::new(Partial::new_1({
        (arr.as_pointer() as Ptr<i32>)
    })));
    if {
        let _lhs = (*(*it.borrow()).p.borrow()).clone();
        _lhs != (arr.as_pointer() as Ptr<i32>)
    } {
        return 1;
    }
    let def: Value<Partial> = Rc::new(RefCell::new(<Partial>::default()));
    if !((*(*def.borrow()).p.borrow()).is_null()) {
        return 1;
    }
    let s: Value<S> = Rc::new(RefCell::new(S {
        i: Rc::new(RefCell::new(7)),
        d: Rc::new(RefCell::new(Ptr::<Declared>::null())),
    }));
    if ((*(*s.borrow()).i.borrow()) != 7) || (!((*(*s.borrow()).d.borrow()).is_null())) {
        return 1;
    }
    return 0;
}
pub trait GraphImpl {
    fn push(&self, src: u32, dst: u32);
}
impl GraphImpl for Ptr<Graph> {
    fn push(&self, src: u32, dst: u32) {
        let src: Value<u32> = Rc::new(RefCell::new(src));
        let dst: Value<u32> = Rc::new(RefCell::new(dst));
        let __rhs = Ptr::alloc(GraphNode {
            dst: Rc::new(RefCell::new((*dst.borrow()))),
            next: Rc::new(RefCell::new(
                ((*(*(*self).upgrade().deref()).adj.borrow())
                    .offset((*src.borrow()) as isize)
                    .read())
                .clone(),
            )),
        });
        (*(*(*self).upgrade().deref()).adj.borrow())
            .offset((*src.borrow()) as isize)
            .write(__rhs);
        let __rhs = Ptr::alloc(GraphNode {
            dst: Rc::new(RefCell::new((*src.borrow()))),
            next: Rc::new(RefCell::new(
                ((*(*(*self).upgrade().deref()).adj.borrow())
                    .offset((*dst.borrow()) as isize)
                    .read())
                .clone(),
            )),
        });
        (*(*(*self).upgrade().deref()).adj.borrow())
            .offset((*dst.borrow()) as isize)
            .write(__rhs);
    }
}
pub trait PartialImpl {
    fn get(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn next(&self) -> Ptr<Partial> {
        unimplemented!()
    }
    fn next_i32(&self, _a0: i32) -> Partial {
        unimplemented!()
    }
}
impl PartialImpl for Ptr<Partial> {}
pub fn __cpp2rust_init_globals() {}
