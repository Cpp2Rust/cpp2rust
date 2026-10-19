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
    let x: Value<Vec<i8>> = Rc::new(RefCell::new({
        let mut __bytes = Ptr::<i8>::from_string_literal(b"hello").to_c_bytes();
        __bytes.push(0);
        __bytes
    }));
    let mut __begin1: Ptr<i8> = (x.as_pointer() as Ptr<i8>);
    let mut __end1: Ptr<i8> = (x.as_pointer() as Ptr<i8>).to_last();
    'loop_: while __begin1 != __end1 {
        let c: Ptr<i8> = __begin1.iter_deref();
        c.with_mut(|__v| __v.prefix_inc());
        __begin1.iter_inc();
    }
    let mut __begin1: Ptr<i8> = (x.as_pointer() as Ptr<i8>);
    let mut __end1: Ptr<i8> = (x.as_pointer() as Ptr<i8>).to_last();
    'loop_: while __begin1 != __end1 {
        let c: Ptr<i8> = __begin1.iter_deref();
        println!("{}", ((c.read()) as i32) as u8 as char);
        __begin1.iter_inc();
    }
    let mut __begin1: Ptr<i8> = (x.as_pointer() as Ptr<i8>);
    let mut __end1: Ptr<i8> = (x.as_pointer() as Ptr<i8>).to_last();
    'loop_: while __begin1 != __end1 {
        let mut c: i8 = (__begin1.iter_deref().read());
        println!("{}", (c as i32) as u8 as char);
        __begin1.iter_inc();
    }
    let v: Value<Vec<Ptr<i32>>> = Rc::new(RefCell::new(Vec::new()));
    {
        let __a1 = Ptr::alloc(2);
        (*v.borrow_mut()).push(__a1)
    };
    {
        let __a1 = Ptr::alloc(3);
        (*v.borrow_mut()).push(__a1)
    };
    let __begin1: Value<RandomAccessIterator<Ptr<i32>>> = Rc::new(RefCell::new(
        RandomAccessIterator::Vec((v.as_pointer() as Ptr<Ptr<i32>>)),
    ));
    let __end1: Value<RandomAccessIterator<Ptr<i32>>> = Rc::new(RefCell::new(
        RandomAccessIterator::Vec((v.as_pointer() as Ptr<Ptr<i32>>).to_end()),
    ));
    'loop_: while (*__begin1.borrow()) != (*__end1.borrow()) {
        let mut p: Ptr<i32> = ((*__begin1.borrow()).iter_deref().read());
        println!("{}", (p.read()));
        (*__begin1.borrow_mut()).iter_inc();
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
