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
    pub v: u32,
}
pub fn operator_bitnot_0(a: Ptr<S>) -> S {
    return S {
        v: !a.with(|__s| __s.v),
    };
}
pub fn operator_bitand_1(a: Ptr<S>, b: Ptr<S>) -> S {
    return S {
        v: ({ a.with(|__s| __s.v) } & { b.with(|__s| __s.v) }),
    };
}
pub fn operator_bitor_2(a: Ptr<S>, b: Ptr<S>) -> S {
    return S {
        v: ({ a.with(|__s| __s.v) } | { b.with(|__s| __s.v) }),
    };
}
pub fn operator_bitxor_3(a: Ptr<S>, b: Ptr<S>) -> S {
    return S {
        v: ({ a.with(|__s| __s.v) } ^ { b.with(|__s| __s.v) }),
    };
}
pub fn operator_shl_4(a: Ptr<S>, n: i32) -> S {
    let n: Value<i32> = Rc::new(RefCell::new(n));
    return S {
        v: ({ a.with(|__s| __s.v) } << { (*n.borrow()) }),
    };
}
pub fn operator_shr_5(a: Ptr<S>, n: i32) -> S {
    let n: Value<i32> = Rc::new(RefCell::new(n));
    return S {
        v: ({ a.with(|__s| __s.v) } >> { (*n.borrow()) }),
    };
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let a: Value<S> = Rc::new(RefCell::new(S { v: 12_u32 }));
    let b: Value<S> = Rc::new(RefCell::new(S { v: 10_u32 }));
    assert!(
        ({
            ({
                let _a: Ptr<S> = a.as_pointer();
                operator_bitnot_0(_a)
            })
            .v
        } == !12_u32)
    );
    assert!(
        ({
            ({
                let _a: Ptr<S> = a.as_pointer();
                operator_bitand_1(_a, b.as_pointer())
            })
            .v
        } == 8_u32)
    );
    assert!(
        ({
            ({
                let _a: Ptr<S> = a.as_pointer();
                operator_bitor_2(_a, b.as_pointer())
            })
            .v
        } == 14_u32)
    );
    assert!(
        ({
            ({
                let _a: Ptr<S> = a.as_pointer();
                operator_bitxor_3(_a, b.as_pointer())
            })
            .v
        } == 6_u32)
    );
    assert!(
        ({
            ({
                let _a: Ptr<S> = a.as_pointer();
                operator_shl_4(_a, 2)
            })
            .v
        } == 48_u32)
    );
    assert!(
        ({
            ({
                let _a: Ptr<S> = a.as_pointer();
                operator_shr_5(_a, 2)
            })
            .v
        } == 3_u32)
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
