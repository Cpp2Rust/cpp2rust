extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_getpwuid_0() {
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
    assert!((((!((*pw.borrow()).is_null())) as i32) != 0));
    assert!(
        ((({
            let _lhs = (*(*(*pw.borrow()).upgrade().deref()).pw_uid.borrow());
            _lhs == nix::unistd::geteuid().as_raw()
        }) as i32)
            != 0)
    );
    assert!(
        ((((*(*(*pw.borrow()).upgrade().deref()).pw_name.borrow())
            .to_c_string_iterator()
            .count()
            > 0_usize) as i32)
            != 0)
    );
    assert!((((!((*(*(*pw.borrow()).upgrade().deref()).pw_dir.borrow()).is_null())) as i32) != 0));
    println!(
        "{}",
        (*(*(*pw.borrow()).upgrade().deref()).pw_name.borrow())
    );
}
pub fn test_getpwuid_missing_1() {
    libcc2rs::cpp2rust_errno().write(0);
    let pw: Value<Ptr<libcc2rs::Passwd>> = Rc::new(RefCell::new(
        match nix::unistd::User::from_uid(nix::unistd::Uid::from_raw(2147483646_u32)) {
            Ok(Some(__u)) => Ptr::alloc(Passwd::from_user(&__u)),
            Ok(None) => Ptr::null(),
            Err(__e) => {
                libcc2rs::cpp2rust_errno().write(__e as i32);
                Ptr::null()
            }
        },
    ));
    assert!(((((*pw.borrow()).is_null()) as i32) != 0));
    assert!(((((libcc2rs::cpp2rust_errno().read()) == 0) as i32) != 0));
}
pub fn test_getpwuid_r_2() {
    let pw: Value<libcc2rs::Passwd> = Rc::new(RefCell::new(Default::default()));
    let buf: Value<Box<[u8]>> = Rc::new(RefCell::new(
        (0..4096).map(|_| <u8>::default()).collect::<Box<[u8]>>(),
    ));
    let result: Value<Ptr<libcc2rs::Passwd>> =
        Rc::new(RefCell::new(Ptr::<libcc2rs::Passwd>::null()));
    assert!(
        ((({
            let __pwbuf = (pw.as_pointer()).clone();
            let __buf = (buf.as_pointer() as Ptr<u8>).clone();
            let __buflen = ::std::mem::size_of::<[u8; 4096]>();
            let __out = (result.as_pointer()).clone();
            match nix::unistd::User::from_uid(nix::unistd::Uid::from_raw(
                nix::unistd::geteuid().as_raw(),
            )) {
                Ok(Some(__u)) => {
                    let __strs: [Vec<u8>; 5] = [
                        __u.name.as_bytes().to_vec(),
                        __u.passwd.as_bytes().to_vec(),
                        __u.gecos.as_bytes().to_vec(),
                        __u.dir.as_os_str().as_encoded_bytes().to_vec(),
                        __u.shell.as_os_str().as_encoded_bytes().to_vec(),
                    ];
                    let __needed: usize = __strs.iter().map(|__s| __s.len() + 1).sum();
                    if __needed > __buflen {
                        __out.write(Ptr::null());
                        libcc2rs::cpp2rust_errno().write(::libc::ERANGE);
                        ::libc::ERANGE
                    } else {
                        let mut __ptrs: Vec<Ptr<u8>> = Vec::new();
                        let mut __off: usize = 0;
                        for __s in &__strs {
                            __ptrs.push(__buf.offset(__off));
                            let mut __dst = __buf.offset(__off);
                            for __b in __s.iter() {
                                __dst.write(*__b);
                                __dst += 1;
                            }
                            __dst.write(0);
                            __off += __s.len() + 1;
                        }
                        __pwbuf.with_mut(|__pw| *__pw = Passwd::from_user_in(&__u, &__ptrs));
                        __out.write(__pwbuf.clone());
                        0
                    }
                }
                Ok(None) => {
                    __out.write(Ptr::null());
                    0
                }
                Err(__e) => {
                    __out.write(Ptr::null());
                    libcc2rs::cpp2rust_errno().write(__e as i32);
                    __e as i32
                }
            }
        } == 0) as i32)
            != 0)
    );
    assert!(
        ((({
            let _lhs = (*result.borrow()).clone();
            _lhs == (pw.as_pointer())
        }) as i32)
            != 0)
    );
    assert!(((((*(*pw.borrow()).pw_uid.borrow()) == nix::unistd::geteuid().as_raw()) as i32) != 0));
    assert!(
        ((((*(*pw.borrow()).pw_name.borrow())
            .to_c_string_iterator()
            .count()
            > 0_usize) as i32)
            != 0)
    );
    let pw2: Value<Ptr<libcc2rs::Passwd>> = Rc::new(RefCell::new(
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
    assert!((((!((*pw2.borrow()).is_null())) as i32) != 0));
    assert!(
        ((({
            let mut __it1 = (*(*pw.borrow()).pw_name.borrow()).to_c_string_iterator();
            let mut __it2 =
                (*(*(*pw2.borrow()).upgrade().deref()).pw_name.borrow()).to_c_string_iterator();
            loop {
                let __c1 = __it1.next();
                let __c2 = __it2.next();
                if __c1 != __c2 {
                    break (__c1.unwrap_or(0) as i32) - (__c2.unwrap_or(0) as i32);
                }
                if __c1.is_none() {
                    break 0;
                }
            }
        } == 0) as i32)
            != 0)
    );
    println!("{}", (*(*pw.borrow()).pw_name.borrow()));
}
pub fn test_getpwuid_r_erange_3() {
    let pw: Value<libcc2rs::Passwd> = Rc::new(RefCell::new(Default::default()));
    let tiny: Value<Box<[u8]>> = Rc::new(RefCell::new(
        (0..1).map(|_| <u8>::default()).collect::<Box<[u8]>>(),
    ));
    let result: Value<Ptr<libcc2rs::Passwd>> =
        Rc::new(RefCell::new(Ptr::<libcc2rs::Passwd>::null()));
    assert!(
        ((({
            let __pwbuf = (pw.as_pointer()).clone();
            let __buf = (tiny.as_pointer() as Ptr<u8>).clone();
            let __buflen = ::std::mem::size_of::<[u8; 1]>();
            let __out = (result.as_pointer()).clone();
            match nix::unistd::User::from_uid(nix::unistd::Uid::from_raw(
                nix::unistd::geteuid().as_raw(),
            )) {
                Ok(Some(__u)) => {
                    let __strs: [Vec<u8>; 5] = [
                        __u.name.as_bytes().to_vec(),
                        __u.passwd.as_bytes().to_vec(),
                        __u.gecos.as_bytes().to_vec(),
                        __u.dir.as_os_str().as_encoded_bytes().to_vec(),
                        __u.shell.as_os_str().as_encoded_bytes().to_vec(),
                    ];
                    let __needed: usize = __strs.iter().map(|__s| __s.len() + 1).sum();
                    if __needed > __buflen {
                        __out.write(Ptr::null());
                        libcc2rs::cpp2rust_errno().write(::libc::ERANGE);
                        ::libc::ERANGE
                    } else {
                        let mut __ptrs: Vec<Ptr<u8>> = Vec::new();
                        let mut __off: usize = 0;
                        for __s in &__strs {
                            __ptrs.push(__buf.offset(__off));
                            let mut __dst = __buf.offset(__off);
                            for __b in __s.iter() {
                                __dst.write(*__b);
                                __dst += 1;
                            }
                            __dst.write(0);
                            __off += __s.len() + 1;
                        }
                        __pwbuf.with_mut(|__pw| *__pw = Passwd::from_user_in(&__u, &__ptrs));
                        __out.write(__pwbuf.clone());
                        0
                    }
                }
                Ok(None) => {
                    __out.write(Ptr::null());
                    0
                }
                Err(__e) => {
                    __out.write(Ptr::null());
                    libcc2rs::cpp2rust_errno().write(__e as i32);
                    __e as i32
                }
            }
        } == 34) as i32)
            != 0)
    );
    assert!(((((*result.borrow()).is_null()) as i32) != 0));
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    ({ test_getpwuid_0() });
    ({ test_getpwuid_missing_1() });
    ({ test_getpwuid_r_2() });
    ({ test_getpwuid_r_erange_3() });
    return 0;
}
