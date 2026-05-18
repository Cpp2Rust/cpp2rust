extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn strlen_0(ptr: Ptr<u8>) -> u32 {
    let ptr: Value<Ptr<u8>> = Rc::new(RefCell::new(ptr));
    let count: Value<u32> = Rc::new(RefCell::new(0_u32));
    'loop_: while ((((*ptr.borrow_mut()).postfix_inc().read()) as i32) != (('\0' as u8) as i32)) {
        (*count.borrow_mut()).prefix_inc();
    }
    return (*count.borrow());
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let string: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([
        ('h' as u8),
        ('e' as u8),
        ('l' as u8),
        ('l' as u8),
        ('o' as u8),
        ('\0' as u8),
    ])));
    return (({
        let _ptr: Ptr<u8> = ((string.as_pointer() as Ptr<u8>).offset(0 as isize));
        strlen_0(_ptr)
    }) as i32);
}
