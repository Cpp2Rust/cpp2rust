extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::Seek;
use std::io::{Read, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn fn_0(v: Vec<u8>) -> Vec<u8> {
    let v: Value<Vec<u8>> = Rc::new(RefCell::new(v));
    return {
        let mut r = (*v.borrow()).clone();
        r.pop();
        r.extend(Ptr::from_string_literal(" str").to_c_string_iterator());
        r.push(0);
        r
    };
}
pub fn fn2_1(v: Ptr<Vec<u8>>) -> Ptr<Vec<u8>> {
    return (v).clone();
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    println!("{}", Ptr::from_string_literal("fprintf stdout"));
    println!("{} {} {}", 1, 2_u32, 3_i64);
    print!("hello world");
    println!("{}", Ptr::from_string_literal("printf"));
    print!("hello world");
    let s: Value<Vec<u8>> = Rc::new(RefCell::new(
        Ptr::from_string_literal("a string")
            .to_c_string_iterator()
            .chain(std::iter::once(0))
            .collect::<Vec<u8>>(),
    ));
    println!("{}", (s.as_pointer() as Ptr<u8>));
    println!(
        "{}",
        (Rc::new(RefCell::new(
            ({
                let _v: Vec<u8> = Ptr::from_string_literal("foo")
                    .to_c_string_iterator()
                    .chain(std::iter::once(0))
                    .collect::<Vec<u8>>();
                fn_0(_v)
            })
        ))
        .as_pointer() as Ptr<u8>)
    );
    println!(
        "{}",
        (({
            let _v: Ptr<Vec<u8>> = s.as_pointer();
            fn2_1(_v)
        })
        .to_strong()
        .as_pointer() as Ptr<u8>)
    );
    return 0;
}
