extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct S {
    pub v: Value<u32>,
}
impl Clone for S {
    fn clone(&self) -> Self {
        let __this: Value<S> = Rc::new(RefCell::new(Self {
            v: Rc::new(RefCell::new((*self.v.borrow()))),
        }));
        let this: Ptr<S> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for S {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.v.borrow()).to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            v: Rc::new(RefCell::new(<u32>::from_bytes(&buf[0..4]))),
        }
    }
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let a: Value<S> = Rc::new(RefCell::new(S {
        v: Rc::new(RefCell::new(6_u32)),
    }));
    let b: Value<S> = Rc::new(RefCell::new(S {
        v: Rc::new(RefCell::new(4_u32)),
    }));
    ({ SImpl::operator_add_assign(&a.as_pointer(), b.as_pointer()) });
    assert!(((*(*a.borrow()).v.borrow()) == 10_u32));
    ({ SImpl::operator_sub_assign(&a.as_pointer(), b.as_pointer()) });
    assert!(((*(*a.borrow()).v.borrow()) == 6_u32));
    ({ SImpl::operator_mul_assign(&a.as_pointer(), b.as_pointer()) });
    assert!(((*(*a.borrow()).v.borrow()) == 24_u32));
    ({ SImpl::operator_div_assign(&a.as_pointer(), b.as_pointer()) });
    assert!(((*(*a.borrow()).v.borrow()) == 6_u32));
    ({ SImpl::operator_rem_assign(&a.as_pointer(), b.as_pointer()) });
    assert!(((*(*a.borrow()).v.borrow()) == 2_u32));
    ({ SImpl::operator_bitor_assign(&a.as_pointer(), b.as_pointer()) });
    assert!(((*(*a.borrow()).v.borrow()) == 6_u32));
    ({ SImpl::operator_bitand_assign(&a.as_pointer(), b.as_pointer()) });
    assert!(((*(*a.borrow()).v.borrow()) == 4_u32));
    ({ SImpl::operator_bitxor_assign(&a.as_pointer(), b.as_pointer()) });
    assert!(((*(*a.borrow()).v.borrow()) == 0_u32));
    ({ SImpl::operator_assign_u32(&a.as_pointer(), 3_u32) });
    assert!(((*(*a.borrow()).v.borrow()) == 3_u32));
    ({ SImpl::operator_shl_assign(&a.as_pointer(), 2) });
    assert!(((*(*a.borrow()).v.borrow()) == 12_u32));
    ({ SImpl::operator_shr_assign(&a.as_pointer(), 1) });
    assert!(((*(*a.borrow()).v.borrow()) == 6_u32));
    ({
        let _o: Ptr<S> = b.as_pointer();
        SImpl::operator_add_assign(
            &({ SImpl::operator_add_assign(&a.as_pointer(), b.as_pointer()) }),
            _o,
        )
    });
    assert!(((*(*a.borrow()).v.borrow()) == 14_u32));
    let c: Value<S> = Rc::new(RefCell::new(S {
        v: Rc::new(RefCell::new(0_u32)),
    }));
    (*c.borrow_mut()) = (*({ SImpl::operator_assign_u32(&a.as_pointer(), 1_u32) })
        .upgrade()
        .deref())
    .clone();
    assert!(((*(*a.borrow()).v.borrow()) == 1_u32));
    assert!(((*(*c.borrow()).v.borrow()) == 1_u32));
    return 0;
}
pub trait SImpl {
    fn operator_assign_u32(&self, n: u32) -> Ptr<S>;
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
    fn operator_assign_u32(&self, n: u32) -> Ptr<S> {
        let n: Value<u32> = Rc::new(RefCell::new(n));
        (*(*(*self).upgrade().deref()).v.borrow_mut()) = (*n.borrow());
        return (*self).clone();
    }
    fn operator_add_assign(&self, o: Ptr<S>) -> Ptr<S> {
        {
            let rhs_0 = (*(*(*self).upgrade().deref()).v.borrow())
                .wrapping_add((*(*o.upgrade().deref()).v.borrow()));
            (*(*(*self).upgrade().deref()).v.borrow_mut()) = rhs_0
        };
        return (*self).clone();
    }
    fn operator_sub_assign(&self, o: Ptr<S>) -> Ptr<S> {
        {
            let rhs_0 = (*(*(*self).upgrade().deref()).v.borrow())
                .wrapping_sub((*(*o.upgrade().deref()).v.borrow()));
            (*(*(*self).upgrade().deref()).v.borrow_mut()) = rhs_0
        };
        return (*self).clone();
    }
    fn operator_mul_assign(&self, o: Ptr<S>) -> Ptr<S> {
        {
            let rhs_0 = (*(*(*self).upgrade().deref()).v.borrow())
                .wrapping_mul((*(*o.upgrade().deref()).v.borrow()));
            (*(*(*self).upgrade().deref()).v.borrow_mut()) = rhs_0
        };
        return (*self).clone();
    }
    fn operator_div_assign(&self, o: Ptr<S>) -> Ptr<S> {
        {
            let rhs_0 = (*(*(*self).upgrade().deref()).v.borrow())
                .wrapping_div((*(*o.upgrade().deref()).v.borrow()));
            (*(*(*self).upgrade().deref()).v.borrow_mut()) = rhs_0
        };
        return (*self).clone();
    }
    fn operator_rem_assign(&self, o: Ptr<S>) -> Ptr<S> {
        {
            let rhs_0 = (*(*(*self).upgrade().deref()).v.borrow())
                .wrapping_rem((*(*o.upgrade().deref()).v.borrow()));
            (*(*(*self).upgrade().deref()).v.borrow_mut()) = rhs_0
        };
        return (*self).clone();
    }
    fn operator_bitand_assign(&self, o: Ptr<S>) -> Ptr<S> {
        let __rhs = (*(*o.upgrade().deref()).v.borrow());
        (*(*(*self).upgrade().deref()).v.borrow_mut()) &= __rhs;
        return (*self).clone();
    }
    fn operator_bitor_assign(&self, o: Ptr<S>) -> Ptr<S> {
        let __rhs = (*(*o.upgrade().deref()).v.borrow());
        (*(*(*self).upgrade().deref()).v.borrow_mut()) |= __rhs;
        return (*self).clone();
    }
    fn operator_bitxor_assign(&self, o: Ptr<S>) -> Ptr<S> {
        let __rhs = (*(*o.upgrade().deref()).v.borrow());
        (*(*(*self).upgrade().deref()).v.borrow_mut()) ^= __rhs;
        return (*self).clone();
    }
    fn operator_shl_assign(&self, n: i32) -> Ptr<S> {
        let n: Value<i32> = Rc::new(RefCell::new(n));
        (*(*(*self).upgrade().deref()).v.borrow_mut()) <<= (*n.borrow());
        return (*self).clone();
    }
    fn operator_shr_assign(&self, n: i32) -> Ptr<S> {
        let n: Value<i32> = Rc::new(RefCell::new(n));
        (*(*(*self).upgrade().deref()).v.borrow_mut()) >>= (*n.borrow());
        return (*self).clone();
    }
}
