extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, VaArg, FnPtrArg, Default)]
pub struct S {
    #[offset(0)]
    pub v: i32,
}
impl S {}
impl ByteRepr for S {
    fn byte_size() -> usize {
        4
    }
}
pub trait Base {
    fn apply(&mut self, x: i32) -> i32;
}
#[derive(Record, VaArg, FnPtrArg, Default)]
pub struct Derived {
    #[offset(8)]
    pub factor: i32,
}
impl Derived {}
impl Clone for Derived {
    fn clone(&self) -> Self {
        let __this: Value<Derived> = Rc::new(RefCell::new(Self {
            factor: { self.factor },
        }));
        let this: Ptr<Derived> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for Derived {
    fn byte_size() -> usize {
        16
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let s: Value<S> = Rc::new(RefCell::new(S::new({ 1 })));
    assert!((({ SImpl::get(&s.as_pointer(),) }) == 1));
    ({ SImpl::set(&s.as_pointer(), 4) });
    assert!((({ SImpl::get(&s.as_pointer(),) }) == 4));
    assert!((({ SImpl::add(&s.as_pointer(), 2,) }) == 6));
    let derived: Value<Derived> = Rc::new(RefCell::new(Derived::new({ 3 })));
    let base: Value<PtrDyn<dyn Base>> = Rc::new(RefCell::new(
        (derived.as_pointer()).to_dyn::<dyn Base>(|w| w),
    ));
    assert!((({ (*(*base.borrow()).upgrade().deref_mut()).apply(5,) }) == 15));
    return 0;
}
impl S {
    pub fn new(x: i32) -> Self {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        let __this: Value<S> = Rc::new(RefCell::new(Self { v: (*x.borrow()) }));
        let this: Ptr<S> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Derived {
    pub fn new(factor: i32) -> Self {
        let factor: Value<i32> = Rc::new(RefCell::new(factor));
        let __this: Value<Derived> = Rc::new(RefCell::new(Self {
            factor: (*factor.borrow()),
        }));
        let this: Ptr<Derived> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl S {}
impl Derived {}
impl Base for Derived {
    fn apply(&mut self, x: i32) -> i32 {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        return ({ self.factor } * (*x.borrow()));
    }
}
pub trait SImpl {
    fn destructor(&self) {
        unimplemented!()
    }
    fn get(&self) -> i32;
    fn set(&self, x: i32) {
        unimplemented!()
    }
    fn add(&self, x: i32) -> i32 {
        unimplemented!()
    }
}
impl SImpl for Ptr<S> {
    fn get(&self) -> i32 {
        return (*self).with(|__s| __s.v);
    }
    fn destructor(&self) {}
    fn set(&self, x: i32) {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        field!((*self), v).write((*x.borrow()));
    }
    fn add(&self, x: i32) -> i32 {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        {
            let _ptr = field!((*self), v);
            _ptr.write(_ptr.read() + (*x.borrow()))
        };
        return (*self).with(|__s| __s.v);
    }
}
pub fn __cpp2rust_init_globals() {}
