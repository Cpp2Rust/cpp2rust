extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static shared_0: Value<i32> = Rc::new(RefCell::new(3));
);
thread_local!(
    pub static calls_1: Value<i32> = Rc::new(RefCell::new(0));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct p3_2_static_member_is_ordinary_pointer_HasStaticMembers {}
impl p3_2_static_member_is_ordinary_pointer_HasStaticMembers {
    pub fn run() {
        (*calls_1.with(Value::clone).borrow_mut()).prefix_inc();
    }
}
pub fn test_2() {
    let mut object_pointer: Ptr<i32> = (shared_0.with(|v| v.as_pointer()));
    assert!(((object_pointer.read()) == 3));
    object_pointer.write(4);
    assert!((shared_0.with(|rc| *rc.borrow()) == 4));
    let mut function_pointer: FnPtr<fn()> =
        (FnPtr::<fn()>::new(p3_2_static_member_is_ordinary_pointer_HasStaticMembers::run));
    ({ function_pointer.call() });
    assert!((calls_1.with(|rc| *rc.borrow()) == 1));
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct p4_parenthesized_qualified_id_is_object_pointer_TakesAddressOfOwnMember {
    #[offset(0)]
    pub value: i32,
}
pub fn test_3() {
    let first: Value<p4_parenthesized_qualified_id_is_object_pointer_TakesAddressOfOwnMember> =
        Rc::new(RefCell::new(
            p4_parenthesized_qualified_id_is_object_pointer_TakesAddressOfOwnMember { value: 3 },
        ));
    let second: Value<p4_parenthesized_qualified_id_is_object_pointer_TakesAddressOfOwnMember> =
        Rc::new(RefCell::new(
            p4_parenthesized_qualified_id_is_object_pointer_TakesAddressOfOwnMember { value: 4 },
        ));
    let mut p: Ptr<i32> = ({
        p4_parenthesized_qualified_id_is_object_pointer_TakesAddressOfOwnMemberImpl::parenthesized(
            &first.as_pointer(),
        )
    });
    assert!(({ (p).clone() } == { (field_ptr!(first.as_pointer(), value)) }));
    p.write(5);
    assert!(({ ({ (*first.borrow()).value } == 5) }));
    assert!(({ ({ (*second.borrow()).value } == 4) }));
    let mut q: Ptr<i32> = ({
        p4_parenthesized_qualified_id_is_object_pointer_TakesAddressOfOwnMemberImpl::unqualified(
            &second.as_pointer(),
        )
    });
    assert!(({ (q).clone() } == { (field_ptr!(second.as_pointer(), value)) }));
    q.write(6);
    assert!(({ ({ (*first.borrow()).value } == 5) }));
    assert!(({ ({ (*second.borrow()).value } == 6) }));
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    ({ test_2() });
    ({ test_3() });
    return 0;
}
pub trait p4_parenthesized_qualified_id_is_object_pointer_TakesAddressOfOwnMemberImpl {
    fn parenthesized(&self) -> Ptr<i32>;
    fn unqualified(&self) -> Ptr<i32>;
}
impl p4_parenthesized_qualified_id_is_object_pointer_TakesAddressOfOwnMemberImpl
    for Ptr<p4_parenthesized_qualified_id_is_object_pointer_TakesAddressOfOwnMember>
{
    fn parenthesized(&self) -> Ptr<i32> {
        return (field_ptr!((*self), value));
    }
    fn unqualified(&self) -> Ptr<i32> {
        return (field_ptr!((*self), value));
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = shared_0.with(|_| ());
    let _ = calls_1.with(|_| ());
}
