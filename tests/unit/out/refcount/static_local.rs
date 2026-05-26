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
        static s_static_i: Value<i32> = <Value<i32>>::default();
    );
    thread_local!(
        static s_static_f: Value<f32> = <Value<f32>>::default();
    );
    thread_local!(
        static s_static_b: Value<bool> = <Value<bool>>::default();
    );
    thread_local!(
        static s_kX1: Value<i32> = Rc::new(RefCell::new(1));
    );
    thread_local!(
        static s_kX2: Value<i32> = Rc::new(RefCell::new(2));
    );
    (*s_kX1.with(Value::clone).borrow_mut()) += 1;
    return (((*s_kX1.with(Value::clone).borrow()) + (*s_kX2.with(Value::clone).borrow()))
        + (*s_static_i.with(Value::clone).borrow()));
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    return ((({ foo_0() }) + ({ foo_0() })) + ({ foo_0() }));
}
