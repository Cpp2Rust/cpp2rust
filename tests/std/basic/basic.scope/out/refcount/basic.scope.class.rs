extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static balance_0: Value<i32> = Rc::new(RefCell::new(50));
);
#[derive(Copy, Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct p1_member_hides_namespace_variable_HasBalance {
    #[offset(0)]
    pub balance: i32,
}
impl Default for p1_member_hides_namespace_variable_HasBalance {
    fn default() -> Self {
        { p1_member_hides_namespace_variable_HasBalance { balance: 7 } }
    }
}
pub fn test_1() {
    let object: Value<p1_member_hides_namespace_variable_HasBalance> = Rc::new(RefCell::new(
        <p1_member_hides_namespace_variable_HasBalance>::default(),
    ));
    assert!(
        (({
            p1_member_hides_namespace_variable_HasBalanceImpl::deposit(&object.as_pointer(), 3)
        }) == 10)
    );
    assert!(({ ({ (*object.borrow()).balance } == 10) }));
    assert!((balance_0.with(|rc| *rc.borrow()) == 50));
    assert!(
        (({
            p1_member_hides_namespace_variable_HasBalanceImpl::deposit_globally(
                &object.as_pointer(),
                4,
            )
        }) == 54)
    );
    assert!(({ ({ (*object.borrow()).balance } == 10) }));
    assert!((balance_0.with(|rc| *rc.borrow()) == 54));
}
thread_local!(
    pub static initial_2: Value<i32> = Rc::new(RefCell::new(100));
);
#[derive(Copy, Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct p1_out_of_class_definition_is_in_class_scope_HasStaticTotal {
    #[offset(0)]
    pub held: i32,
}
impl Default for p1_out_of_class_definition_is_in_class_scope_HasStaticTotal {
    fn default() -> Self {
        { p1_out_of_class_definition_is_in_class_scope_HasStaticTotal { held: 1 } }
    }
}
thread_local!(
    pub static initial_3: Value<i32> = Rc::new(RefCell::new(6));
);
thread_local!(
    pub static total_4: Value<i32> = Rc::new(RefCell::new((initial_3.with(|rc| *rc.borrow()) + 1)));
);
pub fn test_5() {
    assert!((total_4.with(|rc| *rc.borrow()) == 7));
    let object: Value<p1_out_of_class_definition_is_in_class_scope_HasStaticTotal> = Rc::new(
        RefCell::new(<p1_out_of_class_definition_is_in_class_scope_HasStaticTotal>::default()),
    );
    assert!(
        (({
            p1_out_of_class_definition_is_in_class_scope_HasStaticTotalImpl::add(
                &object.as_pointer(),
                4,
            )
        }) == 5)
    );
    assert!((total_4.with(|rc| *rc.borrow()) == 13));
    assert!((initial_3.with(|rc| *rc.borrow()) == 6));
    assert!((initial_2.with(|rc| *rc.borrow()) == 100));
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    ({ test_1() });
    ({ test_5() });
    return 0;
}
pub trait p1_member_hides_namespace_variable_HasBalanceImpl {
    fn deposit(&self, amount: i32) -> i32;
    fn deposit_globally(&self, amount: i32) -> i32;
}
impl p1_member_hides_namespace_variable_HasBalanceImpl
    for Ptr<p1_member_hides_namespace_variable_HasBalance>
{
    fn deposit(&self, mut amount: i32) -> i32 {
        {
            let __rhs = amount;
            field!((*self), balance).with_mut(|__v| *__v = *__v + __rhs)
        };
        return (*self).with(|__s| __s.balance);
    }
    fn deposit_globally(&self, mut amount: i32) -> i32 {
        balance_0.with(|rc| *rc.borrow_mut() += amount);
        return balance_0.with(|rc| *rc.borrow());
    }
}
pub trait p1_out_of_class_definition_is_in_class_scope_HasStaticTotalImpl {
    fn add(&self, amount: i32) -> i32;
}
impl p1_out_of_class_definition_is_in_class_scope_HasStaticTotalImpl
    for Ptr<p1_out_of_class_definition_is_in_class_scope_HasStaticTotal>
{
    fn add(&self, mut amount: i32) -> i32 {
        {
            let __rhs = amount;
            field!((*self), held).with_mut(|__v| *__v = *__v + __rhs)
        };
        total_4.with(|rc| *rc.borrow_mut() += initial_3.with(|rc| *rc.borrow()));
        return (*self).with(|__s| __s.held);
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = balance_0.with(|_| ());
    let _ = initial_2.with(|_| ());
    let _ = initial_3.with(|_| ());
    let _ = total_4.with(|_| ());
}
