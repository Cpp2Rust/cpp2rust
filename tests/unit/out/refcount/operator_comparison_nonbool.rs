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
pub struct X {
    #[offset(0)]
    pub v: i32,
}
impl CppLess for X {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        ((operator_lt_0(__lhs.read(), __rhs.read())) != 0)
    }
}
impl CppEq for X {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        ((operator_eq_1(__lhs.read(), __rhs.read())) != 0)
    }
}
pub fn operator_eq_1(mut a: X, mut b: X) -> i32 {
    return if (a.v == b.v) { 2 } else { 0 };
}
pub fn operator_ne_2(mut a: X, mut b: X) -> i32 {
    return if (a.v != b.v) { 3 } else { 0 };
}
pub fn operator_lt_0(mut a: X, mut b: X) -> i32 {
    return if (a.v < b.v) { 4 } else { 0 };
}
pub fn operator_gt_3(mut a: X, mut b: X) -> i32 {
    return if (a.v > b.v) { 5 } else { 0 };
}
pub fn operator_le_4(mut a: X, mut b: X) -> i32 {
    return if (a.v <= b.v) { 6 } else { 0 };
}
pub fn operator_ge_5(mut a: X, mut b: X) -> i32 {
    return if (a.v >= b.v) { 7 } else { 0 };
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Result {
    #[offset(0)]
    pub r: i32,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Custom {
    #[offset(0)]
    pub v: i32,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Mixed {
    #[offset(0)]
    pub v: i32,
}
impl CppLess for Mixed {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        ((MixedImpl::operator_lt(__lhs, __rhs.clone())) != 0)
    }
}
impl CppEq for Mixed {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        ((MixedImpl::operator_eq(__lhs, __rhs.clone())) != 0)
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_into(bool, to_bool)]
#[byte_size(1)]
pub struct MyBool {
    #[offset(0)]
    pub value: bool,
}
impl MyBool {
    pub fn new(mut v: bool) -> Self {
        Self { value: v }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Boolish {
    #[offset(0)]
    pub v: i32,
}
impl CppLess for Boolish {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        {
            let __cmp = operator_lt_6(__lhs.read(), __rhs.read());
            MyBoolImpl::to_bool(&Rc::new(RefCell::new(__cmp)).as_pointer())
        }
    }
}
impl CppEq for Boolish {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        {
            let __cmp = operator_eq_7(__lhs.read(), __rhs.read());
            MyBoolImpl::to_bool(&Rc::new(RefCell::new(__cmp)).as_pointer())
        }
    }
}
pub fn operator_eq_7(mut a: Boolish, mut b: Boolish) -> MyBool {
    return MyBool::new({ (a.v == b.v) });
}
pub fn operator_lt_6(mut a: Boolish, mut b: Boolish) -> MyBool {
    return MyBool::new({ (a.v < b.v) });
}
#[derive(Record, ByteRepr, FnPtrArg, CppFrom, Default)]
#[cpp_into(bool, to_bool)]
#[byte_size(1)]
pub struct RefBool {
    #[offset(0)]
    pub value: bool,
}
impl RefBool {
    pub fn new(mut v: bool) -> Self {
        Self { value: v }
    }
}
thread_local!(
    pub static ref_true_8: Value<RefBool> = Rc::new(RefCell::new(RefBool::new({ true })));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct RefInt {
    #[offset(0)]
    pub v: i32,
}
impl CppEq for RefInt {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        {
            let __cmp = RefIntImpl::operator_eq(__lhs, __rhs.clone());
            RefBoolImpl::to_bool(&__cmp)
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut a: X = X { v: 1 };
    let mut b: X = X { v: 2 };
    let mut c: X = X { v: 1 };
    assert!(
        (({
            let _a: X = (a).clone();
            operator_eq_1(_a, (c).clone())
        }) == 2)
    );
    assert!(
        (({
            let _a: X = (a).clone();
            operator_eq_1(_a, (b).clone())
        }) == 0)
    );
    assert!(
        (({
            let _a: X = (a).clone();
            operator_ne_2(_a, (b).clone())
        }) == 3)
    );
    assert!(
        (({
            let _a: X = (a).clone();
            operator_lt_0(_a, (b).clone())
        }) == 4)
    );
    assert!(
        (({
            let _a: X = (b).clone();
            operator_gt_3(_a, (a).clone())
        }) == 5)
    );
    assert!(
        (({
            let _a: X = (a).clone();
            operator_le_4(_a, (c).clone())
        }) == 6)
    );
    assert!(
        (({
            let _a: X = (a).clone();
            operator_ge_5(_a, (c).clone())
        }) == 7)
    );
    let xs: Value<Box<[X]>> = Rc::new(RefCell::new(Box::new([X { v: 3 }, X { v: 1 }, X { v: 2 }])));
    match ({ (xs.as_pointer() as Ptr<X>) },) {
        (mut __tmp_0,) => {
            let __n = (xs.as_pointer() as Ptr<X>)
                .offset((3) as isize)
                .iter_distance(&__tmp_0);
            let mut __order: Vec<isize> = (0..__n).collect();
            __order.sort_by(|&__x, &__y| cpp_cmp(&__tmp_0.iter_at(__x), &__tmp_0.iter_at(__y)));
            reorder(__order, |__i| __tmp_0.iter_at(__i));
        }
    };
    assert!(
        (({ (*xs.borrow())[(0) as usize].v } == 1) && ({ (*xs.borrow())[(1) as usize].v } == 2))
            && ({ (*xs.borrow())[(2) as usize].v } == 3)
    );
    let p: Value<Custom> = Rc::new(RefCell::new(Custom { v: 5 }));
    let q: Value<Custom> = Rc::new(RefCell::new(Custom { v: 2 }));
    let mut r: Result = ({ CustomImpl::operator_cmp(&p.as_pointer(), q.as_pointer()) });
    assert!((r.r == 3));
    let m: Value<Mixed> = Rc::new(RefCell::new(Mixed { v: 4 }));
    let n: Value<Mixed> = Rc::new(RefCell::new(Mixed { v: 4 }));
    assert!((({ MixedImpl::operator_eq(&m.as_pointer(), n.as_pointer(),) }) == 9));
    assert!(({ ({ MixedImpl::operator_cmp(&m.as_pointer(), n.as_pointer(),) }).r } == 0));
    let ms: Value<Box<[Mixed]>> = Rc::new(RefCell::new(Box::new([
        Mixed { v: 6 },
        Mixed { v: 4 },
        Mixed { v: 5 },
    ])));
    match ({ (ms.as_pointer() as Ptr<Mixed>) },) {
        (mut __tmp_1,) => {
            let __n = (ms.as_pointer() as Ptr<Mixed>)
                .offset((3) as isize)
                .iter_distance(&__tmp_1);
            let mut __order: Vec<isize> = (0..__n).collect();
            __order.sort_by(|&__x, &__y| cpp_cmp(&__tmp_1.iter_at(__x), &__tmp_1.iter_at(__y)));
            reorder(__order, |__i| __tmp_1.iter_at(__i));
        }
    };
    assert!(
        (({ (*ms.borrow())[(0) as usize].v } == 4) && ({ (*ms.borrow())[(1) as usize].v } == 5))
            && ({ (*ms.borrow())[(2) as usize].v } == 6)
    );
    assert!(
        (({
            let _o: Ptr<Mixed> = (ms.as_pointer() as Ptr<Mixed>).offset(1);
            MixedImpl::operator_lt(&(ms.as_pointer() as Ptr<Mixed>).offset(0), _o)
        }) == 8)
    );
    let mut b1: Boolish = Boolish { v: 1 };
    let mut b2: Boolish = Boolish { v: 2 };
    assert!(
        ({
            MyBoolImpl::to_bool(
                &Rc::new(RefCell::new(
                    ({
                        let _a: Boolish = (b1).clone();
                        operator_eq_7(_a, Boolish { v: 1 })
                    }),
                ))
                .as_pointer(),
            )
        })
    );
    assert!(
        !({
            MyBoolImpl::to_bool(
                &Rc::new(RefCell::new(
                    ({
                        let _a: Boolish = (b2).clone();
                        operator_lt_6(_a, (b1).clone())
                    }),
                ))
                .as_pointer(),
            )
        })
    );
    let mut lt: MyBool = ({
        let _a: Boolish = (b1).clone();
        operator_lt_6(_a, (b2).clone())
    });
    assert!(lt.value);
    let bs: Value<Box<[Boolish]>> = Rc::new(RefCell::new(Box::new([
        Boolish { v: 3 },
        Boolish { v: 1 },
        Boolish { v: 2 },
    ])));
    match ({ (bs.as_pointer() as Ptr<Boolish>) },) {
        (mut __tmp_2,) => {
            let __n = (bs.as_pointer() as Ptr<Boolish>)
                .offset((3) as isize)
                .iter_distance(&__tmp_2);
            let mut __order: Vec<isize> = (0..__n).collect();
            __order.sort_by(|&__x, &__y| cpp_cmp(&__tmp_2.iter_at(__x), &__tmp_2.iter_at(__y)));
            reorder(__order, |__i| __tmp_2.iter_at(__i));
        }
    };
    assert!(
        (({ (*bs.borrow())[(0) as usize].v } == 1) && ({ (*bs.borrow())[(1) as usize].v } == 2))
            && ({ (*bs.borrow())[(2) as usize].v } == 3)
    );
    let ri1: Value<RefInt> = Rc::new(RefCell::new(RefInt { v: 1 }));
    let ri2: Value<RefInt> = Rc::new(RefCell::new(RefInt { v: 2 }));
    assert!(
        ({
            RefBoolImpl::to_bool(
                &({ RefIntImpl::operator_eq(&ri1.as_pointer(), ri2.as_pointer()) }),
            )
        })
    );
    let ris: Value<Box<[RefInt]>> =
        Rc::new(RefCell::new(Box::new([RefInt { v: 1 }, RefInt { v: 2 }])));
    assert!(
        (match ({ (ris.as_pointer() as Ptr<RefInt>).offset((2) as isize) },) {
            (mut __tmp_3,) => {
                {
                    let mut __it = (ris.as_pointer() as Ptr<RefInt>);
                    while __it != __tmp_3 && !CppEq::equal(&__it.iter_deref(), &ri2.as_pointer()) {
                        __it.iter_inc();
                    }
                    __it
                }
            }
        } == (ris.as_pointer() as Ptr<RefInt>))
    );
    return 0;
}
pub trait CustomImpl {
    fn operator_cmp(&self, o: Ptr<Custom>) -> Result;
}
impl CustomImpl for Ptr<Custom> {
    fn operator_cmp(&self, o: Ptr<Custom>) -> Result {
        return Result {
            r: ({ (*self).with(|__s| __s.v) } - { o.with(|__s| __s.v) }),
        };
    }
}
pub trait MixedImpl {
    fn operator_cmp(&self, o: Ptr<Mixed>) -> Result;
    fn operator_lt(&self, o: Ptr<Mixed>) -> i32;
    fn operator_eq(&self, o: Ptr<Mixed>) -> i32;
}
impl MixedImpl for Ptr<Mixed> {
    fn operator_cmp(&self, o: Ptr<Mixed>) -> Result {
        return Result {
            r: ({ (*self).with(|__s| __s.v) } - { o.with(|__s| __s.v) }),
        };
    }
    fn operator_lt(&self, o: Ptr<Mixed>) -> i32 {
        return if ({ (*self).with(|__s| __s.v) } < { o.with(|__s| __s.v) }) {
            8
        } else {
            0
        };
    }
    fn operator_eq(&self, o: Ptr<Mixed>) -> i32 {
        return if ({ (*self).with(|__s| __s.v) } == { o.with(|__s| __s.v) }) {
            9
        } else {
            0
        };
    }
}
pub trait MyBoolImpl {
    fn to_bool(&self) -> bool;
}
impl MyBoolImpl for Ptr<MyBool> {
    fn to_bool(&self) -> bool {
        return (*self).with(|__s| __s.value);
    }
}
pub trait RefBoolImpl {
    fn to_bool(&self) -> bool;
}
impl RefBoolImpl for Ptr<RefBool> {
    fn to_bool(&self) -> bool {
        return (*self).with(|__s| __s.value);
    }
}
pub trait RefIntImpl {
    fn operator_eq(&self, _a0: Ptr<RefInt>) -> Ptr<RefBool>;
}
impl RefIntImpl for Ptr<RefInt> {
    fn operator_eq(&self, _a0: Ptr<RefInt>) -> Ptr<RefBool> {
        return ref_true_8.with(|v| v.as_pointer());
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = ref_true_8.with(|_| ());
}
