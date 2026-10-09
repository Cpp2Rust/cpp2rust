extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn foo_0() -> i32 {
    kX1_4.with(|rc| *rc.borrow_mut() += 1);
    return ((kX1_4.with(|rc| *rc.borrow()) + kX2_5.with(|rc| *rc.borrow()))
        + static_i_1.with(|rc| *rc.borrow()));
}
thread_local!(
    static static_i_1: Value<i32> = Rc::new(RefCell::new(0_i32));
);
thread_local!(
    static static_f_2: Value<f32> = Rc::new(RefCell::new(0_f32));
);
thread_local!(
    static static_b_3: Value<bool> = Rc::new(RefCell::new(false));
);
thread_local!(
    static kX1_4: Value<i32> = Rc::new(RefCell::new(1));
);
thread_local!(
    static kX2_5: Value<i32> = Rc::new(RefCell::new(2));
);
pub fn from_local_class_6() -> i32 {
    return (({ S_8Impl::get(&Rc::new(RefCell::new(<S_8>::default())).as_pointer()) })
        + ({ S_8Impl::get(&Rc::new(RefCell::new(<S_8>::default())).as_pointer()) }));
}
thread_local!(
    static x_7: Value<i32> = Rc::new(RefCell::new(3));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct S_8 {}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!((((({ foo_0() }) + ({ foo_0() })) + ({ foo_0() })) == 15));
    assert!((({ from_local_class_6() }) == 9));
    assert!((({ from_local_class_6() }) == 13));
    return 0;
}
pub trait S_8Impl {
    fn get(&self) -> i32;
}
impl S_8Impl for Ptr<S_8> {
    fn get(&self) -> i32 {
        return (*x_7.with(Value::clone).borrow_mut()).prefix_inc();
    }
}
pub fn __cpp2rust_init_globals() {}
