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
pub struct p1_result_is_updated_operand_Counter {
    pub n: i32,
}
pub unsafe fn test_0() {
    let mut x: i32 = 0;
    let r: *mut i32 = &mut (*x.prefix_inc());
    (*r) = 5;
    assert!(((x) == (5)));
    assert!(((r) == (&mut x as *mut i32)));
    (*x.prefix_inc()) = 7;
    assert!(((x) == (7)));
    let mut y: i32 = (*x.prefix_inc());
    assert!(((y) == (8)) && ((x) == (8)));
    let s: *mut i32 = r;
    (*(*s).prefix_inc()) = 1;
    assert!(((x) == (1)));
    let mut t: i32 = (*(*s).prefix_inc());
    assert!(((t) == (2)) && ((x) == (2)));
    let mut a: [i32; 3] = [1, 2, 3];
    let mut p: *mut i32 = a.as_mut_ptr();
    let q: *mut *mut i32 = &mut (*p.prefix_inc());
    assert!(((q) == (&mut p as *mut *mut i32)));
    assert!(((*(*q)) == (2)));
    assert!(((*(*p.prefix_inc())) == (3)));
    let mut c: p1_result_is_updated_operand_Counter = p1_result_is_updated_operand_Counter { n: 0 };
    let mut pc: *mut p1_result_is_updated_operand_Counter =
        (&mut c as *mut p1_result_is_updated_operand_Counter);
    let mut v: i32 = (*c.n.prefix_inc());
    assert!(((v) == (1)) && (((*pc).n) == (1)));
    (*c.n.prefix_inc()) = 4;
    assert!((((*pc).n) == (4)));
    let mut sum: i32 = 0;
    {
        let mut k: i32 = 0;
        'loop_: while ((k) < (3)) {
            sum += (*y.prefix_inc());
            k.prefix_inc();
        }
    }
    assert!(((sum) == (30)) && ((y) == (11)));
}
pub unsafe fn test_1() {
    let mut counter: u32 = 0_u32;
    let pre: *mut u32 = &mut (*counter.prefix_dec());
    assert!(((pre) == (&mut counter as *mut u32)) && ((counter) == (!0_u32)));
    let mut x: i32 = 5;
    let r: *mut i32 = &mut x;
    (*r).prefix_dec();
    assert!(((x) == (4)));
    (*(*r).prefix_dec()) = 9;
    assert!(((x) == (9)));
    let mut a: [i32; 3] = [1, 2, 3];
    let mut p: *mut i32 = a.as_mut_ptr().offset((2) as isize);
    let mut n: *mut i32 = (*p.prefix_dec());
    assert!(((*n) == (2)) && ((n) == (p)));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_0() });
    (unsafe { test_1() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
