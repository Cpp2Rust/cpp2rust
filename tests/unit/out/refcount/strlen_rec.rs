extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn strlen_0(s: Ptr<::core::ffi::c_char>, n: i32) -> i32 {
    let s: Value<Ptr<::core::ffi::c_char>> = Rc::new(RefCell::new(s));
    let n: Value<i32> = Rc::new(RefCell::new(n));
    return if (((*s.borrow()).read()) != 0) {
        ({
            let _s: Ptr<::core::ffi::c_char> = (*s.borrow()).offset((1) as isize);
            let _n: i32 = ((*n.borrow()) + 1);
            strlen_0(_s, _n)
        })
    } else {
        (*n.borrow())
    };
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let s: Value<Box<[::core::ffi::c_char]>> = Rc::new(RefCell::new(Box::new([
        ('s' as ::core::ffi::c_char),
        ('t' as ::core::ffi::c_char),
        ('r' as ::core::ffi::c_char),
        ('\0' as ::core::ffi::c_char),
    ])));
    return ({
        let _s: Ptr<::core::ffi::c_char> =
            ((s.as_pointer() as Ptr<::core::ffi::c_char>).offset(0 as isize));
        strlen_0(_s, 0)
    });
}
