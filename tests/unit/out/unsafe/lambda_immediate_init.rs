extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn pick_0(mut x: Option<i32>) -> i32 {
    let mut x: i32 = x.unwrap_or(
        (unsafe {
            (|| {
                return 237;
            })()
        }),
    );
    return x;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct S {
    pub j: i32,
    pub i: i32,
    pub k: i32,
}
impl Default for S {
    fn default() -> Self {
        S {
            j: 10,
            i: (unsafe {
                (|| {
                    return ((self.j) * (2));
                })()
            }),
            k: ((unsafe {
                (|| {
                    return 3;
                })()
            }) + (1)),
        }
    }
}
pub static mut g_1: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe {
    (unsafe {
        (|| {
            let mut s: i32 = 0;
            let mut i: i32 = 1;
            'loop_: while ((i) <= (4)) {
                s += i;
                i.postfix_inc();
            }
            return s;
        })()
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
    assert!(((s.i) == (20)));
    assert!(((s.k) == (4)));
    let mut t: S = S {
        j: 5,
        i: (unsafe {
            (|| {
                return ((self.j) * (2));
            })()
        }),
        k: ((unsafe {
            (|| {
                return 3;
            })()
        }) + (1)),
    };
    assert!(((t.i) == (10)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut g_1)) == (10)));
    let mut a: i32 = 2;
    let c: i32 = (unsafe {
        (|| {
            a.postfix_inc();
            return ((a) * (10));
        })()
    });
    assert!(((c) == (30)));
    assert!(((a) == (3)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const g_1);
}
