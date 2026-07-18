extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_errno_0() {
    libcc2rs::cpp2rust_errno().write(0);
    assert!(((((libcc2rs::cpp2rust_errno().read()) == 0) as i32) != 0));
    libcc2rs::cpp2rust_errno().write(42);
    assert!(((((libcc2rs::cpp2rust_errno().read()) == 42) as i32) != 0));
    let saved: Value<i32> = Rc::new(RefCell::new((libcc2rs::cpp2rust_errno().read())));
    assert!(((((*saved.borrow()) == 42) as i32) != 0));
    libcc2rs::cpp2rust_errno().write(0);
}
pub fn test_errno_preserved_across_strdup_1() {
    libcc2rs::cpp2rust_errno().write(99);
    let d: Value<Ptr<u8>> = Rc::new(RefCell::new(libcc2rs::strdup_refcount(
        Ptr::from_string_literal(b"hello").clone(),
    )));
    assert!((((!((*d.borrow()).is_null())) as i32) != 0));
    assert!(((((libcc2rs::cpp2rust_errno().read()) == 99) as i32) != 0));
    libcc2rs::free_refcount(((*d.borrow()).clone() as Ptr<u8>).to_any());
    libcc2rs::cpp2rust_errno().write(0);
}
pub fn test_errno_from_fseek_2() {
    libcc2rs::cpp2rust_errno().write(0);
    let r: Value<i32> = Rc::new(RefCell::new(
        if (match 0 {
            0 => libcc2rs::cin().with_mut(|__v: &mut ::std::fs::File| {
                __v.seek(std::io::SeekFrom::Start(0_i64 as u64))
            }),
            1 => libcc2rs::cin()
                .with_mut(|__v: &mut ::std::fs::File| __v.seek(std::io::SeekFrom::Current(0_i64))),
            2 => libcc2rs::cin()
                .with_mut(|__v: &mut ::std::fs::File| __v.seek(std::io::SeekFrom::End(0_i64))),
            _ => Err(std::io::Error::other("unsupported whence for fseek.")),
        })
        .is_ok()
        {
            0
        } else {
            -1
        },
    ));
    assert!(((((*r.borrow()) == -1_i32) as i32) != 0));
    assert!(((((libcc2rs::cpp2rust_errno().read()) == libc::ESPIPE) as i32) != 0));
    libcc2rs::cpp2rust_errno().write(0);
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    ({ test_errno_0() });
    ({ test_errno_preserved_across_strdup_1() });
    ({ test_errno_from_fseek_2() });
    return 0;
}
