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
pub struct HasConstOverload {
    #[offset(0)]
    pub plain_calls: i32,
    #[offset(4)]
    pub const_calls: i32,
}
impl Default for HasConstOverload {
    fn default() -> Self {
        {
            HasConstOverload {
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
    let object: Value<HasConstOverload> = Rc::new(RefCell::new(<HasConstOverload>::default()));
    let view: Ptr<HasConstOverload> = object.as_pointer();
    assert!((({ HasConstOverloadImpl::get_1(&object.as_pointer(),) }) == 1));
    assert!(({ ({ (*object.borrow()).plain_calls } == 1) }));
    assert!(({ ({ (*object.borrow()).const_calls } == 0) }));
    assert!((({ HasConstOverloadImpl::get_2(&view,) }) == 2));
    assert!(({ ({ (*object.borrow()).plain_calls } == 1) }));
    assert!(({ ({ (*object.borrow()).const_calls } == 1) }));
    return 0;
}
pub trait HasConstOverloadImpl {
    fn get_1(&self) -> i32;
    fn get_2(&self) -> i32;
}
impl HasConstOverloadImpl for Ptr<HasConstOverload> {
    fn get_1(&self) -> i32 {
        field!((*self), plain_calls).with_mut(|__v| __v.prefix_inc());
        return 1;
    }
    fn get_2(&self) -> i32 {
        field!((*self), const_calls).with_mut(|__v| __v.prefix_inc());
        return 2;
    }
}
pub fn __cpp2rust_init_globals() {}
