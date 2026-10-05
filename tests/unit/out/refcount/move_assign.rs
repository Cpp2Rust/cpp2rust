extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Record, ByteRepr, Default)]
#[byte_size(4)]
pub struct MoveOnly {
    #[offset(0)]
    pub v: i32,
}
impl MoveOnly {
    pub fn new(mut v: i32) -> Self {
        let __this: Value<MoveOnly> = Rc::new(RefCell::new(Self { v: v }));
        let this: Ptr<MoveOnly> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn move_from(o: Ptr<MoveOnly>) -> Self {
        let __this: Value<MoveOnly> = Rc::new(RefCell::new(Self {
            v: o.with(|__s| __s.v),
        }));
        let this: Ptr<MoveOnly> = __this.as_pointer();
        field!(o, v).write(0);
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
#[derive(Record, ByteRepr)]
#[byte_size(4)]
pub struct ConstMoveAssign {
    #[offset(0)]
    pub mark: i32,
}
impl ConstMoveAssign {
    pub fn new() -> Self {
        let __this: Value<ConstMoveAssign> = Rc::new(RefCell::new(Self { mark: 0 }));
        let this: Ptr<ConstMoveAssign> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for ConstMoveAssign {
    fn default() -> Self {
        { ConstMoveAssign::new() }
    }
}
pub fn make_0(mut v: i32) -> MoveOnly {
    let m: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ v })));
    return MoveOnly::move_from({ m.as_pointer() });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let a: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ 1 })));
    let b: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ 2 })));
    let c: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ 3 })));
    ({ MoveOnlyImpl::move_assign(&a.as_pointer(), b.as_pointer()) });
    assert!(({ (*a.borrow()).v } == 2));
    assert!(({ (*b.borrow()).v } == 0));
    ({
        let _o: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ 3 })));
        MoveOnlyImpl::move_assign(&b.as_pointer(), _o.as_pointer())
    });
    ({
        MoveOnlyImpl::move_assign(
            &c.as_pointer(),
            ({ MoveOnlyImpl::move_assign(&a.as_pointer(), b.as_pointer()) }),
        )
    });
    assert!(
        (({ (*b.borrow()).v } == 0) && ({ (*a.borrow()).v } == 0)) && ({ (*c.borrow()).v } == 3)
    );
    ({
        let _o: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ 5 })));
        MoveOnlyImpl::move_assign(&a.as_pointer(), _o.as_pointer())
    });
    assert!(({ (*a.borrow()).v } == 5));
    ({
        let _o: Value<MoveOnly> = Rc::new(RefCell::new(({ make_0(6) })));
        MoveOnlyImpl::move_assign(&a.as_pointer(), _o.as_pointer())
    });
    assert!(({ (*a.borrow()).v } == 6));
    ({
        let _o: Ptr<MoveOnly> = a.as_pointer();
        MoveOnlyImpl::move_assign(&a.as_pointer(), _o)
    });
    assert!(({ (*a.borrow()).v } == 6));
    let vec_: Value<Vec<MoveOnly>> = Rc::new(RefCell::new(Vec::new()));
    {
        let __a1 = MoveOnly::new({ 7 });
        (*vec_.borrow_mut()).push(__a1)
    };
    let d: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ 8 })));
    ({
        MoveOnlyImpl::move_assign(
            &(vec_.as_pointer() as Ptr<MoveOnly>).offset(0_usize),
            d.as_pointer(),
        )
    });
    assert!(
        ({
            (*elem!((vec_.as_pointer() as Ptr<MoveOnly>), 0_usize)
                .upgrade()
                .deref())
            .v
        } == 8)
    );
    assert!(({ (*d.borrow()).v } == 0));
    let m: Value<ConstMoveAssign> = Rc::new(RefCell::new(ConstMoveAssign::new()));
    let m1: Value<ConstMoveAssign> = Rc::new(RefCell::new(ConstMoveAssign::new()));
    let m2: Value<ConstMoveAssign> = Rc::new(RefCell::new(ConstMoveAssign::new()));
    let cm: Value<ConstMoveAssign> = Rc::new(RefCell::new(ConstMoveAssign::new()));
    ({ ConstMoveAssignImpl::operator_assign_2(&m1.as_pointer(), m.as_pointer()) });
    ({ ConstMoveAssignImpl::operator_assign_3(&m2.as_pointer(), cm.as_pointer()) });
    assert!(({ (*m1.borrow()).mark } == 1));
    assert!(({ (*m2.borrow()).mark } == 10));
    return 0;
}
pub trait ConstMoveAssignImpl {
    fn operator_assign_2(&self, o: Ptr<ConstMoveAssign>) -> Ptr<ConstMoveAssign>;
    fn operator_assign_3(&self, o: Ptr<ConstMoveAssign>) -> Ptr<ConstMoveAssign>;
}
impl ConstMoveAssignImpl for Ptr<ConstMoveAssign> {
    fn operator_assign_2(&self, o: Ptr<ConstMoveAssign>) -> Ptr<ConstMoveAssign> {
        field!((*self), mark).write({ (o.with(|__s| __s.mark) + 1) });
        return (*self).clone();
    }
    fn operator_assign_3(&self, o: Ptr<ConstMoveAssign>) -> Ptr<ConstMoveAssign> {
        field!((*self), mark).write({ (o.with(|__s| __s.mark) + 10) });
        return (*self).clone();
    }
}
pub trait MoveOnlyImpl {
    fn move_assign(&self, o: Ptr<MoveOnly>) -> Ptr<MoveOnly>;
}
impl MoveOnlyImpl for Ptr<MoveOnly> {
    fn move_assign(&self, o: Ptr<MoveOnly>) -> Ptr<MoveOnly> {
        if ((*self) == (o)) {
            return (*self).clone();
        }
        field!((*self), v).write({ o.with(|__s| __s.v) });
        field!(o, v).write(0);
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
