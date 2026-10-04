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
pub fn sum_inner_0(i: Ptr<Inner>) -> i32 {
    let i: Value<Ptr<Inner>> = Rc::new(RefCell::new(i));
    return ({ (*i.borrow()).with(|__s| __s.a) } + { (*i.borrow()).with(|__s| __s.b) });
}
#[derive(ByteRepr, DeepClone)]
#[byte_size(16)]
pub struct anon_1 {
    #[offset(0)]
    #[byte_size(16)]
    __bytes: Value<Box<[u8]>>,
}
impl anon_1 {
    pub fn inner(&self) -> Ptr<Inner> {
        (self.__bytes.as_pointer() as Ptr<u8>).reinterpret_cast()
    }
    pub fn raw_(&self) -> Ptr<i8> {
        (self.__bytes.as_pointer() as Ptr<u8>).reinterpret_cast()
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
    field!((*outer.borrow_mut()).u.inner(), a).write(3);
    field!((*outer.borrow_mut()).u.inner(), b).write(4);
    assert!((((({ sum_inner_0(((*outer.borrow()).u.inner()).clone(),) }) == 7) as i32) != 0));
    assert!(
        ((((((elem!(
            ((*outer.borrow()).u.raw_().reinterpret_cast::<i8>() as Ptr::<i8>),
            0
        )
        .read()) as u8) as i32)
            == 3) as i32)
            != 0)
    );
    assert!(
        ((((((elem!(
            ((*outer.borrow()).u.raw_().reinterpret_cast::<i8>() as Ptr::<i8>),
            4
        )
        .read()) as u8) as i32)
            == 4) as i32)
            != 0)
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
