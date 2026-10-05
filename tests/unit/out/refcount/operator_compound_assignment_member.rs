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
    pub v: u32,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let a: Value<S> = Rc::new(RefCell::new(S { v: 6_u32 }));
    let b: Value<S> = Rc::new(RefCell::new(S { v: 4_u32 }));
    ({ SImpl::operator_add_assign(&a.as_pointer(), b.as_pointer()) });
    assert!(({ (*a.borrow()).v } == 10_u32));
    ({ SImpl::operator_sub_assign(&a.as_pointer(), b.as_pointer()) });
    assert!(({ (*a.borrow()).v } == 6_u32));
    ({ SImpl::operator_mul_assign(&a.as_pointer(), b.as_pointer()) });
    assert!(({ (*a.borrow()).v } == 24_u32));
    ({ SImpl::operator_div_assign(&a.as_pointer(), b.as_pointer()) });
    assert!(({ (*a.borrow()).v } == 6_u32));
    ({ SImpl::operator_rem_assign(&a.as_pointer(), b.as_pointer()) });
    assert!(({ (*a.borrow()).v } == 2_u32));
    ({ SImpl::operator_bitor_assign(&a.as_pointer(), b.as_pointer()) });
    assert!(({ (*a.borrow()).v } == 6_u32));
    ({ SImpl::operator_bitand_assign(&a.as_pointer(), b.as_pointer()) });
    assert!(({ (*a.borrow()).v } == 4_u32));
    ({ SImpl::operator_bitxor_assign(&a.as_pointer(), b.as_pointer()) });
    assert!(({ (*a.borrow()).v } == 0_u32));
    ({ SImpl::operator_assign_1(&a.as_pointer(), 3_u32) });
    assert!(({ (*a.borrow()).v } == 3_u32));
    ({ SImpl::operator_shl_assign(&a.as_pointer(), 2) });
    assert!(({ (*a.borrow()).v } == 12_u32));
    ({ SImpl::operator_shr_assign(&a.as_pointer(), 1) });
    assert!(({ (*a.borrow()).v } == 6_u32));
    ({
        let _o: Ptr<S> = b.as_pointer();
        SImpl::operator_add_assign(
            &({ SImpl::operator_add_assign(&a.as_pointer(), b.as_pointer()) }),
            _o,
        )
    });
    assert!(({ (*a.borrow()).v } == 14_u32));
    let c: Value<S> = Rc::new(RefCell::new(S { v: 0_u32 }));
    (*c.borrow_mut()) = (*({ SImpl::operator_assign_1(&a.as_pointer(), 1_u32) })
        .upgrade()
        .deref())
    .clone();
    assert!(({ (*a.borrow()).v } == 1_u32));
    assert!(({ (*c.borrow()).v } == 1_u32));
    return 0;
}
pub trait SImpl {
    fn operator_assign_1(&self, n: u32) -> Ptr<S>;
    fn operator_add_assign(&self, o: Ptr<S>) -> Ptr<S>;
    fn operator_sub_assign(&self, o: Ptr<S>) -> Ptr<S>;
    fn operator_mul_assign(&self, o: Ptr<S>) -> Ptr<S>;
    fn operator_div_assign(&self, o: Ptr<S>) -> Ptr<S>;
    fn operator_rem_assign(&self, o: Ptr<S>) -> Ptr<S>;
    fn operator_bitand_assign(&self, o: Ptr<S>) -> Ptr<S>;
    fn operator_bitor_assign(&self, o: Ptr<S>) -> Ptr<S>;
    fn operator_bitxor_assign(&self, o: Ptr<S>) -> Ptr<S>;
    fn operator_shl_assign(&self, n: i32) -> Ptr<S>;
    fn operator_shr_assign(&self, n: i32) -> Ptr<S>;
}
impl SImpl for Ptr<S> {
    fn operator_assign_1(&self, mut n: u32) -> Ptr<S> {
        field!((*self), v).write(n);
        return (*self).clone();
    }
    fn operator_add_assign(&self, o: Ptr<S>) -> Ptr<S> {
        field!((*self), v).write({ ((*self).with(|__s| __s.v)).wrapping_add(o.with(|__s| __s.v)) });
        return (*self).clone();
    }
    fn operator_sub_assign(&self, o: Ptr<S>) -> Ptr<S> {
        field!((*self), v).write({ ((*self).with(|__s| __s.v)).wrapping_sub(o.with(|__s| __s.v)) });
        return (*self).clone();
    }
    fn operator_mul_assign(&self, o: Ptr<S>) -> Ptr<S> {
        field!((*self), v).write({ ((*self).with(|__s| __s.v)).wrapping_mul(o.with(|__s| __s.v)) });
        return (*self).clone();
    }
    fn operator_div_assign(&self, o: Ptr<S>) -> Ptr<S> {
        field!((*self), v).write({ ((*self).with(|__s| __s.v)).wrapping_div(o.with(|__s| __s.v)) });
        return (*self).clone();
    }
    fn operator_rem_assign(&self, o: Ptr<S>) -> Ptr<S> {
        field!((*self), v).write({ ((*self).with(|__s| __s.v)).wrapping_rem(o.with(|__s| __s.v)) });
        return (*self).clone();
    }
    fn operator_bitand_assign(&self, o: Ptr<S>) -> Ptr<S> {
        {
            let __rhs = { o.with(|__s| __s.v) };
            field!((*self), v).with_mut(|__v| *__v = *__v & __rhs)
        };
        return (*self).clone();
    }
    fn operator_bitor_assign(&self, o: Ptr<S>) -> Ptr<S> {
        {
            let __rhs = { o.with(|__s| __s.v) };
            field!((*self), v).with_mut(|__v| *__v = *__v | __rhs)
        };
        return (*self).clone();
    }
    fn operator_bitxor_assign(&self, o: Ptr<S>) -> Ptr<S> {
        {
            let __rhs = { o.with(|__s| __s.v) };
            field!((*self), v).with_mut(|__v| *__v = *__v ^ __rhs)
        };
        return (*self).clone();
    }
    fn operator_shl_assign(&self, mut n: i32) -> Ptr<S> {
        {
            let __rhs = n;
            field!((*self), v).with_mut(|__v| *__v = *__v << __rhs)
        };
        return (*self).clone();
    }
    fn operator_shr_assign(&self, mut n: i32) -> Ptr<S> {
        {
            let __rhs = n;
            field!((*self), v).with_mut(|__v| *__v = *__v >> __rhs)
        };
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
