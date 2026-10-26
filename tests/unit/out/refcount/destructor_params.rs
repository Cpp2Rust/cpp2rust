extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static destroyed_0: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static last_1: Value<i32> = Rc::new(RefCell::new(0));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(4)]
pub struct S {
    #[offset(0)]
    pub v: i32,
}
pub fn by_value_2(s: S) -> i32 {
    let s: Value<S> = Rc::new(RefCell::new(s));
    assert!((destroyed_0.with(|rc| *rc.borrow()) == 0));
    return { (*s.borrow()).v };
}
pub fn with_default_3(s: Option<S>) -> i32 {
    let s: Value<S> = Rc::new(RefCell::new(s.unwrap_or_else(|| S { v: 5 })));
    return { (*s.borrow()).v };
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Holder {
    #[offset(0)]
    pub v: i32,
}
impl Holder {
    pub fn new(s: S) -> Self {
        let s: Value<S> = Rc::new(RefCell::new(s));
        Self {
            v: { (*s.borrow()).v },
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    {
        let s: Value<S> = Rc::new(RefCell::new(S { v: 1 }));
        let _dtor_s = ScopedDestructor::new(&s, |__p| __p.destructor());
        assert!((({ by_value_2((*s.borrow()).clone(),) }) == 1));
        assert!(
            (destroyed_0.with(|rc| *rc.borrow()) == 1) && (last_1.with(|rc| *rc.borrow()) == 1)
        );
    }
    assert!((destroyed_0.with(|rc| *rc.borrow()) == 2));
    destroyed_0.with(|rc| *rc.borrow_mut() = 0);
    assert!((({ by_value_2(S { v: 2 },) }) == 2));
    assert!((destroyed_0.with(|rc| *rc.borrow()) == 1) && (last_1.with(|rc| *rc.borrow()) == 2));
    destroyed_0.with(|rc| *rc.borrow_mut() = 0);
    assert!((({ with_default_3(None,) }) == 5));
    assert!((destroyed_0.with(|rc| *rc.borrow()) == 1) && (last_1.with(|rc| *rc.borrow()) == 5));
    destroyed_0.with(|rc| *rc.borrow_mut() = 0);
    {
        let h: Value<Holder> = Rc::new(RefCell::new(Holder::new({ S { v: 3 } })));
        assert!(
            (destroyed_0.with(|rc| *rc.borrow()) == 1) && (last_1.with(|rc| *rc.borrow()) == 3)
        );
        assert!((({ HolderImpl::add(&h.as_pointer(), S { v: 4 },) }) == 7));
        assert!(
            (destroyed_0.with(|rc| *rc.borrow()) == 2) && (last_1.with(|rc| *rc.borrow()) == 4)
        );
    }
    assert!((destroyed_0.with(|rc| *rc.borrow()) == 2));
    return 0;
}
pub trait HolderImpl {
    fn add(&self, s: S) -> i32;
}
impl HolderImpl for Ptr<Holder> {
    fn add(&self, s: S) -> i32 {
        let s: Value<S> = Rc::new(RefCell::new(s));
        return ((*self).with(|__s| __s.v) + { (*s.borrow()).v });
    }
}
pub trait SImpl {
    fn destructor(&self);
}
impl SImpl for Ptr<S> {
    fn destructor(&self) {
        (*destroyed_0.with(Value::clone).borrow_mut()).postfix_inc();
        last_1.with(|rc| *rc.borrow_mut() = (*self).with(|__s| __s.v));
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = destroyed_0.with(|_| ());
    let _ = last_1.with(|_| ());
}
