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
    pub v: u32,
}
impl S {
    pub unsafe fn operator_bitnot(&self) -> S {
        return S { v: !self.v };
    }
    pub unsafe fn operator_bitand(&self, o: *const S) -> S {
        return S {
            v: ((self.v) & ((*o).v)),
        };
    }
    pub unsafe fn operator_bitor(&self, o: *const S) -> S {
        return S {
            v: ((self.v) | ((*o).v)),
        };
    }
    pub unsafe fn operator_bitxor(&self, o: *const S) -> S {
        return S {
            v: ((self.v) ^ ((*o).v)),
        };
    }
    pub unsafe fn operator_shl(&self, mut n: i32) -> S {
        return S {
            v: ((self.v) << (n)),
        };
    }
    pub unsafe fn operator_shr(&self, mut n: i32) -> S {
        return S {
            v: ((self.v) >> (n)),
        };
    }
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut a: S = S { v: 12_u32 };
    let mut b: S = S { v: 10_u32 };
    assert!((((unsafe { S::operator_bitnot(&a,) }).v) == (!12_u32)));
    assert!((((unsafe { S::operator_bitand(&a, &b as *const S,) }).v) == (8_u32)));
    assert!((((unsafe { S::operator_bitor(&a, &b as *const S,) }).v) == (14_u32)));
    assert!((((unsafe { S::operator_bitxor(&a, &b as *const S,) }).v) == (6_u32)));
    assert!((((unsafe { S::operator_shl(&a, 2,) }).v) == (48_u32)));
    assert!((((unsafe { S::operator_shr(&a, 2,) }).v) == (3_u32)));
    return 0;
}
