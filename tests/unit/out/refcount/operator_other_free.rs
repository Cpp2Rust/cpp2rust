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
pub fn operator_comma_0(a: Ptr<S>, b: Ptr<S>) -> S {
    return S {
        v: {
            let _lhs = (a.with(|__s| __s.v) * 10);
            _lhs + b.with(|__s| __s.v)
        },
    };
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
    return 0;
}
pub fn __cpp2rust_init_globals() {}
