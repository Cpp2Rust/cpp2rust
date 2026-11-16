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
pub struct p1_1_parameter_hides_member_in_constructor_HasMemberNamedLikeParameter {
    #[offset(0)]
    pub amount: i32,
}
impl p1_1_parameter_hides_member_in_constructor_HasMemberNamedLikeParameter {
    pub fn new(mut amount: i32) -> Self {
        let __this: Value<p1_1_parameter_hides_member_in_constructor_HasMemberNamedLikeParameter> =
            Rc::new(RefCell::new(Self {
                amount: (amount + 1),
            }));
        let this: Ptr<p1_1_parameter_hides_member_in_constructor_HasMemberNamedLikeParameter> =
            __this.as_pointer();
        amount += 10;
        assert!((amount == 14));
        assert!((this.with(|__s| __s.amount) == 5));
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
pub fn test_0() {
    let object: Value<p1_1_parameter_hides_member_in_constructor_HasMemberNamedLikeParameter> =
        Rc::new(RefCell::new(
            p1_1_parameter_hides_member_in_constructor_HasMemberNamedLikeParameter::new({ 4 }),
        ));
    assert!(({ ({ (*object.borrow()).amount } == 5) }));
    assert!(
        (({
            p1_1_parameter_hides_member_in_constructor_HasMemberNamedLikeParameterImpl::add(
                &object.as_pointer(),
                20,
            )
        }) == 25)
    );
    assert!(({ ({ (*object.borrow()).amount } == 5) }));
}
thread_local!(
    pub static rate_1: Value<i32> = Rc::new(RefCell::new(3));
);
pub fn scale_2(mut rate: i32) -> i32 {
    rate *= 2;
    return rate;
}
pub fn read_rate_3() -> i32 {
    return rate_1.with(|rc| *rc.borrow());
}
pub fn test_4() {
    assert!((({ scale_2(5,) }) == 10));
    assert!((rate_1.with(|rc| *rc.borrow()) == 3));
    assert!((({ read_rate_3() }) == 3));
}
pub fn test_5() {
    let mut amount: i32 = 6;
    let total: Value<i32> = Rc::new(RefCell::new(0));
    let add_doubled: Value<FnPtr<fn(i32) -> i32>> = Rc::new(RefCell::new(lambda!(
        {
            let total: Ptr<i32> = total.as_pointer();
        },
        |mut amount: i32| -> i32 {
            amount *= 2;
            {
                let __rhs = amount;
                total.with_mut(|__v| *__v = *__v + __rhs)
            };
            return amount;
        }
    )));
    assert!(({ (({ (*add_doubled.borrow()).call(4,) }) == 8) }));
    assert!(({ ((*total.borrow()) == 8) }));
    assert!((amount == 6));
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    ({ test_0() });
    ({ test_4() });
    ({ test_5() });
    return 0;
}
pub trait p1_1_parameter_hides_member_in_constructor_HasMemberNamedLikeParameterImpl {
    fn add(&self, amount: i32) -> i32;
}
impl p1_1_parameter_hides_member_in_constructor_HasMemberNamedLikeParameterImpl
    for Ptr<p1_1_parameter_hides_member_in_constructor_HasMemberNamedLikeParameter>
{
    fn add(&self, mut amount: i32) -> i32 {
        return ((*self).with(|__s| __s.amount) + amount);
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = rate_1.with(|_| ());
}
