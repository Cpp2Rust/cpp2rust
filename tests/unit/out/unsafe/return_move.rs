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
pub struct node {
    pub value: i32,
    pub next: *mut node,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct pair_t {
    pub a: i32,
    pub n: *mut node,
}
pub static mut global_node_0: std::cell::LazyCell<*mut node> =
    std::cell::LazyCell::new(|| unsafe { std::ptr::null_mut() });
pub unsafe fn id_1(mut n: *mut node) -> *mut node {
    return n;
}
pub unsafe fn pick_2(mut a: *mut node, mut b: *mut node) -> *mut node {
    return if !(a).is_null() { a } else { b };
}
pub unsafe fn local_ptr_3(mut n: *mut node) -> *mut node {
    let mut p: *mut node = (*n).next;
    return p;
}
pub unsafe fn call_once_4(mut n: *mut node, mut m: *mut node) -> *mut node {
    return (unsafe { pick_2(n, m) });
}
pub unsafe fn call_twice_5(mut n: *mut node) -> *mut node {
    return (unsafe {
        let _a: *mut node = n;
        let _b: *mut node = n;
        pick_2(_a, _b)
    });
}
pub unsafe fn next_of_6(mut n: *mut node) -> *mut node {
    return (*n).next;
}
pub unsafe fn ret_global_7() -> *mut node {
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut global_node_0));
}
pub unsafe fn address_taken_8(mut n: *mut node) -> *mut node {
    let mut pp: *mut *mut node = (&mut n as *mut *mut node);
    return if !(*pp).is_null() {
        n
    } else {
        std::ptr::null_mut()
    };
}
pub unsafe fn ret_struct_9(mut a: i32, mut n: *mut node) -> pair_t {
    let mut p: pair_t = <pair_t>::default();
    p.a = a;
    p.n = n;
    return p;
}
pub unsafe fn ret_struct_param_10(mut p: pair_t) -> pair_t {
    return p;
}
pub unsafe fn ret_struct_ref_11(p: *mut pair_t) -> pair_t {
    return (*p);
}
pub unsafe fn ret_vec_12(mut v: Vec<i32>) -> Vec<i32> {
    return std::mem::take(&mut v);
}
pub unsafe fn ret_vec_local_13() -> Vec<i32> {
    let mut v: Vec<i32> = Vec::new();
    {
        let __a1 = 1;
        v.push(__a1)
    };
    return std::mem::take(&mut v);
}
pub unsafe fn ret_loop_14(mut n: *mut node) -> *mut node {
    'loop_: while !((*n).next).is_null() {
        if (((*n).value) == (2)) {
            return n;
        }
        n = (*n).next;
    }
    return n;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut c: node = node {
        value: 3,
        next: std::ptr::null_mut(),
    };
    let mut b: node = node {
        value: 2,
        next: (&mut c as *mut node),
    };
    let mut a: node = node {
        value: 1,
        next: (&mut b as *mut node),
    };
    (*std::cell::LazyCell::force_mut(&mut *&raw mut global_node_0)) = (&mut a as *mut node);
    assert!(((unsafe { id_1((&mut a as *mut node),) }) == (&mut a as *mut node)));
    assert!(((unsafe { local_ptr_3((&mut a as *mut node),) }) == (&mut b as *mut node)));
    assert!(
        ((unsafe { call_once_4(std::ptr::null_mut(), (&mut b as *mut node),) })
            == (&mut b as *mut node))
    );
    assert!(((unsafe { call_twice_5((&mut c as *mut node),) }) == (&mut c as *mut node)));
    assert!(((unsafe { next_of_6((&mut b as *mut node),) }) == (&mut c as *mut node)));
    assert!(((unsafe { ret_global_7() }) == (&mut a as *mut node)));
    assert!(((unsafe { address_taken_8((&mut a as *mut node),) }) == (&mut a as *mut node)));
    let mut p: pair_t = (unsafe { ret_struct_9(4, (&mut a as *mut node)) });
    assert!(((p.a) == (4)) && ((p.n) == (&mut a as *mut node)));
    let mut q: pair_t = (unsafe { ret_struct_param_10(p) });
    assert!(((q.a) == (4)) && ((q.n) == (&mut a as *mut node)));
    let mut r: pair_t = (unsafe { ret_struct_ref_11(&mut q) });
    assert!(((r.a) == (4)) && ((r.n) == (&mut a as *mut node)));
    assert!((((unsafe { ret_vec_12((unsafe { ret_vec_local_13() }),) }).len()) == (1_usize)));
    assert!(((unsafe { ret_loop_14((&mut a as *mut node),) }) == (&mut b as *mut node)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const global_node_0);
}
