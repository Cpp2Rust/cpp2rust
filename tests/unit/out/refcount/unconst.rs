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
    let a: Value<i32> = Rc::new(RefCell::new(1));
    let p: Value<Ptr<i32>> = Rc::new(RefCell::new((a.as_pointer())));
    let q: Value<Ptr<i32>> = Rc::new(RefCell::new(
        (<AnyPtr>::from_int(((*p.borrow()).clone() as Ptr<i32>).to_any().to_int()))
            .reinterpret_cast::<i32>(),
    ));
    assert!({
        let _lhs = (*p.borrow()).clone();
        _lhs == (*q.borrow()).clone()
    });
    return 0;
}
