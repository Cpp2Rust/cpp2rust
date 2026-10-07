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
pub struct context {
    pub verbose: i32,
    pub last_error: i32,
}
pub unsafe fn set_error_0(mut ctx: *mut context, mut fmt: *const libc::c_char, __args: &[VaArg]) {
    if ((*ctx).verbose != 0) {
        let mut ap: VaList = VaList::default();
        ap = VaList::new(__args);
        (*ctx).last_error = ap.arg::<i32>();
    }
}
#[repr(C)]
#[derive(Copy, Clone, FnPtrArg, VaArg)]
pub union value {
    pub i: i32,
    pub l: i64,
}
impl Default for value {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
pub unsafe fn pick_1(mut use_long: i32, __args: &[VaArg]) -> i64 {
    let mut ap: VaList = VaList::default();
    ap = VaList::new(__args);
    let mut v: value = ap.arg::<value>();
    if (use_long != 0) {
        return v.l;
    }
    return (v.i as i64);
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut ctx: context = <context>::default();
    ctx.verbose = 1;
    ctx.last_error = 0;
    (unsafe {
        set_error_0(
            (&mut ctx as *mut context),
            (c"error %d".as_ptr().cast_mut()).cast_const(),
            &[(42).into()],
        )
    });
    assert!(((((ctx.last_error) == (42)) as i32) != 0));
    ctx.verbose = 0;
    (unsafe {
        set_error_0(
            (&mut ctx as *mut context),
            (c"error %d".as_ptr().cast_mut()).cast_const(),
            &[(99).into()],
        )
    });
    assert!(((((ctx.last_error) == (42)) as i32) != 0));
    let mut v: value = <value>::default();
    v.l = ((1_i64) << (40));
    assert!(((((unsafe { pick_1(1, &[(v).into(),]) }) == ((1_i64) << (40))) as i32) != 0));
    v.i = 7;
    assert!(((((unsafe { pick_1(0, &[(v).into(),]) }) == (7_i64)) as i32) != 0));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
