extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct S {
    #[offset(0)]
    pub v: i32,
}
impl S {}
pub trait Base {
    fn apply(&mut self, x: i32) -> i32;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct Derived {
    #[offset(8)]
    pub factor: i32,
}
impl Derived {}
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
    let mut base: PtrDyn<dyn Base> = (derived.as_pointer()).to_dyn::<dyn Base>(|w| w);
    assert!((({ (*base.upgrade().deref_mut()).apply(5,) }) == 15));
    return 0;
}
impl S {
    pub fn new(mut x: i32) -> Self {
        Self { v: x }
    }
}
impl Derived {
    pub fn new(mut factor: i32) -> Self {
        Self { factor: factor }
    }
}
impl S {}
impl Derived {}
impl Base for Derived {
    fn apply(&mut self, mut x: i32) -> i32 {
        return ({ self.factor } * x);
    }
}
pub trait SImpl {
    fn destructor(&self) {
        unimplemented!()
    }
    fn get(&self) -> i32;
    fn set(&self, mut x: i32) {
        unimplemented!()
    }
    fn add(&self, mut x: i32) -> i32 {
        unimplemented!()
    }
}
impl SImpl for Ptr<S> {
    fn get(&self) -> i32 {
        return (*self).with(|__s| __s.v);
    }
    fn destructor(&self) {}
    fn set(&self, mut x: i32) {
        field!((*self), v).write(x);
    }
    fn add(&self, mut x: i32) -> i32 {
        {
            let __rhs = x;
            field!((*self), v).with_mut(|__v| *__v = *__v + __rhs)
        };
        return (*self).with(|__s| __s.v);
    }
}
pub fn __cpp2rust_init_globals() {}
