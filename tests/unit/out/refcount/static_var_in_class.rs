extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    static inner_const_0: Value<i32> = Rc::new(RefCell::new(1));
);
#[derive(Clone, ByteRepr, Default)]
pub struct C {}
thread_local!(
    pub static inner_const_1: Value<i32> = Rc::new(RefCell::new(2));
);
pub type anon_3 = u32;
pub const anon_3_kValue: anon_3 = 3;
#[derive(Clone, ByteRepr, Default)]
pub struct S {}
thread_local!(
    pub static counter_2: Value<i32> = Rc::new(RefCell::new(10));
);
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let c: Value<C> = Rc::new(RefCell::new(<C>::default()));
    assert!((({ CImpl::get(&c.as_pointer(),) }) == 1));
    assert!((inner_const_1.with(|rc| *rc.borrow()) == 2));
    let s: Value<S> = Rc::new(RefCell::new(<S>::default()));
    let p: Value<Ptr<S>> = Rc::new(RefCell::new((s.as_pointer())));
    assert!((inner_const_1.with(|rc| *rc.borrow()) == 2));
    assert!((inner_const_1.with(|rc| *rc.borrow()) == 2));
    assert!(((anon_3_kValue as i32) == 3));
    assert!(((anon_3_kValue as i32) == 3));
    (*counter_2.with(Value::clone).borrow_mut()) = 20;
    assert!((counter_2.with(|rc| *rc.borrow()) == 20));
    (*counter_2.with(Value::clone).borrow_mut()) += 5;
    assert!((counter_2.with(|rc| *rc.borrow()) == 25));
    assert!((counter_2.with(|rc| *rc.borrow()) == 25));
    return 0;
}
pub trait CImpl {
    fn get(&self) -> i32;
}
impl CImpl for Ptr<C> {
    fn get(&self) -> i32 {
        return inner_const_0.with(|rc| *rc.borrow());
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = inner_const_0.with(|_| ());
    let _ = inner_const_1.with(|_| ());
    let _ = counter_2.with(|_| ());
}
