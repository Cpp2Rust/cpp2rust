extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn foo_mut_0(mut str: Ptr<i8>) {}
pub fn foo_const_1(mut str: Ptr<i8>) {}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut mutable_strings: [Ptr<i8>; 3] = [
        Ptr::<i8>::from_string_literal(b"a"),
        Ptr::<i8>::from_string_literal(b"b"),
        Ptr::<i8>::from_string_literal(b"c"),
    ];
    let mut immutable_strings: [Ptr<i8>; 3] = [
        Ptr::<i8>::from_string_literal(b"a"),
        Ptr::<i8>::from_string_literal(b"b"),
        Ptr::<i8>::from_string_literal(b"c"),
    ];
    let mut mutable_string: Ptr<i8> = Ptr::<i8>::from_string_literal(b"hello");
    let mut immutable_string: Ptr<i8> = Ptr::<i8>::from_string_literal(b"hello");
    let mutable_string_arr: Value<Box<[i8]>> =
        Rc::new(RefCell::new(i8::array_from_literal(b"papanasi\0")));
    let immutable_string_arr: Value<Box<[i8]>> =
        Rc::new(RefCell::new(i8::array_from_literal(b"papanasi\0")));
    let mut mutable_empty: Ptr<i8> = Ptr::<i8>::from_string_literal(b"");
    let mut immutable_empty: Ptr<i8> = Ptr::<i8>::from_string_literal(b"");
    let mutable_empty_arr: Value<Box<[i8]>> =
        Rc::new(RefCell::new(vec![0i8; 1].into_boxed_slice()));
    let immutable_empty_arr: Value<Box<[i8]>> =
        Rc::new(RefCell::new(vec![0i8; 1].into_boxed_slice()));
    ({ foo_mut_0(Ptr::<i8>::from_string_literal(b"world")) });
    ({ foo_mut_0((mutable_string).clone()) });
    ({ foo_mut_0((mutable_string_arr.as_pointer() as Ptr<i8>)) });
    ({ foo_const_1(Ptr::<i8>::from_string_literal(b"world")) });
    ({ foo_const_1((mutable_string).clone()) });
    ({ foo_const_1((immutable_string).clone()) });
    ({ foo_const_1((mutable_string_arr.as_pointer() as Ptr<i8>)) });
    ({ foo_const_1((immutable_string_arr.as_pointer() as Ptr<i8>)) });
    ({ foo_const_1(Ptr::<i8>::from_string_literal(b"")) });
    ({ foo_const_1((mutable_empty).clone()) });
    ({ foo_const_1((immutable_empty).clone()) });
    ({ foo_const_1((mutable_empty_arr.as_pointer() as Ptr<i8>)) });
    ({ foo_const_1((immutable_empty_arr.as_pointer() as Ptr<i8>)) });
    let inited_through_init_list: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(
        b"papanasi cu smantana\0",
    )));
    ({ foo_const_1((inited_through_init_list.as_pointer() as Ptr<i8>)) });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
