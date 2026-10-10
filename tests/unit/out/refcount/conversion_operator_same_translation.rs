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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct T {
    #[offset(0)]
    pub v: i32,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let s: Value<S> = Rc::new(RefCell::new(S { v: 65 }));
    let mut a: i8 = ({ SImpl::to_i8(&s.as_pointer()) });
    let mut b: u8 = ({ SImpl::to_u8(&s.as_pointer()) });
    assert!(((a as i32) == (('A' as i8) as i32)));
    assert!(((b as i32) == ((66 as u8) as i32)));
    assert!(
        (((({ SImpl::to_i8(&s.as_pointer(),) }) as i32)
            + (({ SImpl::to_u8(&s.as_pointer(),) }) as i32))
            == 131)
    );
    let t: Value<T> = Rc::new(RefCell::new(T { v: 3 }));
    let mut c: i64 = ({ TImpl::to_i64_1(&t.as_pointer()) });
    let mut d: i64 = ({ TImpl::to_i64_2(&t.as_pointer()) });
    assert!((c == 3_i64));
    assert!((d == 4_i64));
    return 0;
}
pub trait SImpl {
    fn to_i8(&self) -> i8;
    fn to_u8(&self) -> u8;
}
impl SImpl for Ptr<S> {
    fn to_i8(&self) -> i8 {
        return ((*self).with(|__s| __s.v) as i8);
    }
    fn to_u8(&self) -> u8 {
        return (((*self).with(|__s| __s.v) + 1) as u8);
    }
}
pub trait TImpl {
    fn to_i64_1(&self) -> i64;
    fn to_i64_2(&self) -> i64;
}
impl TImpl for Ptr<T> {
    fn to_i64_1(&self) -> i64 {
        return ((*self).with(|__s| __s.v) as i64);
    }
    fn to_i64_2(&self) -> i64 {
        return (((*self).with(|__s| __s.v) + 1) as i64);
    }
}
pub fn __cpp2rust_init_globals() {}
