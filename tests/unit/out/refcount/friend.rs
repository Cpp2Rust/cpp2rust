extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn get_0(v: Ptr<V>) -> i32 {
    return v.with(|__s| __s.x);
}
pub fn operator_eq_1(a: Ptr<V>, b: Ptr<V>) -> bool {
    return ({ a.with(|__s| __s.x) } == { b.with(|__s| __s.x) });
}
pub fn scaled_2(v: Ptr<V>, mut k: i32) -> i32 {
    return ({ v.with(|__s| __s.x) } * { k });
}
pub fn scaled_3(v: Ptr<V>, mut k: f64) -> f64 {
    return ({ (v.with(|__s| __s.x) as f64) } * { k });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct V {
    #[offset(0)]
    pub x: i32,
}
impl std::cmp::PartialEq for V {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_1(
                Rc::new(RefCell::new(V { x: self.x.clone() })).as_pointer(),
                Rc::new(RefCell::new(V { x: other.x.clone() })).as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for V {}
pub fn get_4(w: Ptr<W_int_>) -> i32 {
    return w.with(|__s| __s.x);
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct W_int_ {
    #[offset(0)]
    pub x: i32,
}
pub fn get_5(w: Ptr<W_long_>) -> i64 {
    return w.with(|__s| __s.x);
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct W_long_ {
    #[offset(0)]
    pub x: i64,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct D {
    #[offset(0)]
    pub x: i32,
}
pub fn declared_then_defined_6(d: Ptr<D>) -> i32 {
    return (d.with(|__s| __s.x) + 1);
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let a: Value<V> = Rc::new(RefCell::new(V { x: 3 }));
    let b: Value<V> = Rc::new(RefCell::new(V { x: 3 }));
    let c: Value<V> = Rc::new(RefCell::new(V { x: 4 }));
    assert!((({ get_0(a.as_pointer(),) }) == 3));
    assert!(
        ({
            let _a: Ptr<V> = a.as_pointer();
            operator_eq_1(_a, b.as_pointer())
        })
    );
    assert!(
        !({
            let _a: Ptr<V> = a.as_pointer();
            operator_eq_1(_a, c.as_pointer())
        })
    );
    assert!((({ scaled_2(c.as_pointer(), 2,) }) == 8));
    assert!((({ scaled_3(c.as_pointer(), 1.5_f64,) }) == 6_f64));
    let wi: Value<W_int_> = Rc::new(RefCell::new(W_int_ { x: 5 }));
    let wl: Value<W_long_> = Rc::new(RefCell::new(W_long_ { x: 6_i64 }));
    assert!((({ get_4(wi.as_pointer(),) }) == 5));
    assert!((({ get_5(wl.as_pointer(),) }) == 6_i64));
    let d: Value<D> = Rc::new(RefCell::new(D { x: 7 }));
    assert!((({ declared_then_defined_6(d.as_pointer(),) }) == 8));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
