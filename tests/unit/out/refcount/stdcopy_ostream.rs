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
    let str: Value<Vec<i8>> = Rc::new(RefCell::new({
        let mut __bytes = Ptr::<i8>::from_string_literal(b"Hello, world!\n").to_c_bytes();
        __bytes.push(0);
        __bytes
    }));
    let file: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(
        b"test_stdcopy_ostream.txt\0",
    )));
    {
        let ofs: Value<libcc2rs::CppStream> = Rc::new(RefCell::new(
            ::std::fs::File::create((file.as_pointer() as Ptr<i8>).to_string())
                .map(libcc2rs::CppStream::from)
                .expect("Failed to open file"),
        ));
        match ({ RandomAccessIterator::Vec(str.decay()) }, {
            RandomAccessIterator::Vec(str.decay().to_last())
        }) {
            (mut __tmp_0, mut __tmp_1) => {
                let mut __it = (__tmp_0).clone();
                let __out = &mut (*ofs.borrow_mut());
                let mut __bytes: Vec<u8> = Vec::new();
                while __it != __tmp_1 {
                    __bytes.push(__it.iter_deref().read() as u8);
                    __it.iter_inc();
                }
                __out.write_all(&__bytes);
                __out.try_clone().unwrap()
            }
        };
    }
    let mut fd: i32 = {
        let __mode = match Box::<[VaArg]>::from([]).first() {
            Some(__m) => nix::sys::stat::Mode::from_bits_truncate(i32::get(__m) as ::libc::mode_t),
            None => nix::sys::stat::Mode::empty(),
        };
        match nix::fcntl::open(
            (file.as_pointer() as Ptr<i8>).to_rust_string().as_str(),
            nix::fcntl::OFlag::from_bits_retain(::libc::O_RDONLY),
            __mode,
        ) {
            Ok(__ofd) => FdRegistry::register(__ofd),
            Err(__e) => {
                libcc2rs::cpp2rust_errno().write(__e as i32);
                -1
            }
        }
    };
    let buf: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
        0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8,
    ])));
    assert!(
        ((match ({ ((buf.as_pointer() as Ptr::<i8>) as Ptr<i8>).to_any() },) {
            (mut __tmp_2,) => {
                match FdRegistry::with_fd(fd, |__fd| {
                    __tmp_2
                        .reinterpret_cast::<u8>()
                        .with_slice_mut(::std::mem::size_of::<[i8; 15]>(), |__buf| {
                            nix::unistd::read(__fd, __buf)
                        })
                }) {
                    Ok(__n) => __n as isize,
                    Err(__e) => {
                        libcc2rs::cpp2rust_errno().write(__e as i32);
                        -1
                    }
                }
            }
        } as usize)
            == ((*str.borrow()).len() - 1))
    );
    assert!(
        (buf.as_pointer() as Ptr::<i8>)
            .with_c_str(|__s| (*str.borrow())[..(*str.borrow()).len().saturating_sub(1)] == *__s)
    );
    assert!((FdRegistry::close(fd) == 0));
    match nix::unistd::unlink((file.as_pointer() as Ptr<i8>).to_rust_string().as_str()) {
        Ok(()) => 0,
        Err(__e) => {
            libcc2rs::cpp2rust_errno().write(__e as i32);
            -1
        }
    };
    return 0;
}
pub fn __cpp2rust_init_globals() {}
