extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct S {
    pub v: u32,
}
impl S {
    pub unsafe fn operator_assign_1(&mut self, mut n: u32) -> *mut S {
        self.v = n;
        return &mut (*(self as *mut S));
    }
    pub unsafe fn operator_add_assign(&mut self, o: *const S) -> *mut S {
        match ({ (*o).v }, &mut (self.v)) {
            (__tmp_0, __tmp_1) => {
                (*__tmp_1) = (((*__tmp_1) as u32).wrapping_add((__tmp_0 as u32))) as u32;
            }
        };
        return &mut (*(self as *mut S));
    }
    pub unsafe fn operator_sub_assign(&mut self, o: *const S) -> *mut S {
        match ({ (*o).v }, &mut (self.v)) {
            (__tmp_2, __tmp_3) => {
                (*__tmp_3) = (((*__tmp_3) as u32).wrapping_sub((__tmp_2 as u32))) as u32;
            }
        };
        return &mut (*(self as *mut S));
    }
    pub unsafe fn operator_mul_assign(&mut self, o: *const S) -> *mut S {
        match ({ (*o).v }, &mut (self.v)) {
            (__tmp_4, __tmp_5) => {
                (*__tmp_5) = (((*__tmp_5) as u32).wrapping_mul((__tmp_4 as u32))) as u32;
            }
        };
        return &mut (*(self as *mut S));
    }
    pub unsafe fn operator_div_assign(&mut self, o: *const S) -> *mut S {
        match ({ (*o).v }, &mut (self.v)) {
            (__tmp_6, __tmp_7) => {
                (*__tmp_7) = (((*__tmp_7) as u32).wrapping_div((__tmp_6 as u32))) as u32;
            }
        };
        return &mut (*(self as *mut S));
    }
    pub unsafe fn operator_rem_assign(&mut self, o: *const S) -> *mut S {
        match ({ (*o).v }, &mut (self.v)) {
            (__tmp_8, __tmp_9) => {
                (*__tmp_9) = (((*__tmp_9) as u32).wrapping_rem((__tmp_8 as u32))) as u32;
            }
        };
        return &mut (*(self as *mut S));
    }
    pub unsafe fn operator_bitand_assign(&mut self, o: *const S) -> *mut S {
        self.v &= (*o).v;
        return &mut (*(self as *mut S));
    }
    pub unsafe fn operator_bitor_assign(&mut self, o: *const S) -> *mut S {
        self.v |= (*o).v;
        return &mut (*(self as *mut S));
    }
    pub unsafe fn operator_bitxor_assign(&mut self, o: *const S) -> *mut S {
        self.v ^= (*o).v;
        return &mut (*(self as *mut S));
    }
    pub unsafe fn operator_shl_assign(&mut self, mut n: i32) -> *mut S {
        self.v <<= n;
        return &mut (*(self as *mut S));
    }
    pub unsafe fn operator_shr_assign(&mut self, mut n: i32) -> *mut S {
        self.v >>= n;
        return &mut (*(self as *mut S));
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut a: S = S { v: 6_u32 };
    let mut b: S = S { v: 4_u32 };
    (unsafe { S::operator_add_assign(&mut a, &b) });
    assert!(((a.v) == (10_u32)));
    (unsafe { S::operator_sub_assign(&mut a, &b) });
    assert!(((a.v) == (6_u32)));
    (unsafe { S::operator_mul_assign(&mut a, &b) });
    assert!(((a.v) == (24_u32)));
    (unsafe { S::operator_div_assign(&mut a, &b) });
    assert!(((a.v) == (6_u32)));
    (unsafe { S::operator_rem_assign(&mut a, &b) });
    assert!(((a.v) == (2_u32)));
    (unsafe { S::operator_bitor_assign(&mut a, &b) });
    assert!(((a.v) == (6_u32)));
    (unsafe { S::operator_bitand_assign(&mut a, &b) });
    assert!(((a.v) == (4_u32)));
    (unsafe { S::operator_bitxor_assign(&mut a, &b) });
    assert!(((a.v) == (0_u32)));
    (unsafe { S::operator_assign_1(&mut a, 3_u32) });
    assert!(((a.v) == (3_u32)));
    (unsafe { S::operator_shl_assign(&mut a, 2) });
    assert!(((a.v) == (12_u32)));
    (unsafe { S::operator_shr_assign(&mut a, 1) });
    assert!(((a.v) == (6_u32)));
    (unsafe {
        let _o: *const S = &b;
        S::operator_add_assign(&mut (*(unsafe { S::operator_add_assign(&mut a, &b) })), _o)
    });
    assert!(((a.v) == (14_u32)));
    let mut c: S = S { v: 0_u32 };
    c = (*(unsafe { S::operator_assign_1(&mut a, 1_u32) }));
    assert!(((a.v) == (1_u32)));
    assert!(((c.v) == (1_u32)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
