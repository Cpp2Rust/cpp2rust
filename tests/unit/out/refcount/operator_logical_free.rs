extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, VaArg, FnPtrArg, Default)]
pub struct S {
    #[offset(0)]
    pub v: i32,
}
impl ByteRepr for S {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.v.to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            v: <i32>::from_bytes(&buf[0..4]),
        }
    }
}
pub fn operator_not_0(a: Ptr<S>) -> bool {
    return (a.with(|__s| __s.v) == 0);
}
pub fn operator_and_1(a: Ptr<S>, b: Ptr<S>) -> bool {
    return (a.with(|__s| __s.v) != 0) && (b.with(|__s| __s.v) != 0);
}
pub fn operator_or_2(a: Ptr<S>, b: Ptr<S>) -> bool {
    return (a.with(|__s| __s.v) != 0) || (b.with(|__s| __s.v) != 0);
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let t: Value<S> = Rc::new(RefCell::new(S { v: 1 }));
    let f: Value<S> = Rc::new(RefCell::new(S { v: 0 }));
    assert!(
        ({
            let _a: Ptr<S> = f.as_pointer();
            operator_not_0(_a)
        })
    );
    assert!(
        !({
            let _a: Ptr<S> = t.as_pointer();
            operator_not_0(_a)
        })
    );
    assert!(
        ({
            let _a: Ptr<S> = t.as_pointer();
            let _b: Ptr<S> = t.as_pointer();
            operator_and_1(_a, _b)
        })
    );
    assert!(
        !({
            let _a: Ptr<S> = t.as_pointer();
            operator_and_1(_a, f.as_pointer())
        })
    );
    assert!(
        ({
            let _a: Ptr<S> = t.as_pointer();
            operator_or_2(_a, f.as_pointer())
        })
    );
    assert!(
        !({
            let _a: Ptr<S> = f.as_pointer();
            let _b: Ptr<S> = f.as_pointer();
            operator_or_2(_a, _b)
        })
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
