extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn extract_first_0(
    buf: Ptr<core::ffi::c_char>,
    size: i32,
    fmt: Ptr<core::ffi::c_char>,
    __args: &[VaArg],
) -> i32 {
    let buf: Value<Ptr<core::ffi::c_char>> = Rc::new(RefCell::new(buf));
    let size: Value<i32> = Rc::new(RefCell::new(size));
    let fmt: Value<Ptr<core::ffi::c_char>> = Rc::new(RefCell::new(fmt));
    let ap: Value<VaList> = Rc::new(RefCell::new(VaList::default()));
    (*ap.borrow_mut()) = VaList::new(__args);
    let n: Value<i32> = Rc::new(RefCell::new(((*ap.borrow_mut()).arg::<i32>()).clone()));
    let __rhs = ((*n.borrow()) as core::ffi::c_char);
    (*buf.borrow()).offset((0) as isize).write(__rhs);
    return (*n.borrow());
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let buf: Value<Box<[core::ffi::c_char]>> = Rc::new(RefCell::new(
        (0..64)
            .map(|_| <core::ffi::c_char>::default())
            .collect::<Box<[core::ffi::c_char]>>(),
    ));
    assert!(
        (((({
            let _buf: Ptr<core::ffi::c_char> = (buf.as_pointer() as Ptr<core::ffi::c_char>);
            extract_first_0(_buf, 1, Ptr::from_string_literal(b"%d"), &[(42).into()])
        }) == 42) as i32)
            != 0)
    );
    assert!((((((*buf.borrow())[(0) as usize] as i32) == 42) as i32) != 0));
    assert!(
        (((({
            let _buf: Ptr<core::ffi::c_char> = (buf.as_pointer() as Ptr<core::ffi::c_char>);
            extract_first_0(_buf, 1, Ptr::from_string_literal(b"%d"), &[(65).into()])
        }) == 65) as i32)
            != 0)
    );
    assert!((((((*buf.borrow())[(0) as usize] as i32) == ('A' as i32)) as i32) != 0));
    return 0;
}
