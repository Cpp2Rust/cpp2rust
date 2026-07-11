extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn bump_0(arg: AnyPtr) -> i32 {
    let arg: Value<AnyPtr> = Rc::new(RefCell::new(arg));
    let value: Value<Ptr<i32>> = Rc::new(RefCell::new((*arg.borrow()).reinterpret_cast::<i32>()));
    {
        let _ptr = (*value.borrow()).clone();
        _ptr.write(_ptr.read() + 1)
    };
    return ((*value.borrow()).read());
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let value: Value<i32> = Rc::new(RefCell::new(41));
    let opaque: Value<AnyPtr> = Rc::new(RefCell::new(((value.as_pointer()) as Ptr<i32>).to_any()));
    let typed: Value<Ptr<i32>> =
        Rc::new(RefCell::new((*opaque.borrow()).reinterpret_cast::<i32>()));
    assert!((((({ bump_0((*opaque.borrow()).clone(),) }) == 42) as i32) != 0));
    assert!((((((*typed.borrow()).read()) == 42) as i32) != 0));
    (*typed.borrow()).write(7);
    assert!(((((*value.borrow()) == 7) as i32) != 0));
    return 0;
}
