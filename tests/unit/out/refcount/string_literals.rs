extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn foo_mut_0(str: Ptr<core::ffi::c_char>) {
    let str: Value<Ptr<core::ffi::c_char>> = Rc::new(RefCell::new(str));
}
pub fn foo_const_1(str: Ptr<core::ffi::c_char>) {
    let str: Value<Ptr<core::ffi::c_char>> = Rc::new(RefCell::new(str));
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let immutable_strings: Value<Box<[Ptr<core::ffi::c_char>]>> =
        Rc::new(RefCell::new(Box::new([
            Ptr::from_string_literal(b"a"),
            Ptr::from_string_literal(b"b"),
            Ptr::from_string_literal(b"c"),
        ])));
    let immutable_string: Value<Ptr<core::ffi::c_char>> =
        Rc::new(RefCell::new(Ptr::from_string_literal(b"hello")));
    let mutable_string_arr: Value<Box<[core::ffi::c_char]>> =
        Rc::new(RefCell::new(Box::from(libcc2rs::char_array(b"papanasi\0"))));
    let immutable_string_arr: Value<Box<[core::ffi::c_char]>> =
        Rc::new(RefCell::new(Box::from(libcc2rs::char_array(b"papanasi\0"))));
    let immutable_empty: Value<Ptr<core::ffi::c_char>> =
        Rc::new(RefCell::new(Ptr::from_string_literal(b"")));
    let mutable_empty_arr: Value<Box<[core::ffi::c_char]>> = Rc::new(RefCell::new(
        vec![0 as core::ffi::c_char; 1].into_boxed_slice(),
    ));
    let immutable_empty_arr: Value<Box<[core::ffi::c_char]>> = Rc::new(RefCell::new(
        vec![0 as core::ffi::c_char; 1].into_boxed_slice(),
    ));
    ({
        let _str: Ptr<core::ffi::c_char> =
            (mutable_string_arr.as_pointer() as Ptr<core::ffi::c_char>);
        foo_mut_0(_str)
    });
    ({ foo_const_1(Ptr::from_string_literal(b"world")) });
    ({
        let _str: Ptr<core::ffi::c_char> = (*immutable_string.borrow()).clone();
        foo_const_1(_str)
    });
    ({
        let _str: Ptr<core::ffi::c_char> =
            (immutable_string_arr.as_pointer() as Ptr<core::ffi::c_char>);
        foo_const_1(_str)
    });
    ({ foo_const_1(Ptr::from_string_literal(b"")) });
    ({
        let _str: Ptr<core::ffi::c_char> = (*immutable_empty.borrow()).clone();
        foo_const_1(_str)
    });
    ({
        let _str: Ptr<core::ffi::c_char> =
            (immutable_empty_arr.as_pointer() as Ptr<core::ffi::c_char>);
        foo_const_1(_str)
    });
    let inited_through_init_list: Value<Box<[core::ffi::c_char]>> = Rc::new(RefCell::new(
        Box::from(libcc2rs::char_array(b"papanasi cu smantana\0")),
    ));
    ({
        let _str: Ptr<core::ffi::c_char> =
            (inited_through_init_list.as_pointer() as Ptr<core::ffi::c_char>);
        foo_const_1(_str)
    });
    return 0;
}
