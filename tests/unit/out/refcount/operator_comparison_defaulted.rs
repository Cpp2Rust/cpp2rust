extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct Eq {
    pub a: Value<i32>,
    pub b: Value<i32>,
}
impl std::cmp::PartialEq for Eq {
    fn eq(&self, other: &Self) -> bool {
        {
            EqImpl::operator_eq(
                &Rc::new(RefCell::new(Eq {
                    a: self.a.clone(),
                    b: self.b.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(Eq {
                    a: other.a.clone(),
                    b: other.b.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for Eq {}
impl Clone for Eq {
    fn clone(&self) -> Self {
        let __this: Value<Eq> = Rc::new(RefCell::new(Self {
            a: Rc::new(RefCell::new((*self.a.borrow()))),
            b: Rc::new(RefCell::new((*self.b.borrow()))),
        }));
        let this: Ptr<Eq> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for Eq {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.a.borrow()).to_bytes(&mut buf[0..4]);
        (*self.b.borrow()).to_bytes(&mut buf[4..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            a: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
            b: Rc::new(RefCell::new(<i32>::from_bytes(&buf[4..8]))),
        }
    }
}
#[derive(Default)]
pub struct Cmp {
    pub a: Value<i32>,
    pub b: Value<i32>,
}
impl std::cmp::Ord for Cmp {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            CmpImpl::operator_cmp(
                &Rc::new(RefCell::new(Cmp {
                    a: self.a.clone(),
                    b: self.b.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(Cmp {
                    a: other.a.clone(),
                    b: other.b.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::PartialOrd for Cmp {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for Cmp {
    fn eq(&self, other: &Self) -> bool {
        {
            CmpImpl::operator_cmp(
                &Rc::new(RefCell::new(Cmp {
                    a: self.a.clone(),
                    b: self.b.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(Cmp {
                    a: other.a.clone(),
                    b: other.b.clone(),
                }))
                .as_pointer(),
            ) == std::cmp::Ordering::Equal
        }
    }
}
impl std::cmp::Eq for Cmp {}
impl Clone for Cmp {
    fn clone(&self) -> Self {
        let __this: Value<Cmp> = Rc::new(RefCell::new(Self {
            a: Rc::new(RefCell::new((*self.a.borrow()))),
            b: Rc::new(RefCell::new((*self.b.borrow()))),
        }));
        let this: Ptr<Cmp> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for Cmp {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.a.borrow()).to_bytes(&mut buf[0..4]);
        (*self.b.borrow()).to_bytes(&mut buf[4..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            a: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
            b: Rc::new(RefCell::new(<i32>::from_bytes(&buf[4..8]))),
        }
    }
}
#[derive(Default)]
pub struct Both {
    pub a: Value<i32>,
}
impl std::cmp::Ord for Both {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            BothImpl::operator_cmp(
                &Rc::new(RefCell::new(Both { a: self.a.clone() })).as_pointer(),
                Rc::new(RefCell::new(Both { a: other.a.clone() })).as_pointer(),
            )
        }
    }
}
impl std::cmp::PartialOrd for Both {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for Both {
    fn eq(&self, other: &Self) -> bool {
        {
            BothImpl::operator_eq(
                &Rc::new(RefCell::new(Both { a: self.a.clone() })).as_pointer(),
                Rc::new(RefCell::new(Both { a: other.a.clone() })).as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for Both {}
impl Clone for Both {
    fn clone(&self) -> Self {
        let __this: Value<Both> = Rc::new(RefCell::new(Self {
            a: Rc::new(RefCell::new((*self.a.borrow()))),
        }));
        let this: Ptr<Both> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for Both {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.a.borrow()).to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            a: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
        }
    }
}
#[derive(Default)]
pub struct OrdOnly {
    pub a: Value<i32>,
}
impl std::cmp::Ord for OrdOnly {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            OrdOnlyImpl::operator_cmp(
                &Rc::new(RefCell::new(OrdOnly { a: self.a.clone() })).as_pointer(),
                Rc::new(RefCell::new(OrdOnly { a: other.a.clone() })).as_pointer(),
            )
        }
    }
}
impl std::cmp::PartialOrd for OrdOnly {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for OrdOnly {
    fn eq(&self, other: &Self) -> bool {
        {
            OrdOnlyImpl::operator_cmp(
                &Rc::new(RefCell::new(OrdOnly { a: self.a.clone() })).as_pointer(),
                Rc::new(RefCell::new(OrdOnly { a: other.a.clone() })).as_pointer(),
            ) == std::cmp::Ordering::Equal
        }
    }
}
impl std::cmp::Eq for OrdOnly {}
impl Clone for OrdOnly {
    fn clone(&self) -> Self {
        let __this: Value<OrdOnly> = Rc::new(RefCell::new(Self {
            a: Rc::new(RefCell::new((*self.a.borrow()))),
        }));
        let this: Ptr<OrdOnly> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for OrdOnly {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.a.borrow()).to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            a: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
        }
    }
}
#[derive(Default)]
pub struct Inner {
    pub x: Value<i32>,
}
impl std::cmp::Ord for Inner {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            InnerImpl::operator_cmp(
                &Rc::new(RefCell::new(Inner { x: self.x.clone() })).as_pointer(),
                Rc::new(RefCell::new(Inner { x: other.x.clone() })).as_pointer(),
            )
        }
    }
}
impl std::cmp::PartialOrd for Inner {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for Inner {
    fn eq(&self, other: &Self) -> bool {
        {
            InnerImpl::operator_cmp(
                &Rc::new(RefCell::new(Inner { x: self.x.clone() })).as_pointer(),
                Rc::new(RefCell::new(Inner { x: other.x.clone() })).as_pointer(),
            ) == std::cmp::Ordering::Equal
        }
    }
}
impl std::cmp::Eq for Inner {}
impl Clone for Inner {
    fn clone(&self) -> Self {
        let __this: Value<Inner> = Rc::new(RefCell::new(Self {
            x: Rc::new(RefCell::new((*self.x.borrow()))),
        }));
        let this: Ptr<Inner> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for Inner {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.x.borrow()).to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            x: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
        }
    }
}
#[derive(Default)]
pub struct Outer {
    pub i: Value<Inner>,
    pub y: Value<i32>,
}
impl std::cmp::Ord for Outer {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            OuterImpl::operator_cmp(
                &Rc::new(RefCell::new(Outer {
                    i: self.i.clone(),
                    y: self.y.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(Outer {
                    i: other.i.clone(),
                    y: other.y.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::PartialOrd for Outer {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for Outer {
    fn eq(&self, other: &Self) -> bool {
        {
            OuterImpl::operator_cmp(
                &Rc::new(RefCell::new(Outer {
                    i: self.i.clone(),
                    y: self.y.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(Outer {
                    i: other.i.clone(),
                    y: other.y.clone(),
                }))
                .as_pointer(),
            ) == std::cmp::Ordering::Equal
        }
    }
}
impl std::cmp::Eq for Outer {}
impl Clone for Outer {
    fn clone(&self) -> Self {
        let __this: Value<Outer> = Rc::new(RefCell::new(Self {
            i: Rc::new(RefCell::new((*self.i.borrow()).clone())),
            y: Rc::new(RefCell::new((*self.y.borrow()))),
        }));
        let this: Ptr<Outer> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for Outer {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.i.borrow()).to_bytes(&mut buf[0..4]);
        (*self.y.borrow()).to_bytes(&mut buf[4..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            i: Rc::new(RefCell::new(<Inner>::from_bytes(&buf[0..4]))),
            y: Rc::new(RefCell::new(<i32>::from_bytes(&buf[4..8]))),
        }
    }
}
#[derive(Default)]
pub struct Secondary {
    pub a: Value<i32>,
}
impl std::cmp::Ord for Secondary {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            SecondaryImpl::operator_cmp(
                &Rc::new(RefCell::new(Secondary { a: self.a.clone() })).as_pointer(),
                Rc::new(RefCell::new(Secondary { a: other.a.clone() })).as_pointer(),
            )
        }
    }
}
impl std::cmp::PartialOrd for Secondary {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for Secondary {
    fn eq(&self, other: &Self) -> bool {
        {
            SecondaryImpl::operator_eq(
                &Rc::new(RefCell::new(Secondary { a: self.a.clone() })).as_pointer(),
                Rc::new(RefCell::new(Secondary { a: other.a.clone() })).as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for Secondary {}
impl Clone for Secondary {
    fn clone(&self) -> Self {
        let __this: Value<Secondary> = Rc::new(RefCell::new(Self {
            a: Rc::new(RefCell::new((*self.a.borrow()))),
        }));
        let this: Ptr<Secondary> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for Secondary {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.a.borrow()).to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            a: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
        }
    }
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let e1: Value<Eq> = Rc::new(RefCell::new(Eq {
        a: Rc::new(RefCell::new(1)),
        b: Rc::new(RefCell::new(2)),
    }));
    let e2: Value<Eq> = Rc::new(RefCell::new(Eq {
        a: Rc::new(RefCell::new(1)),
        b: Rc::new(RefCell::new(2)),
    }));
    let e3: Value<Eq> = Rc::new(RefCell::new(Eq {
        a: Rc::new(RefCell::new(1)),
        b: Rc::new(RefCell::new(3)),
    }));
    assert!(({ EqImpl::operator_eq(&e1.as_pointer(), e2.as_pointer(),) }));
    assert!(!({ EqImpl::operator_eq(&e1.as_pointer(), e3.as_pointer(),) }));
    let c1: Value<Cmp> = Rc::new(RefCell::new(Cmp {
        a: Rc::new(RefCell::new(1)),
        b: Rc::new(RefCell::new(2)),
    }));
    let c2: Value<Cmp> = Rc::new(RefCell::new(Cmp {
        a: Rc::new(RefCell::new(1)),
        b: Rc::new(RefCell::new(3)),
    }));
    let c3: Value<Cmp> = Rc::new(RefCell::new(Cmp {
        a: Rc::new(RefCell::new(2)),
        b: Rc::new(RefCell::new(0)),
    }));
    let c4: Value<Cmp> = Rc::new(RefCell::new(Cmp {
        a: Rc::new(RefCell::new(1)),
        b: Rc::new(RefCell::new(9)),
    }));
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
    let b1: Value<Both> = Rc::new(RefCell::new(Both {
        a: Rc::new(RefCell::new(1)),
    }));
    let b2: Value<Both> = Rc::new(RefCell::new(Both {
        a: Rc::new(RefCell::new(2)),
    }));
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
    let o1: Value<OrdOnly> = Rc::new(RefCell::new(OrdOnly {
        a: Rc::new(RefCell::new(1)),
    }));
    let o2: Value<OrdOnly> = Rc::new(RefCell::new(OrdOnly {
        a: Rc::new(RefCell::new(2)),
    }));
    assert!(
        ({ OrdOnlyImpl::operator_cmp(&o1.as_pointer(), o2.as_pointer(),) })
            == std::cmp::Ordering::Less
    );
    assert!(
        ({ OrdOnlyImpl::operator_cmp(&o2.as_pointer(), o1.as_pointer(),) })
            == std::cmp::Ordering::Greater
    );
    let x1: Value<Outer> = Rc::new(RefCell::new(Outer {
        i: Rc::new(RefCell::new(Inner {
            x: Rc::new(RefCell::new(1)),
        })),
        y: Rc::new(RefCell::new(9)),
    }));
    let x2: Value<Outer> = Rc::new(RefCell::new(Outer {
        i: Rc::new(RefCell::new(Inner {
            x: Rc::new(RefCell::new(2)),
        })),
        y: Rc::new(RefCell::new(0)),
    }));
    let x3: Value<Outer> = Rc::new(RefCell::new(Outer {
        i: Rc::new(RefCell::new(Inner {
            x: Rc::new(RefCell::new(1)),
        })),
        y: Rc::new(RefCell::new(9)),
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
    let s1: Value<Secondary> = Rc::new(RefCell::new(Secondary {
        a: Rc::new(RefCell::new(1)),
    }));
    let s2: Value<Secondary> = Rc::new(RefCell::new(Secondary {
        a: Rc::new(RefCell::new(2)),
    }));
    assert!(({ SecondaryImpl::operator_ne(&s1.as_pointer(), s2.as_pointer(),) }));
    assert!(({ SecondaryImpl::operator_lt(&s1.as_pointer(), s2.as_pointer(),) }));
    assert!(({ SecondaryImpl::operator_ge(&s2.as_pointer(), s1.as_pointer(),) }));
    assert!(!({ SecondaryImpl::operator_lt(&s2.as_pointer(), s1.as_pointer(),) }));
    return 0;
}
pub trait BothImpl {
    fn operator_eq(&self, _arg0: Ptr<Both>) -> bool;
    fn operator_cmp(&self, _arg0: Ptr<Both>) -> std::cmp::Ordering;
}
impl BothImpl for Ptr<Both> {
    fn operator_eq(&self, _arg0: Ptr<Both>) -> bool {
        return {
            let _lhs = (*(*(*self).upgrade().deref()).a.borrow());
            _lhs == (*(*_arg0.upgrade().deref()).a.borrow())
        };
    }
    fn operator_cmp(&self, _arg0: Ptr<Both>) -> std::cmp::Ordering {
        {
            let cmp: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
                (*(*(*self).upgrade().deref()).a.borrow())
                    .cmp(&(*(*_arg0.upgrade().deref()).a.borrow())),
            ));
            if !((*cmp.borrow()) == std::cmp::Ordering::Equal) {
                return (*cmp.borrow_mut()).clone();
            }
        }
        return std::cmp::Ordering::Equal;
    }
}
pub trait CmpImpl {
    fn operator_cmp(&self, _arg0: Ptr<Cmp>) -> std::cmp::Ordering;
    fn operator_eq(&self, _arg0: Ptr<Cmp>) -> bool;
}
impl CmpImpl for Ptr<Cmp> {
    fn operator_cmp(&self, _arg0: Ptr<Cmp>) -> std::cmp::Ordering {
        {
            let cmp: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
                (*(*(*self).upgrade().deref()).a.borrow())
                    .cmp(&(*(*_arg0.upgrade().deref()).a.borrow())),
            ));
            if !((*cmp.borrow()) == std::cmp::Ordering::Equal) {
                return (*cmp.borrow_mut()).clone();
            }
        }
        {
            let cmp: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
                (*(*(*self).upgrade().deref()).b.borrow())
                    .cmp(&(*(*_arg0.upgrade().deref()).b.borrow())),
            ));
            if !((*cmp.borrow()) == std::cmp::Ordering::Equal) {
                return (*cmp.borrow_mut()).clone();
            }
        }
        return std::cmp::Ordering::Equal;
    }
    fn operator_eq(&self, _arg0: Ptr<Cmp>) -> bool {
        return ({
            let _lhs = (*(*(*self).upgrade().deref()).a.borrow());
            _lhs == (*(*_arg0.upgrade().deref()).a.borrow())
        }) && ({
            let _lhs = (*(*(*self).upgrade().deref()).b.borrow());
            _lhs == (*(*_arg0.upgrade().deref()).b.borrow())
        });
    }
}
pub trait EqImpl {
    fn operator_eq(&self, _arg0: Ptr<Eq>) -> bool;
}
impl EqImpl for Ptr<Eq> {
    fn operator_eq(&self, _arg0: Ptr<Eq>) -> bool {
        return ({
            let _lhs = (*(*(*self).upgrade().deref()).a.borrow());
            _lhs == (*(*_arg0.upgrade().deref()).a.borrow())
        }) && ({
            let _lhs = (*(*(*self).upgrade().deref()).b.borrow());
            _lhs == (*(*_arg0.upgrade().deref()).b.borrow())
        });
    }
}
pub trait InnerImpl {
    fn operator_cmp(&self, _arg0: Ptr<Inner>) -> std::cmp::Ordering;
    fn operator_eq(&self, _arg0: Ptr<Inner>) -> bool;
}
impl InnerImpl for Ptr<Inner> {
    fn operator_cmp(&self, _arg0: Ptr<Inner>) -> std::cmp::Ordering {
        {
            let cmp: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
                (*(*(*self).upgrade().deref()).x.borrow())
                    .cmp(&(*(*_arg0.upgrade().deref()).x.borrow())),
            ));
            if !((*cmp.borrow()) == std::cmp::Ordering::Equal) {
                return (*cmp.borrow_mut()).clone();
            }
        }
        return std::cmp::Ordering::Equal;
    }
    fn operator_eq(&self, _arg0: Ptr<Inner>) -> bool {
        return {
            let _lhs = (*(*(*self).upgrade().deref()).x.borrow());
            _lhs == (*(*_arg0.upgrade().deref()).x.borrow())
        };
    }
}
pub trait OrdOnlyImpl {
    fn operator_cmp(&self, _arg0: Ptr<OrdOnly>) -> std::cmp::Ordering;
}
impl OrdOnlyImpl for Ptr<OrdOnly> {
    fn operator_cmp(&self, _arg0: Ptr<OrdOnly>) -> std::cmp::Ordering {
        {
            let cmp: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
                (*(*(*self).upgrade().deref()).a.borrow())
                    .cmp(&(*(*_arg0.upgrade().deref()).a.borrow())),
            ));
            if !((*cmp.borrow()) == std::cmp::Ordering::Equal) {
                return (*cmp.borrow_mut()).clone();
            }
        }
        return std::cmp::Ordering::Equal;
    }
}
pub trait OuterImpl {
    fn operator_cmp(&self, _arg0: Ptr<Outer>) -> std::cmp::Ordering;
    fn operator_eq(&self, _arg0: Ptr<Outer>) -> bool;
}
impl OuterImpl for Ptr<Outer> {
    fn operator_cmp(&self, _arg0: Ptr<Outer>) -> std::cmp::Ordering {
        {
            let cmp: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
                ({
                    let _arg0: Ptr<Inner> = (*_arg0.upgrade().deref()).i.as_pointer();
                    InnerImpl::operator_cmp(&(*(*self).upgrade().deref()).i.as_pointer(), _arg0)
                }),
            ));
            if !((*cmp.borrow()) == std::cmp::Ordering::Equal) {
                return (*cmp.borrow_mut()).clone();
            }
        }
        {
            let cmp: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
                (*(*(*self).upgrade().deref()).y.borrow())
                    .cmp(&(*(*_arg0.upgrade().deref()).y.borrow())),
            ));
            if !((*cmp.borrow()) == std::cmp::Ordering::Equal) {
                return (*cmp.borrow_mut()).clone();
            }
        }
        return std::cmp::Ordering::Equal;
    }
    fn operator_eq(&self, _arg0: Ptr<Outer>) -> bool {
        return ({
            let _arg0: Ptr<Inner> = (*_arg0.upgrade().deref()).i.as_pointer();
            InnerImpl::operator_eq(&(*(*self).upgrade().deref()).i.as_pointer(), _arg0)
        }) && ({
            let _lhs = (*(*(*self).upgrade().deref()).y.borrow());
            _lhs == (*(*_arg0.upgrade().deref()).y.borrow())
        });
    }
}
pub trait SecondaryImpl {
    fn operator_eq(&self, _arg0: Ptr<Secondary>) -> bool;
    fn operator_ne(&self, _arg0: Ptr<Secondary>) -> bool;
    fn operator_cmp(&self, _arg0: Ptr<Secondary>) -> std::cmp::Ordering;
    fn operator_lt(&self, _arg0: Ptr<Secondary>) -> bool;
    fn operator_ge(&self, _arg0: Ptr<Secondary>) -> bool;
}
impl SecondaryImpl for Ptr<Secondary> {
    fn operator_eq(&self, _arg0: Ptr<Secondary>) -> bool {
        return {
            let _lhs = (*(*(*self).upgrade().deref()).a.borrow());
            _lhs == (*(*_arg0.upgrade().deref()).a.borrow())
        };
    }
    fn operator_ne(&self, _arg0: Ptr<Secondary>) -> bool {
        return !({
            let _arg0: Ptr<Secondary> = (_arg0).clone();
            SecondaryImpl::operator_eq(&(*self), _arg0)
        });
    }
    fn operator_cmp(&self, _arg0: Ptr<Secondary>) -> std::cmp::Ordering {
        {
            let cmp: Value<std::cmp::Ordering> = Rc::new(RefCell::new(
                (*(*(*self).upgrade().deref()).a.borrow())
                    .cmp(&(*(*_arg0.upgrade().deref()).a.borrow())),
            ));
            if !((*cmp.borrow()) == std::cmp::Ordering::Equal) {
                return (*cmp.borrow_mut()).clone();
            }
        }
        return std::cmp::Ordering::Equal;
    }
    fn operator_lt(&self, _arg0: Ptr<Secondary>) -> bool {
        return ({
            let _arg0: Ptr<Secondary> = (_arg0).clone();
            SecondaryImpl::operator_cmp(&(*self), _arg0)
        }) == std::cmp::Ordering::Less;
    }
    fn operator_ge(&self, _arg0: Ptr<Secondary>) -> bool {
        return ({
            let _arg0: Ptr<Secondary> = (_arg0).clone();
            SecondaryImpl::operator_cmp(&(*self), _arg0)
        }) != std::cmp::Ordering::Less;
    }
}
