extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static ctor_calls_0: Value<i32> = Rc::new(RefCell::new(0_i32));
);
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(4)]
pub struct HasUndefinedCopyCtor {
    #[offset(0)]
    pub value: i32,
}
impl HasUndefinedCopyCtor {
    pub fn new() -> Self {
        let __this: HasUndefinedCopyCtor = Self { value: 3 };
        (*ctor_calls_0.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
}
impl Default for HasUndefinedCopyCtor {
    fn default() -> Self {
        { HasUndefinedCopyCtor::new() }
    }
}
pub fn read_1(object: Ptr<HasUndefinedCopyCtor>) -> i32 {
    return object.with(|__s| __s.value);
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let a: Value<HasUndefinedCopyCtor> = Rc::new(RefCell::new(HasUndefinedCopyCtor::new()));
    assert!((({ read_1(a.as_pointer(),) }) == 3));
    assert!((ctor_calls_0.with(|rc| *rc.borrow()) == 1));
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = ctor_calls_0.with(|_| ());
}
