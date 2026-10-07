extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct context {
    #[offset(0)]
    pub verbose: i32,
    #[offset(4)]
    pub last_error: i32,
}
pub fn set_error_0(mut ctx: Ptr<context>, fmt: Ptr<i8>, __args: &[VaArg]) {
    let fmt: Value<Ptr<i8>> = Rc::new(RefCell::new(fmt));
    if (ctx.with(|__s| __s.verbose) != 0) {
        let ap: Value<VaList> = Rc::new(RefCell::new(VaList::default()));
        (*ap.borrow_mut()) = VaList::new(__args);
        let __rhs = (*ap.borrow_mut()).arg::<i32>();
        field!(ctx, last_error).write(__rhs);
    }
}
#[derive(VaArg, FnPtrArg, ByteRepr, DeepClone)]
#[byte_size(8)]
pub struct value {
    #[offset(0)]
    #[byte_size(8)]
    __bytes: Value<Box<[u8]>>,
}
impl value {
    pub fn i(&self) -> Ptr<i32> {
        (self.__bytes.as_pointer() as Ptr<u8>).reinterpret_cast()
    }
    pub fn l(&self) -> Ptr<i64> {
        (self.__bytes.as_pointer() as Ptr<u8>).reinterpret_cast()
    }
}
impl Default for value {
    fn default() -> Self {
        value {
            __bytes: Rc::new(RefCell::new(Box::from([0u8; 8]))),
        }
    }
}
pub fn pick_1(use_long: i32, __args: &[VaArg]) -> i64 {
    let use_long: Value<i32> = Rc::new(RefCell::new(use_long));
    let ap: Value<VaList> = Rc::new(RefCell::new(VaList::default()));
    (*ap.borrow_mut()) = VaList::new(__args);
    let v: Value<value> = Rc::new(RefCell::new((*ap.borrow_mut()).arg::<value>()));
    if ((*use_long.borrow()) != 0) {
        return ((*v.borrow()).l().read());
    }
    return (((*v.borrow()).i().read()) as i64);
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let ctx: Value<context> = <Value<context>>::default();
    (*ctx.borrow_mut()).verbose = 1;
    (*ctx.borrow_mut()).last_error = 0;
    ({
        set_error_0(
            (ctx.as_pointer()),
            Ptr::<i8>::from_string_literal(b"error %d"),
            &[(42).into()],
        )
    });
    assert!(((({ (*ctx.borrow()).last_error } == 42) as i32) != 0));
    (*ctx.borrow_mut()).verbose = 0;
    ({
        set_error_0(
            (ctx.as_pointer()),
            Ptr::<i8>::from_string_literal(b"error %d"),
            &[(99).into()],
        )
    });
    assert!(((({ (*ctx.borrow()).last_error } == 42) as i32) != 0));
    let v: Value<value> = <Value<value>>::default();
    (*v.borrow_mut()).l().write((1_i64 << 40));
    assert!((((({ pick_1(1, &[((*v.borrow()).clone()).into(),]) }) == (1_i64 << 40)) as i32) != 0));
    (*v.borrow_mut()).i().write(7);
    assert!((((({ pick_1(0, &[((*v.borrow()).clone()).into(),]) }) == 7_i64) as i32) != 0));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
