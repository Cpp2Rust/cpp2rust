extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let fp: Value<Ptr<CFile>> = Rc::new(RefCell::new((libcc2rs::c_stdout()).clone()));
    let p: Value<AnyPtr> = Rc::new(RefCell::new((*fp.borrow()).clone().to_any()));
    let fp2: Value<Ptr<CFile>> = Rc::new(RefCell::new((*p.borrow()).reinterpret_cast::<CFile>()));
    assert!(
        ((({
            let _lhs = (*fp.borrow()).clone();
            _lhs == (*fp2.borrow()).clone()
        }) as i32)
            != 0)
    );
    return 0;
}
