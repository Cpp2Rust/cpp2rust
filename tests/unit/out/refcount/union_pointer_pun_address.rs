extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct node_a {
    pub n: Value<i32>,
}
impl ByteRepr for node_a {}
#[derive(Default)]
pub struct node_b {
    pub data: Value<AnyPtr>,
    pub next: Value<Ptr<node_b>>,
}
impl ByteRepr for node_b {}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let a: Value<node_a> = Rc::new(RefCell::new(node_a {
        n: Rc::new(RefCell::new(123)),
    }));
    #[derive(Default)]
    pub struct anon_0 {
        pub to_a: Value<Ptr<node_a>>,
        pub to_b: Value<Ptr<node_b>>,
    }
    impl ByteRepr for anon_0 {};
    let ptr: Value<anon_0> = <Value<anon_0>>::default();
    (*(*ptr.borrow()).to_a.borrow_mut()) = (a.as_pointer());
    let out: Value<Ptr<node_b>> = Rc::new(RefCell::new((*(*ptr.borrow()).to_b.borrow()).clone()));
    assert!({
        let _lhs = ((*out.borrow()).to_strong().as_pointer() as AnyPtr).clone();
        _lhs == ((a.as_pointer()).to_strong().as_pointer() as AnyPtr)
    });
    return 0;
}
