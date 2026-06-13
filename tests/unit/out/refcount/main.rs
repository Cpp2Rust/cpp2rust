extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};

pub fn main() {
    let argv: Vec<Value<Vec<::core::ffi::c_char>>> = ::std::env::args()
        .map(|x| {
            Rc::new(RefCell::new(
                x.bytes().map(|b| b as ::core::ffi::c_char).collect(),
            ))
        })
        .collect();
    let mut argv: Value<Vec<Ptr<::core::ffi::c_char>>> = Rc::new(RefCell::new(
        argv.iter()
            .map(|x| {
                x.borrow_mut().push(0);
                x.as_pointer()
            })
            .collect(),
    ));
    (*argv.borrow_mut()).push(Ptr::null());
    ::std::process::exit(main_0(::std::env::args().len() as i32, argv.as_pointer()));
}
fn main_0(argc: i32, argv: Ptr<Ptr<::core::ffi::c_char>>) -> i32 {
    let argc: Value<i32> = Rc::new(RefCell::new(argc));
    let argv: Value<Ptr<Ptr<::core::ffi::c_char>>> = Rc::new(RefCell::new(argv));
    let s: Value<Vec<::core::ffi::c_char>> = Rc::new(RefCell::new(
        ((*argv.borrow()).offset((0) as isize).read())
            .to_c_string_iterator()
            .chain(std::iter::once(0))
            .collect::<Vec<::core::ffi::c_char>>(),
    ));
    assert!(((*argc.borrow()) == 1));
    assert!((((*s.borrow()).len() - 1) > 0_usize));
    return ((*argc.borrow()) + ((((*s.borrow()).len() - 1) > 0_usize) as i32));
}
