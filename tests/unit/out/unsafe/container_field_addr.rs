extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Clone, VaArg, Default)]
pub struct S {
    pub tag: i32,
    pub v: Vec<i32>,
    pub m: BTreeMap<i32, Box<i32>>,
}
pub unsafe fn add_0(mut v: *mut Vec<i32>, mut k: i32) {
    {
        let a0_clone = k.clone();
        (*v).push(a0_clone)
    };
}
pub unsafe fn put_1(mut m: *mut BTreeMap<i32, Box<i32>>, mut k: i32, mut v: i32) {
    (*(*m).entry(k).or_default().as_mut()) = v;
}
pub unsafe fn run_2(mut h: *mut S) {
    (unsafe {
        let _v: *mut Vec<i32> = (&mut (*h).v as *mut Vec<i32>);
        let _k: i32 = (*h).tag;
        add_0(_v, _k)
    });
    (unsafe {
        let _m: *mut BTreeMap<i32, Box<i32>> = (&mut (*h).m as *mut BTreeMap<i32, Box<i32>>);
        let _k: i32 = (*h).tag;
        put_1(_m, _k, 2)
    });
    let mut pv: *mut Vec<i32> = (&mut (*h).v as *mut Vec<i32>);
    {
        let __a1 = ((*(pv).cast_const()).len() as i32);
        (*pv).push(__a1)
    };
    assert!(
        ((((*h).v.len()) == (2_usize)) && (((&mut (*h)).v[(0_usize)]) == (7)))
            && (((&mut (*h)).v[(1_usize)]) == (1))
    );
    assert!(((*(*h).m.entry(7).or_default().as_mut()) == (2)));
    assert!((((*h).tag) == (7)));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut local: S = <S>::default();
    local.tag = 7;
    (unsafe { run_2((&mut local as *mut S)) });
    let mut heap: *mut S = (Box::leak(Box::new(<S>::default())) as *mut S);
    (*heap).tag = 7;
    (unsafe { run_2(heap) });
    ::std::mem::drop(Box::from_raw(heap));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
