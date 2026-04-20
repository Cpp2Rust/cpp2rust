extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};

pub fn main() {
    let argv: Vec<Value<Vec<u8>>> = ::std::env::args()
        .map(|x| Rc::new(RefCell::new(x.as_bytes().to_vec())))
        .collect();
    let mut argv: Value<Vec<Ptr<u8>>> = Rc::new(RefCell::new(
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
fn main_0(argc: i32, argv: Ptr<Ptr<u8>>) -> i32 {
    let argc: Value<i32> = Rc::new(RefCell::new(argc));
    let argv: Value<Ptr<Ptr<u8>>> = Rc::new(RefCell::new(argv));
    let s: Value<Vec<u8>> = Rc::new(RefCell::new(
        ((*argv.borrow()).offset((0) as isize).read())
            .to_c_string_iterator()
            .chain(std::iter::once(0))
            .collect::<Vec<u8>>(),
    ));
    assert!(((*argc.borrow()) == 1));
    assert!((((*s.borrow()).len() - 1) as u64 > 0_u64));
    return ((*argc.borrow()) + (((((*s.borrow()).len() - 1) as u64 > 0_u64) as bool) as i32));
}
