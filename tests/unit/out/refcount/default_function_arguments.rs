extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn foo_0(a: i32, b: Option<i32>) -> i32 {
    let a: Value<i32> = Rc::new(RefCell::new(a));
    let b: Value<i32> = Rc::new(RefCell::new(b.unwrap_or(10)));
    return ((*a.borrow()) + (*b.borrow()));
}
pub fn baz_1(a: Ptr<i32>, b: Option<Ptr<i32>>) -> bool {
    let a: Value<Ptr<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Ptr<i32>> = Rc::new(RefCell::new(b.unwrap_or(Ptr::<i32>::null())));
    return {
        let _lhs = (*a.borrow()).clone();
        _lhs == (*b.borrow()).clone()
    };
}
#[derive()]
pub struct Bar {
    pub v: Value<i32>,
}
impl Bar {
    pub fn Bar(v: Option<i32>) -> Self {
        let v: Value<i32> = Rc::new(RefCell::new(v.unwrap_or(1)));
        let __this: Value<Bar> = Rc::new(RefCell::new(Self {
            v: Rc::new(RefCell::new((*v.borrow()))),
        }));
        let this: Ptr<Bar> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for Bar {
    fn clone(&self) -> Self {
        let __this: Value<Bar> = Rc::new(RefCell::new(Self {
            v: Rc::new(RefCell::new((*self.v.borrow()))),
        }));
        let this: Ptr<Bar> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for Bar {
    fn default() -> Self {
        { Bar::Bar(None) }
    }
}
impl ByteRepr for Bar {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.v.borrow()).to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            v: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
        }
    }
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!((({ foo_0(1, None,) }) == 11));
    assert!((({ foo_0(1, Some(2),) }) == 3));
    let a: Value<i32> = Rc::new(RefCell::new(0));
    assert!(((({ baz_1((a.as_pointer()), None,) }) as i32) == (false as i32)));
    assert!(
        ((({
            let _a: Ptr<i32> = (a.as_pointer());
            let _b: Ptr<i32> = (a.as_pointer());
            baz_1(_a, Some(_b))
        }) as i32)
            == (true as i32))
    );
    let b: Value<Bar> = Rc::new(RefCell::new(Bar::Bar({ Some() })));
    assert!(((*(*b.borrow()).v.borrow()) == 1));
    assert!(((*Bar::Bar({ Some(2) },).v.borrow()) == 2));
    let arr: Value<Box<[Bar]>> = Rc::new(RefCell::new(Box::new([
        Bar::Bar({ Some() }),
        Bar::Bar({ Some() }),
        Bar::Bar({ Some() }),
    ])));
    assert!(((*(*arr.borrow())[(0) as usize].v.borrow()) == 1));
    assert!(((*(*arr.borrow())[(2) as usize].v.borrow()) == 1));
    return 0;
}
