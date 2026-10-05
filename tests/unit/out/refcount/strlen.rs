extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn strlen_0(ptr: Ptr<i8>) -> u32 {
    let ptr: Value<Ptr<i8>> = Rc::new(RefCell::new(ptr));
    let count: Value<u32> = Rc::new(RefCell::new(0_u32));
    'loop_: while ((((*ptr.borrow_mut()).postfix_inc().read()) as i32) != (('\0' as i8) as i32)) {
        (*count.borrow_mut()).prefix_inc();
    }
    return (*count.borrow());
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let string: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
        ('h' as i8),
        ('e' as i8),
        ('l' as i8),
        ('l' as i8),
        ('o' as i8),
        ('\0' as i8),
    ])));
    assert!((({ strlen_0(((string.as_pointer() as Ptr<i8>).offset(0)),) }) == 5_u32));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
