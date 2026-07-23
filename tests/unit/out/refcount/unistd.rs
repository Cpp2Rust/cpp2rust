extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_close_0() {
    let fds: Value<Box<[i32]>> = Rc::new(RefCell::new(
        (0..2).map(|_| <i32>::default()).collect::<Box<[i32]>>(),
    ));
    assert!(
        (((match nix::unistd::pipe() {
            Ok((__r, __w)) => {
                let __fds = (fds.as_pointer() as Ptr<i32>).clone();
                __fds.write(FdRegistry::register(__r));
                __fds.offset(1).write(FdRegistry::register(__w));
                0
            }
            Err(__e) => {
                libcc2rs::cpp2rust_errno().write(__e as i32);
                -1
            }
        } == 0) as i32)
            != 0)
    );
    assert!((((FdRegistry::close((*fds.borrow())[(0) as usize]) == 0) as i32) != 0));
    let buf: Value<Box<[u8]>> = Rc::new(RefCell::new(
        (0..1).map(|_| <u8>::default()).collect::<Box<[u8]>>(),
    ));
    assert!(
        (((match FdRegistry::with_fd((*fds.borrow())[(0) as usize], |__fd| {
            ((buf.as_pointer() as Ptr<u8>) as Ptr<u8>)
                .to_any()
                .reinterpret_cast::<u8>()
                .with_slice_mut(1_usize, |__buf| nix::unistd::read(__fd, __buf))
        }) {
            Ok(__n) => __n as isize,
            Err(__e) => {
                libcc2rs::cpp2rust_errno().write(__e as i32);
                -1
            }
        } == (-1_i32 as isize)) as i32)
            != 0)
    );
    assert!((((FdRegistry::close((*fds.borrow())[(1) as usize]) == 0) as i32) != 0));
}
pub fn test_lseek_1() {
    let path: Value<Ptr<u8>> = Rc::new(RefCell::new(Ptr::from_string_literal(
        b"/tmp/cpp2rust_lseek_test.tmp",
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
    assert!(
        ((({
            let __r = (*fp.borrow()).with(|__f| __f.close());
            (*fp.borrow()).delete();
            __r
        } == 0) as i32)
            != 0)
    );
    (*fp.borrow_mut()) = match CFile::open(
        &(*path.borrow()).to_rust_string(),
        &Ptr::from_string_literal(b"rb").to_rust_string(),
    ) {
        Some(__f) => Ptr::alloc(__f),
        None => Ptr::null(),
    };
    assert!((((!((*fp.borrow()).is_null())) as i32) != 0));
    let fd: Value<i32> = Rc::new(RefCell::new((*fp.borrow()).with(|__f| __f.fd)));
    assert!(
        ((({
            let __whence = match 2 {
                0 => nix::unistd::Whence::SeekSet,
                1 => nix::unistd::Whence::SeekCur,
                2 => nix::unistd::Whence::SeekEnd,
                __w => panic!("lseek: unsupported whence {__w}"),
            };
            match FdRegistry::with_fd((*fd.borrow()), |__fd| {
                nix::unistd::lseek(__fd, 0_i64, __whence)
            }) {
                Ok(__off) => __off,
                Err(__e) => {
                    libcc2rs::cpp2rust_errno().write(__e as i32);
                    -1
                }
            }
        } == 11_i64) as i32)
            != 0)
    );
    assert!(
        ((({
            let __whence = match 0 {
                0 => nix::unistd::Whence::SeekSet,
                1 => nix::unistd::Whence::SeekCur,
                2 => nix::unistd::Whence::SeekEnd,
                __w => panic!("lseek: unsupported whence {__w}"),
            };
            match FdRegistry::with_fd((*fd.borrow()), |__fd| {
                nix::unistd::lseek(__fd, 6_i64, __whence)
            }) {
                Ok(__off) => __off,
                Err(__e) => {
                    libcc2rs::cpp2rust_errno().write(__e as i32);
                    -1
                }
            }
        } == 6_i64) as i32)
            != 0)
    );
    let buf: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([
        0_u8,
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
    ])));
    assert!(
        (((match FdRegistry::with_fd((*fd.borrow()), |__fd| {
            ((buf.as_pointer() as Ptr<u8>) as Ptr<u8>)
                .to_any()
                .reinterpret_cast::<u8>()
                .with_slice_mut(5_usize, |__buf| nix::unistd::read(__fd, __buf))
        }) {
            Ok(__n) => __n as isize,
            Err(__e) => {
                libcc2rs::cpp2rust_errno().write(__e as i32);
                -1
            }
        } == 5_isize) as i32)
            != 0)
    );
    assert!(
        (((((buf.as_pointer() as Ptr::<u8>) as Ptr::<u8>)
            .to_any()
            .memcmp(&Ptr::from_string_literal(b"world").to_any(), 5_usize)
            == 0) as i32)
            != 0)
    );
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
pub fn test_read_2() {
    let path: Value<Ptr<u8>> = Rc::new(RefCell::new(Ptr::from_string_literal(
        b"/tmp/cpp2rust_read_test.tmp",
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
    assert!(
        ((({
            let __r = (*fp.borrow()).with(|__f| __f.close());
            (*fp.borrow()).delete();
            __r
        } == 0) as i32)
            != 0)
    );
    (*fp.borrow_mut()) = match CFile::open(
        &(*path.borrow()).to_rust_string(),
        &Ptr::from_string_literal(b"rb").to_rust_string(),
    ) {
        Some(__f) => Ptr::alloc(__f),
        None => Ptr::null(),
    };
    assert!((((!((*fp.borrow()).is_null())) as i32) != 0));
    let fd: Value<i32> = Rc::new(RefCell::new((*fp.borrow()).with(|__f| __f.fd)));
    let buf: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([
        0_u8,
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
    ])));
    assert!(
        (((match FdRegistry::with_fd((*fd.borrow()), |__fd| {
            ((buf.as_pointer() as Ptr<u8>) as Ptr<u8>)
                .to_any()
                .reinterpret_cast::<u8>()
                .with_slice_mut(16_usize, |__buf| nix::unistd::read(__fd, __buf))
        }) {
            Ok(__n) => __n as isize,
            Err(__e) => {
                libcc2rs::cpp2rust_errno().write(__e as i32);
                -1
            }
        } == 11_isize) as i32)
            != 0)
    );
    assert!(
        (((((buf.as_pointer() as Ptr::<u8>) as Ptr::<u8>)
            .to_any()
            .memcmp(&Ptr::from_string_literal(b"hello world").to_any(), 11_usize)
            == 0) as i32)
            != 0)
    );
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
pub fn test_unlink_3() {
    let path: Value<Ptr<u8>> = Rc::new(RefCell::new(Ptr::from_string_literal(
        b"/tmp/cpp2rust_unlink_test.tmp",
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
    assert!(
        ((({
            let __r = (*fp.borrow()).with(|__f| __f.close());
            (*fp.borrow()).delete();
            __r
        } == 0) as i32)
            != 0)
    );
    assert!(
        (((match nix::unistd::unlink((*path.borrow()).to_rust_string().as_str()) {
            Ok(()) => 0,
            Err(__e) => {
                libcc2rs::cpp2rust_errno().write(__e as i32);
                -1
            }
        } == 0) as i32)
            != 0)
    );
    assert!(
        (((match nix::unistd::unlink((*path.borrow()).to_rust_string().as_str()) {
            Ok(()) => 0,
            Err(__e) => {
                libcc2rs::cpp2rust_errno().write(__e as i32);
                -1
            }
        } == -1_i32) as i32)
            != 0)
    );
}
pub fn test_pipe_4() {
    let fds: Value<Box<[i32]>> = Rc::new(RefCell::new(
        (0..2).map(|_| <i32>::default()).collect::<Box<[i32]>>(),
    ));
    assert!(
        (((match nix::unistd::pipe() {
            Ok((__r, __w)) => {
                let __fds = (fds.as_pointer() as Ptr<i32>).clone();
                __fds.write(FdRegistry::register(__r));
                __fds.offset(1).write(FdRegistry::register(__w));
                0
            }
            Err(__e) => {
                libcc2rs::cpp2rust_errno().write(__e as i32);
                -1
            }
        } == 0) as i32)
            != 0)
    );
    let msg: Value<Ptr<u8>> = Rc::new(RefCell::new(Ptr::from_string_literal(b"world")));
    assert!(
        (((match FdRegistry::with_fd((*fds.borrow())[(1) as usize], |__fd| {
            ((*msg.borrow()).clone() as Ptr<u8>)
                .to_any()
                .reinterpret_cast::<u8>()
                .with_slice(5_usize, |__buf| nix::unistd::write(__fd, __buf))
        }) {
            Ok(__n) => __n as isize,
            Err(__e) => {
                libcc2rs::cpp2rust_errno().write(__e as i32);
                -1
            }
        } == 5_isize) as i32)
            != 0)
    );
    let buf: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([
        0_u8,
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
    ])));
    assert!(
        (((match FdRegistry::with_fd((*fds.borrow())[(0) as usize], |__fd| {
            ((buf.as_pointer() as Ptr<u8>) as Ptr<u8>)
                .to_any()
                .reinterpret_cast::<u8>()
                .with_slice_mut(8_usize, |__buf| nix::unistd::read(__fd, __buf))
        }) {
            Ok(__n) => __n as isize,
            Err(__e) => {
                libcc2rs::cpp2rust_errno().write(__e as i32);
                -1
            }
        } == 5_isize) as i32)
            != 0)
    );
    assert!(
        (((((buf.as_pointer() as Ptr::<u8>) as Ptr::<u8>)
            .to_any()
            .memcmp(&((*msg.borrow()).clone() as Ptr::<u8>).to_any(), 5_usize)
            == 0) as i32)
            != 0)
    );
    assert!((((FdRegistry::close((*fds.borrow())[(1) as usize]) == 0) as i32) != 0));
    assert!(
        (((match FdRegistry::with_fd((*fds.borrow())[(0) as usize], |__fd| {
            ((buf.as_pointer() as Ptr<u8>) as Ptr<u8>)
                .to_any()
                .reinterpret_cast::<u8>()
                .with_slice_mut(8_usize, |__buf| nix::unistd::read(__fd, __buf))
        }) {
            Ok(__n) => __n as isize,
            Err(__e) => {
                libcc2rs::cpp2rust_errno().write(__e as i32);
                -1
            }
        } == 0_isize) as i32)
            != 0)
    );
    assert!((((FdRegistry::close((*fds.borrow())[(0) as usize]) == 0) as i32) != 0));
}
pub fn test_ftruncate_5() {
    let path: Value<Ptr<u8>> = Rc::new(RefCell::new(Ptr::from_string_literal(
        b"/tmp/cpp2rust_ftruncate_test.tmp",
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
    assert!(
        (((match FdRegistry::with_fd((*fd.borrow()), |__fd| nix::unistd::ftruncate(__fd, 5_i64)) {
            Ok(()) => 0,
            Err(__e) => {
                libcc2rs::cpp2rust_errno().write(__e as i32);
                -1
            }
        } == 0) as i32)
            != 0)
    );
    assert!(
        ((({
            let __r = (*fp.borrow()).with(|__f| __f.close());
            (*fp.borrow()).delete();
            __r
        } == 0) as i32)
            != 0)
    );
    (*fp.borrow_mut()) = match CFile::open(
        &(*path.borrow()).to_rust_string(),
        &Ptr::from_string_literal(b"rb").to_rust_string(),
    ) {
        Some(__f) => Ptr::alloc(__f),
        None => Ptr::null(),
    };
    assert!((((!((*fp.borrow()).is_null())) as i32) != 0));
    (*fd.borrow_mut()) = (*fp.borrow()).with(|__f| __f.fd);
    assert!(
        ((({
            let __whence = match 2 {
                0 => nix::unistd::Whence::SeekSet,
                1 => nix::unistd::Whence::SeekCur,
                2 => nix::unistd::Whence::SeekEnd,
                __w => panic!("lseek: unsupported whence {__w}"),
            };
            match FdRegistry::with_fd((*fd.borrow()), |__fd| {
                nix::unistd::lseek(__fd, 0_i64, __whence)
            }) {
                Ok(__off) => __off,
                Err(__e) => {
                    libcc2rs::cpp2rust_errno().write(__e as i32);
                    -1
                }
            }
        } == 5_i64) as i32)
            != 0)
    );
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
pub fn test_open_6() {
    let fd: Value<i32> = Rc::new(RefCell::new({
        let __mode = match &[(420).into()].first() {
            Some(__m) => nix::sys::stat::Mode::from_bits_truncate(i32::get(__m) as ::libc::mode_t),
            None => nix::sys::stat::Mode::empty(),
        };
        match nix::fcntl::open(
            Ptr::from_string_literal(b"/dev/null")
                .to_rust_string()
                .as_str(),
            nix::fcntl::OFlag::from_bits_retain(0),
            __mode,
        ) {
            Ok(__ofd) => FdRegistry::register(__ofd),
            Err(__e) => {
                libcc2rs::cpp2rust_errno().write(__e as i32);
                -1
            }
        }
    }));
    assert!(((((*fd.borrow()) >= -1_i32) as i32) != 0));
    if ((((*fd.borrow()) >= 0) as i32) != 0) {
        FdRegistry::close((*fd.borrow()));
    }
    (*fd.borrow_mut()) = {
        let __mode = match &[].first() {
            Some(__m) => nix::sys::stat::Mode::from_bits_truncate(i32::get(__m) as ::libc::mode_t),
            None => nix::sys::stat::Mode::empty(),
        };
        match nix::fcntl::open(
            Ptr::from_string_literal(b"/dev/null")
                .to_rust_string()
                .as_str(),
            nix::fcntl::OFlag::from_bits_retain(0),
            __mode,
        ) {
            Ok(__ofd) => FdRegistry::register(__ofd),
            Err(__e) => {
                libcc2rs::cpp2rust_errno().write(__e as i32);
                -1
            }
        }
    };
    assert!(((((*fd.borrow()) >= -1_i32) as i32) != 0));
    if ((((*fd.borrow()) >= 0) as i32) != 0) {
        FdRegistry::close((*fd.borrow()));
    }
}
pub fn test_fcntl_7() {
    assert!(
        (((panic!(
            "fcntl is not supported in the refcount model (fd={}, cmd={}, varargs={})",
            0,
            1,
            &[].len()
        ) >= -1_i32) as i32)
            != 0)
    );
    let duped: Value<i32> = Rc::new(RefCell::new(panic!(
        "fcntl is not supported in the refcount model (fd={}, cmd={}, varargs={})",
        0,
        0,
        &[(100).into(),].len()
    )));
    assert!(((((*duped.borrow()) >= -1_i32) as i32) != 0));
    if ((((*duped.borrow()) >= 0) as i32) != 0) {
        FdRegistry::close((*duped.borrow()));
    }
}
pub fn test_ioctl_8() {
    let arg: Value<i32> = Rc::new(RefCell::new(0));
    assert!(
        (((panic!(
            "ioctl is not supported in the refcount model (fd={}, request={}, varargs={})",
            0,
            0_u64,
            &[(arg.as_pointer()).into(),].len()
        ) >= -1_i32) as i32)
            != 0)
    );
}
pub fn test_isatty_9() {
    println!(
        "{}",
        match FdRegistry::with_fd(0, |__fd| nix::unistd::isatty(__fd)) {
            Ok(__tty) => __tty as i32,
            Err(__e) => {
                libcc2rs::cpp2rust_errno().write(__e as i32);
                0
            }
        }
    );
}
pub fn test_geteuid_10() {
    println!("{}", nix::unistd::geteuid().as_raw());
}
pub fn test_gethostname_11() {
    let name: Value<Box<[u8]>> = Rc::new(RefCell::new(
        (0..256).map(|_| <u8>::default()).collect::<Box<[u8]>>(),
    ));
    assert!(
        (((match nix::unistd::gethostname() {
            Ok(__name) => {
                let __bytes = __name.as_encoded_bytes();
                let __n = __bytes
                    .len()
                    .min(::std::mem::size_of::<[u8; 256]>().saturating_sub(1));
                if ::std::mem::size_of::<[u8; 256]>() > 0 {
                    (name.as_pointer() as Ptr<u8>).with_slice_mut(__n + 1, |__s| {
                        __s[..__n].copy_from_slice(&__bytes[..__n]);
                        __s[__n] = 0;
                    });
                }
                0
            }
            Err(__e) => {
                libcc2rs::cpp2rust_errno().write(__e as i32);
                -1
            }
        } == 0) as i32)
            != 0)
    );
    println!("{}", (name.as_pointer() as Ptr::<u8>));
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    ({ test_close_0() });
    ({ test_lseek_1() });
    ({ test_read_2() });
    ({ test_unlink_3() });
    ({ test_pipe_4() });
    ({ test_ftruncate_5() });
    ({ test_open_6() });
    ({ test_fcntl_7() });
    ({ test_ioctl_8() });
    ({ test_isatty_9() });
    ({ test_geteuid_10() });
    ({ test_gethostname_11() });
    return 0;
}
