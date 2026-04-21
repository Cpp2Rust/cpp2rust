extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn extract_first_0(buf: Ptr<u8>, size: i32, fmt: Ptr<u8>, args: &[VaArg]) -> i32 {
    let buf: Value<Ptr<u8>> = Rc::new(RefCell::new(buf));
    let size: Value<i32> = Rc::new(RefCell::new(size));
    let fmt: Value<Ptr<u8>> = Rc::new(RefCell::new(fmt));
    let ap: Value<VaList> = Rc::new(RefCell::new(VaList::default()));
    (*ap.borrow_mut()) = VaList::new(args);
    let n: Value<i32> = Rc::new(RefCell::new(((*ap.borrow_mut()).arg::<i32>()).clone()));
    let __rhs = ((*n.borrow()) as u8);
    (*buf.borrow()).offset((0) as isize).write(__rhs);
    return (*n.borrow());
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let buf: Value<Box<[u8]>> = Rc::new(RefCell::new(
        (0..64).map(|_| <u8>::default()).collect::<Box<[u8]>>(),
    ));
    assert!(
        (({
            let _buf: Ptr<u8> = (buf.as_pointer() as Ptr<u8>);
            let _size: i32 = 1;
            let _fmt: Ptr<u8> = Ptr::from_string_literal("%d");
            extract_first_0(_buf, _size, _fmt, &[42.into()])
        }) == 42)
    );
    assert!((((*buf.borrow())[(0) as usize] as i32) == 42));
    assert!(
        (({
            let _buf: Ptr<u8> = (buf.as_pointer() as Ptr<u8>);
            let _size: i32 = 1;
            let _fmt: Ptr<u8> = Ptr::from_string_literal("%d");
            extract_first_0(_buf, _size, _fmt, &[65.into()])
        }) == 65)
    );
    assert!((((*buf.borrow())[(0) as usize] as i32) == ('A' as i32)));
    return 0;
}
