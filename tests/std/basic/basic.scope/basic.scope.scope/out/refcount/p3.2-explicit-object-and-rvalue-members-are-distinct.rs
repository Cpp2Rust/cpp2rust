extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static explicit_calls_0: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static rvalue_calls_1: Value<i32> = Rc::new(RefCell::new(0));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct HasExplicitObjectOverload {
    #[offset(0)]
    pub base: i32,
}
impl Default for HasExplicitObjectOverload {
    fn default() -> Self {
        { HasExplicitObjectOverload { base: 10 } }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let object: Value<HasExplicitObjectOverload> =
        Rc::new(RefCell::new(<HasExplicitObjectOverload>::default()));
    assert!((({ HasExplicitObjectOverloadImpl::pick_1(object.as_pointer(), 1,) }) == 11));
    assert!((explicit_calls_0.with(|rc| *rc.borrow()) == 1));
    assert!((rvalue_calls_1.with(|rc| *rc.borrow()) == 0));
    assert!(
        (({
            HasExplicitObjectOverloadImpl::pick_2(
                &Rc::new(RefCell::new(<HasExplicitObjectOverload>::default())).as_pointer(),
                2,
            )
        }) == 112)
    );
    assert!((explicit_calls_0.with(|rc| *rc.borrow()) == 1));
    assert!((rvalue_calls_1.with(|rc| *rc.borrow()) == 1));
    return 0;
}
pub trait HasExplicitObjectOverloadImpl {
    fn pick_1(self_: Ptr<HasExplicitObjectOverload>, amount: i32) -> i32;
    fn pick_2(&self, amount: i32) -> i32;
}
impl HasExplicitObjectOverloadImpl for Ptr<HasExplicitObjectOverload> {
    fn pick_1(self_: Ptr<HasExplicitObjectOverload>, mut amount: i32) -> i32 {
        (*explicit_calls_0.with(Value::clone).borrow_mut()).prefix_inc();
        return ({ self_.with(|__s| __s.base) } + { amount });
    }
    fn pick_2(&self, mut amount: i32) -> i32 {
        (*rvalue_calls_1.with(Value::clone).borrow_mut()).prefix_inc();
        return (((*self).with(|__s| __s.base) + amount) + 100);
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = explicit_calls_0.with(|_| ());
    let _ = rvalue_calls_1.with(|_| ());
}
