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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom)]
#[cpp_from(i32, new, some_value)]
#[byte_size(4)]
pub struct Fallback {
    #[offset(0)]
    pub value: i32,
}
impl Fallback {
    pub fn new(v: Option<i32>) -> Self {
        let mut v: i32 = v.unwrap_or_else(|| -1_i32);
        Self { value: v }
    }
}
impl Default for Fallback {
    fn default() -> Self {
        { Fallback::new(None) }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Counter {
    #[offset(0)]
    pub hits: i32,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(Counter, new, mut_ptr)]
#[byte_size(4)]
pub struct Observer {
    #[offset(0)]
    pub seen: i32,
}
impl Observer {
    pub fn new(c: Ptr<Counter>) -> Self {
        let __this: Observer = Self {
            seen: c.with(|__s| __s.hits),
        };
        field!(c, hits).with_mut(|__v| __v.postfix_inc());
        __this
    }
}
impl Tag {
    pub fn new(u: Ptr<Celsius>) -> Self {
        Self {
            code: (u.with(|__s| __s.degrees) + 100),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(Celsius, new, ptr)]
#[byte_size(4)]
pub struct Tag {
    #[offset(0)]
    pub code: i32,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(Ptr< i8  >  , new, value)]
#[byte_size(4)]
pub struct Name {
    #[offset(0)]
    pub len: i32,
}
impl Name {
    pub fn new(mut s: Ptr<i8>) -> Self {
        let __this: Value<Name> = Rc::new(RefCell::new(Self { len: 0 }));
        let this: Ptr<Name> = __this.as_pointer();
        'loop_: while ((elem!(s, this.with(|__s| __s.len)).read()) != 0) {
            field!(this, len).with_mut(|__v| __v.prefix_inc());
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
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
    let from_moved_pair: Value<(Value<i64>, Value<f64>)> = Rc::new(RefCell::new(
        match (Rc::new(RefCell::new((
            Rc::new(RefCell::new(3)),
            Rc::new(RefCell::new(4)),
        )))
        .as_pointer(),)
        {
            (__tmp_0,) => (
                Rc::new(RefCell::new(
                    (((*__tmp_0.upgrade().deref()).0.as_pointer().read()) as i64),
                )),
                Rc::new(RefCell::new(
                    (((*__tmp_0.upgrade().deref()).1.as_pointer().read()) as f64),
                )),
            ),
        },
    ));
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
        Rc::new(RefCell::new(
            (((*ints.borrow()).1.as_pointer().read()) as i64),
        )),
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
    let from_moved_record_pair: Value<(Value<Label>, Value<Owner>)> = Rc::new(RefCell::new(
        match (Rc::new(RefCell::new((
            Rc::new(RefCell::new(Celsius::new({ 12 }))),
            Rc::new(RefCell::new(Celsius::new({ 13 }))),
        )))
        .as_pointer(),)
        {
            (__tmp_1,) => (
                Rc::new(RefCell::new(<Label as CppFrom<Celsius>>::cpp_from_move(
                    (*__tmp_1.upgrade().deref()).0.as_pointer(),
                ))),
                Rc::new(RefCell::new(<Owner as CppFrom<Celsius>>::cpp_from_move(
                    (*__tmp_1.upgrade().deref()).1.as_pointer(),
                ))),
            ),
        },
    ));
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
    let from_moved_by_value_pair: Value<(Value<Holder>, Value<Holder>)> = Rc::new(RefCell::new(
        match (Rc::new(RefCell::new((
            Rc::new(RefCell::new(Celsius::new({ 14 }))),
            Rc::new(RefCell::new(Celsius::new({ 15 }))),
        )))
        .as_pointer(),)
        {
            (__tmp_2,) => (
                Rc::new(RefCell::new(<Holder as CppFrom<Celsius>>::cpp_from_move(
                    (*__tmp_2.upgrade().deref()).0.as_pointer(),
                ))),
                Rc::new(RefCell::new(<Holder as CppFrom<Celsius>>::cpp_from_move(
                    (*__tmp_2.upgrade().deref()).1.as_pointer(),
                ))),
            ),
        },
    ));
    assert!(({ (*(*from_moved_by_value_pair.borrow()).0.borrow()).value } == 14));
    assert!(({ (*(*from_moved_by_value_pair.borrow()).1.borrow()).value } == 15));
    let mut defaulted: Fallback = Fallback::new(None);
    assert!((defaulted.value == -1_i32));
    let from_defaulted_ctor: Value<(Value<Fallback>, Value<i64>)> = Rc::new(RefCell::new((
        Rc::new(RefCell::new(<Fallback as CppFrom<i32>>::cpp_from(
            (*ints.borrow()).0.as_pointer(),
        ))),
        Rc::new(RefCell::new(
            (((*ints.borrow()).1.as_pointer().read()) as i64),
        )),
    )));
    assert!(({ (*(*from_defaulted_ctor.borrow()).0.borrow()).value } == 5));
    assert!(((*(*from_defaulted_ctor.borrow()).1.borrow()) == 6_i64));
    let counter: Value<Counter> = Rc::new(RefCell::new(Counter { hits: 3 }));
    let forwarded_mutable_ref: Value<(Value<Observer>, Value<i32>)> = Rc::new(RefCell::new((
        Rc::new(RefCell::new(Observer::new({ counter.as_pointer() }))),
        Rc::new(RefCell::new(0)),
    )));
    assert!(({ (*(*forwarded_mutable_ref.borrow()).0.borrow()).seen } == 3));
    assert!(({ (*counter.borrow()).hits } == 4));
    let from_template_ctor: Value<(Value<Tag>, Value<Tag>)> = Rc::new(RefCell::new((
        Rc::new(RefCell::new(<Tag as CppFrom<Celsius>>::cpp_from(
            (*celsius.borrow()).0.as_pointer(),
        ))),
        Rc::new(RefCell::new(<Tag as CppFrom<Celsius>>::cpp_from(
            (*celsius.borrow()).1.as_pointer(),
        ))),
    )));
    assert!(({ (*(*from_template_ctor.borrow()).0.borrow()).code } == 110));
    assert!(({ (*(*from_template_ctor.borrow()).1.borrow()).code } == 111));
    let lvalue_source: Value<Celsius> = Rc::new(RefCell::new(Celsius::new({ 16 })));
    let moved_source: Value<Celsius> = Rc::new(RefCell::new(Celsius::new({ 17 })));
    let forwarded_records: Value<(Value<Label>, Value<Owner>)> = Rc::new(RefCell::new((
        Rc::new(RefCell::new(Label::new({ lvalue_source.as_pointer() }))),
        Rc::new(RefCell::new(Owner::new_2({ moved_source.as_pointer() }))),
    )));
    assert!(({ (*(*forwarded_records.borrow()).0.borrow()).id } == 16));
    assert!(({ (*(*forwarded_records.borrow()).1.borrow()).taken } == 17));
    assert!(({ (*(*forwarded_records.borrow()).1.borrow()).copied } == 0));
    let from_string_literals: Value<(Value<Name>, Value<Name>)> = Rc::new(RefCell::new((
        Rc::new(RefCell::new(Name::new({
            Ptr::<i8>::from_string_literal(b"ab")
        }))),
        Rc::new(RefCell::new(Name::new({
            Ptr::<i8>::from_string_literal(b"cde")
        }))),
    )));
    assert!(({ (*(*from_string_literals.borrow()).0.borrow()).len } == 2));
    assert!(({ (*(*from_string_literals.borrow()).1.borrow()).len } == 3));
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
