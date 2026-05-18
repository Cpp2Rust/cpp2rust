extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct Inner {
    pub value: Value<i32>,
}
impl Clone for Inner {
    fn clone(&self) -> Self {
        let mut this = Self {
            value: Rc::new(RefCell::new((*self.value.borrow()))),
        };
        this
    }
}
impl ByteRepr for Inner {}
#[derive(Default)]
pub struct Outer {
    pub p: Value<Ptr<Inner>>,
}
impl Clone for Outer {
    fn clone(&self) -> Self {
        let mut this = Self {
            p: Rc::new(RefCell::new((*self.p.borrow()).clone())),
        };
        this
    }
}
impl ByteRepr for Outer {}
thread_local!(
    pub static alpha: Value<Inner> = Rc::new(RefCell::new(Inner {
        value: Rc::new(RefCell::new(1)),
    }));
);
thread_local!(
    pub static beta: Value<Inner> = Rc::new(RefCell::new(Inner {
        value: Rc::new(RefCell::new(2)),
    }));
);
thread_local!(
    pub static shared: Value<Inner> = Rc::new(RefCell::new(Inner {
        value: Rc::new(RefCell::new(42)),
    }));
);
thread_local!(
    pub static items: Value<Box<[Ptr<Inner>]>> = Rc::new(RefCell::new(Box::new([
        (alpha.with(Value::clone).as_pointer()),
        (beta.with(Value::clone).as_pointer()),
    ])));
);
thread_local!(
    pub static obj: Value<Outer> = Rc::new(RefCell::new(Outer {
        p: Rc::new(RefCell::new((shared.with(Value::clone).as_pointer()))),
    }));
);
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!(
        ((*(*(*items.with(Value::clone).borrow())[(0) as usize]
            .upgrade()
            .deref())
        .value
        .borrow())
            == 1)
    );
    assert!(
        ((*(*(*items.with(Value::clone).borrow())[(1) as usize]
            .upgrade()
            .deref())
        .value
        .borrow())
            == 2)
    );
    assert!(
        ((*(*(*(*obj.with(Value::clone).borrow()).p.borrow())
            .upgrade()
            .deref())
        .value
        .borrow())
            == 42)
    );
    thread_local!(
        static cache: Value<Box<[Ptr<Inner>]>> = Rc::new(RefCell::new(Box::new([
            (alpha.with(Value::clone).as_pointer()),
            (beta.with(Value::clone).as_pointer()),
        ])));
    );
    assert!(
        ((*(*(*cache.with(Value::clone).borrow())[(0) as usize]
            .upgrade()
            .deref())
        .value
        .borrow())
            == 1)
    );
    assert!(
        ((*(*(*cache.with(Value::clone).borrow())[(1) as usize]
            .upgrade()
            .deref())
        .value
        .borrow())
            == 2)
    );
    return 0;
}
