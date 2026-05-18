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
pub struct context {
    pub verbose: i32,
    pub last_error: i32,
}
pub unsafe fn set_error_0(mut ctx: *mut context, mut fmt: *const u8, __args: &[VaArg]) {
    if ((*ctx).verbose != 0) {
        let mut ap: VaList = VaList::default();
        ap = VaList::new(__args);
        (*ctx).last_error = (ap.arg::<i32>()).clone();
    }
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut ctx: context = <context>::default();
    ctx.verbose = 1;
    ctx.last_error = 0;
    (unsafe {
        let _ctx: *mut context = (&mut ctx as *mut context);
        let _fmt: *const u8 = b"error %d\0".as_ptr().cast_mut().cast_const();
        set_error_0(_ctx, _fmt, &[42.into()])
    });
    assert!(((((ctx.last_error) == (42)) as i32) != 0));
    ctx.verbose = 0;
    (unsafe {
        let _ctx: *mut context = (&mut ctx as *mut context);
        let _fmt: *const u8 = b"error %d\0".as_ptr().cast_mut().cast_const();
        set_error_0(_ctx, _fmt, &[99.into()])
    });
    assert!(((((ctx.last_error) == (42)) as i32) != 0));
    return 0;
}
