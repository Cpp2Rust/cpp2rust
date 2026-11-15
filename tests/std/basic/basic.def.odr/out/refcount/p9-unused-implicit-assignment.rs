extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static m_ctors_0: Value<i32> = Rc::new(RefCell::new(0_i32));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct HasUndefinedCopyAssignment {}
impl HasUndefinedCopyAssignment {
    pub fn new() -> Self {
        let __this: HasUndefinedCopyAssignment = Self {};
        (*m_ctors_0.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
}
impl Default for HasUndefinedCopyAssignment {
    fn default() -> Self {
        { HasUndefinedCopyAssignment::new() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct HasMemberWithUndefinedCopyAssignment {
    #[offset(0)]
    #[byte_size(1)]
    pub m: HasUndefinedCopyAssignment,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut a: HasMemberWithUndefinedCopyAssignment =
        <HasMemberWithUndefinedCopyAssignment>::default();
    let mut b: HasMemberWithUndefinedCopyAssignment =
        <HasMemberWithUndefinedCopyAssignment>::default();
    assert!((m_ctors_0.with(|rc| *rc.borrow()) == 2));
    return 0;
}
pub trait HasUndefinedCopyAssignmentImpl {
    fn copy_assign(&self, _a0: Ptr<HasUndefinedCopyAssignment>) -> Ptr<HasUndefinedCopyAssignment> {
        unimplemented!()
    }
}
impl HasUndefinedCopyAssignmentImpl for Ptr<HasUndefinedCopyAssignment> {}
pub fn __cpp2rust_init_globals() {
    let _ = m_ctors_0.with(|_| ());
}
