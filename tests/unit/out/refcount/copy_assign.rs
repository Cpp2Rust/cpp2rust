extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static assigns_0: Value<i32> = Rc::new(RefCell::new(0));
);
#[derive(Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Partial {
    #[offset(0)]
    pub v: i32,
    #[offset(4)]
    pub keep: i32,
}
impl Partial {
    pub fn new(v: i32, keep: i32) -> Self {
        let v: Value<i32> = Rc::new(RefCell::new(v));
        let keep: Value<i32> = Rc::new(RefCell::new(keep));
        let __this: Value<Partial> = Rc::new(RefCell::new(Self {
            v: (*v.borrow()),
            keep: (*keep.borrow()),
        }));
        let this: Ptr<Partial> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn copy_from(o: Ptr<Partial>) -> Self {
        let __this: Value<Partial> = Rc::new(RefCell::new(Self {
            v: o.with(|__s| __s.v),
            keep: o.with(|__s| __s.keep),
        }));
        let this: Ptr<Partial> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for Partial {
    fn clone(&self) -> Self {
        let __src: Value<Partial> = Rc::new(RefCell::new(Partial {
            v: self.v.clone(),
            keep: self.keep.clone(),
        }));
        Partial::copy_from(__src.as_pointer())
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct NonConstAssign {
    #[offset(0)]
    pub mark: i32,
}
impl NonConstAssign {
    pub fn new() -> Self {
        let __this: Value<NonConstAssign> = Rc::new(RefCell::new(Self { mark: 0 }));
        let this: Ptr<NonConstAssign> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for NonConstAssign {
    fn default() -> Self {
        { NonConstAssign::new() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct RefQualified {
    #[offset(0)]
    pub mark: i32,
}
impl RefQualified {
    pub fn new() -> Self {
        let __this: Value<RefQualified> = Rc::new(RefCell::new(Self { mark: 0 }));
        let this: Ptr<RefQualified> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for RefQualified {
    fn default() -> Self {
        { RefQualified::new() }
    }
}
#[derive(DeepClone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(24)]
pub struct Holder {
    #[offset(0)]
    #[byte_size(8)]
    pub p: Partial,
    #[offset(8)]
    #[byte_size(16)]
    pub arr: Value<Box<[Partial]>>,
}
impl Default for Holder {
    fn default() -> Self {
        Holder {
            p: <Partial>::default(),
            arr: Rc::new(RefCell::new(
                (0..2)
                    .map(|_| <Partial>::default())
                    .collect::<Box<[Partial]>>(),
            )),
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let a: Value<Partial> = Rc::new(RefCell::new(Partial::new({ 1 }, { 100 })));
    let b: Value<Partial> = Rc::new(RefCell::new(Partial::new({ 2 }, { 200 })));
    let c: Value<Partial> = Rc::new(RefCell::new(Partial::new({ 3 }, { 300 })));
    ({ PartialImpl::copy_assign(&a.as_pointer(), b.as_pointer()) });
    assert!(({ (*a.borrow()).v } == 2) && ({ (*a.borrow()).keep } == 100));
    assert!((assigns_0.with(|rc| *rc.borrow()) == 1));
    ({
        PartialImpl::copy_assign(
            &c.as_pointer(),
            ({ PartialImpl::copy_assign(&a.as_pointer(), b.as_pointer()) }),
        )
    });
    assert!(({ (*c.borrow()).v } == 2) && ({ (*c.borrow()).keep } == 300));
    assert!((assigns_0.with(|rc| *rc.borrow()) == 3));
    ({
        let _o: Ptr<Partial> = a.as_pointer();
        PartialImpl::copy_assign(&a.as_pointer(), _o)
    });
    assert!((assigns_0.with(|rc| *rc.borrow()) == 3));
    ({
        let _o: Value<Partial> = Rc::new(RefCell::new(Partial::new({ 9 }, { 900 })));
        PartialImpl::copy_assign(&a.as_pointer(), _o.as_pointer())
    });
    assert!(({ (*a.borrow()).v } == 9) && ({ (*a.borrow()).keep } == 100));
    assert!((assigns_0.with(|rc| *rc.borrow()) == 4));
    let ra: Ptr<Partial> = a.as_pointer();
    ({
        let _o: Ptr<Partial> = c.as_pointer();
        PartialImpl::copy_assign(&ra, _o)
    });
    assert!(({ (*a.borrow()).v } == 2));
    let pa: Value<Ptr<Partial>> = Rc::new(RefCell::new((a.as_pointer())));
    ({
        let _o: Ptr<Partial> = b.as_pointer();
        PartialImpl::copy_assign(&(*pa.borrow()), _o)
    });
    assert!(({ (*a.borrow()).v } == 2));
    assert!((assigns_0.with(|rc| *rc.borrow()) == 6));
    let h: Value<Holder> = Rc::new(RefCell::new(Holder {
        p: Partial::new({ 4 }, { 40 }),
        arr: Rc::new(RefCell::new(Box::new([
            Partial::new({ 5 }, { 50 }),
            Partial::new({ 6 }, { 60 }),
        ]))),
    }));
    ({ PartialImpl::copy_assign(&field_ptr!(h.as_pointer(), p), b.as_pointer()) });
    ({
        PartialImpl::copy_assign(
            &(array_field_ptr!(h.as_pointer(), arr) as Ptr<Partial>).offset((1) as isize),
            c.as_pointer(),
        )
    });
    assert!(({ (*h.borrow()).p.v } == 2) && ({ (*h.borrow()).p.keep } == 40));
    assert!(
        ({
            (*elem!((array_field_ptr!(h.as_pointer(), arr) as Ptr<Partial>), 1)
                .upgrade()
                .deref())
            .v
        } == 2)
            && ({
                (*elem!((array_field_ptr!(h.as_pointer(), arr) as Ptr<Partial>), 1)
                    .upgrade()
                    .deref())
                .keep
            } == 60)
    );
    assert!((assigns_0.with(|rc| *rc.borrow()) == 8));
    let n: Value<NonConstAssign> = Rc::new(RefCell::new(NonConstAssign::new()));
    let n1: Value<NonConstAssign> = Rc::new(RefCell::new(NonConstAssign::new()));
    let n2: Value<NonConstAssign> = Rc::new(RefCell::new(NonConstAssign::new()));
    let cn: Value<NonConstAssign> = Rc::new(RefCell::new(NonConstAssign::new()));
    ({ NonConstAssignImpl::operator_assign_2(&n1.as_pointer(), n.as_pointer()) });
    ({ NonConstAssignImpl::operator_assign_3(&n2.as_pointer(), cn.as_pointer()) });
    assert!(({ (*n1.borrow()).mark } == 1));
    assert!(({ (*n2.borrow()).mark } == 10));
    let r: Value<RefQualified> = Rc::new(RefCell::new(RefQualified::new()));
    let r1: Value<RefQualified> = Rc::new(RefCell::new(RefQualified::new()));
    ({ RefQualifiedImpl::copy_assign(&r1.as_pointer(), r.as_pointer()) });
    assert!(({ (*r1.borrow()).mark } == 1));
    return 0;
}
pub trait NonConstAssignImpl {
    fn operator_assign_2(&self, o: Ptr<NonConstAssign>) -> Ptr<NonConstAssign>;
    fn operator_assign_3(&self, o: Ptr<NonConstAssign>) -> Ptr<NonConstAssign>;
}
impl NonConstAssignImpl for Ptr<NonConstAssign> {
    fn operator_assign_2(&self, o: Ptr<NonConstAssign>) -> Ptr<NonConstAssign> {
        field!((*self), mark).write({ (o.with(|__s| __s.mark) + 1) });
        return (*self).clone();
    }
    fn operator_assign_3(&self, o: Ptr<NonConstAssign>) -> Ptr<NonConstAssign> {
        field!((*self), mark).write({ (o.with(|__s| __s.mark) + 10) });
        return (*self).clone();
    }
}
pub trait PartialImpl {
    fn copy_assign(&self, o: Ptr<Partial>) -> Ptr<Partial>;
}
impl PartialImpl for Ptr<Partial> {
    fn copy_assign(&self, o: Ptr<Partial>) -> Ptr<Partial> {
        if ((*self) == (o)) {
            return (*self).clone();
        }
        field!((*self), v).write({ o.with(|__s| __s.v) });
        (*assigns_0.with(Value::clone).borrow_mut()).prefix_inc();
        return (*self).clone();
    }
}
pub trait RefQualifiedImpl {
    fn copy_assign(&self, o: Ptr<RefQualified>) -> Ptr<RefQualified>;
}
impl RefQualifiedImpl for Ptr<RefQualified> {
    fn copy_assign(&self, o: Ptr<RefQualified>) -> Ptr<RefQualified> {
        field!((*self), mark).write({ (o.with(|__s| __s.mark) + 1) });
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = assigns_0.with(|_| ());
}
