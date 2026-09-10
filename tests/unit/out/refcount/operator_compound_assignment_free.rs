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
pub fn operator_add_assign_0(a: Ptr<S>, b: Ptr<S>) -> Ptr<S> {
    {
        let rhs_0 =
            (*(*a.upgrade().deref()).v.borrow()).wrapping_add((*(*b.upgrade().deref()).v.borrow()));
        (*(*a.upgrade().deref()).v.borrow_mut()) = rhs_0
    };
    return (a).clone();
}
pub fn operator_sub_assign_1(a: Ptr<S>, b: Ptr<S>) -> Ptr<S> {
    {
        let rhs_0 =
            (*(*a.upgrade().deref()).v.borrow()).wrapping_sub((*(*b.upgrade().deref()).v.borrow()));
        (*(*a.upgrade().deref()).v.borrow_mut()) = rhs_0
    };
    return (a).clone();
}
pub fn operator_mul_assign_2(a: Ptr<S>, b: Ptr<S>) -> Ptr<S> {
    {
        let rhs_0 =
            (*(*a.upgrade().deref()).v.borrow()).wrapping_mul((*(*b.upgrade().deref()).v.borrow()));
        (*(*a.upgrade().deref()).v.borrow_mut()) = rhs_0
    };
    return (a).clone();
}
pub fn operator_div_assign_3(a: Ptr<S>, b: Ptr<S>) -> Ptr<S> {
    {
        let rhs_0 =
            (*(*a.upgrade().deref()).v.borrow()).wrapping_div((*(*b.upgrade().deref()).v.borrow()));
        (*(*a.upgrade().deref()).v.borrow_mut()) = rhs_0
    };
    return (a).clone();
}
pub fn operator_rem_assign_4(a: Ptr<S>, b: Ptr<S>) -> Ptr<S> {
    {
        let rhs_0 =
            (*(*a.upgrade().deref()).v.borrow()).wrapping_rem((*(*b.upgrade().deref()).v.borrow()));
        (*(*a.upgrade().deref()).v.borrow_mut()) = rhs_0
    };
    return (a).clone();
}
pub fn operator_bitand_assign_5(a: Ptr<S>, b: Ptr<S>) -> Ptr<S> {
    let __rhs = (*(*b.upgrade().deref()).v.borrow());
    (*(*a.upgrade().deref()).v.borrow_mut()) &= __rhs;
    return (a).clone();
}
pub fn operator_bitor_assign_6(a: Ptr<S>, b: Ptr<S>) -> Ptr<S> {
    let __rhs = (*(*b.upgrade().deref()).v.borrow());
    (*(*a.upgrade().deref()).v.borrow_mut()) |= __rhs;
    return (a).clone();
}
pub fn operator_bitxor_assign_7(a: Ptr<S>, b: Ptr<S>) -> Ptr<S> {
    let __rhs = (*(*b.upgrade().deref()).v.borrow());
    (*(*a.upgrade().deref()).v.borrow_mut()) ^= __rhs;
    return (a).clone();
}
pub fn operator_shl_assign_8(a: Ptr<S>, n: i32) -> Ptr<S> {
    let n: Value<i32> = Rc::new(RefCell::new(n));
    (*(*a.upgrade().deref()).v.borrow_mut()) <<= (*n.borrow());
    return (a).clone();
}
pub fn operator_shr_assign_9(a: Ptr<S>, n: i32) -> Ptr<S> {
    let n: Value<i32> = Rc::new(RefCell::new(n));
    (*(*a.upgrade().deref()).v.borrow_mut()) >>= (*n.borrow());
    return (a).clone();
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
    ({
        let _a: Ptr<S> = a.as_pointer();
        operator_add_assign_0(_a, b.as_pointer())
    });
    assert!(((*(*a.borrow()).v.borrow()) == 10_u32));
    ({
        let _a: Ptr<S> = a.as_pointer();
        operator_sub_assign_1(_a, b.as_pointer())
    });
    assert!(((*(*a.borrow()).v.borrow()) == 6_u32));
    ({
        let _a: Ptr<S> = a.as_pointer();
        operator_mul_assign_2(_a, b.as_pointer())
    });
    assert!(((*(*a.borrow()).v.borrow()) == 24_u32));
    ({
        let _a: Ptr<S> = a.as_pointer();
        operator_div_assign_3(_a, b.as_pointer())
    });
    assert!(((*(*a.borrow()).v.borrow()) == 6_u32));
    ({
        let _a: Ptr<S> = a.as_pointer();
        operator_rem_assign_4(_a, b.as_pointer())
    });
    assert!(((*(*a.borrow()).v.borrow()) == 2_u32));
    ({
        let _a: Ptr<S> = a.as_pointer();
        operator_bitor_assign_6(_a, b.as_pointer())
    });
    assert!(((*(*a.borrow()).v.borrow()) == 6_u32));
    ({
        let _a: Ptr<S> = a.as_pointer();
        operator_bitand_assign_5(_a, b.as_pointer())
    });
    assert!(((*(*a.borrow()).v.borrow()) == 4_u32));
    ({
        let _a: Ptr<S> = a.as_pointer();
        operator_bitxor_assign_7(_a, b.as_pointer())
    });
    assert!(((*(*a.borrow()).v.borrow()) == 0_u32));
    (*(*a.borrow()).v.borrow_mut()) = 3_u32;
    ({
        let _a: Ptr<S> = a.as_pointer();
        operator_shl_assign_8(_a, 2)
    });
    assert!(((*(*a.borrow()).v.borrow()) == 12_u32));
    ({
        let _a: Ptr<S> = a.as_pointer();
        operator_shr_assign_9(_a, 1)
    });
    assert!(((*(*a.borrow()).v.borrow()) == 6_u32));
    ({
        let _a: Ptr<S> = ({
            let _a: Ptr<S> = a.as_pointer();
            operator_add_assign_0(_a, b.as_pointer())
        });
        let _b: Ptr<S> = b.as_pointer();
        operator_add_assign_0(_a, _b)
    });
    assert!(((*(*a.borrow()).v.borrow()) == 14_u32));
    return 0;
}
