extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct MoveOnly {
    pub v: Value<i32>,
}
impl MoveOnly {
    pub fn MoveOnly(v: i32) -> Self {
        let v: Value<i32> = Rc::new(RefCell::new(v));
        let __this: Value<MoveOnly> = Rc::new(RefCell::new(Self {
            v: Rc::new(RefCell::new((*v.borrow()))),
        }));
        let this: Ptr<MoveOnly> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn MoveOnly_pmutMoveOnly(o: Ptr<MoveOnly>) -> Self {
        let __this: Value<MoveOnly> = Rc::new(RefCell::new(Self {
            v: Rc::new(RefCell::new((*(*o.upgrade().deref()).v.borrow()))),
        }));
        let this: Ptr<MoveOnly> = __this.as_pointer();
        (*(*o.upgrade().deref()).v.borrow_mut()) = 0;
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for MoveOnly {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.v.borrow()).to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            v: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
        }
    }
}
#[derive()]
pub struct ConstMoveAssign {
    pub mark: Value<i32>,
}
impl ConstMoveAssign {
    pub fn ConstMoveAssign() -> Self {
        let __this: Value<ConstMoveAssign> = Rc::new(RefCell::new(Self {
            mark: Rc::new(RefCell::new(0)),
        }));
        let this: Ptr<ConstMoveAssign> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for ConstMoveAssign {
    fn default() -> Self {
        { ConstMoveAssign::ConstMoveAssign() }
    }
}
impl ByteRepr for ConstMoveAssign {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.mark.borrow()).to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            mark: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
        }
    }
}
pub fn make_0(v: i32) -> MoveOnly {
    let v: Value<i32> = Rc::new(RefCell::new(v));
    let m: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::MoveOnly({ (*v.borrow()) })));
    return MoveOnly::MoveOnly_pmutMoveOnly({ m.as_pointer() });
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let a: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::MoveOnly({ 1 })));
    let b: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::MoveOnly({ 2 })));
    let c: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::MoveOnly({ 3 })));
    ({ MoveOnlyImpl::operator_assign_pmutMoveOnly(&a.as_pointer(), b.as_pointer()) });
    assert!(((*(*a.borrow()).v.borrow()) == 2));
    assert!(((*(*b.borrow()).v.borrow()) == 0));
    ({
        let _o: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::MoveOnly({ 3 })));
        MoveOnlyImpl::operator_assign_pmutMoveOnly(&b.as_pointer(), _o.as_pointer())
    });
    ({
        MoveOnlyImpl::operator_assign_pmutMoveOnly(
            &c.as_pointer(),
            ({ MoveOnlyImpl::operator_assign_pmutMoveOnly(&a.as_pointer(), b.as_pointer()) }),
        )
    });
    assert!(
        (((*(*b.borrow()).v.borrow()) == 0) && ((*(*a.borrow()).v.borrow()) == 0))
            && ((*(*c.borrow()).v.borrow()) == 3)
    );
    ({
        let _o: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::MoveOnly({ 5 })));
        MoveOnlyImpl::operator_assign_pmutMoveOnly(&a.as_pointer(), _o.as_pointer())
    });
    assert!(((*(*a.borrow()).v.borrow()) == 5));
    ({
        let _o: Value<MoveOnly> = Rc::new(RefCell::new(({ make_0(6) })));
        MoveOnlyImpl::operator_assign_pmutMoveOnly(&a.as_pointer(), _o.as_pointer())
    });
    assert!(((*(*a.borrow()).v.borrow()) == 6));
    ({
        let _o: Ptr<MoveOnly> = a.as_pointer();
        MoveOnlyImpl::operator_assign_pmutMoveOnly(&a.as_pointer(), _o)
    });
    assert!(((*(*a.borrow()).v.borrow()) == 6));
    let vec_: Value<Vec<MoveOnly>> = Rc::new(RefCell::new(Vec::new()));
    (*vec_.borrow_mut()).push(MoveOnly::MoveOnly({ 7 }));
    let d: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::MoveOnly({ 8 })));
    ({
        MoveOnlyImpl::operator_assign_pmutMoveOnly(
            &(vec_.as_pointer() as Ptr<MoveOnly>).offset(0_usize),
            d.as_pointer(),
        )
    });
    assert!(
        ((*(*(vec_.as_pointer() as Ptr<MoveOnly>)
            .offset(0_usize)
            .upgrade()
            .deref())
        .v
        .borrow())
            == 8)
    );
    assert!(((*(*d.borrow()).v.borrow()) == 0));
    let m: Value<ConstMoveAssign> = Rc::new(RefCell::new(ConstMoveAssign::ConstMoveAssign()));
    let m1: Value<ConstMoveAssign> = Rc::new(RefCell::new(ConstMoveAssign::ConstMoveAssign()));
    let m2: Value<ConstMoveAssign> = Rc::new(RefCell::new(ConstMoveAssign::ConstMoveAssign()));
    let cm: Value<ConstMoveAssign> = Rc::new(RefCell::new(ConstMoveAssign::ConstMoveAssign()));
    ({
        ConstMoveAssignImpl::operator_assign_pmutConstMoveAssign(&m1.as_pointer(), m.as_pointer())
    });
    ({
        ConstMoveAssignImpl::operator_assign_pconstConstMoveAssign(
            &m2.as_pointer(),
            cm.as_pointer(),
        )
    });
    assert!(((*(*m1.borrow()).mark.borrow()) == 1));
    assert!(((*(*m2.borrow()).mark.borrow()) == 10));
    return 0;
}
pub trait ConstMoveAssignImpl {
    fn operator_assign_pmutConstMoveAssign(&self, o: Ptr<ConstMoveAssign>) -> Ptr<ConstMoveAssign>;
    fn operator_assign_pconstConstMoveAssign(
        &self,
        o: Ptr<ConstMoveAssign>,
    ) -> Ptr<ConstMoveAssign>;
}
impl ConstMoveAssignImpl for Ptr<ConstMoveAssign> {
    fn operator_assign_pmutConstMoveAssign(&self, o: Ptr<ConstMoveAssign>) -> Ptr<ConstMoveAssign> {
        let __rhs = ((*(*o.upgrade().deref()).mark.borrow()) + 1);
        (*(*(*self).upgrade().deref()).mark.borrow_mut()) = __rhs;
        return (*self).clone();
    }
    fn operator_assign_pconstConstMoveAssign(
        &self,
        o: Ptr<ConstMoveAssign>,
    ) -> Ptr<ConstMoveAssign> {
        let __rhs = ((*(*o.upgrade().deref()).mark.borrow()) + 10);
        (*(*(*self).upgrade().deref()).mark.borrow_mut()) = __rhs;
        return (*self).clone();
    }
}
pub trait MoveOnlyImpl {
    fn operator_assign_pmutMoveOnly(&self, o: Ptr<MoveOnly>) -> Ptr<MoveOnly>;
}
impl MoveOnlyImpl for Ptr<MoveOnly> {
    fn operator_assign_pmutMoveOnly(&self, o: Ptr<MoveOnly>) -> Ptr<MoveOnly> {
        if ((*self) == (o)) {
            return (*self).clone();
        }
        let __rhs = (*(*o.upgrade().deref()).v.borrow());
        (*(*(*self).upgrade().deref()).v.borrow_mut()) = __rhs;
        (*(*o.upgrade().deref()).v.borrow_mut()) = 0;
        return (*self).clone();
    }
}
