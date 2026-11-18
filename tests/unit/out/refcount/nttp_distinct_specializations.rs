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
pub struct S_2_ {
    #[offset(0)]
    pub v: i32,
    #[offset(4)]
    pub w: i32,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct S_1_ {
    #[offset(0)]
    pub v: i32,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct S_3_ {
    #[offset(0)]
    pub v: i32,
}
pub fn scale_0(mut x: i32) -> i32 {
    return (x * 1);
}
pub fn scale_1(mut x: i32) -> i32 {
    return (x * 3);
}
pub fn scale_2(mut x: i32) -> i32 {
    return (x - 2);
}
pub fn get_float_3() -> f32 {
    return f32::from_bits(0x7fc00000);
}
pub fn get_float_4() -> f32 {
    return f32::NEG_INFINITY;
}
pub fn get_double_5() -> f64 {
    return f64::from_bits(0x7ff8000000000005);
}
pub fn get_double_6() -> f64 {
    return f64::NEG_INFINITY;
}
pub fn get_long_double_7() -> f64 {
    return f64::from_bits(0x7ff8000000000000);
}
pub fn get_long_double_8() -> f64 {
    return f64::NEG_INFINITY;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let s1: Value<S_1_> = Rc::new(RefCell::new(S_1_ { v: 5 }));
    let s2: Value<S_2_> = Rc::new(RefCell::new(S_2_ { v: 5, w: 1 }));
    let s3: Value<S_3_> = Rc::new(RefCell::new(S_3_ { v: 5 }));
    assert!((({ S_1_Impl::get(&s1.as_pointer(),) }) == 5));
    assert!((({ S_2_Impl::get(&s2.as_pointer(),) }) == 4));
    assert!((({ S_3_Impl::get(&s3.as_pointer(),) }) == 15));
    assert!((({ scale_0(7,) }) == 7));
    assert!((({ scale_2(7,) }) == 5));
    assert!((({ scale_1(7,) }) == 21));
    let mut f_nan: f32 = ({ get_float_3() });
    let mut f_inf: f32 = ({ get_float_4() });
    assert!((f_nan != f_nan));
    assert!((f_inf < 0_f32));
    let mut d_nan: f64 = ({ get_double_5() });
    let mut d_inf: f64 = ({ get_double_6() });
    assert!((d_nan != d_nan));
    assert!((d_inf < 0_f64));
    let mut ld_nan: f64 = ({ get_long_double_7() });
    let mut ld_inf: f64 = ({ get_long_double_8() });
    assert!((ld_nan != ld_nan));
    assert!((ld_inf < 0_f64));
    return 0;
}
pub trait S_1_Impl {
    fn get(&self) -> i32;
}
impl S_1_Impl for Ptr<S_1_> {
    fn get(&self) -> i32 {
        return ((*self).with(|__s| __s.v) * 1);
    }
}
pub trait S_2_Impl {
    fn get(&self) -> i32;
}
impl S_2_Impl for Ptr<S_2_> {
    fn get(&self) -> i32 {
        return ((*self).with(|__s| __s.v) - (*self).with(|__s| __s.w));
    }
}
pub trait S_3_Impl {
    fn get(&self) -> i32;
}
impl S_3_Impl for Ptr<S_3_> {
    fn get(&self) -> i32 {
        return ((*self).with(|__s| __s.v) * 3);
    }
}
pub fn __cpp2rust_init_globals() {}
