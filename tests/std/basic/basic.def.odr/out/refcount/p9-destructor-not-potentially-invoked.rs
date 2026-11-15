extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static t_ctors_0: Value<i32> = Rc::new(RefCell::new(0_i32));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct HasUndefinedDestructor {}
impl HasUndefinedDestructor {
    pub fn new() -> Self {
        let __this: HasUndefinedDestructor = Self {};
        (*t_ctors_0.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
}
impl Default for HasUndefinedDestructor {
    fn default() -> Self {
        { HasUndefinedDestructor::new() }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut p: Ptr<HasUndefinedDestructor> = Ptr::alloc(HasUndefinedDestructor::new());
    assert!(!((p).is_null()));
    assert!((t_ctors_0.with(|rc| *rc.borrow()) == 1));
    return 0;
}
pub trait HasUndefinedDestructorImpl {
    fn destructor(&self) {
        unimplemented!()
    }
}
impl HasUndefinedDestructorImpl for Ptr<HasUndefinedDestructor> {}
pub fn __cpp2rust_init_globals() {
    let _ = t_ctors_0.with(|_| ());
}
