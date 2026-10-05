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
    let mut pw: Ptr<libcc2rs::Passwd> = match nix::unistd::User::from_uid(
        nix::unistd::Uid::from_raw(nix::unistd::geteuid().as_raw()),
    ) {
        Ok(Some(__u)) => Ptr::alloc(Passwd::from_user(&__u)),
        Ok(None) => Ptr::null(),
        Err(__e) => {
            libcc2rs::cpp2rust_errno().write(__e as i32);
            Ptr::null()
        }
    };
    if !(!(pw).is_null()) {
        return 0;
    }
    let mut home: Ptr<i8> = pw.with(|__s| __s.pw_dir.clone());
    let mut d: Ptr<libcc2rs::Dirent> = match nix::dir::Dir::open(
        Ptr::<i8>::from_string_literal(b"/tmp")
            .to_rust_string()
            .as_str(),
        nix::fcntl::OFlag::O_RDONLY,
        nix::sys::stat::Mode::empty(),
    ) {
        Ok(__dir) => Ptr::alloc(CDir::from_dir(__dir)),
        Err(__e) => {
            libcc2rs::cpp2rust_errno().write(__e as i32);
            Ptr::null()
        }
    }
    .with(|__d| {
        let __i = __d.pos.get();
        if __i >= __d.entries.len() {
            Ptr::null()
        } else {
            __d.pos.set(__i + 1);
            let __e = &__d.entries[__i];
            Ptr::alloc(Dirent::from_entry(__e.0, &__e.1, __e.2))
        }
    });
    let mut dname: Ptr<i8> = (array_field_ptr!(d, d_name) as Ptr<i8>);
    return 0;
}
pub fn __cpp2rust_init_globals() {}
