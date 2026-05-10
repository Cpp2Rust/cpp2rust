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
pub struct Point {
    pub x: i32,
    pub y: i32,
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Line {
    pub start: Point,
    pub end: Point,
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Node {
    pub value: i32,
    pub next: *mut Node,
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
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Inner {
    pub a: i32,
    pub b: i32,
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Container {
    pub inner: Inner,
    pub color: Color,
    pub count: i32,
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut p: Point = Point { x: 10, y: 20 };
    assert!(((((p.x) == (10)) as i32) != 0));
    assert!(((((p.y) == (20)) as i32) != 0));
    let mut l: Line = Line {
        start: Point { x: 1, y: 2 },
        end: Point { x: 3, y: 4 },
    };
    assert!(((((l.start.x) == (1)) as i32) != 0));
    assert!(((((l.end.y) == (4)) as i32) != 0));
    let mut a: Node = Node {
        value: 1,
        next: Default::default(),
    };
    let mut b: Node = Node {
        value: 2,
        next: (&mut a as *mut Node),
    };
    assert!((((((*b.next).value) == (1)) as i32) != 0));
    let mut c: Container = Container {
        inner: Inner { a: 5, b: 6 },
        color: Color::GREEN,
        count: 42,
    };
    assert!(((((c.inner.a) == (5)) as i32) != 0));
    assert!(((((c.inner.b) == (6)) as i32) != 0));
    assert!(((((c.color as u32) == ((Color::GREEN as i32) as u32)) as i32) != 0));
    assert!(((((c.count) == (42)) as i32) != 0));
    let mut c2: Container = <Container>::default();
    c2.color = Color::BLUE;
    assert!(((((c2.color as u32) == (2_u32)) as i32) != 0));
    return 0;
}
