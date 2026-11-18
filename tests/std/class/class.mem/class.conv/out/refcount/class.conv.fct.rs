extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Copy, Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_into(i64, to_i64, mut_ptr)]
#[byte_size(8)]
pub struct p2_non_const_conversion_function_Gauge {
    #[offset(0)]
    pub value: i64,
}
pub fn test_0() {
    let gauge: Value<p2_non_const_conversion_function_Gauge> =
        Rc::new(RefCell::new(p2_non_const_conversion_function_Gauge {
            value: 20_i64,
        }));
    let mut converted: i64 =
        ({ p2_non_const_conversion_function_GaugeImpl::to_i64(&gauge.as_pointer()) });
    assert!((converted == 21_i64));
    assert!(({ ({ (*gauge.borrow()).value } == 21_i64) }));
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    ({ test_0() });
    return 0;
}
pub trait p2_non_const_conversion_function_GaugeImpl {
    fn to_i64(&self) -> i64;
}
impl p2_non_const_conversion_function_GaugeImpl for Ptr<p2_non_const_conversion_function_Gauge> {
    fn to_i64(&self) -> i64 {
        return field!((*self), value).with_mut(|__v| __v.prefix_inc().clone());
    }
}
pub fn __cpp2rust_init_globals() {}
