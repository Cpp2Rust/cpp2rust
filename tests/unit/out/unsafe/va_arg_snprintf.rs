extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn extract_first_0(
    mut buf: *mut libc::c_char,
    mut size: i32,
    mut fmt: *const libc::c_char,
    __args: &[VaArg],
) -> i32 {
    let mut ap: VaList = VaList::default();
    ap = VaList::new(__args);
    let mut n: i32 = ap.arg::<i32>();
    (*buf.offset((0) as isize)) = (n as libc::c_char);
    return n;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut buf: [libc::c_char; 64] = [(0 as libc::c_char); 64];
    assert!(
        ((((unsafe {
            extract_first_0(
                buf.as_mut_ptr(),
                1,
                (c"%d".as_ptr().cast_mut()).cast_const(),
                &[(42).into()],
            )
        }) == (42)) as i32)
            != 0)
    );
    assert!(((((buf[(0) as usize] as i32) == (42)) as i32) != 0));
    assert!(
        ((((unsafe {
            extract_first_0(
                buf.as_mut_ptr(),
                1,
                (c"%d".as_ptr().cast_mut()).cast_const(),
                &[(65).into()],
            )
        }) == (65)) as i32)
            != 0)
    );
    assert!(((((buf[(0) as usize] as i32) == ('A' as i32)) as i32) != 0));
    return 0;
}
