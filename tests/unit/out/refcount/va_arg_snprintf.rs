extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn extract_first_0(buf: Ptr<i8>, size: i32, fmt: Ptr<i8>, __args: &[VaArg]) -> i32 {
    let buf: Value<Ptr<i8>> = Rc::new(RefCell::new(buf));
    let size: Value<i32> = Rc::new(RefCell::new(size));
    let fmt: Value<Ptr<i8>> = Rc::new(RefCell::new(fmt));
    let ap: Value<VaList> = Rc::new(RefCell::new(VaList::default()));
    (*ap.borrow_mut()) = VaList::new(__args);
    let n: Value<i32> = Rc::new(RefCell::new((*ap.borrow_mut()).arg::<i32>()));
    elem!((*buf.borrow()), 0).write({ ((*n.borrow()) as i8) });
    return (*n.borrow());
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let buf: Value<Box<[i8]>> = Rc::new(RefCell::new((0..64).map(|_| 0_i8).collect::<Box<[i8]>>()));
    assert!(
        (((({
            extract_first_0(
                (buf.as_pointer() as Ptr<i8>),
                1,
                Ptr::<i8>::from_string_literal(b"%d"),
                &[(42).into()],
            )
        }) == 42) as i32)
            != 0)
    );
    assert!((((((*buf.borrow())[(0) as usize] as i32) == 42) as i32) != 0));
    assert!(
        (((({
            extract_first_0(
                (buf.as_pointer() as Ptr<i8>),
                1,
                Ptr::<i8>::from_string_literal(b"%d"),
                &[(65).into()],
            )
        }) == 65) as i32)
            != 0)
    );
    assert!((((((*buf.borrow())[(0) as usize] as i32) == ('A' as i32)) as i32) != 0));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
