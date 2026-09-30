extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, VaArg, FnPtrArg, Default)]
pub struct GraphNode {
    #[offset(0)]
    pub dst: u32,
    #[offset(8)]
    pub next: Ptr<GraphNode>,
}
impl ByteRepr for GraphNode {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.dst.to_bytes(&mut buf[0..4]);
        self.next.to_bytes(&mut buf[8..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            dst: <u32>::from_bytes(&buf[0..4]),
            next: <Ptr<GraphNode>>::from_bytes(&buf[8..16]),
        }
    }
}
#[derive(Clone, Record, VaArg, FnPtrArg, Default)]
pub struct Graph {
    #[offset(0)]
    pub V: u32,
    #[offset(8)]
    pub adj: Ptr<Ptr<GraphNode>>,
}
impl ByteRepr for Graph {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.V.to_bytes(&mut buf[0..4]);
        self.adj.to_bytes(&mut buf[8..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            V: <u32>::from_bytes(&buf[0..4]),
            adj: <Ptr<Ptr<GraphNode>>>::from_bytes(&buf[8..16]),
        }
    }
}
#[derive(Clone, Record, VaArg, FnPtrArg)]
pub struct Partial {
    #[offset(0)]
    pub p: Ptr<i32>,
}
impl Partial {
    pub fn new_1(q: Ptr<i32>) -> Self {
        let q: Value<Ptr<i32>> = Rc::new(RefCell::new(q));
        let __this: Value<Partial> = Rc::new(RefCell::new(Self {
            p: (*q.borrow()).clone(),
        }));
        let this: Ptr<Partial> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for Partial {
    fn default() -> Self {
        Partial {
            p: Ptr::<i32>::null(),
        }
    }
}
impl ByteRepr for Partial {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.p.to_bytes(&mut buf[0..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            p: <Ptr<i32>>::from_bytes(&buf[0..8]),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
pub struct Declared {}
impl Declared {}
#[derive(Clone, Record, VaArg, FnPtrArg, Default)]
pub struct S {
    #[offset(0)]
    pub i: i32,
    #[offset(8)]
    pub d: Ptr<Declared>,
}
impl ByteRepr for S {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.i.to_bytes(&mut buf[0..4]);
        self.d.to_bytes(&mut buf[8..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            i: <i32>::from_bytes(&buf[0..4]),
            d: <Ptr<Declared>>::from_bytes(&buf[8..16]),
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let g: Value<Graph> = Rc::new(RefCell::new(Graph {
        V: 5_u32,
        adj: Ptr::<Ptr<GraphNode>>::null(),
    }));
    let arr: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([3, 1, 4])));
    let it: Value<Partial> = Rc::new(RefCell::new(Partial::new_1({
        (arr.as_pointer() as Ptr<i32>)
    })));
    if {
        let _lhs = { (*it.borrow()).p.clone() };
        _lhs != (arr.as_pointer() as Ptr<i32>)
    } {
        return 1;
    }
    let def: Value<Partial> = Rc::new(RefCell::new(<Partial>::default()));
    if !(({ (*def.borrow()).p.clone() }).is_null()) {
        return 1;
    }
    let s: Value<S> = Rc::new(RefCell::new(S {
        i: 7,
        d: Ptr::<Declared>::null(),
    }));
    if ({ (*s.borrow()).i } != 7) || (!(({ (*s.borrow()).d.clone() }).is_null())) {
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
            dst: (*dst.borrow()),
            next: {
                ((*self)
                    .with(|__s: &Graph| (__s.adj).clone())
                    .offset((*src.borrow()) as isize)
                    .read())
                .clone()
            },
        });
        {
            (*self)
                .with(|__s: &Graph| (__s.adj).clone())
                .offset((*src.borrow()) as isize)
        }
        .write(__rhs);
        let __rhs = Ptr::alloc(GraphNode {
            dst: (*src.borrow()),
            next: {
                ((*self)
                    .with(|__s: &Graph| (__s.adj).clone())
                    .offset((*dst.borrow()) as isize)
                    .read())
                .clone()
            },
        });
        {
            (*self)
                .with(|__s: &Graph| (__s.adj).clone())
                .offset((*dst.borrow()) as isize)
        }
        .write(__rhs);
    }
}
pub trait PartialImpl {
    fn get(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn next_4(&self) -> Ptr<Partial> {
        unimplemented!()
    }
    fn next_5(&self, _a0: i32) -> Partial {
        unimplemented!()
    }
}
impl PartialImpl for Ptr<Partial> {}
pub fn __cpp2rust_init_globals() {}
