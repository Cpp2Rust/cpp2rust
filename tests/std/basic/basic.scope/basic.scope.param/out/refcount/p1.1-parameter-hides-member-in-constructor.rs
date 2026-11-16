extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(i32, new, value)]
#[byte_size(4)]
pub struct HasMemberNamedLikeParameter {
    #[offset(0)]
    pub amount: i32,
}
impl HasMemberNamedLikeParameter {
    pub fn new(mut amount: i32) -> Self {
        let __this: Value<HasMemberNamedLikeParameter> = Rc::new(RefCell::new(Self {
            amount: (amount + 1),
        }));
        let this: Ptr<HasMemberNamedLikeParameter> = __this.as_pointer();
        amount += 10;
        assert!((amount == 14));
        assert!((this.with(|__s| __s.amount) == 5));
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let object: Value<HasMemberNamedLikeParameter> =
        Rc::new(RefCell::new(HasMemberNamedLikeParameter::new({ 4 })));
    assert!(({ ({ (*object.borrow()).amount } == 5) }));
    assert!((({ HasMemberNamedLikeParameterImpl::add(&object.as_pointer(), 20,) }) == 25));
    assert!(({ ({ (*object.borrow()).amount } == 5) }));
    return 0;
}
pub trait HasMemberNamedLikeParameterImpl {
    fn add(&self, amount: i32) -> i32;
}
impl HasMemberNamedLikeParameterImpl for Ptr<HasMemberNamedLikeParameter> {
    fn add(&self, mut amount: i32) -> i32 {
        return ((*self).with(|__s| __s.amount) + amount);
    }
}
pub fn __cpp2rust_init_globals() {}
