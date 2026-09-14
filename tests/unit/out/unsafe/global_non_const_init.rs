extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn next_0() -> i32 {
    static mut counter_1: i32 = unsafe { 0 };;
    return counter_1.prefix_inc();
}
pub unsafe fn marker_2(mut tag: u8) -> u8 {
    return ((((tag as i32) << (3)) | (2)) as u8);
}
pub static mut signature_3: [u8; 3] = unsafe {
    [
        (unsafe { marker_2(1_u8) }),
        4_u8,
        (('B' as libc::c_char) as u8),
    ]
};
pub static mut single_4: u8 = unsafe { (unsafe { marker_2(2_u8) }) };
pub static mut from_call_5: i32 = unsafe { (unsafe { next_0() }) };
pub static mut depends_on_call_6: i32 = unsafe { ((from_call_5) + (1)) };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Ctor {
    pub v: i32,
}
impl Ctor {
    pub unsafe fn Ctor1() -> Self {
        let mut this = Self {
            v: (unsafe { next_0() }),
        };
        this
    }
    pub unsafe fn Ctor2(mut x: i32) -> Self {
        let mut this = Self { v: x };
        this
    }
}
impl Default for Ctor {
    fn default() -> Self {
        unsafe { Ctor::Ctor1() }
    }
}
pub static mut default_ctor_7: Ctor = unsafe { Ctor::Ctor1() };
pub static mut arg_ctor_8: Ctor = unsafe { Ctor::Ctor2({ 7 }) };
pub static mut str_9: Vec<libc::c_char> = unsafe {
    {
        let s = c"abc".as_ptr();
        std::slice::from_raw_parts(s, (0..).take_while(|&i| *s.add(i) != 0).count() + 1).to_vec()
    }
};
pub static mut inline_member_11: Ctor = unsafe { Ctor::Ctor2({ 5 }) };
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Holder {}
pub static mut member_10: i32 = unsafe { (unsafe { next_0() }) };
pub unsafe fn local_static_12() -> i32 {
    static mut once_13: i32 = unsafe { (unsafe { next_0() }) };;
    static mut local_ctor_14: Ctor = unsafe { Ctor::Ctor2({ 3 }) };;
    return ((once_13) + (local_ctor_14.v));
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Singleton {
    pub hits: i32,
}
impl Singleton {
    pub unsafe fn Singleton() -> Self {
        let mut this = Self { hits: 0 };
        this
    }
    pub unsafe fn instance() -> *mut Singleton {
        static mut s_15: Singleton = unsafe { Singleton::Singleton() };;
        return &mut s_15 as *mut Singleton;
    }
}
impl Default for Singleton {
    fn default() -> Self {
        unsafe { Singleton::Singleton() }
    }
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(((signature_3[(0) as usize] as i32) == (10)));
    assert!(((signature_3[(1) as usize] as i32) == (4)));
    assert!(((single_4 as i32) == (18)));
    assert!(((from_call_5) == (1)));
    assert!(((depends_on_call_6) == (2)));
    assert!(((default_ctor_7.v) == (2)));
    assert!(((arg_ctor_8.v) == (7)));
    assert!(
        str_9 == {
            let s = c"abc".as_ptr();
            std::slice::from_raw_parts(s, (0..).take_while(|&i| *s.add(i) != 0).count() + 1)
                .to_vec()
        }
    );
    assert!(((member_10) == (3)));
    assert!(((inline_member_11.v) == (5)));
    assert!(((unsafe { local_static_12() }) == (7)));
    assert!(((unsafe { local_static_12() }) == (7)));
    (*(unsafe { Singleton::instance() })).hits.postfix_inc();
    (*(unsafe { Singleton::instance() })).hits.postfix_inc();
    assert!((((*(unsafe { Singleton::instance() })).hits) == (2)));
    assert!(((unsafe { Singleton::instance() }) == (unsafe { Singleton::instance() })));
    return 0;
}
