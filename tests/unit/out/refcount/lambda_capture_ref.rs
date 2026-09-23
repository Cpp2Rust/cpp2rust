extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct S {
    pub x: Value<i32>,
    pub y: Value<i32>,
}
impl Clone for S {
    fn clone(&self) -> Self {
        let __this: Value<S> = Rc::new(RefCell::new(Self {
            x: Rc::new(RefCell::new((*self.x.borrow()))),
            y: Rc::new(RefCell::new((*self.y.borrow()))),
        }));
        let this: Ptr<S> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for S {
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
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let base: Value<i32> = Rc::new(RefCell::new(10));
    let add_base: Value<lambda_0> = Rc::new(RefCell::new(
        (lambda_0 {
            base: base.as_pointer(),
        }),
    ));
    assert!((({ lambda_0::operator_call(&(*add_base.borrow_mut()), 5,) }) == 15));
    (*base.borrow_mut()) = 100;
    assert!((({ lambda_0::operator_call(&(*add_base.borrow_mut()), 5,) }) == 105));
    let s: Value<S> = Rc::new(RefCell::new(S {
        x: Rc::new(RefCell::new(1)),
        y: Rc::new(RefCell::new(2)),
    }));
    let sum: Value<lambda_1> = Rc::new(RefCell::new((lambda_1 { s: s.as_pointer() })));
    assert!((({ lambda_1::operator_call(&(*sum.borrow_mut()),) }) == 3));
    (*(*s.borrow()).x.borrow_mut()) = 50;
    assert!((({ lambda_1::operator_call(&(*sum.borrow_mut()),) }) == 52));
    let counter: Value<i32> = Rc::new(RefCell::new(0));
    let bump: Value<lambda_2> = Rc::new(RefCell::new(
        (lambda_2 {
            counter: counter.as_pointer(),
        }),
    ));
    ({ lambda_2::operator_call(&(*bump.borrow_mut())) });
    ({ lambda_2::operator_call(&(*bump.borrow_mut())) });
    assert!(((*counter.borrow()) == 2));
    let arr: Value<Box<[u16]>> = Rc::new(RefCell::new(Box::new([3_u16, 1_u16, 2_u16, 0_u16])));
    let swap: Value<lambda_3> = Rc::new(RefCell::new(
        (lambda_3 {
            arr: (arr.as_pointer() as Ptr<u16>),
        }),
    ));
    ({ lambda_3::operator_call(&(*swap.borrow_mut()), 0_usize, 3_usize) });
    assert!((((*arr.borrow())[(0) as usize] as i32) == 0));
    assert!((((*arr.borrow())[(3) as usize] as i32) == 3));
    return 0;
}
#[derive(Clone, Default)]
pub struct lambda_0 {
    base: Ptr<i32>,
}
impl lambda_0 {
    pub fn operator_call(&self, x: i32) -> i32 {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        return ((*x.borrow()) + (self.base.read()));
    }
}
impl ByteRepr for lambda_0 {}
impl Callable1<i32, i32> for lambda_0 {
    fn call(&self, a1: i32) -> i32 {
        { lambda_0::operator_call(self, a1) }
    }
}
#[derive(Clone, Default)]
pub struct lambda_1 {
    s: Ptr<S>,
}
impl lambda_1 {
    pub fn operator_call(&self) -> i32 {
        return ((*(*self.s.upgrade().deref()).x.borrow())
            + (*(*self.s.upgrade().deref()).y.borrow()));
    }
}
impl ByteRepr for lambda_1 {}
impl Callable0<i32> for lambda_1 {
    fn call(&self) -> i32 {
        { lambda_1::operator_call(self) }
    }
}
#[derive(Clone, Default)]
pub struct lambda_2 {
    counter: Ptr<i32>,
}
impl lambda_2 {
    pub fn operator_call(&self) {
        self.counter.with_mut(|__v| __v.postfix_inc());
    }
}
impl ByteRepr for lambda_2 {}
impl Callable0<()> for lambda_2 {
    fn call(&self) -> () {
        { lambda_2::operator_call(self) }
    }
}
#[derive(Clone, Default)]
pub struct lambda_3 {
    arr: Ptr<u16>,
}
impl lambda_3 {
    pub fn operator_call(&self, i: usize, j: usize) {
        let i: Value<usize> = Rc::new(RefCell::new(i));
        let j: Value<usize> = Rc::new(RefCell::new(j));
        let t: Value<u16> = Rc::new(RefCell::new(
            ((self.arr).offset((*j.borrow()) as isize).read()),
        ));
        let __rhs = ((self.arr).offset((*i.borrow()) as isize).read());
        (self.arr).offset((*j.borrow()) as isize).write(__rhs);
        (self.arr)
            .offset((*i.borrow()) as isize)
            .write((*t.borrow()));
    }
}
impl ByteRepr for lambda_3 {}
impl Callable2<usize, usize, ()> for lambda_3 {
    fn call(&self, a1: usize, a2: usize) -> () {
        { lambda_3::operator_call(self, a1, a2) }
    }
}
pub fn __cpp2rust_init_globals() {}
