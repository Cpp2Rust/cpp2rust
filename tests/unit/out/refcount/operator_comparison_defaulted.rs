extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Eq {
    #[offset(0)]
    pub a: i32,
    #[offset(4)]
    pub b: i32,
}
impl CppEq for Eq {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        EqImpl::operator_eq(__lhs, __rhs.clone())
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Cmp {
    #[offset(0)]
    pub a: i32,
    #[offset(4)]
    pub b: i32,
}
impl CppLess for Cmp {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        CmpImpl::operator_cmp(__lhs, __rhs.clone()) == std::cmp::Ordering::Less
    }
}
impl CppEq for Cmp {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        CmpImpl::operator_cmp(__lhs, __rhs.clone()) == std::cmp::Ordering::Equal
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Both {
    #[offset(0)]
    pub a: i32,
}
impl CppLess for Both {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        BothImpl::operator_cmp(__lhs, __rhs.clone()) == std::cmp::Ordering::Less
    }
}
impl CppEq for Both {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        BothImpl::operator_eq(__lhs, __rhs.clone())
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct OrdOnly {
    #[offset(0)]
    pub a: i32,
}
impl CppLess for OrdOnly {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        OrdOnlyImpl::operator_cmp(__lhs, __rhs.clone()) == std::cmp::Ordering::Less
    }
}
impl CppEq for OrdOnly {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        OrdOnlyImpl::operator_cmp(__lhs, __rhs.clone()) == std::cmp::Ordering::Equal
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Inner {
    #[offset(0)]
    pub x: i32,
}
impl CppLess for Inner {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        InnerImpl::operator_cmp(__lhs, __rhs.clone()) == std::cmp::Ordering::Less
    }
}
impl CppEq for Inner {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        InnerImpl::operator_cmp(__lhs, __rhs.clone()) == std::cmp::Ordering::Equal
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Outer {
    #[offset(0)]
    #[byte_size(4)]
    pub i: Inner,
    #[offset(4)]
    pub y: i32,
}
impl CppLess for Outer {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        OuterImpl::operator_cmp(__lhs, __rhs.clone()) == std::cmp::Ordering::Less
    }
}
impl CppEq for Outer {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        OuterImpl::operator_cmp(__lhs, __rhs.clone()) == std::cmp::Ordering::Equal
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Secondary {
    #[offset(0)]
    pub a: i32,
}
impl CppLess for Secondary {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        SecondaryImpl::operator_lt(__lhs, __rhs.clone())
    }
}
impl CppEq for Secondary {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        SecondaryImpl::operator_eq(__lhs, __rhs.clone())
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct PtrMember {
    #[offset(0)]
    #[byte_size(8)]
    pub p: Ptr<i32>,
}
impl CppLess for PtrMember {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        PtrMemberImpl::operator_cmp(__lhs, __rhs.clone()) == std::cmp::Ordering::Less
    }
}
impl CppEq for PtrMember {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        PtrMemberImpl::operator_cmp(__lhs, __rhs.clone()) == std::cmp::Ordering::Equal
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let e1: Value<Eq> = Rc::new(RefCell::new(Eq { a: 1, b: 2 }));
    let e2: Value<Eq> = Rc::new(RefCell::new(Eq { a: 1, b: 2 }));
    let e3: Value<Eq> = Rc::new(RefCell::new(Eq { a: 1, b: 3 }));
    assert!(({ EqImpl::operator_eq(&e1.as_pointer(), e2.as_pointer(),) }));
    assert!(!({ EqImpl::operator_eq(&e1.as_pointer(), e3.as_pointer(),) }));
    let c1: Value<Cmp> = Rc::new(RefCell::new(Cmp { a: 1, b: 2 }));
    let c2: Value<Cmp> = Rc::new(RefCell::new(Cmp { a: 1, b: 3 }));
    let c3: Value<Cmp> = Rc::new(RefCell::new(Cmp { a: 2, b: 0 }));
    let c4: Value<Cmp> = Rc::new(RefCell::new(Cmp { a: 1, b: 9 }));
    assert!(
        ({ CmpImpl::operator_cmp(&c1.as_pointer(), c2.as_pointer(),) }) == std::cmp::Ordering::Less
    );
    assert!(
        ({ CmpImpl::operator_cmp(&c3.as_pointer(), c4.as_pointer(),) })
            == std::cmp::Ordering::Greater
    );
    assert!(
        ({
            let _arg0: Ptr<Cmp> = c1.as_pointer();
            CmpImpl::operator_eq(&c1.as_pointer(), _arg0)
        })
    );
    assert!(
        ({ CmpImpl::operator_cmp(&c1.as_pointer(), c2.as_pointer(),) }) == std::cmp::Ordering::Less
    );
    let b1: Value<Both> = Rc::new(RefCell::new(Both { a: 1 }));
    let b2: Value<Both> = Rc::new(RefCell::new(Both { a: 2 }));
    assert!(
        ({ BothImpl::operator_cmp(&b1.as_pointer(), b2.as_pointer(),) })
            == std::cmp::Ordering::Less
    );
    assert!(
        ({
            let _arg0: Ptr<Both> = b2.as_pointer();
            BothImpl::operator_eq(&b2.as_pointer(), _arg0)
        })
    );
    let o1: Value<OrdOnly> = Rc::new(RefCell::new(OrdOnly { a: 1 }));
    let o2: Value<OrdOnly> = Rc::new(RefCell::new(OrdOnly { a: 2 }));
    assert!(
        ({ OrdOnlyImpl::operator_cmp(&o1.as_pointer(), o2.as_pointer(),) })
            == std::cmp::Ordering::Less
    );
    assert!(
        ({ OrdOnlyImpl::operator_cmp(&o2.as_pointer(), o1.as_pointer(),) })
            == std::cmp::Ordering::Greater
    );
    let x1: Value<Outer> = Rc::new(RefCell::new(Outer {
        i: Inner { x: 1 },
        y: 9,
    }));
    let x2: Value<Outer> = Rc::new(RefCell::new(Outer {
        i: Inner { x: 2 },
        y: 0,
    }));
    let x3: Value<Outer> = Rc::new(RefCell::new(Outer {
        i: Inner { x: 1 },
        y: 9,
    }));
    assert!(
        ({ OuterImpl::operator_cmp(&x1.as_pointer(), x2.as_pointer(),) })
            == std::cmp::Ordering::Less
    );
    assert!(({ OuterImpl::operator_eq(&x1.as_pointer(), x3.as_pointer(),) }));
    assert!(
        ({ OuterImpl::operator_cmp(&x2.as_pointer(), x1.as_pointer(),) })
            == std::cmp::Ordering::Greater
    );
    let s1: Value<Secondary> = Rc::new(RefCell::new(Secondary { a: 1 }));
    let s2: Value<Secondary> = Rc::new(RefCell::new(Secondary { a: 2 }));
    assert!(({ SecondaryImpl::operator_ne(&s1.as_pointer(), s2.as_pointer(),) }));
    assert!(({ SecondaryImpl::operator_lt(&s1.as_pointer(), s2.as_pointer(),) }));
    assert!(({ SecondaryImpl::operator_ge(&s2.as_pointer(), s1.as_pointer(),) }));
    assert!(!({ SecondaryImpl::operator_lt(&s2.as_pointer(), s1.as_pointer(),) }));
    let arr: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0])));
    let p1: Value<PtrMember> = Rc::new(RefCell::new(PtrMember {
        p: (arr.as_pointer() as Ptr<i32>),
    }));
    let p2: Value<PtrMember> = Rc::new(RefCell::new(PtrMember {
        p: (arr.as_pointer() as Ptr<i32>).offset((1) as isize),
    }));
    let p3: Value<PtrMember> = Rc::new(RefCell::new(PtrMember {
        p: (arr.as_pointer() as Ptr<i32>),
    }));
    assert!(
        ({ PtrMemberImpl::operator_cmp(&p1.as_pointer(), p2.as_pointer(),) })
            == std::cmp::Ordering::Less
    );
    assert!(({ PtrMemberImpl::operator_eq(&p1.as_pointer(), p3.as_pointer(),) }));
    assert!(
        ({ PtrMemberImpl::operator_cmp(&p2.as_pointer(), p1.as_pointer(),) })
            == std::cmp::Ordering::Greater
    );
    return 0;
}
pub trait BothImpl {
    fn operator_eq(&self, _a0: Ptr<Both>) -> bool;
    fn operator_cmp(&self, _a0: Ptr<Both>) -> std::cmp::Ordering;
}
impl BothImpl for Ptr<Both> {
    fn operator_eq(&self, _a0: Ptr<Both>) -> bool {
        return ({ (*self).with(|__s| __s.a) } == { _a0.with(|__s| __s.a) });
    }
    fn operator_cmp(&self, _a0: Ptr<Both>) -> std::cmp::Ordering {
        {
            let cmp: Value<std::cmp::Ordering> = Rc::new(RefCell::new(std::cmp::Ord::cmp(
                &((*self).with(|__s| __s.a)),
                &(_a0.with(|__s| __s.a)),
            )));
            if !(({ (*cmp.borrow()) }) == std::cmp::Ordering::Equal) {
                return (*cmp.borrow_mut());
            }
        }
        return std::cmp::Ordering::Equal;
    }
}
pub trait CmpImpl {
    fn operator_cmp(&self, _a0: Ptr<Cmp>) -> std::cmp::Ordering;
    fn operator_eq(&self, _a0: Ptr<Cmp>) -> bool;
}
impl CmpImpl for Ptr<Cmp> {
    fn operator_cmp(&self, _a0: Ptr<Cmp>) -> std::cmp::Ordering {
        {
            let cmp: Value<std::cmp::Ordering> = Rc::new(RefCell::new(std::cmp::Ord::cmp(
                &((*self).with(|__s| __s.a)),
                &(_a0.with(|__s| __s.a)),
            )));
            if !(({ (*cmp.borrow()) }) == std::cmp::Ordering::Equal) {
                return (*cmp.borrow_mut());
            }
        }
        {
            let cmp: Value<std::cmp::Ordering> = Rc::new(RefCell::new(std::cmp::Ord::cmp(
                &((*self).with(|__s| __s.b)),
                &(_a0.with(|__s| __s.b)),
            )));
            if !(({ (*cmp.borrow()) }) == std::cmp::Ordering::Equal) {
                return (*cmp.borrow_mut());
            }
        }
        return std::cmp::Ordering::Equal;
    }
    fn operator_eq(&self, _a0: Ptr<Cmp>) -> bool {
        return ({ (*self).with(|__s| __s.a) } == { _a0.with(|__s| __s.a) })
            && ({ (*self).with(|__s| __s.b) } == { _a0.with(|__s| __s.b) });
    }
}
pub trait EqImpl {
    fn operator_eq(&self, _a0: Ptr<Eq>) -> bool;
}
impl EqImpl for Ptr<Eq> {
    fn operator_eq(&self, _a0: Ptr<Eq>) -> bool {
        return ({ (*self).with(|__s| __s.a) } == { _a0.with(|__s| __s.a) })
            && ({ (*self).with(|__s| __s.b) } == { _a0.with(|__s| __s.b) });
    }
}
pub trait InnerImpl {
    fn operator_cmp(&self, _a0: Ptr<Inner>) -> std::cmp::Ordering;
    fn operator_eq(&self, _a0: Ptr<Inner>) -> bool;
}
impl InnerImpl for Ptr<Inner> {
    fn operator_cmp(&self, _a0: Ptr<Inner>) -> std::cmp::Ordering {
        {
            let cmp: Value<std::cmp::Ordering> = Rc::new(RefCell::new(std::cmp::Ord::cmp(
                &((*self).with(|__s| __s.x)),
                &(_a0.with(|__s| __s.x)),
            )));
            if !(({ (*cmp.borrow()) }) == std::cmp::Ordering::Equal) {
                return (*cmp.borrow_mut());
            }
        }
        return std::cmp::Ordering::Equal;
    }
    fn operator_eq(&self, _a0: Ptr<Inner>) -> bool {
        return ({ (*self).with(|__s| __s.x) } == { _a0.with(|__s| __s.x) });
    }
}
pub trait OrdOnlyImpl {
    fn operator_cmp(&self, _a0: Ptr<OrdOnly>) -> std::cmp::Ordering;
}
impl OrdOnlyImpl for Ptr<OrdOnly> {
    fn operator_cmp(&self, _a0: Ptr<OrdOnly>) -> std::cmp::Ordering {
        {
            let cmp: Value<std::cmp::Ordering> = Rc::new(RefCell::new(std::cmp::Ord::cmp(
                &((*self).with(|__s| __s.a)),
                &(_a0.with(|__s| __s.a)),
            )));
            if !(({ (*cmp.borrow()) }) == std::cmp::Ordering::Equal) {
                return (*cmp.borrow_mut());
            }
        }
        return std::cmp::Ordering::Equal;
    }
}
pub trait OuterImpl {
    fn operator_cmp(&self, _a0: Ptr<Outer>) -> std::cmp::Ordering;
    fn operator_eq(&self, _a0: Ptr<Outer>) -> bool;
}
impl OuterImpl for Ptr<Outer> {
    fn operator_cmp(&self, _a0: Ptr<Outer>) -> std::cmp::Ordering {
        {
            let cmp: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
                ({
                    let _arg0: Ptr<Inner> = field_ptr!(_a0, i);
                    InnerImpl::operator_cmp(&field_ptr!((*self), i), _arg0)
                }),
            ));
            if !(({ (*cmp.borrow()) }) == std::cmp::Ordering::Equal) {
                return (*cmp.borrow_mut());
            }
        }
        {
            let cmp: Value<std::cmp::Ordering> = Rc::new(RefCell::new(std::cmp::Ord::cmp(
                &((*self).with(|__s| __s.y)),
                &(_a0.with(|__s| __s.y)),
            )));
            if !(({ (*cmp.borrow()) }) == std::cmp::Ordering::Equal) {
                return (*cmp.borrow_mut());
            }
        }
        return std::cmp::Ordering::Equal;
    }
    fn operator_eq(&self, _a0: Ptr<Outer>) -> bool {
        return ({
            let _arg0: Ptr<Inner> = field_ptr!(_a0, i);
            InnerImpl::operator_eq(&field_ptr!((*self), i), _arg0)
        }) && ({ (*self).with(|__s| __s.y) } == { _a0.with(|__s| __s.y) });
    }
}
pub trait PtrMemberImpl {
    fn operator_cmp(&self, _a0: Ptr<PtrMember>) -> std::cmp::Ordering;
    fn operator_eq(&self, _a0: Ptr<PtrMember>) -> bool;
}
impl PtrMemberImpl for Ptr<PtrMember> {
    fn operator_cmp(&self, _a0: Ptr<PtrMember>) -> std::cmp::Ordering {
        {
            let cmp: Value<std::cmp::Ordering> = Rc::new(RefCell::new(std::cmp::Ord::cmp(
                &((*self).with(|__s| __s.p.clone())),
                &(_a0.with(|__s| __s.p.clone())),
            )));
            if !(({ (*cmp.borrow()) }) == std::cmp::Ordering::Equal) {
                return (*cmp.borrow_mut());
            }
        }
        return std::cmp::Ordering::Equal;
    }
    fn operator_eq(&self, _a0: Ptr<PtrMember>) -> bool {
        return ({ (*self).with(|__s| __s.p.clone()) } == { _a0.with(|__s| __s.p.clone()) });
    }
}
pub trait SecondaryImpl {
    fn operator_eq(&self, _a0: Ptr<Secondary>) -> bool;
    fn operator_ne(&self, _a0: Ptr<Secondary>) -> bool;
    fn operator_cmp(&self, _a0: Ptr<Secondary>) -> std::cmp::Ordering;
    fn operator_lt(&self, _a0: Ptr<Secondary>) -> bool;
    fn operator_ge(&self, _a0: Ptr<Secondary>) -> bool;
}
impl SecondaryImpl for Ptr<Secondary> {
    fn operator_eq(&self, _a0: Ptr<Secondary>) -> bool {
        return ({ (*self).with(|__s| __s.a) } == { _a0.with(|__s| __s.a) });
    }
    fn operator_ne(&self, _a0: Ptr<Secondary>) -> bool {
        return !({
            let _arg0: Ptr<Secondary> = (_a0).clone();
            SecondaryImpl::operator_eq(&(*self), _arg0)
        });
    }
    fn operator_cmp(&self, _a0: Ptr<Secondary>) -> std::cmp::Ordering {
        {
            let cmp: Value<std::cmp::Ordering> = Rc::new(RefCell::new(std::cmp::Ord::cmp(
                &((*self).with(|__s| __s.a)),
                &(_a0.with(|__s| __s.a)),
            )));
            if !(({ (*cmp.borrow()) }) == std::cmp::Ordering::Equal) {
                return (*cmp.borrow_mut());
            }
        }
        return std::cmp::Ordering::Equal;
    }
    fn operator_lt(&self, _a0: Ptr<Secondary>) -> bool {
        return ({
            let _arg0: Ptr<Secondary> = (_a0).clone();
            SecondaryImpl::operator_cmp(&(*self), _arg0)
        }) == std::cmp::Ordering::Less;
    }
    fn operator_ge(&self, _a0: Ptr<Secondary>) -> bool {
        return ({
            let _arg0: Ptr<Secondary> = (_a0).clone();
            SecondaryImpl::operator_cmp(&(*self), _arg0)
        }) != std::cmp::Ordering::Less;
    }
}
pub fn __cpp2rust_init_globals() {}
