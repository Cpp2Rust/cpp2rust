extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct S {
    pub v: i32,
}
impl S {
    pub unsafe fn operator_add_pconstS_const(&self, o: *const S) -> S {
        return S {
            v: ((self.v) + ((*o).v)),
        };
    }
    pub unsafe fn operator_sub_pconstS_const(&self, o: *const S) -> S {
        return S {
            v: ((self.v) - ((*o).v)),
        };
    }
    pub unsafe fn operator_mul(&self, o: *const S) -> S {
        return S {
            v: ((self.v) * ((*o).v)),
        };
    }
    pub unsafe fn operator_div(&self, o: *const S) -> S {
        return S {
            v: ((self.v) / ((*o).v)),
        };
    }
    pub unsafe fn operator_rem(&self, o: *const S) -> S {
        return S {
            v: ((self.v) % ((*o).v)),
        };
    }
    pub unsafe fn operator_pos_const(&self) -> S {
        return S { v: self.v };
    }
    pub unsafe fn operator_neg_const(&self) -> S {
        return S { v: -self.v };
    }
    pub unsafe fn operator_inc(&mut self) -> *mut S {
        self.v.prefix_inc();
        return &mut (*self) as *mut S;
    }
    pub unsafe fn operator_post_inc_i32(&mut self, _: i32) -> S {
        let mut old: S = (*self);
        self.v.prefix_inc();
        return old;
    }
    pub unsafe fn operator_dec(&mut self) -> *mut S {
        self.v.prefix_dec();
        return &mut (*self) as *mut S;
    }
    pub unsafe fn operator_post_dec_i32(&mut self, _: i32) -> S {
        let mut old: S = (*self);
        self.v.prefix_dec();
        return old;
    }
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut a: S = S { v: 7 };
    let mut b: S = S { v: 2 };
    assert!((((unsafe { S::operator_add_pconstS_const(&a, &b as *const S,) }).v) == (9)));
    assert!((((unsafe { S::operator_sub_pconstS_const(&a, &b as *const S,) }).v) == (5)));
    assert!((((unsafe { S::operator_mul(&a, &b as *const S,) }).v) == (14)));
    assert!((((unsafe { S::operator_div(&a, &b as *const S,) }).v) == (3)));
    assert!((((unsafe { S::operator_rem(&a, &b as *const S,) }).v) == (1)));
    assert!((((unsafe { S::operator_pos_const(&a,) }).v) == (7)));
    assert!((((unsafe { S::operator_neg_const(&a,) }).v) == (-7_i32)));
    assert!((((*(unsafe { S::operator_inc(&mut a,) })).v) == (8)));
    assert!((((unsafe { S::operator_post_inc_i32(&mut a, 0,) }).v) == (8)));
    assert!(((a.v) == (9)));
    assert!((((*(unsafe { S::operator_dec(&mut a,) })).v) == (8)));
    assert!((((unsafe { S::operator_post_dec_i32(&mut a, 0,) }).v) == (8)));
    assert!(((a.v) == (7)));
    assert!(
        (((*(unsafe { S::operator_inc(&mut (*(unsafe { S::operator_inc(&mut a,) })),) })).v)
            == (9))
    );
    return 0;
}
