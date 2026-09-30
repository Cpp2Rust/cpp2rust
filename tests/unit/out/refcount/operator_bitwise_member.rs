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
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let a: Value<S> = Rc::new(RefCell::new(S { v: 12_u32 }));
    let b: Value<S> = Rc::new(RefCell::new(S { v: 10_u32 }));
    assert!(({ ({ SImpl::operator_bitnot(&a.as_pointer(),) }).v } == !12_u32));
    assert!(({ ({ SImpl::operator_bitand(&a.as_pointer(), b.as_pointer(),) }).v } == 8_u32));
    assert!(({ ({ SImpl::operator_bitor(&a.as_pointer(), b.as_pointer(),) }).v } == 14_u32));
    assert!(({ ({ SImpl::operator_bitxor(&a.as_pointer(), b.as_pointer(),) }).v } == 6_u32));
    assert!(({ ({ SImpl::operator_shl(&a.as_pointer(), 2,) }).v } == 48_u32));
    assert!(({ ({ SImpl::operator_shr(&a.as_pointer(), 2,) }).v } == 3_u32));
    return 0;
}
pub trait SImpl {
    fn operator_bitnot(&self) -> S;
    fn operator_bitand(&self, o: Ptr<S>) -> S;
    fn operator_bitor(&self, o: Ptr<S>) -> S;
    fn operator_bitxor(&self, o: Ptr<S>) -> S;
    fn operator_shl(&self, n: i32) -> S;
    fn operator_shr(&self, n: i32) -> S;
}
impl SImpl for Ptr<S> {
    fn operator_bitnot(&self) -> S {
        return S {
            v: !(*self).with(|__s| __s.v),
        };
    }
    fn operator_bitand(&self, o: Ptr<S>) -> S {
        return S {
            v: ({ (*self).with(|__s| __s.v) } & { o.with(|__s| __s.v) }),
        };
    }
    fn operator_bitor(&self, o: Ptr<S>) -> S {
        return S {
            v: ({ (*self).with(|__s| __s.v) } | { o.with(|__s| __s.v) }),
        };
    }
    fn operator_bitxor(&self, o: Ptr<S>) -> S {
        return S {
            v: ({ (*self).with(|__s| __s.v) } ^ { o.with(|__s| __s.v) }),
        };
    }
    fn operator_shl(&self, n: i32) -> S {
        let n: Value<i32> = Rc::new(RefCell::new(n));
        return S {
            v: ((*self).with(|__s| __s.v) << (*n.borrow())),
        };
    }
    fn operator_shr(&self, n: i32) -> S {
        let n: Value<i32> = Rc::new(RefCell::new(n));
        return S {
            v: ((*self).with(|__s| __s.v) >> (*n.borrow())),
        };
    }
}
pub fn __cpp2rust_init_globals() {}
