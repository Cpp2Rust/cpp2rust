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
