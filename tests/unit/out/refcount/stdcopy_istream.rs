extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static data_0: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([0_u8, 127_u8, 255_u8])));
);
pub fn check_1(s: Ptr<Vec<i8>>) {
    assert!(({ ({ ((*s.upgrade().deref()).len() - 1) } == { ::std::mem::size_of::<[u8; 3]>() }) }));
    let mut i: u32 = 0_u32;
    'loop_: while ((i as usize) < ::std::mem::size_of::<[u8; 3]>()) {
        assert!(
            ({ (((index!(s, (i as usize)).read()) as u8) as i32) } == {
                (({
                    let __idx0 = (i) as usize;
                    data_0.with(|rc| rc.borrow()[__idx0])
                }) as i32)
            })
        );
        i.prefix_inc();
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let file: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(
        b"test_stdcopy_istream.txt\0",
    )));
    let mut fd: i32 = {
        let __mode = match Box::<[VaArg]>::from([(384).into()]).first() {
            Some(__m) => nix::sys::stat::Mode::from_bits_truncate(i32::get(__m) as ::libc::mode_t),
            None => nix::sys::stat::Mode::empty(),
        };
        match nix::fcntl::open(
            (file.as_pointer() as Ptr<i8>).to_rust_string().as_str(),
            nix::fcntl::OFlag::from_bits_retain(
                ((::libc::O_WRONLY | ::libc::O_CREAT) | ::libc::O_TRUNC),
            ),
            __mode,
        ) {
            Ok(__ofd) => FdRegistry::register(__ofd),
            Err(__e) => {
                libcc2rs::cpp2rust_errno().write(__e as i32);
                -1
            }
        }
    };
    assert!((fd >= 0));
    assert!(
        ((match FdRegistry::with_fd(fd, |__fd| {
            ((data_0.with(|v| v.as_pointer()) as Ptr<u8>) as Ptr<u8>)
                .to_any()
                .reinterpret_cast::<u8>()
                .with_slice(::std::mem::size_of::<[u8; 3]>(), |__buf| {
                    nix::unistd::write(__fd, __buf)
                })
        }) {
            Ok(__n) => __n as isize,
            Err(__e) => {
                libcc2rs::cpp2rust_errno().write(__e as i32);
                -1
            }
        } as usize)
            == ::std::mem::size_of::<[u8; 3]>())
    );
    assert!((FdRegistry::close(fd) == 0));
    {
        let f: Value<libcc2rs::CppStream> = Rc::new(RefCell::new(
            ::std::fs::File::open((file.as_pointer() as Ptr<i8>).to_string())
                .map(libcc2rs::CppStream::from)
                .expect("Failed to open file"),
        ));
        let s: Value<Vec<i8>> = Rc::new(RefCell::new({
            use std::io::Read;
            let mut __bytes: Vec<u8> = Vec::new();
            (f.as_pointer())
                .with_mut(|__f| __f.read_to_end(&mut __bytes))
                .expect("couldn't read the file");
            __bytes.push(0);
            CChar::from_byte_vec(__bytes)
        }));
        ({ check_1(s.as_pointer()) });
    }
    {
        let f: Value<libcc2rs::CppStream> = Rc::new(RefCell::new(
            ::std::fs::File::open((file.as_pointer() as Ptr<i8>).to_string())
                .map(libcc2rs::CppStream::from)
                .expect("Failed to open file"),
        ));
        let mut begin: Ptr<libcc2rs::CppStream> = f.as_pointer();
        let mut copy: Ptr<libcc2rs::CppStream> = (begin).clone();
        let mut end: Ptr<libcc2rs::CppStream> = Ptr::null();
        let s: Value<Vec<i8>> = Rc::new(RefCell::new({
            use std::io::Read;
            let mut __bytes: Vec<u8> = Vec::new();
            (copy)
                .clone()
                .with_mut(|__f| __f.read_to_end(&mut __bytes))
                .expect("couldn't read the file");
            __bytes.push(0);
            CChar::from_byte_vec(__bytes)
        }));
        ({ check_1(s.as_pointer()) });
    }
    match nix::unistd::unlink((file.as_pointer() as Ptr<i8>).to_rust_string().as_str()) {
        Ok(()) => 0,
        Err(__e) => {
            libcc2rs::cpp2rust_errno().write(__e as i32);
            -1
        }
    };
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = data_0.with(|_| ());
}
