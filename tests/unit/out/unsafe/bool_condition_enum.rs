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
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut code: Code = Code::CODE_OK;
    let mut err: Code = Code::CODE_ERR;
    if (code != Code::from(0)) {
        assert!(false);
    }
    if !(code != Code::from(0)) {
        assert!(true);
    }
    if (err != Code::from(0)) {
        assert!(true);
    }
    if !(err != Code::from(0)) {
        assert!(false);
    }
    let mut t9: i32 = (!(code != Code::from(0)) as i32);
    assert!(((t9) == (1)));
    let mut b4: bool = (code != Code::from(0));
    assert!(!b4);
    return 0;
}
