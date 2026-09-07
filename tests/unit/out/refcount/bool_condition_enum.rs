extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type Code = u32;
pub const Code_CODE_OK: Code = 0;
pub const Code_CODE_ERR: Code = 1;
pub const Code_CODE_FATAL: Code = 2;
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let code: Value<Code> = Rc::new(RefCell::new(Code_CODE_OK));
    let err: Value<Code> = Rc::new(RefCell::new(Code_CODE_ERR));
    if ((*code.borrow()) != 0) {
        assert!(false);
    }
    if !((*code.borrow()) != 0) {
        assert!(true);
    }
    if ((*err.borrow()) != 0) {
        assert!(true);
    }
    if !((*err.borrow()) != 0) {
        assert!(false);
    }
    let t9: Value<i32> = Rc::new(RefCell::new((!((*code.borrow()) != 0) as i32)));
    assert!(((*t9.borrow()) == 1));
    let b4: Value<bool> = Rc::new(RefCell::new(((*code.borrow()) != 0)));
    assert!(!(*b4.borrow()));
    return 0;
}
