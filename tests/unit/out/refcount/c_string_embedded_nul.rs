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
    let buf: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
        ('a' as i8),
        ('b' as i8),
        ('\0' as i8),
        ('c' as i8),
        ('d' as i8),
        ('\0' as i8),
    ])));
    match (buf.as_pointer() as Ptr<i8>).with_c_bytes(|__bytes| {
        libcc2rs::c_stdout().with_mut(|__f| __f.write(__bytes)) == __bytes.len()
    }) {
        true => 0,
        false => -1,
    };
    {
        let __c = (('|' as i8) as i32) as u8;
        match libcc2rs::c_stdout().with_mut(|__f| __f.write(&[__c])) {
            1 => __c as i32,
            _ => -1,
        }
    };
    {
        let mut __bytes = (buf.as_pointer() as Ptr<i8>).to_c_u8_bytes();
        __bytes.push(b'\n');
        match libcc2rs::c_stdout().with_mut(|__f| __f.write(&__bytes)) == __bytes.len() {
            true => 0,
            false => -1,
        }
    };
    let mut s: Vec<i8> = {
        let mut __bytes = (buf.as_pointer() as Ptr<i8>).to_c_bytes();
        __bytes.push(0);
        __bytes
    };
    println!("{}", (s.len() - 1));
    println!(
        "{} {}",
        (buf.as_pointer() as Ptr::<i8>)
            .to_c_string_iterator()
            .count(),
        (buf.as_pointer() as Ptr::<i8>).with_c_str(|__lookup| {
            s.iter()
                .take(s.len().saturating_sub(1))
                .rposition(|&x| __lookup.contains(&x))
                .unwrap_or(usize::MAX)
        })
    );
    let mut lit: Ptr<i8> = Ptr::<i8>::from_string_literal(b"xy\0zw");
    let mut t: Vec<i8> = {
        let mut __bytes = lit.to_c_bytes();
        __bytes.push(0);
        __bytes
    };
    println!("{}", (t.len() - 1));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
