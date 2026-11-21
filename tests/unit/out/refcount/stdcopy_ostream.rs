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
    let str: Value<Vec<i8>> = Rc::new(RefCell::new({
        let mut __bytes = Ptr::<i8>::from_string_literal(b"Hello, world!\n").to_c_bytes();
        __bytes.push(0);
        __bytes
    }));
    let file: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(
        b"test_stdcopy_ostream\xc3\xa9.txt\0",
    )));
    {
        let ofs: Value<::std::fs::File> = Rc::new(RefCell::new(
            file.decay()
                .with_c_os_str(|__path| ::std::fs::File::create(__path))
                .expect("Failed to open file"),
        ));
        {
            let mut __tmp_0 = { RandomAccessIterator::Vec(str.decay()) };
            let mut __tmp_1 = { RandomAccessIterator::Vec(str.decay().to_last()) };
            let mut __it = (__tmp_0).clone();
            let __out = &mut (*ofs.borrow_mut());
            let mut __bytes: Vec<u8> = Vec::new();
            while __it != __tmp_1 {
                __bytes.push(__it.iter_deref().read() as u8);
                __it.iter_inc();
            }
            __out.write_all(&__bytes);
            __out.try_clone().unwrap()
        };
        write!(ofs.as_pointer(), "{:}", 42,);
    }
    let mut f: Ptr<CFile> = match CFile::open(
        &file.decay().to_c_u8_bytes(),
        &Ptr::<i8>::from_string_literal(b"rb").to_c_u8_bytes(),
    ) {
        Some(__f) => Ptr::alloc(__f),
        None => Ptr::null(),
    };
    let buf: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([0_i8; 32])));
    let mut n: usize = libcc2rs::fread_refcount(
        (buf.decay()).to_any(),
        1_usize,
        ::std::mem::size_of::<[i8; 32]>(),
        (f).clone(),
    );
    {
        let __r = f.with(|__f| __f.close());
        f.delete();
        __r
    };
    assert!((n == 16_usize));
    assert!(
        ({
            (((*buf.borrow())[(14) as usize] as i32) == (('4' as i8) as i32))
                && (((*buf.borrow())[(15) as usize] as i32) == (('2' as i8) as i32))
        })
    );
    match file
        .decay()
        .with_c_bytes(|__path| nix::unistd::unlink(__path))
    {
        Ok(()) => 0,
        Err(__e) => {
            libcc2rs::cpp2rust_errno().write(__e as i32);
            -1
        }
    };
    return 0;
}
pub fn __cpp2rust_init_globals() {}
