extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, VaArg, FnPtrArg, Default)]
pub struct Point {
    #[offset(0)]
    pub x: i32,
    #[offset(4)]
    pub y: i32,
}
impl ByteRepr for Point {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.x.to_bytes(&mut buf[0..4]);
        self.y.to_bytes(&mut buf[4..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            x: <i32>::from_bytes(&buf[0..4]),
            y: <i32>::from_bytes(&buf[4..8]),
        }
    }
}
#[derive(Clone, Record, VaArg, FnPtrArg, Default)]
pub struct Line {
    #[offset(0)]
    pub start: Point,
    #[offset(8)]
    pub end: Point,
}
impl ByteRepr for Line {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.start.to_bytes(&mut buf[0..8]);
        self.end.to_bytes(&mut buf[8..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            start: <Point>::from_bytes(&buf[0..8]),
            end: <Point>::from_bytes(&buf[8..16]),
        }
    }
}
#[derive(Clone, Record, VaArg, FnPtrArg, Default)]
pub struct Node {
    #[offset(0)]
    pub value: i32,
    #[offset(8)]
    pub next: Ptr<Node>,
}
impl ByteRepr for Node {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.value.to_bytes(&mut buf[0..4]);
        self.next.to_bytes(&mut buf[8..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            value: <i32>::from_bytes(&buf[0..4]),
            next: <Ptr<Node>>::from_bytes(&buf[8..16]),
        }
    }
}
pub type Color = u32;
pub const Color_RED: Color = 0;
pub const Color_GREEN: Color = 1;
pub const Color_BLUE: Color = 2;
#[derive(Clone, Record, VaArg, FnPtrArg, Default)]
pub struct Inner {
    #[offset(0)]
    pub a: i32,
    #[offset(4)]
    pub b: i32,
}
impl ByteRepr for Inner {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.a.to_bytes(&mut buf[0..4]);
        self.b.to_bytes(&mut buf[4..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            a: <i32>::from_bytes(&buf[0..4]),
            b: <i32>::from_bytes(&buf[4..8]),
        }
    }
}
#[derive(Clone, Record, VaArg, FnPtrArg, Default)]
pub struct Container {
    #[offset(0)]
    pub inner: Inner,
    #[offset(8)]
    pub color: Color,
    #[offset(12)]
    pub count: i32,
}
impl ByteRepr for Container {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.inner.to_bytes(&mut buf[0..8]);
        self.color.to_bytes(&mut buf[8..12]);
        self.count.to_bytes(&mut buf[12..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            inner: <Inner>::from_bytes(&buf[0..8]),
            color: <Color>::from_bytes(&buf[8..12]),
            count: <i32>::from_bytes(&buf[12..16]),
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let p: Value<Point> = Rc::new(RefCell::new(Point { x: 10, y: 20 }));
    assert!(((({ (*p.borrow()).x } == 10) as i32) != 0));
    assert!(((({ (*p.borrow()).y } == 20) as i32) != 0));
    let q: Value<Point> = Rc::new(RefCell::new((*p.borrow()).clone()));
    (*q.borrow_mut()).x = 99;
    assert!(((({ (*p.borrow()).x } == 10) as i32) != 0));
    assert!(((({ (*q.borrow()).x } == 99) as i32) != 0));
    assert!(((({ (*q.borrow()).y } == 20) as i32) != 0));
    let l: Value<Line> = Rc::new(RefCell::new(Line {
        start: Point { x: 1, y: 2 },
        end: Point { x: 3, y: 4 },
    }));
    assert!(((({ (*l.borrow()).start.x } == 1) as i32) != 0));
    assert!(((({ (*l.borrow()).end.y } == 4) as i32) != 0));
    let a: Value<Node> = Rc::new(RefCell::new(Node {
        value: 1,
        next: Ptr::<Node>::null(),
    }));
    let b: Value<Node> = Rc::new(RefCell::new(Node {
        value: 2,
        next: (a.as_pointer()),
    }));
    assert!(((({ (*b.borrow()).next.clone() }.with(|__s: &Node| __s.value) == 1) as i32) != 0));
    let c: Value<Container> = Rc::new(RefCell::new(Container {
        inner: Inner { a: 5, b: 6 },
        color: Color_GREEN,
        count: 42,
    }));
    assert!(((({ (*c.borrow()).inner.a } == 5) as i32) != 0));
    assert!(((({ (*c.borrow()).inner.b } == 6) as i32) != 0));
    assert!((((({ (*c.borrow()).color } as u32) == ((Color_GREEN as i32) as u32)) as i32) != 0));
    assert!(((({ (*c.borrow()).count } == 42) as i32) != 0));
    let c2: Value<Container> = <Value<Container>>::default();
    (*c2.borrow_mut()).color = Color_BLUE;
    assert!((((({ (*c2.borrow()).color } as u32) == 2_u32) as i32) != 0));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
