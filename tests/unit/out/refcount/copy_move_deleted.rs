extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Record, ByteRepr, FnPtrArg, Default)]
#[byte_size(4)]
pub struct NoCopy {
    #[offset(0)]
    pub v: i32,
}
impl NoCopy {
    pub fn new(mut v: i32) -> Self {
        Self { v: v }
    }
    pub fn move_from(o: Ptr<NoCopy>) -> Self {
        let __this: NoCopy = Self {
            v: o.with(|__s| __s.v),
        };
        field!(o, v).write(0);
        __this
    }
}
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(4)]
pub struct PrivateCopy {
    #[offset(0)]
    pub v: i32,
}
impl PrivateCopy {
    pub fn new() -> Self {
        Self { v: 0 }
    }
    pub fn move_from(o: Ptr<PrivateCopy>) -> Self {
        let __this: PrivateCopy = Self {
            v: o.with(|__s| __s.v),
        };
        field!(o, v).write(0);
        __this
    }
}
impl Default for PrivateCopy {
    fn default() -> Self {
        { PrivateCopy::new() }
    }
}
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(4)]
pub struct Immovable {
    #[offset(0)]
    pub v: i32,
}
impl Immovable {
    pub fn new() -> Self {
        Self { v: 0 }
    }
}
impl Default for Immovable {
    fn default() -> Self {
        { Immovable::new() }
    }
}
#[derive(Record, ByteRepr, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Container {
    #[offset(0)]
    #[byte_size(4)]
    pub inner: NoCopy,
    #[offset(4)]
    pub tag: i32,
}
impl Container {
    pub fn move_from(_a0: Ptr<Container>) -> Self {
        Self {
            inner: NoCopy::move_from({ field_ptr!(_a0, inner) }),
            tag: { (*_a0.upgrade().deref()).tag },
        }
    }
}
pub fn bump_0(mut p: Ptr<NoCopy>) {
    field!(p, v).with_mut(|__v| __v.postfix_inc());
}
pub fn bump_ref_1(r: Ptr<Immovable>) {
    field!(r, v).with_mut(|__v| __v.postfix_inc());
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let a: Value<NoCopy> = Rc::new(RefCell::new(NoCopy::new({ 1 })));
    let b: Value<NoCopy> = Rc::new(RefCell::new(NoCopy::move_from({ a.as_pointer() })));
    assert!(({ (*b.borrow()).v } == 1) && ({ (*a.borrow()).v } == 0));
    ({ NoCopyImpl::move_assign(&a.as_pointer(), b.as_pointer()) });
    assert!(({ (*a.borrow()).v } == 1) && ({ (*b.borrow()).v } == 0));
    ({ bump_0((a.as_pointer())) });
    assert!(({ (*a.borrow()).v } == 2));
    let p: Value<PrivateCopy> = Rc::new(RefCell::new(PrivateCopy::new()));
    (*p.borrow_mut()).v = 3;
    let q: Value<PrivateCopy> = Rc::new(RefCell::new(PrivateCopy::move_from({ p.as_pointer() })));
    assert!(({ (*q.borrow()).v } == 3) && ({ (*p.borrow()).v } == 0));
    ({ PrivateCopyImpl::move_assign(&p.as_pointer(), q.as_pointer()) });
    assert!(({ (*p.borrow()).v } == 3) && ({ (*q.borrow()).v } == 0));
    let im: Value<Immovable> = Rc::new(RefCell::new(Immovable::new()));
    (*im.borrow_mut()).v = 4;
    ({ bump_ref_1(im.as_pointer()) });
    let mut pim: Ptr<Immovable> = (im.as_pointer());
    assert!((pim.with(|__s| __s.v) == 5));
    let c: Value<Container> = Rc::new(RefCell::new(Container {
        inner: NoCopy::new({ 6 }),
        tag: 7,
    }));
    let mut d: Container = Container::move_from({ c.as_pointer() });
    assert!(((d.inner.v == 6) && (d.tag == 7)) && ({ (*c.borrow()).inner.v } == 0));
    return 0;
}
pub trait ContainerImpl {
    fn move_assign(&self, _a0: Ptr<Container>) -> Ptr<Container>;
}
impl ContainerImpl for Ptr<Container> {
    fn move_assign(&self, _a0: Ptr<Container>) -> Ptr<Container> {
        ({
            let _o: Ptr<NoCopy> = field_ptr!(_a0, inner);
            NoCopyImpl::move_assign(&field_ptr!((*self), inner), _o)
        });
        field!((*self), tag).write({ { (*_a0.upgrade().deref()).tag } });
        return (*self).clone();
    }
}
pub trait NoCopyImpl {
    fn move_assign(&self, o: Ptr<NoCopy>) -> Ptr<NoCopy>;
}
impl NoCopyImpl for Ptr<NoCopy> {
    fn move_assign(&self, o: Ptr<NoCopy>) -> Ptr<NoCopy> {
        field!((*self), v).write({ o.with(|__s| __s.v) });
        field!(o, v).write(0);
        return (*self).clone();
    }
}
pub trait PrivateCopyImpl {
    fn copy_assign(&self, _a0: Ptr<PrivateCopy>) -> Ptr<PrivateCopy> {
        unimplemented!()
    }
    fn move_assign(&self, o: Ptr<PrivateCopy>) -> Ptr<PrivateCopy>;
}
impl PrivateCopyImpl for Ptr<PrivateCopy> {
    fn move_assign(&self, o: Ptr<PrivateCopy>) -> Ptr<PrivateCopy> {
        field!((*self), v).write({ o.with(|__s| __s.v) });
        field!(o, v).write(0);
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
