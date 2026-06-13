extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[derive(Clone, Copy, PartialEq, Debug, Default)]
enum Tag_enum {
    #[default]
    T_NUM_S = 0,
    T_NUM_U = 1,
    T_TEXT = 2,
    T_FLOAT = 3,
    T_REF = 4,
}
impl From<i32> for Tag_enum {
    fn from(n: i32) -> Tag_enum {
        match n {
            0 => Tag_enum::T_NUM_S,
            1 => Tag_enum::T_NUM_U,
            2 => Tag_enum::T_TEXT,
            3 => Tag_enum::T_FLOAT,
            4 => Tag_enum::T_REF,
            _ => panic!("invalid Tag_enum value: {}", n),
        }
    }
}
libcc2rs::impl_enum_inc_dec!(Tag_enum);
#[repr(C)]
#[derive(Copy, Clone)]
pub union anon_0 {
    pub text: *const ::core::ffi::c_char,
    pub handle: *mut ::libc::c_void,
    pub signed_n: i64,
    pub unsigned_n: u64,
    pub f: f64,
}
impl Default for anon_0 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Slot {
    pub tag: Tag_enum,
    pub payload: anon_0,
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut a: Slot = <Slot>::default();
    a.tag = Tag_enum::T_NUM_S;
    a.payload.signed_n = (-7_i32 as i64);
    assert!(((((a.payload.signed_n) == (-7_i32 as i64)) as i32) != 0));
    let mut b: Slot = <Slot>::default();
    b.tag = Tag_enum::T_NUM_U;
    b.payload.unsigned_n = 3735928559_u64;
    assert!(((((b.payload.unsigned_n) == (3735928559_u64)) as i32) != 0));
    let mut c: Slot = <Slot>::default();
    c.tag = Tag_enum::T_TEXT;
    c.payload.text = (c"hello".as_ptr().cast_mut()).cast_const();
    assert!((((((*c.payload.text.offset((0) as isize)) as i32) == ('h' as i32)) as i32) != 0));
    let mut d: Slot = <Slot>::default();
    d.tag = Tag_enum::T_FLOAT;
    d.payload.f = 1.5E+0;
    assert!(((((d.payload.f) == (1.5E+0)) as i32) != 0));
    let mut x: i32 = 0;
    let mut e: Slot = <Slot>::default();
    e.tag = Tag_enum::T_REF;
    e.payload.handle = ((&mut x as *mut i32) as *mut i32 as *mut ::libc::c_void);
    assert!(
        ((((e.payload.handle) == ((&mut x as *mut i32) as *mut i32 as *mut ::libc::c_void))
            as i32)
            != 0)
    );
    return 0;
}
