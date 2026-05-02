extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
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
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let code: Value<Code> = Rc::new(RefCell::new(Code::from((Code::CODE_OK as i32))));
    let err: Value<Code> = Rc::new(RefCell::new(Code::from((Code::CODE_ERR as i32))));
    if ((*code.borrow()) != Code::from(0)) {
        assert!((0 != 0));
    }
    if !((*code.borrow()) != Code::from(0)) {
        assert!((1 != 0));
    }
    if ((*err.borrow()) != Code::from(0)) {
        assert!((1 != 0));
    }
    if !((*err.borrow()) != Code::from(0)) {
        assert!((0 != 0));
    }
    let t9: Value<i32> = Rc::new(RefCell::new((!((*code.borrow()) != Code::from(0)) as i32)));
    assert!(((*t9.borrow()) == 1));
    let b4: Value<bool> = Rc::new(RefCell::new(((*code.borrow()) != Code::from(0)).clone()));
    assert!(((!(*b4.borrow()) as i32) != 0));
    return 0;
}
