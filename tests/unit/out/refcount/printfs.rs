extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn fn_0(v: Vec<::core::ffi::c_char>) -> Vec<::core::ffi::c_char> {
    let v: Value<Vec<::core::ffi::c_char>> = Rc::new(RefCell::new(v));
    return {
        let mut r = (*v.borrow()).clone();
        r.pop();
        r.extend(Ptr::from_string_literal(b" str").to_c_string_iterator());
        r.push(0);
        r
    };
}
pub fn fn2_1(v: Ptr<Vec<::core::ffi::c_char>>) -> Ptr<Vec<::core::ffi::c_char>> {
    return (v).clone();
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    println!("{}", Ptr::from_string_literal(b"fprintf stdout"));
    println!("{} {} {}", 1, 2_u32, 3_i64);
    print!("hello world");
    let in_: Value<Ptr<::std::fs::File>> = Rc::new(RefCell::new((libcc2rs::cin()).clone()));
    assert!(!((*in_.borrow()).is_null()));
    println!("{}", Ptr::from_string_literal(b"printf"));
    print!("hello world");
    let s: Value<Vec<::core::ffi::c_char>> = Rc::new(RefCell::new(
        Ptr::from_string_literal(b"a string")
            .to_c_string_iterator()
            .chain(std::iter::once(0))
            .collect::<Vec<::core::ffi::c_char>>(),
    ));
    println!("{}", (s.as_pointer() as Ptr<::core::ffi::c_char>));
    println!(
        "{}",
        (Rc::new(RefCell::new(
            ({
                let _v: Vec<::core::ffi::c_char> = Ptr::from_string_literal(b"foo")
                    .to_c_string_iterator()
                    .chain(std::iter::once(0))
                    .collect::<Vec<::core::ffi::c_char>>();
                fn_0(_v)
            })
        ))
        .as_pointer() as Ptr<::core::ffi::c_char>)
    );
    println!(
        "{}",
        (({
            let _v: Ptr<Vec<::core::ffi::c_char>> = s.as_pointer();
            fn2_1(_v)
        })
        .to_strong()
        .as_pointer() as Ptr<::core::ffi::c_char>)
    );
    return 0;
}
