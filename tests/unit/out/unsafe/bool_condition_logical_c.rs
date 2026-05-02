extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[derive(Clone, Copy, PartialEq, Debug, Default)]
enum Code {
    #[default]
    CODE_OK = 0,
    CODE_ERR = 1,
    CODE_FATAL = 2,
}
impl From<i32> for Code {
    fn from(n: i32) -> Code {
        match n {
            0 => Code::CODE_OK,
            1 => Code::CODE_ERR,
            2 => Code::CODE_FATAL,
            _ => panic!("invalid Code value: {}", n),
        }
    }
}
pub static mut side_effect: i32 = 0;
pub unsafe fn observe_0(mut v: i32) -> i32 {
    side_effect.prefix_inc();
    return v;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut n: i32 = 3;
    let mut zero: i32 = 0;
    let mut storage: i32 = 7;
    let mut p: *mut i32 = (&mut storage as *mut i32);
    let mut np: *mut i32 = Default::default();
    let mut u: u32 = 4_u32;
    let mut code: Code = Code::from((Code::CODE_OK as i32));
    if (n != 0) && (!(p).is_null()) {
        assert!((1 != 0));
    }
    if (n != 0) && (!(np).is_null()) {
        assert!((0 != 0));
    }
    if (zero != 0) || (!(p).is_null()) {
        assert!((1 != 0));
    }
    if (zero != 0) || (!(np).is_null()) {
        assert!((0 != 0));
    }
    if (((n != 0) && (u != 0)) && (!(p).is_null()))
        && ((code as u32) == ((Code::CODE_OK as i32) as u32))
    {
        assert!((1 != 0));
    }
    side_effect = 0;
    if (zero != 0)
        && ((unsafe {
            let _v: i32 = 1;
            observe_0(_v)
        }) != 0)
    {
        assert!((0 != 0));
    }
    assert!(((side_effect) == (0)));
    if (n != 0)
        || ((unsafe {
            let _v: i32 = 1;
            observe_0(_v)
        }) != 0)
    {
        assert!((1 != 0));
    }
    assert!(((side_effect) == (0)));
    return 0;
}
