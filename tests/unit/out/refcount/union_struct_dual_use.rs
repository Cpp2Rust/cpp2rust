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
pub struct Inner {
    #[offset(0)]
    pub a: i32,
    #[offset(4)]
    pub b: i32,
}
pub fn sum_inner_0(mut i: Ptr<Inner>) -> i32 {
    return ({ i.with(|__s| __s.a) } + { i.with(|__s| __s.b) });
}
#[derive(VaArg, FnPtrArg, ByteRepr, DeepClone)]
#[byte_size(16)]
pub struct anon_1 {
    #[offset(0)]
    #[byte_size(16)]
    __bytes: Value<Box<[u8]>>,
}
impl anon_1 {
    pub fn inner(this: Ptr<Self>) -> Ptr<Inner> {
        this.reinterpret_cast()
    }
    pub fn raw_(this: Ptr<Self>) -> Ptr<i8> {
        this.reinterpret_cast()
    }
}
impl Default for anon_1 {
    fn default() -> Self {
        anon_1 {
            __bytes: Rc::new(RefCell::new(Box::from([0u8; 16]))),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct Outer {
    #[offset(0)]
    #[byte_size(16)]
    pub u: anon_1,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let standalone: Value<Inner> = <Value<Inner>>::default();
    (*standalone.borrow_mut()).a = 3;
    (*standalone.borrow_mut()).b = 4;
    assert!((((({ sum_inner_0((standalone.as_pointer()),) }) == 7) as i32) != 0));
    let outer: Value<Outer> = <Value<Outer>>::default();
    {
        ((outer.as_pointer()) as Ptr<Outer>)
            .to_any()
            .memset((0) as u8, 16usize as usize);
        ((outer.as_pointer()) as Ptr<Outer>).to_any()
    };
    field!(anon_1::inner(field_ptr!(outer.as_pointer(), u)), a).write(3);
    field!(anon_1::inner(field_ptr!(outer.as_pointer(), u)), b).write(4);
    assert!(
        (((({ sum_inner_0((anon_1::inner(field_ptr!(outer.as_pointer(), u))).clone(),) }) == 7)
            as i32)
            != 0)
    );
    assert!(
        ((((((elem!(
            (anon_1::raw_(field_ptr!(outer.as_pointer(), u)).reinterpret_cast::<i8>() as Ptr::<i8>),
            0
        )
        .read()) as u8) as i32)
            == 3) as i32)
            != 0)
    );
    assert!(
        ((((((elem!(
            (anon_1::raw_(field_ptr!(outer.as_pointer(), u)).reinterpret_cast::<i8>() as Ptr::<i8>),
            4
        )
        .read()) as u8) as i32)
            == 4) as i32)
            != 0)
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
