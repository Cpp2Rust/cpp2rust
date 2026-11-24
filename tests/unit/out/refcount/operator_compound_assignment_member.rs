extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Copy, Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct S {
    #[offset(0)]
    pub v: u32,
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let a: Value<S> = Rc::new(RefCell::new(S { v: 6_u32 }));
    let b: Value<S> = Rc::new(RefCell::new(S { v: 4_u32 }));
    ({ SImpl::operator_add_assign(&a.as_pointer(), b.as_pointer()) });
    assert!(({ ({ (*a.borrow()).v } == 10_u32) }));
    ({ SImpl::operator_sub_assign(&a.as_pointer(), b.as_pointer()) });
    assert!(({ ({ (*a.borrow()).v } == 6_u32) }));
    ({ SImpl::operator_mul_assign(&a.as_pointer(), b.as_pointer()) });
    assert!(({ ({ (*a.borrow()).v } == 24_u32) }));
    ({ SImpl::operator_div_assign(&a.as_pointer(), b.as_pointer()) });
    assert!(({ ({ (*a.borrow()).v } == 6_u32) }));
    ({ SImpl::operator_rem_assign(&a.as_pointer(), b.as_pointer()) });
    assert!(({ ({ (*a.borrow()).v } == 2_u32) }));
    ({ SImpl::operator_bitor_assign(&a.as_pointer(), b.as_pointer()) });
    assert!(({ ({ (*a.borrow()).v } == 6_u32) }));
    ({ SImpl::operator_bitand_assign(&a.as_pointer(), b.as_pointer()) });
    assert!(({ ({ (*a.borrow()).v } == 4_u32) }));
    ({ SImpl::operator_bitxor_assign(&a.as_pointer(), b.as_pointer()) });
    assert!(({ ({ (*a.borrow()).v } == 0_u32) }));
    ({ SImpl::operator_assign_1(&a.as_pointer(), 3_u32) });
    assert!(({ ({ (*a.borrow()).v } == 3_u32) }));
    ({ SImpl::operator_shl_assign(&a.as_pointer(), 2) });
    assert!(({ ({ (*a.borrow()).v } == 12_u32) }));
    ({ SImpl::operator_shr_assign(&a.as_pointer(), 1) });
    assert!(({ ({ (*a.borrow()).v } == 6_u32) }));
    ({
        let _o: Ptr<S> = b.as_pointer();
        SImpl::operator_add_assign(
            &({ SImpl::operator_add_assign(&a.as_pointer(), b.as_pointer()) }),
            _o,
        )
    });
    assert!(({ ({ (*a.borrow()).v } == 14_u32) }));
    let mut c: S = S { v: 0_u32 };
    c = (*({ SImpl::operator_assign_1(&a.as_pointer(), 1_u32) })
        .upgrade()
        .deref());
    assert!(({ ({ (*a.borrow()).v } == 1_u32) }));
    assert!((c.v == 1_u32));
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
        match ({ o.with(|__s| __s.v) }, &(field!((*self), v))) {
            (__tmp_0, __tmp_1) => __tmp_1.with_mut(|__tmp_1| {
                (*__tmp_1) = (((*__tmp_1) as u32).wrapping_add((__tmp_0 as u32))) as u32;
            }),
        };
        return (*self).clone();
    }
    fn operator_sub_assign(&self, o: Ptr<S>) -> Ptr<S> {
        match ({ o.with(|__s| __s.v) }, &(field!((*self), v))) {
            (__tmp_2, __tmp_3) => __tmp_3.with_mut(|__tmp_3| {
                (*__tmp_3) = (((*__tmp_3) as u32).wrapping_sub((__tmp_2 as u32))) as u32;
            }),
        };
        return (*self).clone();
    }
    fn operator_mul_assign(&self, o: Ptr<S>) -> Ptr<S> {
        match ({ o.with(|__s| __s.v) }, &(field!((*self), v))) {
            (__tmp_4, __tmp_5) => __tmp_5.with_mut(|__tmp_5| {
                (*__tmp_5) = (((*__tmp_5) as u32).wrapping_mul((__tmp_4 as u32))) as u32;
            }),
        };
        return (*self).clone();
    }
    fn operator_div_assign(&self, o: Ptr<S>) -> Ptr<S> {
        match ({ o.with(|__s| __s.v) }, &(field!((*self), v))) {
            (__tmp_6, __tmp_7) => __tmp_7.with_mut(|__tmp_7| {
                (*__tmp_7) = (((*__tmp_7) as u32).wrapping_div((__tmp_6 as u32))) as u32;
            }),
        };
        return (*self).clone();
    }
    fn operator_rem_assign(&self, o: Ptr<S>) -> Ptr<S> {
        match ({ o.with(|__s| __s.v) }, &(field!((*self), v))) {
            (__tmp_8, __tmp_9) => __tmp_9.with_mut(|__tmp_9| {
                (*__tmp_9) = (((*__tmp_9) as u32).wrapping_rem((__tmp_8 as u32))) as u32;
            }),
        };
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
