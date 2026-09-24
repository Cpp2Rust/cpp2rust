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
    assert!(
        ((unsafe {
            (|x: i32| {
                return ((x) + (base));
            })(5)
        }) == (15))
    );
    base = 100;
    assert!(
        ((unsafe {
            (|x: i32| {
                return ((x) + (base));
            })(5)
        }) == (105))
    );
    let mut s: S = S { x: 1, y: 2 };
    assert!(
        ((unsafe {
            (|| {
                return ((s.x) + (s.y));
            })()
        }) == (3))
    );
    s.x = 50;
    assert!(
        ((unsafe {
            (|| {
                return ((s.x) + (s.y));
            })()
        }) == (52))
    );
    let mut counter: i32 = 0;
    (unsafe {
        (|| {
            counter.postfix_inc();
        })()
    });
    (unsafe {
        (|| {
            counter.postfix_inc();
        })()
    });
    assert!(((counter) == (2)));
    let mut arr: [u16; 4] = [3_u16, 1_u16, 2_u16, 0_u16];
    (unsafe {
        (|i: usize, j: usize| {
            let mut t: u16 = arr[(j)];
            arr[(j)] = arr[(i)];
            arr[(i)] = t;
        })(0_usize, 3_usize)
    });
    assert!(((arr[(0) as usize] as i32) == (0)));
    assert!(((arr[(3) as usize] as i32) == (3)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
