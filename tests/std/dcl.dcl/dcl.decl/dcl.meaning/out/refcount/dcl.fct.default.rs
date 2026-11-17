extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static scaled_calls_0: Value<i32> = Rc::new(RefCell::new(0_i32));
);
thread_local!(
    pub static rate_1: Value<i32> = Rc::new(RefCell::new(1));
);
pub fn scaled_2(mut amount: i32) -> i32 {
    (*scaled_calls_0.with(Value::clone).borrow_mut()).prefix_inc();
    return (amount * 10);
}
pub fn total_3(base: Option<i32>) -> i32 {
    let mut base: i32 = base.unwrap_or_else(|| ({ scaled_2(rate_1.with(|rc| *rc.borrow())) }));
    return base;
}
pub fn test_4() {
    rate_1.with(|rc| *rc.borrow_mut() = 2);
    {
        let mut rate: i32 = 3;
        assert!((({ total_3(None,) }) == 20));
        assert!((scaled_calls_0.with(|rc| *rc.borrow()) == 1));
        assert!((rate == 3));
    }
    assert!((({ total_3(Some(5),) }) == 5));
    assert!((scaled_calls_0.with(|rc| *rc.borrow()) == 1));
}
thread_local!(
    pub static next_ticket_calls_5: Value<i32> = Rc::new(RefCell::new(0_i32));
);
pub fn next_ticket_6() -> i32 {
    return ((*(*next_ticket_calls_5.with(Value::clone).borrow_mut()).prefix_inc()) * 100);
}
pub fn issue_7(ticket: Option<i32>) -> i32 {
    let mut ticket: i32 = ticket.unwrap_or_else(|| ({ next_ticket_6() }));
    return ticket;
}
pub fn test_8() {
    assert!((next_ticket_calls_5.with(|rc| *rc.borrow()) == 0));
    assert!((({ issue_7(None,) }) == 100));
    assert!((next_ticket_calls_5.with(|rc| *rc.borrow()) == 1));
    assert!((({ issue_7(Some(7),) }) == 7));
    assert!((next_ticket_calls_5.with(|rc| *rc.borrow()) == 1));
    assert!((({ issue_7(None,) }) == 200));
    assert!((next_ticket_calls_5.with(|rc| *rc.borrow()) == 2));
}
thread_local!(
    pub static step_9: Value<i32> = Rc::new(RefCell::new(100));
);
thread_local!(
    pub static step_10: Value<i32> = Rc::new(RefCell::new(4));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct p9_static_member_hides_namespace_variable_HasStaticStep {
    #[offset(0)]
    pub total: i32,
}
impl Default for p9_static_member_hides_namespace_variable_HasStaticStep {
    fn default() -> Self {
        { p9_static_member_hides_namespace_variable_HasStaticStep { total: 1 } }
    }
}
pub fn test_11() {
    let object: Value<p9_static_member_hides_namespace_variable_HasStaticStep> = Rc::new(
        RefCell::new(<p9_static_member_hides_namespace_variable_HasStaticStep>::default()),
    );
    assert!(
        (({
            p9_static_member_hides_namespace_variable_HasStaticStepImpl::advance(
                &object.as_pointer(),
                None,
            )
        }) == 5)
    );
    step_10.with(|rc| *rc.borrow_mut() = 6);
    assert!(
        (({
            p9_static_member_hides_namespace_variable_HasStaticStepImpl::advance(
                &object.as_pointer(),
                None,
            )
        }) == 11)
    );
    assert!(
        (({
            p9_static_member_hides_namespace_variable_HasStaticStepImpl::advance(
                &object.as_pointer(),
                Some(2),
            )
        }) == 13)
    );
    assert!((step_9.with(|rc| *rc.borrow()) == 100));
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    ({ test_4() });
    ({ test_8() });
    ({ test_11() });
    return 0;
}
pub trait p9_static_member_hides_namespace_variable_HasStaticStepImpl {
    fn advance(&self, amount: Option<i32>) -> i32;
}
impl p9_static_member_hides_namespace_variable_HasStaticStepImpl
    for Ptr<p9_static_member_hides_namespace_variable_HasStaticStep>
{
    fn advance(&self, amount: Option<i32>) -> i32 {
        let mut amount: i32 = amount.unwrap_or_else(|| step_10.with(|rc| *rc.borrow()));
        {
            let __rhs = amount;
            field!((*self), total).with_mut(|__v| *__v = *__v + __rhs)
        };
        return (*self).with(|__s| __s.total);
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = scaled_calls_0.with(|_| ());
    let _ = rate_1.with(|_| ());
    let _ = next_ticket_calls_5.with(|_| ());
    let _ = step_9.with(|_| ());
    let _ = step_10.with(|_| ());
}
