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
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let mut sibling: Ptr<i8> = Ptr::<i8>::from_string_literal(b"unlink-\xef\xbf\xbd.tmp");
    let mut fp: Ptr<CFile> = match CFile::open(
        &sibling.to_rust_string(),
        &Ptr::<i8>::from_string_literal(b"wb").to_rust_string(),
    ) {
        Some(__f) => Ptr::alloc(__f),
        None => Ptr::null(),
    };
    assert!((!((fp).is_null())));
    assert!(
        (match Ptr::<i8>::from_string_literal(b"sentinel")
            .with_c_bytes(|__bytes| fp.with_mut(|__f| __f.write(__bytes)) == __bytes.len())
        {
            true => 0,
            false => -1,
        } >= 0)
    );
    assert!(
        ({
            let __r = fp.with(|__f| __f.close());
            fp.delete();
            __r
        } == 0)
    );
    libcc2rs::cpp2rust_errno().write(0);
    assert!(
        (match Ptr::<i8>::from_string_literal(b"unlink-\xff.tmp")
            .with_c_bytes(|__path| nix::unistd::unlink(__path))
        {
            Ok(()) => 0,
            Err(__e) => {
                libcc2rs::cpp2rust_errno().write(__e as i32);
                -1
            }
        } == -1_i32)
    );
    assert!(((libcc2rs::cpp2rust_errno().read()) == libc::ENOENT));
    fp = match CFile::open(
        &sibling.to_rust_string(),
        &Ptr::<i8>::from_string_literal(b"rb").to_rust_string(),
    ) {
        Some(__f) => Ptr::alloc(__f),
        None => Ptr::null(),
    };
    assert!((!((fp).is_null())));
    assert!(
        ({
            let __r = fp.with(|__f| __f.close());
            fp.delete();
            __r
        } == 0)
    );
    assert!(
        (match sibling.with_c_bytes(|__path| nix::unistd::unlink(__path)) {
            Ok(()) => 0,
            Err(__e) => {
                libcc2rs::cpp2rust_errno().write(__e as i32);
                -1
            }
        } == 0)
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
