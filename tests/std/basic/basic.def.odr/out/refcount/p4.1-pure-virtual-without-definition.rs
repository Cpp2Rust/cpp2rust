extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static derived_calls_0: Value<i32> = Rc::new(RefCell::new(0_i32));
);
pub trait HasPureVirtual {
    fn f(&mut self);
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct OverridesPureVirtual {}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let d: Value<OverridesPureVirtual> = Rc::new(RefCell::new(<OverridesPureVirtual>::default()));
    let mut p: PtrDyn<dyn HasPureVirtual> = (d.as_pointer()).to_dyn::<dyn HasPureVirtual>(|w| w);
    ({ (*p.upgrade().deref_mut()).f() });
    assert!((derived_calls_0.with(|rc| *rc.borrow()) == 1));
    return 0;
}
impl HasPureVirtual for OverridesPureVirtual {
    fn f(&mut self) {
        (*derived_calls_0.with(Value::clone).borrow_mut()).prefix_inc();
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = derived_calls_0.with(|_| ());
}
