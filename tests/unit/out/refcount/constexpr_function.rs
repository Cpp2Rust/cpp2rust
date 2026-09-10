extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn runtime_only_0(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    return ((*x.borrow()) * 2);
}
pub fn first_1(p: Ptr<i32>) -> i32 {
    let p: Value<Ptr<i32>> = Rc::new(RefCell::new(p));
    return ((*p.borrow()).read());
}
pub fn scaled_2(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    if ((*x.borrow()) < 0) {
        return ({ runtime_only_0(-(*x.borrow())) });
    }
    return (*x.borrow());
}
pub fn half_3(x: f64) -> f64 {
    let x: Value<f64> = Rc::new(RefCell::new(x));
    return ((*x.borrow()) / 2.0E+0);
}
#[derive(Default)]
pub struct P {
    pub v: Value<i32>,
}
impl Clone for P {
    fn clone(&self) -> Self {
        let __this: Value<P> = Rc::new(RefCell::new(Self {
            v: Rc::new(RefCell::new((*self.v.borrow()))),
        }));
        let this: Ptr<P> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for P {
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
    let arr: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([7, 8])));
    assert!((({ first_1((arr.as_pointer() as Ptr::<i32>),) }) == 7));
    assert!((({ first_1((arr.as_pointer() as Ptr::<i32>).offset((1) as isize),) }) == 8));
    assert!((({ scaled_2(3,) }) == 3));
    assert!((({ scaled_2(-3_i32,) }) == 6));
    assert!((({ half_3(5.0E+0,) }) == 2.5E+0));
    let p: Value<P> = Rc::new(RefCell::new(P {
        v: Rc::new(RefCell::new(9)),
    }));
    assert!((({ PImpl::get(&p.as_pointer(),) }) == 9));
    let k: Value<i32> = Rc::new(RefCell::new(({ scaled_2(4) })));
    assert!(((*k.borrow()) == 4));
    return 0;
}
pub trait PImpl {
    fn get(&self) -> i32;
}
impl PImpl for Ptr<P> {
    fn get(&self) -> i32 {
        return (*(*(*self).upgrade().deref()).v.borrow());
    }
}
