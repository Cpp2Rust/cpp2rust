extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(4)]
pub struct Tracker {
    #[offset(0)]
    pub tag: i32,
}
impl Tracker {
    pub fn new(mut tag: i32) -> Self {
        let __this: Tracker = Self { tag: tag };
        println!("construct {}", tag);
        __this
    }
}
thread_local!(
    pub static global_0: Value<Tracker> = {
        let __static = Rc::new(RefCell::new(Tracker::new({ 1 })));
        libcc2rs::register_static_destructor(|| {
            global_0.with(Value::clone).as_pointer().destructor()
        });
        __static
    };
);
pub fn exit_from_nested_call_1() {
    println!("local static {}", {
        (*local_static_2.with(Value::clone).borrow()).tag
    });
    let automatic: Value<Tracker> = Rc::new(RefCell::new(Tracker::new({ 3 })));
    let _dtor_automatic =
        std::mem::DropGuard::new(automatic.clone(), |__p| __p.as_pointer().destructor());
    libcc2rs::cpp2rust_exit(0);
}
thread_local!(
    static local_static_2: Value<Tracker> = {
        let __static = Rc::new(RefCell::new(Tracker::new({ 2 })));
        libcc2rs::register_static_destructor(|| {
            local_static_2.with(Value::clone).as_pointer().destructor()
        });
        __static
    };
);
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    ({ exit_from_nested_call_1() });
    println!("not reached");
    return 0;
}
pub trait TrackerImpl {
    fn destructor(&self);
}
impl TrackerImpl for Ptr<Tracker> {
    fn destructor(&self) {
        println!("destroy {}", (*self).with(|__s| __s.tag));
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = global_0.with(|_| ());
}
