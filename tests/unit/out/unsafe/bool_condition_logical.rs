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
    let mut code: Code = Code::CODE_OK;
    if ((n != 0) && (!(p).is_null())) {
        assert!(true);
    }
    if ((n != 0) && (!(np).is_null())) {
        assert!(false);
    }
    if ((zero != 0) || (!(p).is_null())) {
        assert!(true);
    }
    if ((zero != 0) || (!(np).is_null())) {
        assert!(false);
    }
    if ((((n != 0) && (u != 0)) && (!(p).is_null())) && ((code as i32) == (Code::CODE_OK as i32))) {
        assert!(true);
    }
    side_effect = 0;
    if ((zero != 0)
        && ((unsafe {
            let _v: i32 = 1;
            observe_0(_v)
        }) != 0))
    {
        assert!(false);
    }
    assert!(((side_effect) == (0)));
    if ((n != 0)
        || ((unsafe {
            let _v: i32 = 1;
            observe_0(_v)
        }) != 0))
    {
        assert!(true);
    }
    assert!(((side_effect) == (0)));
    let mut x: i32 = 5;
    let mut y: i32 = 3;
    let mut flags: u32 = 2_u32;
    if (((x) > (y)) || (((flags) & (1_u32)) != 0)) {
        assert!(true);
    }
    if (((x) < (y)) || (((flags) & (1_u32)) != 0)) {
        assert!(false);
    }
    let mut a: u32 = 1_u32;
    let mut b: u32 = 2_u32;
    let mut c: u32 = 3_u32;
    if (((a) != (c)) && ((b) != (c))) {
        assert!(true);
    }
    let mut s: i32 = -1_i32;
    if ((!((p).is_null())) && ((s) < (0))) {
        assert!(true);
    }
    let mut k: u32 = 2_u32;
    let mut done: bool = false;
    if (((k) > (1_u32)) || (!done)) {
        assert!(true);
    }
    if (((x) > (y)) || (((flags) & (4_u32)) != 0)) {
        assert!(true);
    }
    return 0;
}
