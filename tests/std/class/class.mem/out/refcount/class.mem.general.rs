extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static stock_0: Value<i32> = Rc::new(RefCell::new(100));
);
pub fn minimum_1() -> i32 {
    return 50;
}
pub type anon_2 = u32;
pub const anon_2_reserve: anon_2 = 30;
#[derive(Copy, Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct p7_1_function_body_names_later_member_HasLaterMembers {
    #[offset(0)]
    pub stock: i32,
}
impl Default for p7_1_function_body_names_later_member_HasLaterMembers {
    fn default() -> Self {
        { p7_1_function_body_names_later_member_HasLaterMembers { stock: 9 } }
    }
}
pub fn test_3() {
    let object: Value<p7_1_function_body_names_later_member_HasLaterMembers> = Rc::new(
        RefCell::new(<p7_1_function_body_names_later_member_HasLaterMembers>::default()),
    );
    assert!(
        (({
            p7_1_function_body_names_later_member_HasLaterMembersImpl::take(&object.as_pointer(), 3)
        }) == 38)
    );
    assert!(({ ({ (*object.borrow()).stock } == 6) }));
    assert!((stock_0.with(|rc| *rc.borrow()) == 100));
}
thread_local!(
    pub static step_4: Value<i32> = Rc::new(RefCell::new(100));
);
thread_local!(
    pub static step_5: Value<i32> = Rc::new(RefCell::new(4));
);
#[derive(Copy, Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct p7_2_default_argument_names_later_member_HasLaterStaticStep {
    #[offset(0)]
    pub total: i32,
}
impl Default for p7_2_default_argument_names_later_member_HasLaterStaticStep {
    fn default() -> Self {
        { p7_2_default_argument_names_later_member_HasLaterStaticStep { total: 1 } }
    }
}
pub fn test_6() {
    let object: Value<p7_2_default_argument_names_later_member_HasLaterStaticStep> = Rc::new(
        RefCell::new(<p7_2_default_argument_names_later_member_HasLaterStaticStep>::default()),
    );
    assert!(
        (({
            p7_2_default_argument_names_later_member_HasLaterStaticStepImpl::advance(
                &object.as_pointer(),
                step_5.with(|rc| *rc.borrow()),
            )
        }) == 5)
    );
    assert!(
        (({
            p7_2_default_argument_names_later_member_HasLaterStaticStepImpl::advance(
                &object.as_pointer(),
                2,
            )
        }) == 7)
    );
    assert!((step_4.with(|rc| *rc.borrow()) == 100));
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    ({ test_3() });
    ({ test_6() });
    return 0;
}
pub trait p7_1_function_body_names_later_member_HasLaterMembersImpl {
    fn take(&self, amount: i32) -> i32;
    fn minimum(&self) -> i32;
}
impl p7_1_function_body_names_later_member_HasLaterMembersImpl
    for Ptr<p7_1_function_body_names_later_member_HasLaterMembers>
{
    fn take(&self, mut amount: i32) -> i32 {
        {
            let __rhs = amount;
            field!((*self), stock).with_mut(|__v| *__v = *__v - __rhs)
        };
        return (((*self).with(|__s| __s.stock)
            + ({ p7_1_function_body_names_later_member_HasLaterMembersImpl::minimum(self) }))
            + (anon_2_reserve as i32));
    }
    fn minimum(&self) -> i32 {
        return 2;
    }
}
pub trait p7_2_default_argument_names_later_member_HasLaterStaticStepImpl {
    fn advance(&self, amount: i32) -> i32;
}
impl p7_2_default_argument_names_later_member_HasLaterStaticStepImpl
    for Ptr<p7_2_default_argument_names_later_member_HasLaterStaticStep>
{
    fn advance(&self, mut amount: i32) -> i32 {
        {
            let __rhs = amount;
            field!((*self), total).with_mut(|__v| *__v = *__v + __rhs)
        };
        return (*self).with(|__s| __s.total);
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = stock_0.with(|_| ());
    let _ = step_4.with(|_| ());
    let _ = step_5.with(|_| ());
}
