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
pub struct Lt {
    #[offset(0)]
    pub v: i32,
}
impl CppLess for Lt {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        LtImpl::operator_lt(__lhs, __rhs.clone())
    }
}
impl CppEq for Lt {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        !(LtImpl::operator_lt(__lhs, __rhs.clone())) && !(LtImpl::operator_lt(__rhs, __lhs.clone()))
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Eq {
    #[offset(0)]
    pub v: i32,
}
impl CppEq for Eq {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        EqImpl::operator_eq(__lhs, __rhs.clone())
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Cmp {
    #[offset(0)]
    pub v: i32,
}
impl CppLess for Cmp {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        CmpImpl::operator_cmp(__lhs, __rhs.clone()) == std::cmp::Ordering::Less
    }
}
impl CppEq for Cmp {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        CmpImpl::operator_eq(__lhs, __rhs.clone())
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Free {
    #[offset(0)]
    pub v: i32,
}
impl CppLess for Free {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_lt_0(__lhs.clone(), __rhs.clone())
    }
}
impl CppEq for Free {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_1(__lhs.clone(), __rhs.clone())
    }
}
pub fn operator_lt_0(a: Ptr<Free>, b: Ptr<Free>) -> bool {
    return ({ a.with(|__s| __s.v) } < { b.with(|__s| __s.v) });
}
pub fn operator_eq_1(a: Ptr<Free>, b: Ptr<Free>) -> bool {
    return ({ a.with(|__s| __s.v) } == { b.with(|__s| __s.v) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Wrapped_int_ {
    #[offset(0)]
    pub v: i32,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut w: Wrapped_int_ = Wrapped_int_ { v: 2 };
    assert!((w.v == 2));
    let lts: Value<Vec<Lt>> = Rc::new(RefCell::new(Vec::from([
        Lt { v: 3 },
        Lt { v: 1 },
        Lt { v: 2 },
    ])));
    {
        let __first = lts.decay().begin();
        let __n = lts.decay().end().iter_distance(&__first);
        let mut __order: Vec<isize> = (0..__n).collect();
        __order.sort_by(|&__x, &__y| cpp_cmp(&__first.iter_at(__x), &__first.iter_at(__y)));
        reorder(__order, |__i| __first.iter_at(__i));
    };
    assert!(
        (({ (*lts.borrow())[0_usize].v } == 1) && ({ (*lts.borrow())[1_usize].v } == 2))
            && ({ (*lts.borrow())[2_usize].v } == 3)
    );
    let eqs: Value<Vec<Eq>> = Rc::new(RefCell::new(Vec::from([
        Eq { v: 1 },
        Eq { v: 2 },
        Eq { v: 3 },
    ])));
    let two: Value<Eq> = Rc::new(RefCell::new(Eq { v: 2 }));
    let nine: Value<Eq> = Rc::new(RefCell::new(Eq { v: 9 }));
    assert!(
        ({
            let mut __it = eqs.decay().begin();
            let __last = eqs.decay().end();
            let __value = two.as_pointer();
            while __it != __last && !CppEq::equal(&__it.iter_deref(), &__value) {
                __it.iter_inc();
            }
            __it
        }
        .iter_deref()
        .with(|__s| __s.v)
            == 2)
    );
    assert!(
        {
            let mut __it = eqs.decay().begin();
            let __last = eqs.decay().end();
            let __value = nine.as_pointer();
            while __it != __last && !CppEq::equal(&__it.iter_deref(), &__value) {
                __it.iter_inc();
            }
            __it
        } == eqs.decay().end()
    );
    let cmps: Value<Vec<Cmp>> = Rc::new(RefCell::new(Vec::from([
        Cmp { v: 3 },
        Cmp { v: 1 },
        Cmp { v: 2 },
    ])));
    {
        let __first = cmps.decay().begin();
        let __n = cmps.decay().end().iter_distance(&__first);
        let mut __order: Vec<isize> = (0..__n).collect();
        __order.sort_by(|&__x, &__y| cpp_cmp(&__first.iter_at(__x), &__first.iter_at(__y)));
        reorder(__order, |__i| __first.iter_at(__i));
    };
    assert!(({ (*cmps.borrow())[0_usize].v } == 1) && ({ (*cmps.borrow())[2_usize].v } == 3));
    let three: Value<Cmp> = Rc::new(RefCell::new(Cmp { v: 3 }));
    assert!(
        ({
            let mut __it = cmps.decay().begin();
            let __last = cmps.decay().end();
            let __value = three.as_pointer();
            while __it != __last && !CppEq::equal(&__it.iter_deref(), &__value) {
                __it.iter_inc();
            }
            __it
        }
        .iter_deref()
        .with(|__s| __s.v)
            == 3)
    );
    let frees: Value<Vec<Free>> = Rc::new(RefCell::new(Vec::from([Free { v: 2 }, Free { v: 1 }])));
    {
        let __first = frees.decay().begin();
        let __n = frees.decay().end().iter_distance(&__first);
        let mut __order: Vec<isize> = (0..__n).collect();
        __order.sort_by(|&__x, &__y| cpp_cmp(&__first.iter_at(__x), &__first.iter_at(__y)));
        reorder(__order, |__i| __first.iter_at(__i));
    };
    assert!(({ (*frees.borrow())[0_usize].v } == 1));
    let ftwo: Value<Free> = Rc::new(RefCell::new(Free { v: 2 }));
    assert!(
        ({
            let mut __it = frees.decay().begin();
            let __last = frees.decay().end();
            let __value = ftwo.as_pointer();
            while __it != __last && !CppEq::equal(&__it.iter_deref(), &__value) {
                __it.iter_inc();
            }
            __it
        }
        .iter_deref()
        .with(|__s| __s.v)
            == 2)
    );
    let m: Value<Map<Lt, (Value<Lt>, Value<i32>)>> = Rc::new(RefCell::new(Map::new()));
    Map::subscript_move(
        m.as_pointer(),
        Rc::new(RefCell::new(Lt { v: 2 })).as_pointer(),
        |__k| cpp_move!(__k),
    )
    .write(20);
    Map::subscript_move(
        m.as_pointer(),
        Rc::new(RefCell::new(Lt { v: 1 })).as_pointer(),
        |__k| cpp_move!(__k),
    )
    .write(10);
    assert!(
        ((*(*m
            .as_pointer()
            .with(|__m: &Map<Lt, (Value<Lt>, Value<i32>)>| __m.begin())
            .iter_deref()
            .upgrade()
            .deref())
        .1
        .borrow())
            == 10)
    );
    assert!(
        ((Map::subscript_move(
            m.as_pointer(),
            Rc::new(RefCell::new(Lt { v: 2 })).as_pointer(),
            |__k| cpp_move!(__k)
        )
        .read())
            == 20)
    );
    return 0;
}
pub trait CmpImpl {
    fn operator_cmp(&self, o: Ptr<Cmp>) -> std::cmp::Ordering;
    fn operator_eq(&self, o: Ptr<Cmp>) -> bool;
}
impl CmpImpl for Ptr<Cmp> {
    fn operator_cmp(&self, o: Ptr<Cmp>) -> std::cmp::Ordering {
        return std::cmp::Ord::cmp(&((*self).with(|__s| __s.v)), &(o.with(|__s| __s.v)));
    }
    fn operator_eq(&self, o: Ptr<Cmp>) -> bool {
        return ({ (*self).with(|__s| __s.v) } == { o.with(|__s| __s.v) });
    }
}
pub trait EqImpl {
    fn operator_eq(&self, o: Ptr<Eq>) -> bool;
}
impl EqImpl for Ptr<Eq> {
    fn operator_eq(&self, o: Ptr<Eq>) -> bool {
        return ({ (*self).with(|__s| __s.v) } == { o.with(|__s| __s.v) });
    }
}
pub trait LtImpl {
    fn operator_lt(&self, o: Ptr<Lt>) -> bool;
}
impl LtImpl for Ptr<Lt> {
    fn operator_lt(&self, o: Ptr<Lt>) -> bool {
        return ({ (*self).with(|__s| __s.v) } < { o.with(|__s| __s.v) });
    }
}
pub fn __cpp2rust_init_globals() {}
