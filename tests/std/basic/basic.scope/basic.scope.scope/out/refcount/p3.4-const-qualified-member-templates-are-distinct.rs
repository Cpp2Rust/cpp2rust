extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct HasConstTemplateOverload {
    #[offset(0)]
    pub plain_calls: i32,
    #[offset(4)]
    pub const_calls: i32,
}
impl Default for HasConstTemplateOverload {
    fn default() -> Self {
        {
            HasConstTemplateOverload {
                plain_calls: 0,
                const_calls: 0,
            }
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let object: Value<HasConstTemplateOverload> =
        Rc::new(RefCell::new(<HasConstTemplateOverload>::default()));
    let view: Ptr<HasConstTemplateOverload> = object.as_pointer();
    assert!((({ HasConstTemplateOverloadImpl::get_1_int(&object.as_pointer(), 10,) }) == 11));
    assert!(({ ({ (*object.borrow()).plain_calls } == 1) }));
    assert!(({ ({ (*object.borrow()).const_calls } == 0) }));
    assert!((({ HasConstTemplateOverloadImpl::get_2_int(&view, 10,) }) == 12));
    assert!(({ ({ (*object.borrow()).plain_calls } == 1) }));
    assert!(({ ({ (*object.borrow()).const_calls } == 1) }));
    return 0;
}
pub trait HasConstTemplateOverloadImpl {
    fn get_1_int(&self, value: i32) -> i32;
    fn get_2_int(&self, value: i32) -> i32;
}
impl HasConstTemplateOverloadImpl for Ptr<HasConstTemplateOverload> {
    fn get_1_int(&self, mut value: i32) -> i32 {
        field!((*self), plain_calls).with_mut(|__v| __v.prefix_inc());
        return (value + 1);
    }
    fn get_2_int(&self, mut value: i32) -> i32 {
        field!((*self), const_calls).with_mut(|__v| __v.prefix_inc());
        return (value + 2);
    }
}
pub fn __cpp2rust_init_globals() {}
