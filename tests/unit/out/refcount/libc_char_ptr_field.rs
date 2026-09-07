extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let pw: Value<Ptr<libcc2rs::Passwd>> = Rc::new(RefCell::new(
        match nix::unistd::User::from_uid(nix::unistd::Uid::from_raw(
            nix::unistd::geteuid().as_raw(),
        )) {
            Ok(Some(__u)) => Ptr::alloc(Passwd::from_user(&__u)),
            Ok(None) => Ptr::null(),
            Err(__e) => {
                libcc2rs::cpp2rust_errno().write(__e as i32);
                Ptr::null()
            }
        },
    ));
    if !!(*pw.borrow()).is_null() {
        return 0;
    }
    let home: Value<Ptr<u8>> = Rc::new(RefCell::new(
        (*(*(*pw.borrow()).upgrade().deref()).pw_dir.borrow()).clone(),
    ));
    let d: Value<Ptr<libcc2rs::Dirent>> = Rc::new(RefCell::new(
        match nix::dir::Dir::open(
            Ptr::from_string_literal(b"/tmp").to_rust_string().as_str(),
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
        }),
    ));
    let dname: Value<Ptr<u8>> = Rc::new(RefCell::new(
        ((*(*d.borrow()).upgrade().deref()).d_name.as_pointer() as Ptr<u8>),
    ));
    return 0;
}
