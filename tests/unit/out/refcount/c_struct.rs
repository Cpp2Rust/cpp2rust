extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct Point {
    pub x: Value<i32>,
    pub y: Value<i32>,
}
impl Clone for Point {
    fn clone(&self) -> Self {
        Self {
            x: Rc::new(RefCell::new((*self.x.borrow()).clone())),
            y: Rc::new(RefCell::new((*self.y.borrow()).clone())),
        }
    }
}
impl ByteRepr for Point {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.x.borrow()).to_bytes(&mut buf[0..4]);
        (*self.y.borrow()).to_bytes(&mut buf[4..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            x: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
            y: Rc::new(RefCell::new(<i32>::from_bytes(&buf[4..8]))),
        }
    }
}
#[derive(Default)]
pub struct Line {
    pub start: Value<Point>,
    pub end: Value<Point>,
}
impl Clone for Line {
    fn clone(&self) -> Self {
        Self {
            start: Rc::new(RefCell::new((*self.start.borrow()).clone())),
            end: Rc::new(RefCell::new((*self.end.borrow()).clone())),
        }
    }
}
impl ByteRepr for Line {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.start.borrow()).to_bytes(&mut buf[0..8]);
        (*self.end.borrow()).to_bytes(&mut buf[8..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            start: Rc::new(RefCell::new(<Point>::from_bytes(&buf[0..8]))),
            end: Rc::new(RefCell::new(<Point>::from_bytes(&buf[8..16]))),
        }
    }
}
#[derive(Default)]
pub struct Node {
    pub value: Value<i32>,
    pub next: Value<Ptr<Node>>,
}
impl Clone for Node {
    fn clone(&self) -> Self {
        Self {
            value: Rc::new(RefCell::new((*self.value.borrow()).clone())),
            next: Rc::new(RefCell::new((*self.next.borrow()).clone())),
        }
    }
}
impl ByteRepr for Node {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.value.borrow()).to_bytes(&mut buf[0..4]);
        (*self.next.borrow()).to_bytes(&mut buf[8..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            value: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
            next: Rc::new(RefCell::new(<Ptr<Node>>::from_bytes(&buf[8..16]))),
        }
    }
}
#[derive(Clone, Copy, PartialEq, Debug, Default)]
enum Color {
    #[default]
    RED = 0,
    GREEN = 1,
    BLUE = 2,
}
impl From<i32> for Color {
    fn from(n: i32) -> Color {
        match n {
            0 => Color::RED,
            1 => Color::GREEN,
            2 => Color::BLUE,
            _ => panic!("invalid Color value: {}", n),
        }
    }
}
libcc2rs::impl_enum_inc_dec!(Color);
impl ByteRepr for Color {
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self as i32).to_bytes(buf);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        <Color>::from(i32::from_bytes(buf))
    }
}
#[derive(Default)]
pub struct Inner {
    pub a: Value<i32>,
    pub b: Value<i32>,
}
impl Clone for Inner {
    fn clone(&self) -> Self {
        Self {
            a: Rc::new(RefCell::new((*self.a.borrow()).clone())),
            b: Rc::new(RefCell::new((*self.b.borrow()).clone())),
        }
    }
}
impl ByteRepr for Inner {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.a.borrow()).to_bytes(&mut buf[0..4]);
        (*self.b.borrow()).to_bytes(&mut buf[4..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            a: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
            b: Rc::new(RefCell::new(<i32>::from_bytes(&buf[4..8]))),
        }
    }
}
#[derive(Default)]
pub struct Container {
    pub inner: Value<Inner>,
    pub color: Value<Color>,
    pub count: Value<i32>,
}
impl Clone for Container {
    fn clone(&self) -> Self {
        Self {
            inner: Rc::new(RefCell::new((*self.inner.borrow()).clone())),
            color: Rc::new(RefCell::new((*self.color.borrow()).clone())),
            count: Rc::new(RefCell::new((*self.count.borrow()).clone())),
        }
    }
}
impl ByteRepr for Container {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.inner.borrow()).to_bytes(&mut buf[0..8]);
        (*self.color.borrow()).to_bytes(&mut buf[8..12]);
        (*self.count.borrow()).to_bytes(&mut buf[12..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            inner: Rc::new(RefCell::new(<Inner>::from_bytes(&buf[0..8]))),
            color: Rc::new(RefCell::new(<Color>::from_bytes(&buf[8..12]))),
            count: Rc::new(RefCell::new(<i32>::from_bytes(&buf[12..16]))),
        }
    }
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let p: Value<Point> = Rc::new(RefCell::new(Point {
        x: Rc::new(RefCell::new(10)),
        y: Rc::new(RefCell::new(20)),
    }));
    assert!(((((*(*p.borrow()).x.borrow()) == 10) as i32) != 0));
    assert!(((((*(*p.borrow()).y.borrow()) == 20) as i32) != 0));
    let q: Value<Point> = Rc::new(RefCell::new((*p.borrow()).clone()));
    (*(*q.borrow()).x.borrow_mut()) = 99;
    assert!(((((*(*p.borrow()).x.borrow()) == 10) as i32) != 0));
    assert!(((((*(*q.borrow()).x.borrow()) == 99) as i32) != 0));
    assert!(((((*(*q.borrow()).y.borrow()) == 20) as i32) != 0));
    let l: Value<Line> = Rc::new(RefCell::new(Line {
        start: Rc::new(RefCell::new(Point {
            x: Rc::new(RefCell::new(1)),
            y: Rc::new(RefCell::new(2)),
        })),
        end: Rc::new(RefCell::new(Point {
            x: Rc::new(RefCell::new(3)),
            y: Rc::new(RefCell::new(4)),
        })),
    }));
    assert!(((((*(*(*l.borrow()).start.borrow()).x.borrow()) == 1) as i32) != 0));
    assert!(((((*(*(*l.borrow()).end.borrow()).y.borrow()) == 4) as i32) != 0));
    let a: Value<Node> = Rc::new(RefCell::new(Node {
        value: Rc::new(RefCell::new(1)),
        next: Rc::new(RefCell::new(Ptr::<Node>::null())),
    }));
    let b: Value<Node> = Rc::new(RefCell::new(Node {
        value: Rc::new(RefCell::new(2)),
        next: Rc::new(RefCell::new((a.as_pointer()))),
    }));
    assert!(
        ((((*(*(*(*b.borrow()).next.borrow()).upgrade().deref())
            .value
            .borrow())
            == 1) as i32)
            != 0)
    );
    let c: Value<Container> = Rc::new(RefCell::new(Container {
        inner: Rc::new(RefCell::new(Inner {
            a: Rc::new(RefCell::new(5)),
            b: Rc::new(RefCell::new(6)),
        })),
        color: Rc::new(RefCell::new(Color::GREEN)),
        count: Rc::new(RefCell::new(42)),
    }));
    assert!(((((*(*(*c.borrow()).inner.borrow()).a.borrow()) == 5) as i32) != 0));
    assert!(((((*(*(*c.borrow()).inner.borrow()).b.borrow()) == 6) as i32) != 0));
    assert!(
        (((((*(*c.borrow()).color.borrow()) as u32) == ((Color::GREEN as i32) as u32)) as i32)
            != 0)
    );
    assert!(((((*(*c.borrow()).count.borrow()) == 42) as i32) != 0));
    let c2: Value<Container> = <Value<Container>>::default();
    (*(*c2.borrow()).color.borrow_mut()) = Color::BLUE;
    assert!((((((*(*c2.borrow()).color.borrow()) as u32) == 2_u32) as i32) != 0));
    return 0;
}
