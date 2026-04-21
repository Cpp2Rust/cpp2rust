extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn foo_0() -> i32 {
    thread_local!(
        static static_i: Value<i32> = <Value<i32>>::default();
    );
    thread_local!(
        static static_f: Value<f32> = <Value<f32>>::default();
    );
    thread_local!(
        static static_b: Value<bool> = <Value<bool>>::default();
    );
    thread_local!(
        static kX1: Value<i32> = Rc::new(RefCell::new(1));
    );
    thread_local!(
        static kX2: Value<i32> = Rc::new(RefCell::new(2));
    );
    (*kX1.with(Value::clone).borrow_mut()) += 1;
    return (((*kX1.with(Value::clone).borrow()) + (*kX2.with(Value::clone).borrow()))
        + (*static_i.with(Value::clone).borrow()));
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    return ((({ foo_0() }) + ({ foo_0() })) + ({ foo_0() }));
}
