extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn fn_0(v: Vec<u8>) -> Vec<u8> {
    let v: Value<Vec<u8>> = Rc::new(RefCell::new(v));
    return {
        let mut r = (*v.borrow()).clone();
        r.pop();
        r.extend(Ptr::from_string_literal(b" str").to_c_string_iterator());
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
    println!("{}", Ptr::from_string_literal(b"fprintf stdout"));
    println!("{} {} {}", 1, 2_u32, 3_i64);
    print!("hello world");
    let in_: Value<Ptr<::std::fs::File>> = Rc::new(RefCell::new((libcc2rs::cin()).clone()));
    assert!(!((*in_.borrow()).is_null()));
    println!("{}", Ptr::from_string_literal(b"printf"));
    print!("hello world");
    let s: Value<Vec<u8>> = Rc::new(RefCell::new(
        Ptr::from_string_literal(b"a string")
            .to_c_string_iterator()
            .chain(std::iter::once(0))
            .collect::<Vec<u8>>(),
    ));
    println!("{}", (s.as_pointer() as Ptr<u8>));
    println!(
        "{}",
        (Rc::new(RefCell::new(
            ({
                fn_0(
                    Ptr::from_string_literal(b"foo")
                        .to_c_string_iterator()
                        .chain(std::iter::once(0))
                        .collect::<Vec<u8>>(),
                )
            })
        ))
        .as_pointer() as Ptr<u8>)
    );
    println!(
        "{}",
        (({ fn2_1(s.as_pointer(),) }).to_strong().as_pointer() as Ptr<u8>)
    );
    return 0;
}
