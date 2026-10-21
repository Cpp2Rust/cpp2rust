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
    let mut n: u32 = 17_u32;
    let d: Value<u32> = Rc::new(RefCell::new(n));
    assert!(((*d.borrow()) == 17_u32));
    let mut name: Ptr<i8> = Ptr::<i8>::from_string_literal(b"dir/file.txt");
    let p: Value<Vec<i8>> = Rc::new(RefCell::new({
        let mut __v = Vec::new();
        name.with_c_str(|__s| __v.extend_from_slice(__s));
        __v.push(0);
        __v
    }));
    let mut s: Vec<i8> = (*p.borrow()).clone();
    assert!(((s.len() - 1) == 12_usize));
    assert!(
        Ptr::<i8>::from_string_literal(b"dir/file.txt")
            .with_c_str(|__s| s[..s.len().saturating_sub(1)] == *__s)
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
