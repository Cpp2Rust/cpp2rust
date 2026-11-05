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
    let mut p1: (f64, i32) = ((1 as f64), 2);
    assert!(((p1.0) == (1_f64)));
    assert!(((p1.1) == (2)));
    let mut p2: (f64, i32) = ((3 as f64), 4);
    assert!(((p2.0) == (3_f64)));
    assert!(((p2.1) == (4)));
    let mut p3: (f64, i32) = match ((&mut (5, 6) as *mut (i32, i32)),) {
        (__tmp_0,) => (((*&mut (*__tmp_0).0) as f64), (*&mut (*__tmp_0).1)),
    };
    assert!(((p3.0) == (5_f64)));
    assert!(((p3.1) == (6)));
    let arr: [i32; 2] = [1, 2];
    let mut n: u32 = 2_u32;
    let mut const_ptr_pairs: Vec<(*const i32, u64)> = (0..(1_usize) as usize)
        .map(|_| <(*const i32, u64)>::default())
        .collect::<Vec<_>>();
    match ((&mut (arr.as_ptr().offset((1) as isize), n) as *mut (*const i32, u32)),) {
        (__tmp_1,) => {
            const_ptr_pairs[(0_usize)] = (
                <*const i32 as CppFromUnsafe<*const i32>>::cpp_from_move(&mut (*__tmp_1).0),
                ((*&mut (*__tmp_1).1) as u64),
            )
        }
    };
    assert!(
        ((*const_ptr_pairs[(0_usize)].0) == (2)) && ((const_ptr_pairs[(0_usize)].1) == (2_u64))
    );
    let mut p: (i32, i32) = (1, 2);
    let mut q: (i32, i32) = (3, 4);
    let mut f: *mut i32 = (&mut p.0 as *mut i32);
    p = q.clone();
    assert!(((*f) == (3)));
    let alias: *mut (i32, i32) = &mut p;
    p = (*alias).clone();
    assert!(((*f) == (3)));
    let mut a: (i32, i32) = (1, 2);
    let mut pa: *mut i32 = (&mut a.0 as *mut i32);
    let mut b: (i32, i32) = std::mem::take(&mut a);
    (*pa) = 9;
    assert!(((b.0) == (1)));
    assert!(((a.0) == (9)));
    let mut d: (i32, i32) = (1, 2);
    let mut s: (i32, i32) = (3, 4);
    let mut pd: *mut i32 = (&mut d.0 as *mut i32);
    let mut ps: *mut i32 = (&mut s.0 as *mut i32);
    d = std::mem::take(&mut s);
    assert!(((*pd) == (3)));
    (*ps) = 5;
    assert!(((d.0) == (3)));
    assert!(((s.0) == (5)));
    let mut w: (i64, i64) = ((1 as i64), (2 as i64));
    let mut narrow: (i32, i32) = (3, 4);
    let mut pw: *mut i64 = (&mut w.0 as *mut i64);
    w = (((*&narrow.0) as i64), ((*&narrow.1) as i64));
    assert!(((*pw) == (3_i64)));
    let mut m: (i32, i32) = (5, 6);
    w = (((*&mut m.0) as i64), ((*&mut m.1) as i64));
    assert!(((*pw) == (5_i64)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
