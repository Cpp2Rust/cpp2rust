extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_stat_0() {
    let path: Value<Ptr<u8>> = Rc::new(RefCell::new(Ptr::from_string_literal(
        b"/tmp/cpp2rust_stat_test.tmp",
    )));
    let fp: Value<Ptr<CFile>> = Rc::new(RefCell::new(
        match CFile::open(
            &(*path.borrow()).to_rust_string(),
            &Ptr::from_string_literal(b"wb").to_rust_string(),
        ) {
            Some(__f) => Ptr::alloc(__f),
            None => Ptr::null(),
        },
    ));
    assert!((((!((*fp.borrow()).is_null())) as i32) != 0));
    {
        let __bytes: Vec<u8> = Ptr::from_string_literal(b"hello")
            .to_c_string_iterator()
            .collect();
        match (*fp.borrow()).with_mut(|__f| __f.write(&__bytes)) == __bytes.len() {
            true => 0,
            false => -1,
        }
    };
    assert!(
        ((({
            let __r = (*fp.borrow()).with(|__f| __f.close());
            (*fp.borrow()).delete();
            __r
        } == 0) as i32)
            != 0)
    );
    let st: Value<libcc2rs::Stat> = Rc::new(RefCell::new(Default::default()));
    assert!(
        (((match nix::sys::stat::stat((*path.borrow()).to_rust_string().as_str()) {
            Ok(__s) => {
                (st.as_pointer()).with_mut(|__st| *__st = Stat::from_libc(&__s));
                0
            }
            Err(__e) => {
                libcc2rs::cpp2rust_errno().write(__e as i32);
                -1
            }
        } == 0) as i32)
            != 0)
    );
    assert!(((((*(*st.borrow()).st_size.borrow()) == 5_i64) as i32) != 0));
    assert!(((((*(*st.borrow()).st_mtime.borrow()) > 0_i64) as i32) != 0));
    match nix::unistd::unlink((*path.borrow()).to_rust_string().as_str()) {
        Ok(()) => 0,
        Err(__e) => {
            libcc2rs::cpp2rust_errno().write(__e as i32);
            -1
        }
    };
}
pub fn test_fstat_1() {
    let path: Value<Ptr<u8>> = Rc::new(RefCell::new(Ptr::from_string_literal(
        b"/tmp/cpp2rust_fstat_test.tmp",
    )));
    let fp: Value<Ptr<CFile>> = Rc::new(RefCell::new(
        match CFile::open(
            &(*path.borrow()).to_rust_string(),
            &Ptr::from_string_literal(b"wb").to_rust_string(),
        ) {
            Some(__f) => Ptr::alloc(__f),
            None => Ptr::null(),
        },
    ));
    assert!((((!((*fp.borrow()).is_null())) as i32) != 0));
    {
        let __bytes: Vec<u8> = Ptr::from_string_literal(b"hello world")
            .to_c_string_iterator()
            .collect();
        match (*fp.borrow()).with_mut(|__f| __f.write(&__bytes)) == __bytes.len() {
            true => 0,
            false => -1,
        }
    };
    0;
    let fd: Value<i32> = Rc::new(RefCell::new((*fp.borrow()).with(|__f| __f.fd)));
    let st: Value<libcc2rs::Stat> = Rc::new(RefCell::new(Default::default()));
    assert!((((libc::fstat((*fd.borrow()), (st.as_pointer())) == 0) as i32) != 0));
    assert!(((((*(*st.borrow()).st_size.borrow()) == 11_i64) as i32) != 0));
    assert!(((((*(*st.borrow()).st_mtime.borrow()) > 0_i64) as i32) != 0));
    assert!(
        ((({
            let __r = (*fp.borrow()).with(|__f| __f.close());
            (*fp.borrow()).delete();
            __r
        } == 0) as i32)
            != 0)
    );
    match nix::unistd::unlink((*path.borrow()).to_rust_string().as_str()) {
        Ok(()) => 0,
        Err(__e) => {
            libcc2rs::cpp2rust_errno().write(__e as i32);
            -1
        }
    };
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    ({ test_stat_0() });
    ({ test_fstat_1() });
    return 0;
}
