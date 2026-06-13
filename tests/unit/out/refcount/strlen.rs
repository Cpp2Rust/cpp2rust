extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn strlen_0(ptr: Ptr<::core::ffi::c_char>) -> u32 {
    let ptr: Value<Ptr<::core::ffi::c_char>> = Rc::new(RefCell::new(ptr));
    let count: Value<u32> = Rc::new(RefCell::new(0_u32));
    'loop_: while ((((*ptr.borrow_mut()).postfix_inc().read()) as i32)
        != (('\0' as ::core::ffi::c_char) as i32))
    {
        (*count.borrow_mut()).prefix_inc();
    }
    return (*count.borrow());
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let string: Value<Box<[::core::ffi::c_char]>> = Rc::new(RefCell::new(Box::new([
        ('h' as ::core::ffi::c_char),
        ('e' as ::core::ffi::c_char),
        ('l' as ::core::ffi::c_char),
        ('l' as ::core::ffi::c_char),
        ('o' as ::core::ffi::c_char),
        ('\0' as ::core::ffi::c_char),
    ])));
    return (({
        let _ptr: Ptr<::core::ffi::c_char> =
            ((string.as_pointer() as Ptr<::core::ffi::c_char>).offset(0 as isize));
        strlen_0(_ptr)
    }) as i32);
}
