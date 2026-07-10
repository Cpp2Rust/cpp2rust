extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn foo_mut_0(str: Ptr<u8>) {
    let str: Value<Ptr<u8>> = Rc::new(RefCell::new(str));
}
pub fn foo_const_1(str: Ptr<u8>) {
    let str: Value<Ptr<u8>> = Rc::new(RefCell::new(str));
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let immutable_strings: Value<Box<[Ptr<u8>]>> = Rc::new(RefCell::new(Box::new([
        Ptr::from_string_literal(b"a"),
        Ptr::from_string_literal(b"b"),
        Ptr::from_string_literal(b"c"),
    ])));
    let immutable_string: Value<Ptr<u8>> =
        Rc::new(RefCell::new(Ptr::from_string_literal(b"hello")));
    let mutable_string_arr: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::from(*b"papanasi\0")));
    let immutable_string_arr: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::from(*b"papanasi\0")));
    let immutable_empty: Value<Ptr<u8>> = Rc::new(RefCell::new(Ptr::from_string_literal(b"")));
    let mutable_empty_arr: Value<Box<[u8]>> =
        Rc::new(RefCell::new(vec![0u8; 1].into_boxed_slice()));
    let immutable_empty_arr: Value<Box<[u8]>> =
        Rc::new(RefCell::new(vec![0u8; 1].into_boxed_slice()));
    ({ foo_mut_0((mutable_string_arr.as_pointer() as Ptr<u8>)) });
    ({ foo_const_1(Ptr::from_string_literal(b"world")) });
    ({ foo_const_1((*immutable_string.borrow()).clone()) });
    ({ foo_const_1((immutable_string_arr.as_pointer() as Ptr<u8>)) });
    ({ foo_const_1(Ptr::from_string_literal(b"")) });
    ({ foo_const_1((*immutable_empty.borrow()).clone()) });
    ({ foo_const_1((immutable_empty_arr.as_pointer() as Ptr<u8>)) });
    let inited_through_init_list: Value<Box<[u8]>> =
        Rc::new(RefCell::new(Box::from(*b"papanasi cu smantana\0")));
    ({ foo_const_1((inited_through_init_list.as_pointer() as Ptr<u8>)) });
    return 0;
}
