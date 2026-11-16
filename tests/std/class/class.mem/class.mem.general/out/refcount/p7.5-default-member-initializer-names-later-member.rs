extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static member_calls_0: Value<i32> = Rc::new(RefCell::new(0_i32));
);
thread_local!(
    pub static base_1: Value<i32> = Rc::new(RefCell::new(100));
);
pub fn scaled_2(mut amount: i32) -> i32 {
    return (amount * 1000);
}
thread_local!(
    pub static base_3: Value<i32> = Rc::new(RefCell::new(4));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct HasLaterInitializerOperands {
    #[offset(0)]
    pub first: i32,
    #[offset(4)]
    pub second: i32,
}
impl Default for HasLaterInitializerOperands {
    fn default() -> Self {
        {
            HasLaterInitializerOperands {
                first: (base_3.with(|rc| *rc.borrow()) + 1),
                second: ({
                    let _amount: i32 = { self.first };
                    HasLaterInitializerOperandsImpl::scaled(&(*self), _amount)
                }),
            }
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut object: HasLaterInitializerOperands = <HasLaterInitializerOperands>::default();
    assert!((object.first == 5));
    assert!((object.second == 50));
    assert!((member_calls_0.with(|rc| *rc.borrow()) == 1));
    assert!((base_1.with(|rc| *rc.borrow()) == 100));
    return 0;
}
pub trait HasLaterInitializerOperandsImpl {
    fn scaled(&self, amount: i32) -> i32;
}
impl HasLaterInitializerOperandsImpl for Ptr<HasLaterInitializerOperands> {
    fn scaled(&self, mut amount: i32) -> i32 {
        (*member_calls_0.with(Value::clone).borrow_mut()).prefix_inc();
        return (amount * 10);
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = member_calls_0.with(|_| ());
    let _ = base_1.with(|_| ());
    let _ = base_3.with(|_| ());
}
