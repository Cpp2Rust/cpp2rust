extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(VaArg, FnPtrArg, ByteRepr, Clone)]
#[byte_size(8)]
pub struct anon_0 {
    #[offset(0)]
    #[byte_size(8)]
    __bytes: Box<[u8]>,
}
impl anon_0 {
    pub fn namespace_count(this: Ptr<Self>) -> Ptr<i32> {
        this.reinterpret_cast()
    }
    pub fn namespace_ratio(this: Ptr<Self>) -> Ptr<f64> {
        this.reinterpret_cast()
    }
}
impl Default for anon_0 {
    fn default() -> Self {
        anon_0 {
            __bytes: Box::from([0u8; 8]),
        }
    }
}
#[derive(VaArg, FnPtrArg, ByteRepr, Clone)]
#[byte_size(8)]
pub struct anon_1 {
    #[offset(0)]
    #[byte_size(8)]
    __bytes: Box<[u8]>,
}
impl anon_1 {
    pub fn member_count(this: Ptr<Self>) -> Ptr<i32> {
        this.reinterpret_cast()
    }
    pub fn member_ratio(this: Ptr<Self>) -> Ptr<f64> {
        this.reinterpret_cast()
    }
}
impl Default for anon_1 {
    fn default() -> Self {
        anon_1 {
            __bytes: Box::from([0u8; 8]),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct HasAnonymousUnion {
    #[offset(0)]
    pub tag: i32,
    #[offset(8)]
    #[byte_size(8)]
    pub anon_1: anon_1,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    anon_0::namespace_count(anon_0.with(|v| v.as_pointer())).write(4);
    assert!(((anon_0::namespace_count(anon_0.with(|v| v.as_pointer())).read()) == 4));
    anon_0::namespace_ratio(anon_0.with(|v| v.as_pointer())).write(0.25_f64);
    assert!(((anon_0::namespace_ratio(anon_0.with(|v| v.as_pointer())).read()) == 0.25_f64));
    let object: Value<HasAnonymousUnion> = Rc::new(RefCell::new(<HasAnonymousUnion>::default()));
    (*object.borrow_mut()).tag = 1;
    anon_1::member_count(field_ptr!(object.as_pointer(), anon_1)).write(6);
    assert!(((anon_1::member_count(field_ptr!(object.as_pointer(), anon_1)).read()) == 6));
    assert!((({ HasAnonymousUnionImpl::read_count(&object.as_pointer(),) }) == 6));
    anon_1::member_ratio(field_ptr!(object.as_pointer(), anon_1)).write(0.5_f64);
    assert!(((anon_1::member_ratio(field_ptr!(object.as_pointer(), anon_1)).read()) == 0.5_f64));
    assert!(({ ({ (*object.borrow()).tag } == 1) }));
    let anon_2: Value<anon_2> = Rc::new(RefCell::new(<anon_2>::default()));
    anon_2::block_count(anon_2.as_pointer()).write(9);
    assert!(((anon_2::block_count(anon_2.as_pointer()).read()) == 9));
    anon_2::block_ratio(anon_2.as_pointer()).write(0.75_f64);
    assert!(((anon_2::block_ratio(anon_2.as_pointer()).read()) == 0.75_f64));
    return 0;
}
#[derive(VaArg, FnPtrArg, ByteRepr, Clone)]
#[byte_size(8)]
pub struct anon_2 {
    #[offset(0)]
    #[byte_size(8)]
    __bytes: Box<[u8]>,
}
impl anon_2 {
    pub fn block_count(this: Ptr<Self>) -> Ptr<i32> {
        this.reinterpret_cast()
    }
    pub fn block_ratio(this: Ptr<Self>) -> Ptr<f64> {
        this.reinterpret_cast()
    }
}
impl Default for anon_2 {
    fn default() -> Self {
        anon_2 {
            __bytes: Box::from([0u8; 8]),
        }
    }
}
pub trait HasAnonymousUnionImpl {
    fn read_count(&self) -> i32;
}
impl HasAnonymousUnionImpl for Ptr<HasAnonymousUnion> {
    fn read_count(&self) -> i32 {
        return (anon_1::member_count(field_ptr!((*self), anon_1)).read());
    }
}
pub fn __cpp2rust_init_globals() {}
