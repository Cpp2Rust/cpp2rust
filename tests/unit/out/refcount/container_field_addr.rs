extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(32)]
pub struct S {
    #[offset(0)]
    pub tag: i32,
    #[offset(8)]
    #[byte_size(24)]
    pub v: Vec<i32>,
}
impl S {
    pub fn move_from(_a0: Ptr<S>) -> Self {
        Self {
            tag: { (*_a0.upgrade().deref()).tag },
            v: field!(_a0, v).with_mut(std::mem::take),
        }
    }
}
pub fn add_0(mut v: Ptr<Vec<i32>>, mut k: i32) {
    {
        let a0_clone = k.clone();
        v.with_mut(|__v: &mut Vec<i32>| __v.push(a0_clone))
    };
}
pub fn run_1(mut h: Ptr<S>) {
    ({
        let _v: Ptr<Vec<i32>> = (field_ref!(h, v).as_pointer());
        let _k: i32 = h.with(|__s| __s.tag);
        add_0(_v, _k)
    });
    let mut pv: Ptr<Vec<i32>> = (field_ref!(h, v).as_pointer());
    {
        let __a1 = ((*pv.upgrade().deref()).len() as i32);
        pv.with_mut(|__v: &mut Vec<i32>| __v.push(__a1))
    };
    assert!(
        ({
            (((*h.upgrade().deref()).v.len() == 2_usize)
                && ((vec_elem!(h, v, 0_usize).read()) == 7))
                && ((vec_elem!(h, v, 1_usize).read()) == 1)
        })
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
pub trait SImpl {
    fn move_assign(&self, _a0: Ptr<S>) -> Ptr<S>;
}
impl SImpl for Ptr<S> {
    fn move_assign(&self, _a0: Ptr<S>) -> Ptr<S> {
        field!((*self), tag).write({ { (*_a0.upgrade().deref()).tag } });
        field_ref!((*self), v)
            .as_pointer()
            .write(field!(_a0, v).with_mut(std::mem::take));
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
