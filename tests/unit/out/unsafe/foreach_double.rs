extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut v: Vec<i32> = Vec::new();
    {
        let __a1 = 1;
        v.push(__a1)
    };
    {
        let __a1 = 2;
        v.push(__a1)
    };
    {
        let __a1 = 3;
        v.push(__a1)
    };
    let mut square: i32 = 0;
    'loop_: for e1 in IterRange::new(v.begin(), v.end()) {
        let mut e1: i32 = (*e1.iter_deref());
        'loop_: for e2 in IterRange::new(v.begin(), v.end()) {
            let mut e2: i32 = (*e2.iter_deref());
            square += ((e1) * (e2));
        }
    }
    'loop_: for e1 in IterRange::new(v.begin(), v.end()) {
        let e1: *mut i32 = &mut (*e1.iter_deref());
        'loop_: for e2 in IterRange::new(v.begin(), v.end()) {
            let e2: *const i32 = &(*e2.iter_deref());
            square += ((*e1) * (*e2));
        }
    }
    'loop_: for e1 in IterRange::new(v.begin(), v.end()) {
        let e1: *const i32 = &(*e1.iter_deref());
        'loop_: for e2 in IterRange::new(v.begin(), v.end()) {
            let e2: *mut i32 = &mut (*e2.iter_deref());
            square += ((*e1) * (*e2));
        }
    }
    'loop_: for e1 in IterRange::new(v.begin(), v.end()) {
        let e1: *mut i32 = &mut (*e1.iter_deref());
        'loop_: for e2 in IterRange::new(v.begin(), v.end()) {
            let e2: *mut i32 = &mut (*e2.iter_deref());
            square += ((*e1) * (*e2));
        }
    }
    let mut m: Vec<Vec<i32>> = Vec::new();
    let mut v1: Vec<i32> = Vec::new();
    {
        let __a1 = std::mem::take(&mut v1);
        m.push(__a1)
    };
    let mut v2: Vec<i32> = Vec::new();
    {
        let __a1 = std::mem::take(&mut v2);
        m.push(__a1)
    };
    let mut v3: Vec<i32> = Vec::new();
    {
        let __a1 = std::mem::take(&mut v3);
        m.push(__a1)
    };
    'loop_: for row in IterRange::new(m.begin(), m.end()) {
        let row: *mut Vec<i32> = &mut (*row.iter_deref());
        'loop_: for col in IterRange::new((*row).begin(), (*row).end()) {
            let col: *mut i32 = &mut (*col.iter_deref());
            square += (*col);
        }
    }
    assert!(((square) == (144)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
