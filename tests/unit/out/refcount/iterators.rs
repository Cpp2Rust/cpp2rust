extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let x: Value<Vec<u8>> = Rc::new(RefCell::new(
        Ptr::from_string_literal("hello")
            .to_c_string_iterator()
            .chain(std::iter::once(0))
            .collect::<Vec<u8>>(),
    ));
    'loop_: for mut c in x.as_pointer().to_string_iterator() as StringIterator<u8> {
        c.with_mut(|__v| __v.prefix_inc());
    }
    'loop_: for mut c in x.as_pointer().to_string_iterator() as StringIterator<u8> {
        println!("{}", ((c.read()) as i32) as u8 as char);
    }
    'loop_: for mut c in x.as_pointer().to_string_iterator() as StringIterator<u8> {
        let c: Value<u8> = Rc::new(RefCell::new(c.read().clone()));
        println!("{}", ((*c.borrow()) as i32) as u8 as char);
    }
    let v: Value<Vec<Ptr<i32>>> = Rc::new(RefCell::new(Vec::new()));
    (*v.borrow_mut()).push(Ptr::alloc(2));
    (*v.borrow_mut()).push(Ptr::alloc(3));
    'loop_: for mut p in v.as_pointer() as Ptr<Ptr<i32>> {
        let p: Value<Ptr<i32>> = Rc::new(RefCell::new(p.read().clone()));
        println!("{}", ((*p.borrow()).read()));
    }
    return 0;
}
