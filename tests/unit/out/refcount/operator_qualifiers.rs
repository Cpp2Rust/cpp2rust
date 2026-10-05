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
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let s: Value<S> = Rc::new(RefCell::new(S { v: 10 }));
    let cs: Value<S> = Rc::new(RefCell::new(S { v: 10 }));
    let vs: Value<S> = Rc::new(RefCell::new(S { v: 10 }));
    assert!((({ SImpl::operator_add_1(&s.as_pointer(), 1,) }) == 11));
    assert!((({ SImpl::operator_add_2(&cs.as_pointer(), 1,) }) == 12));
    assert!((({ SImpl::operator_add_3(&vs.as_pointer(), 1,) }) == 13));
    assert!((({ SImpl::operator_sub_4(&s.as_pointer(), 1,) }) == 9));
    assert!(
        (({ SImpl::operator_sub_5(&Rc::new(RefCell::new(S { v: 10 })).as_pointer(), 1,) }) == 8)
    );
    assert!((({ SImpl::operator_mul_6(&s.as_pointer(), 3,) }) == 30));
    assert!((({ SImpl::operator_mul_6(&cs.as_pointer(), 3,) }) == 30));
    assert!(
        (({ SImpl::operator_mul_7(&Rc::new(RefCell::new(S { v: 10 })).as_pointer(), 3,) }) == 60)
    );
    assert!((({ SImpl::operator_index_8(&s.as_pointer(), 2,) }) == 12));
    assert!((({ SImpl::operator_index_9(&cs.as_pointer(), 2,) }) == 112));
    return 0;
}
pub trait SImpl {
    fn operator_add_1(&self, a: i32) -> i32;
    fn operator_add_2(&self, a: i32) -> i32;
    fn operator_add_3(&self, a: i32) -> i32;
    fn operator_sub_4(&self, a: i32) -> i32;
    fn operator_sub_5(&self, a: i32) -> i32;
    fn operator_mul_6(&self, a: i32) -> i32;
    fn operator_mul_7(&self, a: i32) -> i32;
    fn operator_index_8(&self, i: i32) -> i32;
    fn operator_index_9(&self, i: i32) -> i32;
}
impl SImpl for Ptr<S> {
    fn operator_add_1(&self, mut a: i32) -> i32 {
        return ((*self).with(|__s| __s.v) + a);
    }
    fn operator_add_2(&self, mut a: i32) -> i32 {
        return (((*self).with(|__s| __s.v) + a) + 1);
    }
    fn operator_add_3(&self, mut a: i32) -> i32 {
        return (((*self).with(|__s| __s.v) + a) + 2);
    }
    fn operator_sub_4(&self, mut a: i32) -> i32 {
        return ((*self).with(|__s| __s.v) - a);
    }
    fn operator_sub_5(&self, mut a: i32) -> i32 {
        return (((*self).with(|__s| __s.v) - a) - 1);
    }
    fn operator_mul_6(&self, mut a: i32) -> i32 {
        return ((*self).with(|__s| __s.v) * a);
    }
    fn operator_mul_7(&self, mut a: i32) -> i32 {
        return (((*self).with(|__s| __s.v) * a) * 2);
    }
    fn operator_index_8(&self, mut i: i32) -> i32 {
        return ((*self).with(|__s| __s.v) + i);
    }
    fn operator_index_9(&self, mut i: i32) -> i32 {
        return (((*self).with(|__s| __s.v) + i) + 100);
    }
}
pub fn __cpp2rust_init_globals() {}
