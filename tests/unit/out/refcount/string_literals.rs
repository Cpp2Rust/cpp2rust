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
    let mutable_strings: Value<Box<[Ptr<u8>]>> = Rc::new(RefCell::new(Box::new([
        Ptr::from_string_literal("a"),
        Ptr::from_string_literal("b"),
        Ptr::from_string_literal("c"),
    ])));
    let immutable_strings: Value<Box<[Ptr<u8>]>> = Rc::new(RefCell::new(Box::new([
        Ptr::from_string_literal("a"),
        Ptr::from_string_literal("b"),
        Ptr::from_string_literal("c"),
    ])));
    let mutable_string: Value<Ptr<u8>> = Rc::new(RefCell::new(Ptr::from_string_literal("hello")));
    let immutable_string: Value<Ptr<u8>> = Rc::new(RefCell::new(Ptr::from_string_literal("hello")));
    let mutable_string_arr: Value<Box<[u8]>> =
        Rc::new(RefCell::new(Box::<[u8]>::from(b"papanasi\0".as_slice())));
    let immutable_string_arr: Value<Box<[u8]>> =
        Rc::new(RefCell::new(Box::<[u8]>::from(b"papanasi\0".as_slice())));
    let mutable_empty: Value<Ptr<u8>> = Rc::new(RefCell::new(Ptr::from_string_literal("")));
    let immutable_empty: Value<Ptr<u8>> = Rc::new(RefCell::new(Ptr::from_string_literal("")));
    let mutable_empty_arr: Value<Box<[u8]>> =
        Rc::new(RefCell::new(Box::<[u8]>::from(b"\0".as_slice())));
    let immutable_empty_arr: Value<Box<[u8]>> =
        Rc::new(RefCell::new(Box::<[u8]>::from(b"\0".as_slice())));
    ({
        let _str: Ptr<u8> = Ptr::from_string_literal("world");
        foo_mut_0(_str)
    });
    ({
        let _str: Ptr<u8> = (*mutable_string.borrow()).clone();
        foo_mut_0(_str)
    });
    ({
        let _str: Ptr<u8> = (mutable_string_arr.as_pointer() as Ptr<u8>);
        foo_mut_0(_str)
    });
    ({
        let _str: Ptr<u8> = Ptr::from_string_literal("world");
        foo_const_1(_str)
    });
    ({
        let _str: Ptr<u8> = (*mutable_string.borrow()).clone();
        foo_const_1(_str)
    });
    ({
        let _str: Ptr<u8> = (*immutable_string.borrow()).clone();
        foo_const_1(_str)
    });
    ({
        let _str: Ptr<u8> = (mutable_string_arr.as_pointer() as Ptr<u8>);
        foo_const_1(_str)
    });
    ({
        let _str: Ptr<u8> = (immutable_string_arr.as_pointer() as Ptr<u8>);
        foo_const_1(_str)
    });
    ({
        let _str: Ptr<u8> = Ptr::from_string_literal("");
        foo_const_1(_str)
    });
    ({
        let _str: Ptr<u8> = (*mutable_empty.borrow()).clone();
        foo_const_1(_str)
    });
    ({
        let _str: Ptr<u8> = (*immutable_empty.borrow()).clone();
        foo_const_1(_str)
    });
    ({
        let _str: Ptr<u8> = (mutable_empty_arr.as_pointer() as Ptr<u8>);
        foo_const_1(_str)
    });
    ({
        let _str: Ptr<u8> = (immutable_empty_arr.as_pointer() as Ptr<u8>);
        foo_const_1(_str)
    });
    return 0;
}
