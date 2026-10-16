extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_stat_0() {
    let mut path: Ptr<i8> = Ptr::<i8>::from_string_literal(b"cpp2rust_stat_test.tmp");
    let mut fp: Ptr<CFile> = match CFile::open(
        &path.to_rust_string(),
        &Ptr::<i8>::from_string_literal(b"wb").to_rust_string(),
    ) {
        Some(__f) => Ptr::alloc(__f),
        None => Ptr::null(),
    };
    assert!((((!((fp).is_null())) as i32) != 0));
    match Ptr::<i8>::from_string_literal(b"hello")
        .with_c_bytes(|__bytes| fp.with_mut(|__f| __f.write(__bytes)) == __bytes.len())
    {
        true => 0,
        false => -1,
    };
    assert!(
        ((({
            let __r = fp.with(|__f| __f.close());
            fp.delete();
            __r
        } == 0) as i32)
            != 0)
    );
    let st: Value<libcc2rs::Stat> = Rc::new(RefCell::new(Default::default()));
    assert!(
        (((match nix::sys::stat::stat(path.to_rust_string().as_str()) {
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
    assert!(((({ (*st.borrow()).st_size } == 5_i64) as i32) != 0));
    assert!(
        ((((field_ptr!((st.as_pointer() as Ptr<Stat>), st_mtime).read()) > 0_i64) as i32) != 0)
    );
    match nix::unistd::unlink(path.to_rust_string().as_str()) {
        Ok(()) => 0,
        Err(__e) => {
            libcc2rs::cpp2rust_errno().write(__e as i32);
            -1
        }
    };
}
pub fn test_fstat_1() {
    let mut path: Ptr<i8> = Ptr::<i8>::from_string_literal(b"cpp2rust_fstat_test.tmp");
    let mut fp: Ptr<CFile> = match CFile::open(
        &path.to_rust_string(),
        &Ptr::<i8>::from_string_literal(b"wb").to_rust_string(),
    ) {
        Some(__f) => Ptr::alloc(__f),
        None => Ptr::null(),
    };
    assert!((((!((fp).is_null())) as i32) != 0));
    match Ptr::<i8>::from_string_literal(b"hello world")
        .with_c_bytes(|__bytes| fp.with_mut(|__f| __f.write(__bytes)) == __bytes.len())
    {
        true => 0,
        false => -1,
    };
    0;
    let mut fd: i32 = fp.with(|__f| __f.fd);
    let st: Value<libcc2rs::Stat> = Rc::new(RefCell::new(Default::default()));
    assert!(
        (((match FdRegistry::with_fd(fd, |__fd| nix::sys::stat::fstat(__fd)) {
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
    assert!(((({ (*st.borrow()).st_size } == 11_i64) as i32) != 0));
    assert!(
        ((((field_ptr!((st.as_pointer() as Ptr<Stat>), st_mtime).read()) > 0_i64) as i32) != 0)
    );
    assert!(
        ((({
            let __r = fp.with(|__f| __f.close());
            fp.delete();
            __r
        } == 0) as i32)
            != 0)
    );
    match nix::unistd::unlink(path.to_rust_string().as_str()) {
        Ok(()) => 0,
        Err(__e) => {
            libcc2rs::cpp2rust_errno().write(__e as i32);
            -1
        }
    };
}
pub fn test_st_mtime_access_2() {
    let st: Value<libcc2rs::Stat> = Rc::new(RefCell::new(Default::default()));
    let mut sp: Ptr<libcc2rs::Stat> = (st.as_pointer());
    field_ptr!((st.as_pointer() as Ptr<Stat>), st_mtime).write(5_i64);
    assert!(
        ((((field_ptr!((st.as_pointer() as Ptr<Stat>), st_mtime).read()) == 5_i64) as i32) != 0)
    );
    field_ptr!(((sp).clone() as Ptr<Stat>), st_mtime).write(6_i64);
    assert!(((((field_ptr!(((sp).clone() as Ptr<Stat>), st_mtime).read()) == 6_i64) as i32) != 0));
    {
        field_ptr!((st.as_pointer() as Ptr<Stat>), st_mtime).with_mut(|__v| *__v = *__v + 2_i64)
    };
    assert!(
        ((((field_ptr!((st.as_pointer() as Ptr<Stat>), st_mtime).read()) == 8_i64) as i32) != 0)
    );
    {
        field_ptr!(((sp).clone() as Ptr<Stat>), st_mtime).with_mut(|__v| *__v = *__v - 1_i64)
    };
    assert!(((((field_ptr!(((sp).clone() as Ptr<Stat>), st_mtime).read()) == 7_i64) as i32) != 0));
    field_ptr!((st.as_pointer() as Ptr<Stat>), st_mtime).with_mut(|__v| __v.postfix_inc());
    field_ptr!(((sp).clone() as Ptr<Stat>), st_mtime).with_mut(|__v| __v.prefix_inc());
    assert!(
        ((((field_ptr!((st.as_pointer() as Ptr<Stat>), st_mtime).read()) == 9_i64) as i32) != 0)
    );
    let mut mtime: Ptr<i64> = (field_ptr!((st.as_pointer() as Ptr<Stat>), st_mtime));
    mtime.write(10_i64);
    assert!(
        ((((field_ptr!((st.as_pointer() as Ptr<Stat>), st_mtime).read()) == 10_i64) as i32) != 0)
    );
    let mut mtime_via_ptr: Ptr<i64> = (field_ptr!(((sp).clone() as Ptr<Stat>), st_mtime));
    mtime_via_ptr.write(11_i64);
    assert!(((((mtime.read()) == 11_i64) as i32) != 0));
    let mut copy: i64 = (field_ptr!(((sp).clone() as Ptr<Stat>), st_mtime).read());
    assert!((((copy == 11_i64) as i32) != 0));
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    ({ test_stat_0() });
    ({ test_fstat_1() });
    ({ test_st_mtime_access_2() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
