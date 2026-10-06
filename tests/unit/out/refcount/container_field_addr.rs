extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(DeepClone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(32)]
pub struct S {
    #[offset(0)]
    pub tag: i32,
    #[offset(8)]
    #[byte_size(24)]
    pub v: Value<Vec<i32>>,
}
pub fn add_0(mut v: Ptr<Vec<i32>>, mut k: i32) {
    {
        let a0_clone = k.clone();
        v.with_mut(|__v: &mut Vec<i32>| __v.push(a0_clone))
    };
}
pub fn run_1(mut h: Ptr<S>) {
    ({
        let _v: Ptr<Vec<i32>> = (h.with(|__s| __s.v.as_pointer()));
        let _k: i32 = h.with(|__s| __s.tag);
        add_0(_v, _k)
    });
    let mut pv: Ptr<Vec<i32>> = (h.with(|__s| __s.v.as_pointer()));
    {
        let __a1 = ((*pv.upgrade().deref()).len() as i32);
        pv.with_mut(|__v: &mut Vec<i32>| __v.push(__a1))
    };
    assert!(
        (((*h.with(|__s| __s.v.clone()).borrow()).len() == 2_usize)
            && ((elem!((h.with(|__s| __s.v.as_pointer()) as Ptr<i32>), 0_usize).read()) == 7))
            && ((elem!((h.with(|__s| __s.v.as_pointer()) as Ptr<i32>), 1_usize).read()) == 1)
    );
    assert!((h.with(|__s| __s.tag) == 7));
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let local: Value<S> = Rc::new(RefCell::new(<S>::default()));
    (*local.borrow_mut()).tag = 7;
    ({ run_1((local.as_pointer())) });
    let mut heap: Ptr<S> = Ptr::alloc(<S>::default());
    field!(heap, tag).write(7);
    ({ run_1((heap).clone()) });
    heap.delete();
    return 0;
}
pub fn __cpp2rust_init_globals() {}
