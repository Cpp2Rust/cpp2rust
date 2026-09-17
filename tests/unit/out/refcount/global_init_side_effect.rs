extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static total_0: Value<i32> = Rc::new(RefCell::new(0));
);
#[derive(Default)]
pub struct S {}
impl S {
    pub fn S(x: i32) -> Self {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        let __this: Value<S> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<S> = __this.as_pointer();
        (*total_0.with(Value::clone).borrow_mut()) += (*x.borrow());
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for S {
    fn clone(&self) -> Self {
        let __this: Value<S> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<S> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for S {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
thread_local!(
    pub static a_1: Value<S> = Rc::new(RefCell::new(S::S({ 1 })));
);
thread_local!(
    pub static b_2: Value<S> = Rc::new(RefCell::new(S::S({ 10 })));
);
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!((total_0.with(|rc| rc.borrow().clone()) == 11));
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = total_0.with(|_| ());
    let _ = a_1.with(|_| ());
    let _ = b_2.with(|_| ());
}
