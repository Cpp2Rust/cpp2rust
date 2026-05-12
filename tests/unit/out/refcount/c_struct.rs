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
impl ByteRepr for Point {}
#[derive(Default)]
pub struct Line {
    pub start: Value<Point>,
    pub end: Value<Point>,
}
impl ByteRepr for Line {}
#[derive(Default)]
pub struct Node {
    pub value: Value<i32>,
    pub next: Value<Ptr<Node>>,
}
impl ByteRepr for Node {}
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
#[derive(Default)]
pub struct Inner {
    pub a: Value<i32>,
    pub b: Value<i32>,
}
impl ByteRepr for Inner {}
#[derive(Default)]
pub struct Container {
    pub inner: Value<Inner>,
    pub color: Value<Color>,
    pub count: Value<i32>,
}
impl ByteRepr for Container {}
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
