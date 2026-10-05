extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn bump_0(mut arg: AnyPtr) -> i32 {
    let mut value: Ptr<i32> = arg.reinterpret_cast::<i32>();
    {
        value.with_mut(|__v| *__v = *__v + 1)
    };
    return (value.read());
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let value: Value<i32> = Rc::new(RefCell::new(41));
    let mut opaque: AnyPtr = ((value.as_pointer()) as Ptr<i32>).to_any();
    let mut typed: Ptr<i32> = opaque.reinterpret_cast::<i32>();
    assert!((((({ bump_0((opaque).clone(),) }) == 42) as i32) != 0));
    assert!(((((typed.read()) == 42) as i32) != 0));
    typed.write(7);
    assert!(((((*value.borrow()) == 7) as i32) != 0));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
