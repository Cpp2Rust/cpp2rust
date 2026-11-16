extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static step_0: Value<i32> = Rc::new(RefCell::new(100));
);
thread_local!(
    pub static step_1: Value<i32> = Rc::new(RefCell::new(4));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct HasLaterStaticStep {
    #[offset(0)]
    pub total: i32,
}
impl Default for HasLaterStaticStep {
    fn default() -> Self {
        { HasLaterStaticStep { total: 1 } }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let object: Value<HasLaterStaticStep> = Rc::new(RefCell::new(<HasLaterStaticStep>::default()));
    assert!((({ HasLaterStaticStepImpl::advance(&object.as_pointer(), None,) }) == 5));
    assert!((({ HasLaterStaticStepImpl::advance(&object.as_pointer(), Some(2),) }) == 7));
    assert!((step_0.with(|rc| *rc.borrow()) == 100));
    return 0;
}
pub trait HasLaterStaticStepImpl {
    fn advance(&self, amount: Option<i32>) -> i32;
}
impl HasLaterStaticStepImpl for Ptr<HasLaterStaticStep> {
    fn advance(&self, amount: Option<i32>) -> i32 {
        let mut amount: i32 = amount.unwrap_or_else(|| step_1.with(|rc| *rc.borrow()));
        {
            let __rhs = amount;
            field!((*self), total).with_mut(|__v| *__v = *__v + __rhs)
        };
        return (*self).with(|__s| __s.total);
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = step_0.with(|_| ());
    let _ = step_1.with(|_| ());
}
