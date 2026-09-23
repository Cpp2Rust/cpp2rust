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
    let buf: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([
        ('a' as u8),
        ('b' as u8),
        ('\0' as u8),
        ('c' as u8),
        ('d' as u8),
        ('\0' as u8),
    ])));
    {
        let __bytes = (buf.as_pointer() as Ptr<u8>).to_c_bytes();
        match libcc2rs::c_stdout().with_mut(|__f| __f.write(&__bytes)) == __bytes.len() {
            true => 0,
            false => -1,
        }
    };
    {
        let __c = (('|' as u8) as i32) as u8;
        match libcc2rs::c_stdout().with_mut(|__f| __f.write(&[__c])) {
            1 => __c as i32,
            _ => -1,
        }
    };
    {
        let mut __bytes = (buf.as_pointer() as Ptr<u8>).to_c_bytes();
        __bytes.push(b'\n');
        match libcc2rs::c_stdout().with_mut(|__f| __f.write(&__bytes)) == __bytes.len() {
            true => 0,
            false => -1,
        }
    };
    let s: Value<Vec<u8>> = Rc::new(RefCell::new({
        let mut __bytes = (buf.as_pointer() as Ptr<u8>).to_c_bytes();
        __bytes.push(0);
        __bytes
    }));
    println!("{}", ((*s.borrow()).len() - 1));
    println!(
        "{} {}",
        (buf.as_pointer() as Ptr::<u8>)
            .to_c_string_iterator()
            .count(),
        (buf.as_pointer() as Ptr::<u8>).with_c_str(|__lookup| {
            (*s.borrow())
                .iter()
                .take((*s.borrow()).len().saturating_sub(1))
                .rposition(|&x| __lookup.contains(&x))
                .unwrap_or(usize::MAX)
        })
    );
    let lit: Value<Ptr<u8>> = Rc::new(RefCell::new(Ptr::<u8>::from_string_literal(b"xy\0zw")));
    let t: Value<Vec<u8>> = Rc::new(RefCell::new({
        let mut __bytes = (*lit.borrow()).to_c_bytes();
        __bytes.push(0);
        __bytes
    }));
    println!("{}", ((*t.borrow()).len() - 1));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
