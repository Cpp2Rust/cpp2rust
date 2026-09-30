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
    pub v: u32,
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
            v: <u32>::from_bytes(&buf[0..4]),
        }
    }
}
pub fn operator_add_assign_0(a: Ptr<S>, b: Ptr<S>) -> Ptr<S> {
    {
        let rhs_0 = (a.with(|__s| __s.v)).wrapping_add(b.with(|__s| __s.v));
        field!(a, v).write(rhs_0)
    };
    return (a).clone();
}
pub fn operator_sub_assign_1(a: Ptr<S>, b: Ptr<S>) -> Ptr<S> {
    {
        let rhs_0 = (a.with(|__s| __s.v)).wrapping_sub(b.with(|__s| __s.v));
        field!(a, v).write(rhs_0)
    };
    return (a).clone();
}
pub fn operator_mul_assign_2(a: Ptr<S>, b: Ptr<S>) -> Ptr<S> {
    {
        let rhs_0 = (a.with(|__s| __s.v)).wrapping_mul(b.with(|__s| __s.v));
        field!(a, v).write(rhs_0)
    };
    return (a).clone();
}
pub fn operator_div_assign_3(a: Ptr<S>, b: Ptr<S>) -> Ptr<S> {
    {
        let rhs_0 = (a.with(|__s| __s.v)).wrapping_div(b.with(|__s| __s.v));
        field!(a, v).write(rhs_0)
    };
    return (a).clone();
}
pub fn operator_rem_assign_4(a: Ptr<S>, b: Ptr<S>) -> Ptr<S> {
    {
        let rhs_0 = (a.with(|__s| __s.v)).wrapping_rem(b.with(|__s| __s.v));
        field!(a, v).write(rhs_0)
    };
    return (a).clone();
}
pub fn operator_bitand_assign_5(a: Ptr<S>, b: Ptr<S>) -> Ptr<S> {
    let __rhs = b.with(|__s| __s.v);
    {
        let _ptr = field!(a, v);
        _ptr.write(_ptr.read() & __rhs)
    };
    return (a).clone();
}
pub fn operator_bitor_assign_6(a: Ptr<S>, b: Ptr<S>) -> Ptr<S> {
    let __rhs = b.with(|__s| __s.v);
    {
        let _ptr = field!(a, v);
        _ptr.write(_ptr.read() | __rhs)
    };
    return (a).clone();
}
pub fn operator_bitxor_assign_7(a: Ptr<S>, b: Ptr<S>) -> Ptr<S> {
    let __rhs = b.with(|__s| __s.v);
    {
        let _ptr = field!(a, v);
        _ptr.write(_ptr.read() ^ __rhs)
    };
    return (a).clone();
}
pub fn operator_shl_assign_8(a: Ptr<S>, n: i32) -> Ptr<S> {
    let n: Value<i32> = Rc::new(RefCell::new(n));
    {
        let _ptr = field!(a, v);
        _ptr.write(_ptr.read() << (*n.borrow()))
    };
    return (a).clone();
}
pub fn operator_shr_assign_9(a: Ptr<S>, n: i32) -> Ptr<S> {
    let n: Value<i32> = Rc::new(RefCell::new(n));
    {
        let _ptr = field!(a, v);
        _ptr.write(_ptr.read() >> (*n.borrow()))
    };
    return (a).clone();
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let a: Value<S> = Rc::new(RefCell::new(S { v: 6_u32 }));
    let b: Value<S> = Rc::new(RefCell::new(S { v: 4_u32 }));
    ({
        let _a: Ptr<S> = a.as_pointer();
        operator_add_assign_0(_a, b.as_pointer())
    });
    assert!(({ (*a.borrow()).v } == 10_u32));
    ({
        let _a: Ptr<S> = a.as_pointer();
        operator_sub_assign_1(_a, b.as_pointer())
    });
    assert!(({ (*a.borrow()).v } == 6_u32));
    ({
        let _a: Ptr<S> = a.as_pointer();
        operator_mul_assign_2(_a, b.as_pointer())
    });
    assert!(({ (*a.borrow()).v } == 24_u32));
    ({
        let _a: Ptr<S> = a.as_pointer();
        operator_div_assign_3(_a, b.as_pointer())
    });
    assert!(({ (*a.borrow()).v } == 6_u32));
    ({
        let _a: Ptr<S> = a.as_pointer();
        operator_rem_assign_4(_a, b.as_pointer())
    });
    assert!(({ (*a.borrow()).v } == 2_u32));
    ({
        let _a: Ptr<S> = a.as_pointer();
        operator_bitor_assign_6(_a, b.as_pointer())
    });
    assert!(({ (*a.borrow()).v } == 6_u32));
    ({
        let _a: Ptr<S> = a.as_pointer();
        operator_bitand_assign_5(_a, b.as_pointer())
    });
    assert!(({ (*a.borrow()).v } == 4_u32));
    ({
        let _a: Ptr<S> = a.as_pointer();
        operator_bitxor_assign_7(_a, b.as_pointer())
    });
    assert!(({ (*a.borrow()).v } == 0_u32));
    (*a.borrow_mut()).v = 3_u32;
    ({
        let _a: Ptr<S> = a.as_pointer();
        operator_shl_assign_8(_a, 2)
    });
    assert!(({ (*a.borrow()).v } == 12_u32));
    ({
        let _a: Ptr<S> = a.as_pointer();
        operator_shr_assign_9(_a, 1)
    });
    assert!(({ (*a.borrow()).v } == 6_u32));
    ({
        let _a: Ptr<S> = ({
            let _a: Ptr<S> = a.as_pointer();
            operator_add_assign_0(_a, b.as_pointer())
        });
        let _b: Ptr<S> = b.as_pointer();
        operator_add_assign_0(_a, _b)
    });
    assert!(({ (*a.borrow()).v } == 14_u32));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
