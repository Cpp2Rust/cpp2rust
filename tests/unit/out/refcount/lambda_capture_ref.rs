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
    let add_base: Value<_> = Rc::new(RefCell::new(
        (|x: i32| {
            let x: Value<i32> = Rc::new(RefCell::new(x));
            return ((*x.borrow()) + (*base.borrow()));
        }),
    ));
    assert!((({ (*add_base.borrow_mut())(5,) }) == 15));
    (*base.borrow_mut()) = 100;
    assert!((({ (*add_base.borrow_mut())(5,) }) == 105));
    let s: Value<S> = Rc::new(RefCell::new(S {
        x: Rc::new(RefCell::new(1)),
        y: Rc::new(RefCell::new(2)),
    }));
    let sum: Value<_> = Rc::new(RefCell::new(
        (|| {
            return ((*(*s.borrow()).x.borrow()) + (*(*s.borrow()).y.borrow()));
        }),
    ));
    assert!((({ (*sum.borrow_mut())() }) == 3));
    (*(*s.borrow()).x.borrow_mut()) = 50;
    assert!((({ (*sum.borrow_mut())() }) == 52));
    let counter: Value<i32> = Rc::new(RefCell::new(0));
    let bump: Value<_> = Rc::new(RefCell::new(
        (|| {
            (*counter.borrow_mut()).postfix_inc();
        }),
    ));
    ({ (*bump.borrow_mut())() });
    ({ (*bump.borrow_mut())() });
    assert!(((*counter.borrow()) == 2));
    let arr: Value<Box<[u16]>> = Rc::new(RefCell::new(Box::new([3_u16, 1_u16, 2_u16, 0_u16])));
    let swap: Value<_> = Rc::new(RefCell::new(
        (|i: usize, j: usize| {
            let i: Value<usize> = Rc::new(RefCell::new(i));
            let j: Value<usize> = Rc::new(RefCell::new(j));
            let t: Value<u16> = Rc::new(RefCell::new((*arr.borrow())[(*j.borrow()) as usize]));
            let __rhs = (*arr.borrow())[(*i.borrow()) as usize];
            (*arr.borrow_mut())[(*j.borrow()) as usize] = __rhs;
            (*arr.borrow_mut())[(*i.borrow()) as usize] = (*t.borrow());
        }),
    ));
    ({ (*swap.borrow_mut())(0_usize, 3_usize) });
    assert!((((*arr.borrow())[(0) as usize] as i32) == 0));
    assert!((((*arr.borrow())[(3) as usize] as i32) == 3));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
