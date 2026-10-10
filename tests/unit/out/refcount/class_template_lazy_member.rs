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
pub struct Point {
    #[offset(0)]
    pub x: i32,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Box_int_ {
    #[offset(0)]
    pub val: i32,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Box_Point_ {
    #[offset(0)]
    #[byte_size(4)]
    pub val: Point,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Deduced_int_ {
    #[offset(0)]
    pub val: i32,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Deduced_Point_ {
    #[offset(0)]
    #[byte_size(4)]
    pub val: Point,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let i: Value<Box_int_> = Rc::new(RefCell::new(Box_int_ { val: 3 }));
    assert!((({ Box_int_Impl::twice(&i.as_pointer(),) }) == 6));
    let p: Value<Box_Point_> = Rc::new(RefCell::new(Box_Point_ {
        val: Point { x: 4 },
    }));
    assert!(({ ({ Box_Point_Impl::get(&p.as_pointer(),) }).x } == 4));
    let d: Value<Deduced_int_> = Rc::new(RefCell::new(Deduced_int_ { val: 5 }));
    assert!((({ Deduced_int_Impl::twice(&d.as_pointer(),) }) == 10));
    let dp: Value<Deduced_Point_> = Rc::new(RefCell::new(Deduced_Point_ {
        val: Point { x: 6 },
    }));
    assert!(({ ({ Deduced_Point_Impl::get(&dp.as_pointer(),) }).x } == 6));
    return 0;
}
pub trait Box_Point_Impl {
    fn get(&self) -> Point;
    fn twice(&self) -> Point {
        unimplemented!()
    }
}
impl Box_Point_Impl for Ptr<Box_Point_> {
    fn get(&self) -> Point {
        return (*self).with(|__s| __s.val.clone());
    }
}
pub trait Box_int_Impl {
    fn get(&self) -> i32 {
        unimplemented!()
    }
    fn twice(&self) -> i32;
}
impl Box_int_Impl for Ptr<Box_int_> {
    fn twice(&self) -> i32 {
        return ((*self).with(|__s| __s.val) + (*self).with(|__s| __s.val));
    }
}
pub trait Deduced_Point_Impl {
    fn get(&self) -> Point;
}
impl Deduced_Point_Impl for Ptr<Deduced_Point_> {
    fn get(&self) -> Point {
        return (*self).with(|__s| __s.val.clone());
    }
}
pub trait Deduced_int_Impl {
    fn twice(&self) -> i32;
}
impl Deduced_int_Impl for Ptr<Deduced_int_> {
    fn twice(&self) -> i32 {
        return ((*self).with(|__s| __s.val) + (*self).with(|__s| __s.val));
    }
}
pub fn __cpp2rust_init_globals() {}
