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
pub struct Inner {
    pub a: i32,
    pub b: i32,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Cookie {
    pub x: i32,
    pub data: *mut i32,
    pub in_: *const i32,
    pub inner: Inner,
    pub arr: [i32; 4],
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Counter {
    pub n: i32,
}
impl Counter {
    pub unsafe fn inc(&mut self) {
        self.n.prefix_inc();
    }
}
impl Default for Counter {
    fn default() -> Self {
        Counter { n: 0 }
    }
}
pub unsafe fn set_0(mut p: *mut i32, mut v: i32) {
    (*p) = v;
}
pub unsafe fn get_1(c: *const Cookie) -> i32 {
    return (*c).x;
}
pub unsafe fn first_2(mut p: *const i32) -> i32 {
    return (*p.offset((0) as isize));
}
pub unsafe fn consume_3(cookie: *const Cookie) -> i32 {
    let mut c: Cookie = (*cookie);
    let mut sum: i32 = 0;
    let mut i: i32 = 0;
    'loop_: while ((i) < (4)) {
        (*c.data.offset((i) as isize)) = ((*c.in_.offset((i) as isize)) * (c.x));
        sum += (*c.data.offset((i) as isize));
        i.prefix_inc();
    }
    c.x = sum;
    c.inner.a += c.x;
    c.arr[(1) as usize] = c.inner.a;
    (*c.data) = ((c.arr[(1) as usize]) + (unsafe { first_2(c.in_) }));
    (unsafe {
        let _p: *mut i32 = c.data.offset((1) as isize);
        let _v: i32 = (unsafe { first_2((c.data).cast_const()) });
        set_0(_p, _v)
    });
    return ((c.x) + (c.inner.b));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut data: [i32; 4] = [0, 0, 0, 0];
    let mut in_: [i32; 4] = [1, 2, 3, 4];
    let mut cookie: Cookie = <Cookie>::default();
    cookie.x = 2;
    cookie.data = data.as_mut_ptr();
    cookie.in_ = (in_.as_mut_ptr()).cast_const();
    cookie.inner.a = 1;
    cookie.inner.b = 3;
    assert!(((unsafe { consume_3(&cookie,) }) == (23)));
    assert!(
        (((data[(0) as usize]) == (22)) && ((data[(1) as usize]) == (22)))
            && ((data[(3) as usize]) == (8))
    );
    assert!(((unsafe { get_1(&cookie,) }) == (2)));
    let mut local: Inner = <Inner>::default();
    local.a = 4;
    local.b = ((local.a) * (2));
    local.a.postfix_inc();
    assert!((((local.a) + (local.b)) == (13)));
    let mut field_addr: Inner = Inner { a: 0, b: 0 };
    (unsafe { set_0((&mut field_addr.b as *mut i32), 7) });
    assert!(((field_addr.b) == (7)));
    let mut counter: Counter = <Counter>::default();
    (unsafe { Counter::inc(&mut counter) });
    assert!(((counter.n) == (1)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
