extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Record, ByteRepr)]
#[byte_size(32)]
pub struct S {
    #[offset(0)]
    #[byte_size(24)]
    pub v: Value<Vec<i32>>,
    #[offset(24)]
    #[byte_size(8)]
    pub n: Value<Box<[i32]>>,
}
impl S {
    pub fn new(mut x: i32) -> Self {
        Self {
            v: Rc::new(RefCell::new(vec![x; (x as usize) as usize])),
            n: Rc::new(RefCell::new(Box::new([x, (x + 1)]))),
        }
    }
    pub fn move_from(_a0: Ptr<S>) -> Self {
        Self {
            v: Rc::new(RefCell::new(std::mem::take(
                &mut (*{ (*_a0.upgrade().deref()).v.clone() }.borrow_mut()),
            ))),
            n: Rc::new(RefCell::new(Box::new(std::array::from_fn::<_, 2, _>(
                |__i: usize| (*{ (*_a0.upgrade().deref()).n.clone() }.borrow())[(__i) as usize],
            )))),
        }
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
pub fn sum_0(s: Ptr<S>) -> i32 {
    return ({
        ({ ((*s.with(|__s| __s.v.clone()).borrow()).len() as i32) } + {
            (elem!((array_field_ptr!(s, n) as Ptr::<i32>), 0).read())
        })
    } + { (elem!((array_field_ptr!(s, n) as Ptr::<i32>), 1).read()) });
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
    assert!((*{ (*a.borrow()).v.clone() }.borrow()).is_empty());
    let c: Value<S> = Rc::new(RefCell::new(S::new({ 1 })));
    ({ SImpl::move_assign(&c.as_pointer(), b.as_pointer()) });
    assert!((*{ (*b.borrow()).v.clone() }.borrow()).is_empty());
    return ({ sum_0(c.as_pointer()) });
}
pub trait SImpl {
    fn move_assign(&self, _a0: Ptr<S>) -> Ptr<S>;
}
impl SImpl for Ptr<S> {
    fn move_assign(&self, _a0: Ptr<S>) -> Ptr<S> {
        ((*self).with(|__s| __s.v.as_pointer()) as Ptr<Vec<i32>>).write(std::mem::take(
            &mut (*{ (*_a0.upgrade().deref()).v.clone() }.borrow_mut()),
        ));
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
