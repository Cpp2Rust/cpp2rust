extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn extract_first_0(
    mut buf: *mut u8,
    mut size: i32,
    mut fmt: *const u8,
    __args: &[VaArg],
) -> i32 {
    let mut ap: VaList = VaList::default();
    ap = VaList::new(__args);
    let mut n: i32 = ap.arg::<i32>();
    (*buf.offset((0) as isize)) = (n as u8);
    return n;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut buf: [u8; 64] = [0_u8; 64];
    assert!(
        ((((unsafe {
            let _buf: *mut u8 = buf.as_mut_ptr();
            let _size: i32 = 1;
            let _fmt: *const u8 = b"%d\0".as_ptr().cast_mut().cast_const();
            extract_first_0(_buf, _size, _fmt, &[42.into()])
        }) == (42)) as i32)
            != 0)
    );
    assert!(((((buf[(0) as usize] as i32) == (42)) as i32) != 0));
    assert!(
        ((((unsafe {
            let _buf: *mut u8 = buf.as_mut_ptr();
            let _size: i32 = 1;
            let _fmt: *const u8 = b"%d\0".as_ptr().cast_mut().cast_const();
            extract_first_0(_buf, _size, _fmt, &[65.into()])
        }) == (65)) as i32)
            != 0)
    );
    assert!(((((buf[(0) as usize] as i32) == ('A' as i32)) as i32) != 0));
    return 0;
}
