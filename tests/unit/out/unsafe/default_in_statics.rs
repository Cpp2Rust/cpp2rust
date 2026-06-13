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
pub struct Inner {
    pub v: i32,
    pub name: *const ::core::ffi::c_char,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Outer {
    pub p1: *mut i32,
    pub p2: *const i32,
    pub arr: [*mut i32; 3],
    pub cp: *const ::core::ffi::c_char,
    pub pp: *mut *mut i32,
    pub inner: Inner,
    pub x: i32,
    pub fn_: Option<unsafe fn(i32) -> i32>,
}
impl Default for Outer {
    fn default() -> Self {
        Outer {
            p1: std::ptr::null_mut(),
            p2: std::ptr::null(),
            arr: [std::ptr::null_mut(); 3],
            cp: std::ptr::null(),
            pp: std::ptr::null_mut(),
            inner: <Inner>::default(),
            x: 0_i32,
            fn_: None,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Foo {
    pub s1: *const ::core::ffi::c_char,
    pub s2: *const ::core::ffi::c_char,
    pub fn1: Option<unsafe fn(i32) -> i32>,
    pub fn2: Option<unsafe fn(i32) -> i32>,
    pub n: i32,
}
impl Default for Foo {
    fn default() -> Self {
        Foo {
            s1: std::ptr::null(),
            s2: std::ptr::null(),
            fn1: None,
            fn2: None,
            n: 0_i32,
        }
    }
}
pub static mut static_fn_0: Option<unsafe fn(i32) -> i32> = unsafe { None };
pub static mut static_outer_1: Outer = unsafe {
    Outer {
        p1: std::ptr::null_mut(),
        p2: std::ptr::null(),
        arr: [std::ptr::null_mut(); 3],
        cp: std::ptr::null(),
        pp: std::ptr::null_mut(),
        inner: Inner {
            v: 0_i32,
            name: std::ptr::null(),
        },
        x: 0_i32,
        fn_: None,
    }
};
pub static mut static_inner_array_2: [Inner; 2] = unsafe {
    [Inner {
        v: 0_i32,
        name: std::ptr::null(),
    }; 2]
};
pub static mut static_foo_3: Foo = unsafe {
    Foo {
        s1: c"hello".as_ptr(),
        s2: std::ptr::null(),
        fn1: None,
        fn2: None,
        n: 42,
    }
};
pub static mut static_foo_array_4: [Foo; 2] = unsafe {
    [
        Foo {
            s1: c"first".as_ptr(),
            s2: std::ptr::null(),
            fn1: None,
            fn2: None,
            n: 1,
        },
        Foo {
            s1: c"second".as_ptr(),
            s2: std::ptr::null(),
            fn1: None,
            fn2: None,
            n: 2,
        },
    ]
};
pub unsafe fn check_local_static_5() {
    static mut local_outer_6: Outer = unsafe {
        Outer {
            p1: std::ptr::null_mut(),
            p2: std::ptr::null(),
            arr: [std::ptr::null_mut(); 3],
            cp: std::ptr::null(),
            pp: std::ptr::null_mut(),
            inner: Inner {
                v: 0_i32,
                name: std::ptr::null(),
            },
            x: 0_i32,
            fn_: None,
        }
    };;
    static mut local_fn_7: Option<unsafe fn(i32) -> i32> = unsafe { None };;
    static mut local_p_8: *mut i32 = unsafe { std::ptr::null_mut() };;
    assert!((local_outer_6.p1).is_null());
    assert!((local_outer_6.fn_).is_none());
    assert!((local_fn_7).is_none());
    assert!((local_p_8).is_null());
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!((static_fn_0).is_none());
    assert!((static_outer_1.p1).is_null());
    assert!((static_outer_1.p2).is_null());
    assert!((static_outer_1.cp).is_null());
    assert!((static_outer_1.pp).is_null());
    assert!((static_outer_1.fn_).is_none());
    let mut i: i32 = 0;
    'loop_: while ((i) < (3)) {
        assert!((static_outer_1.arr[(i) as usize]).is_null());
        i.prefix_inc();
    }
    assert!((static_outer_1.inner.name).is_null());
    let mut i: i32 = 0;
    'loop_: while ((i) < (2)) {
        assert!((static_inner_array_2[(i) as usize].name).is_null());
        i.prefix_inc();
    }
    assert!((static_foo_3.s2).is_null());
    assert!((static_foo_3.fn1).is_none());
    assert!((static_foo_3.fn2).is_none());
    assert!(((static_foo_3.n) == (42)));
    let mut i: i32 = 0;
    'loop_: while ((i) < (2)) {
        assert!((static_foo_array_4[(i) as usize].s2).is_null());
        assert!((static_foo_array_4[(i) as usize].fn1).is_none());
        assert!((static_foo_array_4[(i) as usize].fn2).is_none());
        i.prefix_inc();
    }
    (unsafe { check_local_static_5() });
    return 0;
}
