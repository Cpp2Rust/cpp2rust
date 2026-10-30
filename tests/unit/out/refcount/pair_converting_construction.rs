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
pub struct Celsius {
    #[offset(0)]
    pub degrees: i32,
}
impl Celsius {
    pub fn new(mut d: i32) -> Self {
        Self { degrees: d }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_into(i32, to_i32)]
#[byte_size(4)]
pub struct Meters {
    #[offset(0)]
    pub value: i32,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(Celsius, new, ptr)]
#[byte_size(4)]
pub struct Label {
    #[offset(0)]
    pub id: i32,
}
impl Label {
    pub fn new(c: Ptr<Celsius>) -> Self {
        Self {
            id: c.with(|__s| __s.degrees),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(Celsius, new_1, ptr)]
#[cpp_from_move(Celsius, new_2, ptr)]
#[byte_size(8)]
pub struct Owner {
    #[offset(0)]
    pub taken: i32,
    #[offset(4)]
    pub copied: i32,
}
impl Owner {
    pub fn new_1(c: Ptr<Celsius>) -> Self {
        Self {
            taken: 0,
            copied: c.with(|__s| __s.degrees),
        }
    }
    pub fn new_2(c: Ptr<Celsius>) -> Self {
        Self {
            taken: c.with(|__s| __s.degrees),
            copied: 0,
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(Celsius, new, value)]
#[byte_size(4)]
pub struct Holder {
    #[offset(0)]
    pub value: i32,
}
impl Holder {
    pub fn new(mut c: Celsius) -> Self {
        Self { value: c.degrees }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let widened: Value<(Value<i64>, Value<u32>)> = Rc::new(RefCell::new((
        Rc::new(RefCell::new((-1_i32 as i64))),
        Rc::new(RefCell::new(2_u32)),
    )));
    assert!(((*(*widened.borrow()).0.borrow()) == -1_i64));
    assert!(((*(*widened.borrow()).1.borrow()) == 2_u32));
    let from_ints: Value<(Value<Celsius>, Value<Celsius>)> = Rc::new(RefCell::new((
        Rc::new(RefCell::new(Celsius::new({ 1 }))),
        Rc::new(RefCell::new(Celsius::new({ 2 }))),
    )));
    assert!(({ (*(*from_ints.borrow()).0.borrow()).degrees } == 1));
    assert!(({ (*(*from_ints.borrow()).1.borrow()).degrees } == 2));
    let m: Value<Meters> = Rc::new(RefCell::new(Meters { value: 7 }));
    let from_operators: Value<(Value<i32>, Value<i64>)> = Rc::new(RefCell::new((
        Rc::new(RefCell::new(({ MetersImpl::to_i32(&m.as_pointer()) }))),
        Rc::new(RefCell::new(
            (({ MetersImpl::to_i32(&m.as_pointer()) }) as i64),
        )),
    )));
    assert!(((*(*from_operators.borrow()).0.borrow()) == 7));
    assert!(((*(*from_operators.borrow()).1.borrow()) == 7_i64));
    let from_moved_pair: Value<(Value<i64>, Value<f64>)> = Rc::new(RefCell::new((
        Rc::new(RefCell::new(<i64 as CppFrom<i32>>::cpp_from_move(
            (Rc::new(RefCell::new(3)), Rc::new(RefCell::new(4)))
                .0
                .as_pointer(),
        ))),
        Rc::new(RefCell::new(<f64 as CppFrom<i32>>::cpp_from_move(
            (Rc::new(RefCell::new(3)), Rc::new(RefCell::new(4)))
                .1
                .as_pointer(),
        ))),
    )));
    assert!(((*(*from_moved_pair.borrow()).0.borrow()) == 3_i64));
    assert!(((*(*from_moved_pair.borrow()).1.borrow()) == 4_f64));
    let ints: Value<(Value<i32>, Value<i32>)> = Rc::new(RefCell::new((
        Rc::new(RefCell::new(5)),
        Rc::new(RefCell::new(6)),
    )));
    let from_copied_pair: Value<(Value<Celsius>, Value<i64>)> = Rc::new(RefCell::new((
        Rc::new(RefCell::new(<Celsius as CppFrom<i32>>::cpp_from(
            (*ints.borrow()).0.as_pointer(),
        ))),
        Rc::new(RefCell::new(<i64 as CppFrom<i32>>::cpp_from(
            (*ints.borrow()).1.as_pointer(),
        ))),
    )));
    assert!(({ (*(*from_copied_pair.borrow()).0.borrow()).degrees } == 5));
    assert!(((*(*from_copied_pair.borrow()).1.borrow()) == 6_i64));
    let meters: Value<(Value<Meters>, Value<Meters>)> = Rc::new(RefCell::new((
        Rc::new(RefCell::new(Meters { value: 8 })),
        Rc::new(RefCell::new(Meters { value: 9 })),
    )));
    let from_operator_pair: Value<(Value<i32>, Value<i32>)> = Rc::new(RefCell::new((
        Rc::new(RefCell::new(<i32 as CppFrom<Meters>>::cpp_from(
            (*meters.borrow()).0.as_pointer(),
        ))),
        Rc::new(RefCell::new(<i32 as CppFrom<Meters>>::cpp_from(
            (*meters.borrow()).1.as_pointer(),
        ))),
    )));
    assert!(((*(*from_operator_pair.borrow()).0.borrow()) == 8));
    assert!(((*(*from_operator_pair.borrow()).1.borrow()) == 9));
    let celsius: Value<(Value<Celsius>, Value<Celsius>)> = Rc::new(RefCell::new((
        Rc::new(RefCell::new(Celsius::new({ 10 }))),
        Rc::new(RefCell::new(Celsius::new({ 11 }))),
    )));
    let from_record_pair: Value<(Value<Label>, Value<Owner>)> = Rc::new(RefCell::new((
        Rc::new(RefCell::new(<Label as CppFrom<Celsius>>::cpp_from(
            (*celsius.borrow()).0.as_pointer(),
        ))),
        Rc::new(RefCell::new(<Owner as CppFrom<Celsius>>::cpp_from(
            (*celsius.borrow()).1.as_pointer(),
        ))),
    )));
    assert!(({ (*(*from_record_pair.borrow()).0.borrow()).id } == 10));
    assert!(({ (*(*from_record_pair.borrow()).1.borrow()).copied } == 11));
    assert!(({ (*(*from_record_pair.borrow()).1.borrow()).taken } == 0));
    let from_moved_record_pair: Value<(Value<Label>, Value<Owner>)> = Rc::new(RefCell::new((
        Rc::new(RefCell::new(<Label as CppFrom<Celsius>>::cpp_from_move(
            (
                Rc::new(RefCell::new(Celsius::new({ 12 }))),
                Rc::new(RefCell::new(Celsius::new({ 13 }))),
            )
                .0
                .as_pointer(),
        ))),
        Rc::new(RefCell::new(<Owner as CppFrom<Celsius>>::cpp_from_move(
            (
                Rc::new(RefCell::new(Celsius::new({ 12 }))),
                Rc::new(RefCell::new(Celsius::new({ 13 }))),
            )
                .1
                .as_pointer(),
        ))),
    )));
    assert!(({ (*(*from_moved_record_pair.borrow()).0.borrow()).id } == 12));
    assert!(({ (*(*from_moved_record_pair.borrow()).1.borrow()).taken } == 13));
    assert!(({ (*(*from_moved_record_pair.borrow()).1.borrow()).copied } == 0));
    let from_by_value_pair: Value<(Value<Holder>, Value<Holder>)> = Rc::new(RefCell::new((
        Rc::new(RefCell::new(<Holder as CppFrom<Celsius>>::cpp_from(
            (*celsius.borrow()).0.as_pointer(),
        ))),
        Rc::new(RefCell::new(<Holder as CppFrom<Celsius>>::cpp_from(
            (*celsius.borrow()).1.as_pointer(),
        ))),
    )));
    assert!(({ (*(*from_by_value_pair.borrow()).0.borrow()).value } == 10));
    assert!(({ (*(*from_by_value_pair.borrow()).1.borrow()).value } == 11));
    let from_moved_by_value_pair: Value<(Value<Holder>, Value<Holder>)> = Rc::new(RefCell::new((
        Rc::new(RefCell::new(<Holder as CppFrom<Celsius>>::cpp_from_move(
            (
                Rc::new(RefCell::new(Celsius::new({ 14 }))),
                Rc::new(RefCell::new(Celsius::new({ 15 }))),
            )
                .0
                .as_pointer(),
        ))),
        Rc::new(RefCell::new(<Holder as CppFrom<Celsius>>::cpp_from_move(
            (
                Rc::new(RefCell::new(Celsius::new({ 14 }))),
                Rc::new(RefCell::new(Celsius::new({ 15 }))),
            )
                .1
                .as_pointer(),
        ))),
    )));
    assert!(({ (*(*from_moved_by_value_pair.borrow()).0.borrow()).value } == 14));
    assert!(({ (*(*from_moved_by_value_pair.borrow()).1.borrow()).value } == 15));
    return 0;
}
pub trait MetersImpl {
    fn to_i32(&self) -> i32;
}
impl MetersImpl for Ptr<Meters> {
    fn to_i32(&self) -> i32 {
        return (*self).with(|__s| __s.value);
    }
}
pub fn __cpp2rust_init_globals() {}
