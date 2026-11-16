extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct TakesAddressOfOwnMember {
    #[offset(0)]
    pub value: i32,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let first: Value<TakesAddressOfOwnMember> =
        Rc::new(RefCell::new(TakesAddressOfOwnMember { value: 3 }));
    let second: Value<TakesAddressOfOwnMember> =
        Rc::new(RefCell::new(TakesAddressOfOwnMember { value: 4 }));
    let mut p: Ptr<i32> = ({ TakesAddressOfOwnMemberImpl::parenthesized(&first.as_pointer()) });
    assert!(({ (p).clone() } == { (field_ptr!(first.as_pointer(), value)) }));
    p.write(5);
    assert!(({ ({ (*first.borrow()).value } == 5) }));
    assert!(({ ({ (*second.borrow()).value } == 4) }));
    let mut q: Ptr<i32> = ({ TakesAddressOfOwnMemberImpl::unqualified(&second.as_pointer()) });
    assert!(({ (q).clone() } == { (field_ptr!(second.as_pointer(), value)) }));
    q.write(6);
    assert!(({ ({ (*first.borrow()).value } == 5) }));
    assert!(({ ({ (*second.borrow()).value } == 6) }));
    return 0;
}
pub trait TakesAddressOfOwnMemberImpl {
    fn parenthesized(&self) -> Ptr<i32>;
    fn unqualified(&self) -> Ptr<i32>;
}
impl TakesAddressOfOwnMemberImpl for Ptr<TakesAddressOfOwnMember> {
    fn parenthesized(&self) -> Ptr<i32> {
        return (field_ptr!((*self), value));
    }
    fn unqualified(&self) -> Ptr<i32> {
        return (field_ptr!((*self), value));
    }
}
pub fn __cpp2rust_init_globals() {}
