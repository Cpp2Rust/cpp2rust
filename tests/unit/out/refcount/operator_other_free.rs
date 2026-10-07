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
pub fn operator_comma_0(a: Ptr<S>, b: Ptr<S>) -> S {
    return S {
        v: ({ (a.with(|__s| __s.v) * 10) } + { b.with(|__s| __s.v) }),
    };
}
pub fn operator_literal__k_1(mut v: u64) -> i64 {
    return (((v).wrapping_mul(1000_u64)) as i64);
}
pub fn operator_literal__half_2(mut v: f64) -> f64 {
    return (v / 2_f64);
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let s: Value<S> = Rc::new(RefCell::new(S { v: 3 }));
    let t: Value<S> = Rc::new(RefCell::new(S { v: 4 }));
    assert!(
        ({
            ({
                let _a: Ptr<S> = s.as_pointer();
                operator_comma_0(_a, t.as_pointer())
            })
            .v
        } == 34)
    );
    assert!(
        ({
            ({
                let _a: Value<S> = Rc::new(RefCell::new(
                    ({
                        let _a: Ptr<S> = s.as_pointer();
                        operator_comma_0(_a, t.as_pointer())
                    }),
                ));
                let _b: Ptr<S> = s.as_pointer();
                operator_comma_0(_a.as_pointer(), _b)
            })
            .v
        } == 343)
    );
    assert!((({ operator_literal__k_1(2_u64,) }) == 2000_i64));
    assert!((({ operator_literal__half_2(3.0E+0,) }) == 1.5E+0));
    assert!((({ operator_literal__k_1(4_u64,) }) == 4000_i64));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
