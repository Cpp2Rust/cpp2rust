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
        let __a0 = (3, 4);
        ((__a0.0 as i64), (__a0.1 as f64))
    };
    assert!(((from_moved_pair.0) == (3_i64)));
    assert!(((from_moved_pair.1) == (4_f64)));
    let mut ints: (i32, i32) = (5, 6);
    let mut from_copied_pair: (Celsius, i64) = {
        let __a0 = ints;
        (
            <Celsius as CppFromUnsafe<i32>>::cpp_from(&__a0.0),
            (__a0.1 as i64),
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
        let __a0 = (Celsius::new({ 12 }), Celsius::new({ 13 }));
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
        let __a0 = (Celsius::new({ 14 }), Celsius::new({ 15 }));
        (
            <Holder as CppFromUnsafe<Celsius>>::cpp_from_move(&mut __a0.0),
            <Holder as CppFromUnsafe<Celsius>>::cpp_from_move(&mut __a0.1),
        )
    };
    assert!(((from_moved_by_value_pair.0.value) == (14)));
    assert!(((from_moved_by_value_pair.1.value) == (15)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
