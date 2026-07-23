extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_fputc_0() {
    {
        let __c = ('H' as i32) as u8;
        match libcc2rs::c_stdout().with_mut(|__f| __f.write(&[__c])) {
            1 => __c as i32,
            _ => -1,
        }
    };
    {
        let __c = ('i' as i32) as u8;
        match libcc2rs::c_stdout().with_mut(|__f| __f.write(&[__c])) {
            1 => __c as i32,
            _ => -1,
        }
    };
    {
        let __c = ('\n' as i32) as u8;
        match libcc2rs::c_stdout().with_mut(|__f| __f.write(&[__c])) {
            1 => __c as i32,
            _ => -1,
        }
    };
}
pub fn test_fputs_1() {
    {
        let __bytes: Vec<u8> = Ptr::from_string_literal(b"hello")
            .to_c_string_iterator()
            .collect();
        match libcc2rs::c_stdout().with_mut(|__f| __f.write(&__bytes)) == __bytes.len() {
            true => 0,
            false => -1,
        }
    };
    {
        let __c = ('\n' as i32) as u8;
        match libcc2rs::c_stdout().with_mut(|__f| __f.write(&[__c])) {
            1 => __c as i32,
            _ => -1,
        }
    };
    let s: Value<Ptr<u8>> = Rc::new(RefCell::new(Ptr::from_string_literal(b"from variable")));
    {
        let __bytes: Vec<u8> = (*s.borrow()).to_c_string_iterator().collect();
        match libcc2rs::c_stdout().with_mut(|__f| __f.write(&__bytes)) == __bytes.len() {
            true => 0,
            false => -1,
        }
    };
    {
        let __c = ('\n' as i32) as u8;
        match libcc2rs::c_stdout().with_mut(|__f| __f.write(&[__c])) {
            1 => __c as i32,
            _ => -1,
        }
    };
    let buf: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([
        (('b' as i32) as u8),
        (('u' as i32) as u8),
        (('f' as i32) as u8),
        (('\0' as i32) as u8),
    ])));
    {
        let __bytes: Vec<u8> = (buf.as_pointer() as Ptr<u8>)
            .to_c_string_iterator()
            .collect();
        match libcc2rs::c_stdout().with_mut(|__f| __f.write(&__bytes)) == __bytes.len() {
            true => 0,
            false => -1,
        }
    };
    {
        let __c = ('\n' as i32) as u8;
        match libcc2rs::c_stdout().with_mut(|__f| __f.write(&[__c])) {
            1 => __c as i32,
            _ => -1,
        }
    };
}
pub fn test_puts_2() {
    {
        let mut __bytes: Vec<u8> = Ptr::from_string_literal(b"puts hello")
            .to_c_string_iterator()
            .collect();
        __bytes.push(b'\n');
        match libcc2rs::c_stdout().with_mut(|__f| __f.write(&__bytes)) == __bytes.len() {
            true => 0,
            false => -1,
        }
    };
    let s: Value<Ptr<u8>> = Rc::new(RefCell::new(Ptr::from_string_literal(b"puts variable")));
    {
        let mut __bytes: Vec<u8> = (*s.borrow()).to_c_string_iterator().collect();
        __bytes.push(b'\n');
        match libcc2rs::c_stdout().with_mut(|__f| __f.write(&__bytes)) == __bytes.len() {
            true => 0,
            false => -1,
        }
    };
}
pub fn test_fileno_3() {
    assert!((((libcc2rs::c_stdin().with(|__f| __f.fd) == 0) as i32) != 0));
    assert!((((libcc2rs::c_stdout().with(|__f| __f.fd) == 1) as i32) != 0));
    assert!((((libcc2rs::c_stderr().with(|__f| __f.fd) == 2) as i32) != 0));
    let file: Value<Ptr<u8>> = Rc::new(RefCell::new(Ptr::from_string_literal(
        b"/tmp/cpp2rust_fileno_test.tmp",
    )));
    let fp: Value<Ptr<CFile>> = Rc::new(RefCell::new(
        match CFile::open(
            &(*file.borrow()).to_rust_string(),
            &Ptr::from_string_literal(b"wb").to_rust_string(),
        ) {
            Some(__f) => Ptr::alloc(__f),
            None => Ptr::null(),
        },
    ));
    assert!((((!((*fp.borrow()).is_null())) as i32) != 0));
    assert!(((((*fp.borrow()).with(|__f| __f.fd) > 2) as i32) != 0));
    {
        let __r = (*fp.borrow()).with(|__f| __f.close());
        (*fp.borrow()).delete();
        __r
    };
    assert!(
        (((match nix::unistd::unlink((*file.borrow()).to_rust_string().as_str()) {
            Ok(()) => 0,
            Err(__e) => {
                libcc2rs::cpp2rust_errno().write(__e as i32);
                -1
            }
        } == 0) as i32)
            != 0)
    );
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    ({ test_fputc_0() });
    ({ test_fputs_1() });
    ({ test_puts_2() });
    ({ test_fileno_3() });
    return 0;
}
