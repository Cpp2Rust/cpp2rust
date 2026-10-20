extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor)]
#[byte_size(32)]
pub struct S {
    #[offset(0)]
    #[byte_size(24)]
    pub v: Vec<i32>,
    #[offset(24)]
    #[byte_size(8)]
    pub n: Box<[i32]>,
}
impl S {
    pub fn new(mut x: i32) -> Self {
        Self {
            v: vec![x; (x as usize) as usize],
            n: Box::new([x, (x + 1)]),
        }
    }
    pub fn move_from(_a0: Ptr<S>) -> Self {
        Self {
            v: field!(_a0, v).with_mut(std::mem::take),
            n: Box::new(std::array::from_fn::<_, 2, _>(|__i: usize| {
                (*_a0.upgrade().deref()).n[(__i) as usize]
            })),
        }
    }
}
impl Default for S {
    fn default() -> Self {
        {
            S {
                v: Default::default(),
                n: (0..2).map(|_| 0_i32).collect::<Box<[i32]>>(),
            }
        }
    }
}
pub fn sum_0(s: Ptr<S>) -> i32 {
    return ({ ({ ((*s.upgrade().deref()).v.len() as i32) } + { (array_elem!(s, n, 0).read()) }) }
        + { (array_elem!(s, n, 1).read()) });
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
pub fn shuffle_1(mut x: i32) -> i32 {
    let a: Value<S> = Rc::new(RefCell::new(S::new({ x })));
    let b: Value<S> = Rc::new(RefCell::new(S::move_from({ a.as_pointer() })));
    assert!((*a.borrow()).v.is_empty());
    let c: Value<S> = Rc::new(RefCell::new(S::new({ 1 })));
    ({ SImpl::move_assign(&c.as_pointer(), b.as_pointer()) });
    assert!((*b.borrow()).v.is_empty());
    return ({ sum_0(c.as_pointer()) });
}
pub trait SImpl {
    fn move_assign(&self, _a0: Ptr<S>) -> Ptr<S>;
}
impl SImpl for Ptr<S> {
    fn move_assign(&self, _a0: Ptr<S>) -> Ptr<S> {
        field_ref!((*self), v)
            .as_pointer()
            .write(field!(_a0, v).with_mut(std::mem::take));
        {
            ((array_field_ptr!((*self), n)) as Ptr<i32>)
                .to_any()
                .memcpy(
                    &((array_field_ptr!(_a0, n)) as Ptr<i32>).to_any(),
                    8_usize as usize,
                );
            ((array_field_ptr!((*self), n)) as Ptr<i32>).to_any()
        };
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
