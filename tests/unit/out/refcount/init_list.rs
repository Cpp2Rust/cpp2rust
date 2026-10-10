extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn f_0(list: Vec<i32>) {
    let list: Value<Vec<i32>> = Rc::new(RefCell::new(list));
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Pair {
    #[offset(0)]
    pub a: i32,
    #[offset(4)]
    pub b: i32,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct Holder {
    #[offset(0)]
    #[byte_size(8)]
    pub p: Pair,
}
impl Default for Holder {
    fn default() -> Self {
        {
            Holder {
                p: Pair { a: 0, b: 0 },
            }
        }
    }
}
pub fn sum_1(p: Ptr<Pair>) -> i32 {
    return ({ p.with(|__s| __s.a) } + { p.with(|__s| __s.b) });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut i1: i32 = 3;
    let mut i2: i32 = 0_i32;
    let mut carr1: [i32; 2] = [1, 2];
    let mut carr2: [i32; 3] = [1, 0_i32, 0_i32];
    let arr: Value<Vec<i32>> = Rc::new(RefCell::new(vec![1, 2, 3]));
    let mut vec_: Vec<i32> = vec![1, 2, 3];
    ({ f_0(vec![1, 2, 3, 4]) });
    let p: Value<Pair> = Rc::new(RefCell::new(Pair { a: 1, b: 2 }));
    let h: Value<Holder> = Rc::new(RefCell::new(<Holder>::default()));
    ({ HolderImpl::operator_assign_1(&h.as_pointer(), p.as_pointer()) });
    assert!(({ (*h.borrow()).p.a } == 1) && ({ (*h.borrow()).p.b } == 2));
    assert!((({ sum_1(p.as_pointer(),) }) == 3));
    return 0;
}
pub trait HolderImpl {
    fn operator_assign_1(&self, o: Ptr<Pair>) -> Ptr<Holder>;
}
impl HolderImpl for Ptr<Holder> {
    fn operator_assign_1(&self, o: Ptr<Pair>) -> Ptr<Holder> {
        field!((*self), p).write({ (*o.upgrade().deref()).clone() });
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
