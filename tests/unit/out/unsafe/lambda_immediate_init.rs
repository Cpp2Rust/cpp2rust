extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn pick_0(mut x: Option<i32>) -> i32 {
    let mut x: i32 = x.unwrap_or_else(|| unsafe {
        (unsafe {
            FnPtr::<fn() -> i32>::new(|| -> i32 {
                unsafe {
                    return 237;
                }
            })
            .call()
        })
    });
    return x;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct S {
    pub j: i32,
    pub k: i32,
}
impl Default for S {
    fn default() -> Self {
        unsafe {
            S {
                j: 10,
                k: ((unsafe {
                    FnPtr::<fn() -> i32>::new(|| -> i32 {
                        unsafe {
                            return 3;
                        }
                    })
                    .call()
                }) + (1)),
            }
        }
    }
}
pub static mut g_1: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe {
    (unsafe {
        FnPtr::<fn() -> i32>::new(|| -> i32 {
            unsafe {
                let mut s: i32 = 0;
                let mut i: i32 = 1;
                'loop_: while ((i) <= (4)) {
                    s += i;
                    i.postfix_inc();
                }
                return s;
            }
        })
        .call()
    })
});
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(((unsafe { pick_0(None,) }) == (237)));
    assert!(((unsafe { pick_0(Some(1),) }) == (1)));
    let mut s: S = <S>::default();
    assert!(((s.k) == (4)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut g_1)) == (10)));
    let mut a: i32 = 2;
    let c: i32 = (unsafe {
        lambda_unsafe!(
            {
                let a: *mut i32 = &mut a;
            },
            || -> i32 {
                (*a).postfix_inc();
                return ((*a) * (10));
            }
        )
        .call()
    });
    assert!(((c) == (30)));
    assert!(((a) == (3)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const g_1);
}
