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
    pub x: i32,
    pub y: i32,
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut base: i32 = 10;
    let mut add_base: FnPtr<fn(i32) -> i32> = {
        #[repr(C)]
        struct Captures {
            base: *mut i32,
        }
        FnPtr::<fn(i32) -> i32>::with_captures_unsafe(
            Captures { base: &mut base },
            (|this: &mut Captures, x: i32| unsafe {
                return ((x) + (*this.base));
            }),
        )
    };
    assert!(((unsafe { add_base.call(5,) }) == (15)));
    base = 100;
    assert!(((unsafe { add_base.call(5,) }) == (105)));
    let mut s: S = S { x: 1, y: 2 };
    let mut sum: FnPtr<fn() -> i32> = {
        #[repr(C)]
        struct Captures {
            s: *mut S,
        }
        FnPtr::<fn() -> i32>::with_captures_unsafe(
            Captures { s: &mut s },
            (|this: &mut Captures| unsafe {
                return (((*this.s).x) + ((*this.s).y));
            }),
        )
    };
    assert!(((unsafe { sum.call() }) == (3)));
    s.x = 50;
    assert!(((unsafe { sum.call() }) == (52)));
    let mut counter: i32 = 0;
    let mut bump: FnPtr<fn()> = {
        #[repr(C)]
        struct Captures {
            counter: *mut i32,
        }
        FnPtr::<fn()>::with_captures_unsafe(
            Captures {
                counter: &mut counter,
            },
            (|this: &mut Captures| unsafe {
                (*this.counter).postfix_inc();
            }),
        )
    };
    (unsafe { bump.call() });
    (unsafe { bump.call() });
    assert!(((counter) == (2)));
    let mut arr: [u16; 4] = [3_u16, 1_u16, 2_u16, 0_u16];
    let mut swap: FnPtr<fn(usize, usize)> = {
        #[repr(C)]
        struct Captures {
            arr: *mut [u16; 4],
        }
        FnPtr::<fn(usize, usize)>::with_captures_unsafe(
            Captures { arr: &mut arr },
            (|this: &mut Captures, i: usize, j: usize| unsafe {
                let mut t: u16 = (*this.arr)[(j)];
                (*this.arr)[(j)] = (*this.arr)[(i)];
                (*this.arr)[(i)] = t;
            }),
        )
    };
    (unsafe { swap.call(0_usize, 3_usize) });
    assert!(((arr[(0) as usize] as i32) == (0)));
    assert!(((arr[(3) as usize] as i32) == (3)));
    let mut total: i32 = 0;
    let mut add: FnPtr<fn(i32)> = {
        #[repr(C)]
        struct Captures {
            t: *mut i32,
        }
        FnPtr::<fn(i32)>::with_captures_unsafe(
            Captures { t: &mut total },
            (|this: &mut Captures, x: i32| unsafe {
                (*this.t) += x;
            }),
        )
    };
    (unsafe { add.call(2) });
    (unsafe { add.call(3) });
    assert!(((total) == (5)));
    let mut set_y: FnPtr<fn(i32)> = {
        #[repr(C)]
        struct Captures {
            y: *mut i32,
        }
        FnPtr::<fn(i32)>::with_captures_unsafe(
            Captures { y: &mut s.y },
            (|this: &mut Captures, v: i32| unsafe {
                (*this.y) = v;
            }),
        )
    };
    (unsafe { set_y.call(9) });
    assert!(((s.y) == (9)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
