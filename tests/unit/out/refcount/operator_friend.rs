extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn operator_eq_0(_a0: Ptr<Defaulted>, _a1: Ptr<Defaulted>) -> bool {
    return ({
        let _lhs = _a0.with(|__s| __s.a);
        _lhs == _a1.with(|__s| __s.a)
    }) && ({
        let _lhs = _a0.with(|__s| __s.b);
        _lhs == _a1.with(|__s| __s.b)
    });
}
#[derive(Clone, Record, VaArg, FnPtrArg, Default)]
pub struct Defaulted {
    #[offset(0)]
    pub a: i32,
    #[offset(4)]
    pub b: i32,
}
impl std::cmp::PartialEq for Defaulted {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_0(
                Rc::new(RefCell::new(Defaulted {
                    a: self.a.clone(),
                    b: self.b.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(Defaulted {
                    a: other.a.clone(),
                    b: other.b.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for Defaulted {}
impl ByteRepr for Defaulted {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.a.to_bytes(&mut buf[0..4]);
        self.b.to_bytes(&mut buf[4..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            a: <i32>::from_bytes(&buf[0..4]),
            b: <i32>::from_bytes(&buf[4..8]),
        }
    }
}
pub fn operator_cmp_1(_a0: Ptr<DefaultedOrd>, _a1: Ptr<DefaultedOrd>) -> std::cmp::Ordering {
    {
        let cmp: Value<std::cmp::Ordering> = Rc::new(RefCell::new(std::cmp::Ord::cmp(
            &(_a0.with(|__s| __s.a)),
            &(_a1.with(|__s| __s.a)),
        )));
        if !((*cmp.borrow()) == std::cmp::Ordering::Equal) {
            return (*cmp.borrow_mut()).clone();
        }
    }
    return std::cmp::Ordering::Equal;
}
pub fn operator_eq_2(_a0: Ptr<DefaultedOrd>, _a1: Ptr<DefaultedOrd>) -> bool {
    return {
        let _lhs = _a0.with(|__s| __s.a);
        _lhs == _a1.with(|__s| __s.a)
    };
}
#[derive(Clone, Record, VaArg, FnPtrArg, Default)]
pub struct DefaultedOrd {
    #[offset(0)]
    pub a: i32,
}
impl std::cmp::Ord for DefaultedOrd {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            operator_cmp_1(
                Rc::new(RefCell::new(DefaultedOrd { a: self.a.clone() })).as_pointer(),
                Rc::new(RefCell::new(DefaultedOrd { a: other.a.clone() })).as_pointer(),
            )
        }
    }
}
impl std::cmp::PartialOrd for DefaultedOrd {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for DefaultedOrd {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_2(
                Rc::new(RefCell::new(DefaultedOrd { a: self.a.clone() })).as_pointer(),
                Rc::new(RefCell::new(DefaultedOrd { a: other.a.clone() })).as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for DefaultedOrd {}
impl ByteRepr for DefaultedOrd {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.a.to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            a: <i32>::from_bytes(&buf[0..4]),
        }
    }
}
pub fn operator_eq_3(x: Ptr<Inline>, y: Ptr<Inline>) -> bool {
    return {
        let _lhs = x.with(|__s| __s.a);
        _lhs == y.with(|__s| __s.a)
    };
}
pub fn operator_lt_4(x: Ptr<Inline>, y: Ptr<Inline>) -> bool {
    return {
        let _lhs = x.with(|__s| __s.a);
        _lhs < y.with(|__s| __s.a)
    };
}
pub fn operator_add_5(x: Ptr<Inline>, y: Ptr<Inline>) -> Inline {
    return Inline {
        a: {
            let _lhs = x.with(|__s| __s.a);
            _lhs + y.with(|__s| __s.a)
        },
    };
}
#[derive(Clone, Record, VaArg, FnPtrArg, Default)]
pub struct Inline {
    #[offset(0)]
    pub a: i32,
}
impl std::cmp::Ord for Inline {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            if operator_lt_4(
                Rc::new(RefCell::new(Inline { a: self.a.clone() })).as_pointer(),
                Rc::new(RefCell::new(Inline { a: other.a.clone() })).as_pointer(),
            ) {
                std::cmp::Ordering::Less
            } else if operator_lt_4(
                Rc::new(RefCell::new(Inline { a: other.a.clone() })).as_pointer(),
                Rc::new(RefCell::new(Inline { a: self.a.clone() })).as_pointer(),
            ) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for Inline {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for Inline {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_3(
                Rc::new(RefCell::new(Inline { a: self.a.clone() })).as_pointer(),
                Rc::new(RefCell::new(Inline { a: other.a.clone() })).as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for Inline {}
impl ByteRepr for Inline {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.a.to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            a: <i32>::from_bytes(&buf[0..4]),
        }
    }
}
#[derive(Clone, Record, VaArg, FnPtrArg, Default)]
pub struct OutOfLine {
    #[offset(0)]
    pub a: i32,
}
impl std::cmp::PartialEq for OutOfLine {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_6(
                Rc::new(RefCell::new(OutOfLine { a: self.a.clone() })).as_pointer(),
                Rc::new(RefCell::new(OutOfLine { a: other.a.clone() })).as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for OutOfLine {}
impl ByteRepr for OutOfLine {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.a.to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            a: <i32>::from_bytes(&buf[0..4]),
        }
    }
}
pub fn operator_eq_6(x: Ptr<OutOfLine>, y: Ptr<OutOfLine>) -> bool {
    return {
        let _lhs = x.with(|__s| __s.a);
        _lhs == y.with(|__s| __s.a)
    };
}
pub fn operator_ne_7(x: Ptr<OutOfLine>, y: Ptr<OutOfLine>) -> bool {
    return !({
        let _x: Ptr<OutOfLine> = (x).clone();
        let _y: Ptr<OutOfLine> = (y).clone();
        operator_eq_6(_x, _y)
    });
}
pub fn operator_eq_8(x: Ptr<Tmpl_int_>, y: Ptr<Tmpl_int_>) -> bool {
    return {
        let _lhs = x.with(|__s| __s.v);
        _lhs == y.with(|__s| __s.v)
    };
}
pub fn operator_lt_9(x: Ptr<Tmpl_int_>, y: Ptr<Tmpl_int_>) -> bool {
    return {
        let _lhs = x.with(|__s| __s.v);
        _lhs < y.with(|__s| __s.v)
    };
}
pub fn operator_eq_10(x: Ptr<Tmpl_int_>, y: Ptr<Tmpl_long_>) -> bool {
    return {
        let _lhs = (x.with(|__s| __s.v) as i64);
        _lhs == y.with(|__s| __s.v)
    };
}
#[derive(Clone, Record, VaArg, FnPtrArg, Default)]
pub struct Tmpl_int_ {
    #[offset(0)]
    pub v: i32,
}
impl std::cmp::Ord for Tmpl_int_ {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            if operator_lt_9(
                Rc::new(RefCell::new(Tmpl_int_ { v: self.v.clone() })).as_pointer(),
                Rc::new(RefCell::new(Tmpl_int_ { v: other.v.clone() })).as_pointer(),
            ) {
                std::cmp::Ordering::Less
            } else if operator_lt_9(
                Rc::new(RefCell::new(Tmpl_int_ { v: other.v.clone() })).as_pointer(),
                Rc::new(RefCell::new(Tmpl_int_ { v: self.v.clone() })).as_pointer(),
            ) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for Tmpl_int_ {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for Tmpl_int_ {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_8(
                Rc::new(RefCell::new(Tmpl_int_ { v: self.v.clone() })).as_pointer(),
                Rc::new(RefCell::new(Tmpl_int_ { v: other.v.clone() })).as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for Tmpl_int_ {}
impl ByteRepr for Tmpl_int_ {
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
#[derive(Clone, Record, VaArg, FnPtrArg, Default)]
pub struct Tmpl_long_ {
    #[offset(0)]
    pub v: i64,
}
impl ByteRepr for Tmpl_long_ {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.v.to_bytes(&mut buf[0..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            v: <i64>::from_bytes(&buf[0..8]),
        }
    }
}
pub fn operator_eq_11(_a0: Ptr<TmplDefaulted_int_>, _a1: Ptr<TmplDefaulted_int_>) -> bool {
    return {
        let _lhs = _a0.with(|__s| __s.v);
        _lhs == _a1.with(|__s| __s.v)
    };
}
#[derive(Clone, Record, VaArg, FnPtrArg, Default)]
pub struct TmplDefaulted_int_ {
    #[offset(0)]
    pub v: i32,
}
impl std::cmp::PartialEq for TmplDefaulted_int_ {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_11(
                Rc::new(RefCell::new(TmplDefaulted_int_ { v: self.v.clone() })).as_pointer(),
                Rc::new(RefCell::new(TmplDefaulted_int_ { v: other.v.clone() })).as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for TmplDefaulted_int_ {}
impl ByteRepr for TmplDefaulted_int_ {
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
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let d1: Value<Defaulted> = Rc::new(RefCell::new(Defaulted { a: 1, b: 2 }));
    let d2: Value<Defaulted> = Rc::new(RefCell::new(Defaulted { a: 1, b: 2 }));
    let d3: Value<Defaulted> = Rc::new(RefCell::new(Defaulted { a: 1, b: 3 }));
    assert!(
        ({
            let _arg0: Ptr<Defaulted> = d1.as_pointer();
            operator_eq_0(_arg0, d2.as_pointer())
        })
    );
    assert!(
        !({
            let _arg0: Ptr<Defaulted> = d1.as_pointer();
            operator_eq_0(_arg0, d3.as_pointer())
        })
    );
    assert!(
        !({
            let _arg0: Ptr<Defaulted> = d1.as_pointer();
            operator_eq_0(_arg0, d3.as_pointer())
        })
    );
    let o1: Value<DefaultedOrd> = Rc::new(RefCell::new(DefaultedOrd { a: 1 }));
    let o2: Value<DefaultedOrd> = Rc::new(RefCell::new(DefaultedOrd { a: 2 }));
    assert!(
        ({
            let _arg0: Ptr<DefaultedOrd> = o1.as_pointer();
            operator_cmp_1(_arg0, o2.as_pointer())
        }) == std::cmp::Ordering::Less
    );
    assert!(
        ({
            let _arg0: Ptr<DefaultedOrd> = o2.as_pointer();
            operator_cmp_1(_arg0, o1.as_pointer())
        }) == std::cmp::Ordering::Greater
    );
    assert!(
        ({
            let _arg0: Ptr<DefaultedOrd> = o1.as_pointer();
            let _arg1: Ptr<DefaultedOrd> = o1.as_pointer();
            operator_eq_2(_arg0, _arg1)
        })
    );
    assert!(
        ({
            let _arg0: Ptr<DefaultedOrd> = o1.as_pointer();
            operator_cmp_1(_arg0, o2.as_pointer())
        }) == std::cmp::Ordering::Less
    );
    let i1: Value<Inline> = Rc::new(RefCell::new(Inline { a: 1 }));
    let i2: Value<Inline> = Rc::new(RefCell::new(Inline { a: 2 }));
    let i3: Value<Inline> = Rc::new(RefCell::new(Inline { a: 1 }));
    assert!(
        ({
            let _x: Ptr<Inline> = i1.as_pointer();
            operator_eq_3(_x, i3.as_pointer())
        })
    );
    assert!(
        !({
            let _x: Ptr<Inline> = i1.as_pointer();
            operator_eq_3(_x, i2.as_pointer())
        })
    );
    assert!(
        ({
            let _x: Ptr<Inline> = i1.as_pointer();
            operator_lt_4(_x, i2.as_pointer())
        })
    );
    assert!(
        !({
            let _x: Ptr<Inline> = i2.as_pointer();
            operator_lt_4(_x, i1.as_pointer())
        })
    );
    assert!(
        ({
            let _x: Value<Inline> = Rc::new(RefCell::new(
                ({
                    let _x: Ptr<Inline> = i1.as_pointer();
                    operator_add_5(_x, i2.as_pointer())
                }),
            ));
            let _y: Value<Inline> = Rc::new(RefCell::new(Inline { a: 3 }));
            operator_eq_3(_x.as_pointer(), _y.as_pointer())
        })
    );
    let f1: Value<OutOfLine> = Rc::new(RefCell::new(OutOfLine { a: 4 }));
    let f2: Value<OutOfLine> = Rc::new(RefCell::new(OutOfLine { a: 4 }));
    let f3: Value<OutOfLine> = Rc::new(RefCell::new(OutOfLine { a: 5 }));
    assert!(
        ({
            let _x: Ptr<OutOfLine> = f1.as_pointer();
            operator_eq_6(_x, f2.as_pointer())
        })
    );
    assert!(
        ({
            let _x: Ptr<OutOfLine> = f1.as_pointer();
            operator_ne_7(_x, f3.as_pointer())
        })
    );
    assert!(
        !({
            let _x: Ptr<OutOfLine> = f1.as_pointer();
            operator_eq_6(_x, f3.as_pointer())
        })
    );
    let t1: Value<Tmpl_int_> = Rc::new(RefCell::new(Tmpl_int_ { v: 1 }));
    let t2: Value<Tmpl_int_> = Rc::new(RefCell::new(Tmpl_int_ { v: 2 }));
    let t3: Value<Tmpl_int_> = Rc::new(RefCell::new(Tmpl_int_ { v: 1 }));
    assert!(
        ({
            let _x: Ptr<Tmpl_int_> = t1.as_pointer();
            operator_eq_8(_x, t3.as_pointer())
        })
    );
    assert!(
        !({
            let _x: Ptr<Tmpl_int_> = t1.as_pointer();
            operator_eq_8(_x, t2.as_pointer())
        })
    );
    assert!(
        ({
            let _x: Ptr<Tmpl_int_> = t1.as_pointer();
            operator_lt_9(_x, t2.as_pointer())
        })
    );
    let u1: Value<Tmpl_long_> = Rc::new(RefCell::new(Tmpl_long_ { v: 1_i64 }));
    let u2: Value<Tmpl_long_> = Rc::new(RefCell::new(Tmpl_long_ { v: 2_i64 }));
    assert!(
        ({
            let _x: Ptr<Tmpl_int_> = t1.as_pointer();
            operator_eq_10(_x, u1.as_pointer())
        })
    );
    assert!(
        !({
            let _x: Ptr<Tmpl_int_> = t1.as_pointer();
            operator_eq_10(_x, u2.as_pointer())
        })
    );
    let v1: Value<TmplDefaulted_int_> = Rc::new(RefCell::new(TmplDefaulted_int_ { v: 7 }));
    let v2: Value<TmplDefaulted_int_> = Rc::new(RefCell::new(TmplDefaulted_int_ { v: 7 }));
    let v3: Value<TmplDefaulted_int_> = Rc::new(RefCell::new(TmplDefaulted_int_ { v: 8 }));
    assert!(
        ({
            let _arg0: Ptr<TmplDefaulted_int_> = v1.as_pointer();
            operator_eq_11(_arg0, v2.as_pointer())
        })
    );
    assert!(
        !({
            let _arg0: Ptr<TmplDefaulted_int_> = v1.as_pointer();
            operator_eq_11(_arg0, v3.as_pointer())
        })
    );
    assert!(
        !({
            let _arg0: Ptr<TmplDefaulted_int_> = v1.as_pointer();
            operator_eq_11(_arg0, v3.as_pointer())
        })
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
