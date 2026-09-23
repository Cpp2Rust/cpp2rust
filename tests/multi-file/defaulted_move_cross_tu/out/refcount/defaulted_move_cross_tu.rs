extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive()]
pub struct S {
    pub v: Value<Vec<i32>>,
    pub n: Value<Box<[i32]>>,
}
impl S {
    pub fn new(x: i32) -> Self {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        let __this: Value<S> = Rc::new(RefCell::new(Self {
            v: Rc::new(RefCell::new(vec![
                (*x.borrow());
                ((*x.borrow()) as usize) as usize
            ])),
            n: Rc::new(RefCell::new(Box::new([(*x.borrow()), ((*x.borrow()) + 1)]))),
        }));
        let this: Ptr<S> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn S_pmutS_rv(_a0: Ptr<S>) -> Self {
        let __this: Value<S> = Rc::new(RefCell::new(Self {
            v: Rc::new(RefCell::new(std::mem::take(
                &mut (*(*_a0.upgrade().deref()).v.borrow_mut()),
            ))),
            n: Rc::new(RefCell::new(Box::new(std::array::from_fn::<_, 2, _>(
                |__i: usize| (*(*_a0.upgrade().deref()).n.borrow())[(__i) as usize],
            )))),
        }));
        let this: Ptr<S> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for S {
    fn default() -> Self {
        S {
            v: Rc::new(RefCell::new(Default::default())),
            n: Rc::new(RefCell::new((0..2).map(|_| 0_i32).collect::<Box<[i32]>>())),
        }
    }
}
impl ByteRepr for S {
    fn byte_size() -> usize {
        32
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.v.borrow()).to_bytes(&mut buf[0..24]);
        (*self.n.borrow()).to_bytes(&mut buf[24..32]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            v: Rc::new(RefCell::new(<Vec<i32>>::from_bytes(&buf[0..24]))),
            n: Rc::new(RefCell::new(<Box<[i32]>>::from_bytes(&buf[24..32]))),
        }
    }
}
pub fn sum_0(s: Ptr<S>) -> i32 {
    return {
        let _lhs = {
            let _lhs = ((*(*s.upgrade().deref()).v.borrow()).len() as i32);
            _lhs + (*(*s.upgrade().deref()).n.borrow())[(0) as usize]
        };
        _lhs + (*(*s.upgrade().deref()).n.borrow())[(1) as usize]
    };
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let s: Value<S> = Rc::new(RefCell::new(S::new({ 2 })));
    assert!((({ sum_0(s.as_pointer(),) }) == 7));
    assert!((({ shuffle_1(3,) }) == 10));
    return 0;
}
pub fn shuffle_1(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    let a: Value<S> = Rc::new(RefCell::new(S::new({ (*x.borrow()) })));
    let b: Value<S> = Rc::new(RefCell::new(S::S_pmutS_rv({ a.as_pointer() })));
    assert!((*(*a.borrow()).v.borrow()).is_empty());
    let c: Value<S> = Rc::new(RefCell::new(S::new({ 1 })));
    ({ SImpl::operator_assign_pmutS_rv(&c.as_pointer(), b.as_pointer()) });
    assert!((*(*b.borrow()).v.borrow()).is_empty());
    return ({ sum_0(c.as_pointer()) });
}
pub trait SImpl {
    fn operator_assign_pmutS_rv(&self, _a0: Ptr<S>) -> Ptr<S>;
}
impl SImpl for Ptr<S> {
    fn operator_assign_pmutS_rv(&self, _a0: Ptr<S>) -> Ptr<S> {
        ((*(*self).upgrade().deref()).v.as_pointer() as Ptr<Vec<i32>>).write(std::mem::take(
            &mut (*(*_a0.upgrade().deref()).v.borrow_mut()),
        ));
        {
            (((*(*self).upgrade().deref()).n.as_pointer()) as Ptr<i32>)
                .to_any()
                .memcpy(
                    &(((*_a0.upgrade().deref()).n.as_pointer()) as Ptr<i32>).to_any(),
                    8_usize as usize,
                );
            (((*(*self).upgrade().deref()).n.as_pointer()) as Ptr<i32>).to_any()
        };
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
