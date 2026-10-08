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
pub struct S {
    #[offset(0)]
    pub v: i32,
}
pub fn operator_eq_0(mut a: i32, b: Ptr<S>) -> bool {
    return ({ a } == { b.with(|__s| __s.v) });
}
pub fn operator_ne_1(mut a: i32, b: Ptr<S>) -> bool {
    return ({ a } != { b.with(|__s| __s.v) });
}
pub fn operator_lt_2(mut a: i32, b: Ptr<S>) -> bool {
    return ({ a } < { b.with(|__s| __s.v) });
}
pub fn operator_gt_3(mut a: f64, b: Ptr<S>) -> bool {
    return ({ a } > { (b.with(|__s| __s.v) as f64) });
}
pub fn operator_le_4(mut a: i64, b: Ptr<S>) -> bool {
    return ({ a } <= { (b.with(|__s| __s.v) as i64) });
}
pub fn operator_ge_5(mut a: Ptr<i8>, b: Ptr<S>) -> bool {
    return ({ (((a.read()) as i32) - (('0' as i8) as i32)) } >= { b.with(|__s| __s.v) });
}
pub fn operator_lt_6(a: Ptr<S>, mut b: i32) -> bool {
    return ({ (a.with(|__s| __s.v) + 1) } < { b });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let s: Value<S> = Rc::new(RefCell::new(S { v: 5 }));
    let cs: Ptr<S> = s.as_pointer();
    assert!(({ SImpl::operator_eq(&cs, 5,) }));
    assert!(({ SImpl::operator_ne(&cs, 4,) }));
    assert!(({ SImpl::operator_lt(&cs, 6,) }));
    assert!(({ SImpl::operator_gt(&cs, 4.5_f64,) }));
    assert!(({ SImpl::operator_le(&cs, 5_i64,) }));
    assert!(({ SImpl::operator_ge(&cs, Ptr::<i8>::from_string_literal(b"3"),) }));
    assert!(({ operator_eq_0(5, s.as_pointer(),) }));
    assert!(({ operator_ne_1(4, s.as_pointer(),) }));
    assert!(({ operator_lt_2(4, s.as_pointer(),) }));
    assert!(({ operator_gt_3(5.5_f64, s.as_pointer(),) }));
    assert!(({ operator_le_4(5_i64, s.as_pointer(),) }));
    assert!(({ operator_ge_5(Ptr::<i8>::from_string_literal(b"7"), s.as_pointer(),) }));
    assert!(
        ({
            let _a: Ptr<S> = s.as_pointer();
            operator_lt_6(_a, 7)
        })
    );
    assert!(
        !({
            let _a: Ptr<S> = s.as_pointer();
            operator_lt_6(_a, 6)
        })
    );
    return 0;
}
pub trait SImpl {
    fn operator_eq(&self, o: i32) -> bool;
    fn operator_ne(&self, o: i32) -> bool;
    fn operator_lt(&self, o: i32) -> bool;
    fn operator_gt(&self, o: f64) -> bool;
    fn operator_le(&self, o: i64) -> bool;
    fn operator_ge(&self, o: Ptr<i8>) -> bool;
}
impl SImpl for Ptr<S> {
    fn operator_eq(&self, mut o: i32) -> bool {
        return ((*self).with(|__s| __s.v) == o);
    }
    fn operator_ne(&self, mut o: i32) -> bool {
        return ((*self).with(|__s| __s.v) != o);
    }
    fn operator_lt(&self, mut o: i32) -> bool {
        return ((*self).with(|__s| __s.v) < o);
    }
    fn operator_gt(&self, mut o: f64) -> bool {
        return (((*self).with(|__s| __s.v) as f64) > o);
    }
    fn operator_le(&self, mut o: i64) -> bool {
        return (((*self).with(|__s| __s.v) as i64) <= o);
    }
    fn operator_ge(&self, mut o: Ptr<i8>) -> bool {
        return ({ (*self).with(|__s| __s.v) } >= { (((o.read()) as i32) - (('0' as i8) as i32)) });
    }
}
pub fn __cpp2rust_init_globals() {}
