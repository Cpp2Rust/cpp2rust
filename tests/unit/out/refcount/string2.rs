extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let arr: Value<Vec<i8>> = Rc::new(RefCell::new({
        let mut __bytes = Ptr::<i8>::from_string_literal(b"foo").to_c_bytes();
        __bytes.push(0);
        __bytes
    }));
    (*arr.borrow_mut())[1_usize] = ('b' as i8);
    let mut p: Ptr<i8> = (arr.as_pointer() as Ptr<i8>).offset((1) as isize);
    assert!((((p.read()) as i32) == (('b' as i8) as i32)));
    assert!(
        Ptr::<i8>::from_string_literal(b"fbo")
            .with_c_str(|__s| (*arr.borrow())[..(*arr.borrow()).len().saturating_sub(1)] == *__s)
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
