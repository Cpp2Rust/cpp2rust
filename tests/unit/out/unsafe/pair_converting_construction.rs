extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct Celsius {
    pub degrees: i32,
}
impl Celsius {
    pub unsafe fn new(mut d: i32) -> Self {
        let mut this = Self { degrees: d };
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_into(i32, to_i32)]
pub struct Meters {
    pub value: i32,
}
impl Meters {
    pub unsafe fn to_i32(&self) -> i32 {
        return self.value;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(Celsius, new, ptr)]
pub struct Label {
    pub id: i32,
}
impl Label {
    pub unsafe fn new(c: *const Celsius) -> Self {
        let mut this = Self { id: (*c).degrees };
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(Celsius, new_1, ptr)]
#[cpp_from_move(Celsius, new_2, ptr)]
pub struct Owner {
    pub taken: i32,
    pub copied: i32,
}
impl Owner {
    pub unsafe fn new_1(c: *const Celsius) -> Self {
        let mut this = Self {
            taken: 0,
            copied: (*c).degrees,
        };
        this
    }
    pub unsafe fn new_2(c: *mut Celsius) -> Self {
        let mut this = Self {
            taken: (*c).degrees,
            copied: 0,
        };
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(Celsius, new, value)]
pub struct Holder {
    pub value: i32,
}
impl Holder {
    pub unsafe fn new(mut c: Celsius) -> Self {
        let mut this = Self { value: c.degrees };
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe)]
#[cpp_from(i32, new, some_value)]
pub struct Fallback {
    pub value: i32,
}
impl Fallback {
    pub unsafe fn new(mut v: Option<i32>) -> Self {
        let mut v: i32 = v.unwrap_or_else(|| unsafe { -1_i32 });
        let mut this = Self { value: v };
        this
    }
}
impl Default for Fallback {
    fn default() -> Self {
        unsafe { Fallback::new(None) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Counter {
    pub hits: i32,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(Counter, new, mut_ptr)]
pub struct Observer {
    pub seen: i32,
}
impl Observer {
    pub unsafe fn new(c: *mut Counter) -> Self {
        let mut this = Self { seen: (*c).hits };
        (*c).hits.postfix_inc();
        this
    }
}
impl Tag {
    pub unsafe fn new(u: *const Celsius) -> Self {
        let mut this = Self {
            code: (((*u).degrees) + (100)),
        };
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(Celsius, new, ptr)]
pub struct Tag {
    pub code: i32,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(*const libc::c_char , new, value)]
pub struct Name {
    pub len: i32,
}
impl Name {
    pub unsafe fn new(mut s: *const libc::c_char) -> Self {
        let mut this = Self { len: 0 };
        'loop_: while ((*s.offset((this.len) as isize)) != 0) {
            this.len.prefix_inc();
        }
        this
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut widened: (i64, u32) = ((-1_i32 as i64), 2_u32);
    assert!(((widened.0) == (-1_i64)));
    assert!(((widened.1) == (2_u32)));
    let mut from_ints: (Celsius, Celsius) = (Celsius::new({ 1 }), Celsius::new({ 2 }));
    assert!(((from_ints.0.degrees) == (1)));
    assert!(((from_ints.1.degrees) == (2)));
    let mut m: Meters = Meters { value: 7 };
    let mut from_operators: (i32, i64) = (
        (unsafe { Meters::to_i32(&m) }),
        ((unsafe { Meters::to_i32(&m) }) as i64),
    );
    assert!(((from_operators.0) == (7)));
    assert!(((from_operators.1) == (7_i64)));
    let mut from_moved_pair: (i64, f64) = {
        let mut __a0 = (3, 4);
        (((*&mut __a0.0) as i64), ((*&mut __a0.1) as f64))
    };
    assert!(((from_moved_pair.0) == (3_i64)));
    assert!(((from_moved_pair.1) == (4_f64)));
    let mut ints: (i32, i32) = (5, 6);
    let mut from_copied_pair: (Celsius, i64) = {
        let __a0 = ints;
        (
            <Celsius as CppFromUnsafe<i32>>::cpp_from(&__a0.0),
            ((*&__a0.1) as i64),
        )
    };
    assert!(((from_copied_pair.0.degrees) == (5)));
    assert!(((from_copied_pair.1) == (6_i64)));
    let mut meters: (Meters, Meters) = (Meters { value: 8 }, Meters { value: 9 });
    let mut from_operator_pair: (i32, i32) = {
        let __a0 = meters;
        (
            <i32 as CppFromUnsafe<Meters>>::cpp_from(&__a0.0),
            <i32 as CppFromUnsafe<Meters>>::cpp_from(&__a0.1),
        )
    };
    assert!(((from_operator_pair.0) == (8)));
    assert!(((from_operator_pair.1) == (9)));
    let mut celsius: (Celsius, Celsius) = (Celsius::new({ 10 }), Celsius::new({ 11 }));
    let mut from_record_pair: (Label, Owner) = {
        let __a0 = celsius;
        (
            <Label as CppFromUnsafe<Celsius>>::cpp_from(&__a0.0),
            <Owner as CppFromUnsafe<Celsius>>::cpp_from(&__a0.1),
        )
    };
    assert!(((from_record_pair.0.id) == (10)));
    assert!(((from_record_pair.1.copied) == (11)));
    assert!(((from_record_pair.1.taken) == (0)));
    let mut from_moved_record_pair: (Label, Owner) = {
        let mut __a0 = (Celsius::new({ 12 }), Celsius::new({ 13 }));
        (
            <Label as CppFromUnsafe<Celsius>>::cpp_from_move(&mut __a0.0),
            <Owner as CppFromUnsafe<Celsius>>::cpp_from_move(&mut __a0.1),
        )
    };
    assert!(((from_moved_record_pair.0.id) == (12)));
    assert!(((from_moved_record_pair.1.taken) == (13)));
    assert!(((from_moved_record_pair.1.copied) == (0)));
    let mut from_by_value_pair: (Holder, Holder) = {
        let __a0 = celsius;
        (
            <Holder as CppFromUnsafe<Celsius>>::cpp_from(&__a0.0),
            <Holder as CppFromUnsafe<Celsius>>::cpp_from(&__a0.1),
        )
    };
    assert!(((from_by_value_pair.0.value) == (10)));
    assert!(((from_by_value_pair.1.value) == (11)));
    let mut from_moved_by_value_pair: (Holder, Holder) = {
        let mut __a0 = (Celsius::new({ 14 }), Celsius::new({ 15 }));
        (
            <Holder as CppFromUnsafe<Celsius>>::cpp_from_move(&mut __a0.0),
            <Holder as CppFromUnsafe<Celsius>>::cpp_from_move(&mut __a0.1),
        )
    };
    assert!(((from_moved_by_value_pair.0.value) == (14)));
    assert!(((from_moved_by_value_pair.1.value) == (15)));
    let mut defaulted: Fallback = Fallback::new(None);
    assert!(((defaulted.value) == (-1_i32)));
    let mut from_defaulted_ctor: (Fallback, i64) = {
        let __a0 = ints;
        (
            <Fallback as CppFromUnsafe<i32>>::cpp_from(&__a0.0),
            ((*&__a0.1) as i64),
        )
    };
    assert!(((from_defaulted_ctor.0.value) == (5)));
    assert!(((from_defaulted_ctor.1) == (6_i64)));
    let mut counter: Counter = Counter { hits: 3 };
    let mut forwarded_mutable_ref: (Observer, i32) = (Observer::new({ &mut counter }), 0);
    assert!(((forwarded_mutable_ref.0.seen) == (3)));
    assert!(((counter.hits) == (4)));
    let mut from_template_ctor: (Tag, Tag) = {
        let __a0 = celsius;
        (
            <Tag as CppFromUnsafe<Celsius>>::cpp_from(&__a0.0),
            <Tag as CppFromUnsafe<Celsius>>::cpp_from(&__a0.1),
        )
    };
    assert!(((from_template_ctor.0.code) == (110)));
    assert!(((from_template_ctor.1.code) == (111)));
    let mut lvalue_source: Celsius = Celsius::new({ 16 });
    let mut moved_source: Celsius = Celsius::new({ 17 });
    let mut forwarded_records: (Label, Owner) = (
        Label::new({ &lvalue_source }),
        Owner::new_2({ &mut moved_source }),
    );
    assert!(((forwarded_records.0.id) == (16)));
    assert!(((forwarded_records.1.taken) == (17)));
    assert!(((forwarded_records.1.copied) == (0)));
    let mut from_string_literals: (Name, Name) = (
        Name::new({ c"ab".as_ptr() }),
        Name::new({ c"cde".as_ptr() }),
    );
    assert!(((from_string_literals.0.len) == (2)));
    assert!(((from_string_literals.1.len) == (3)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
