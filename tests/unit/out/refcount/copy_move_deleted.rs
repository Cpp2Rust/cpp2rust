extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct NoCopy {
    pub v: Value<i32>,
}
impl NoCopy {
    pub fn NoCopy(v: i32) -> Self {
        let v: Value<i32> = Rc::new(RefCell::new(v));
        let __this: Value<NoCopy> = Rc::new(RefCell::new(Self {
            v: Rc::new(RefCell::new((*v.borrow()))),
        }));
        let this: Ptr<NoCopy> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn move_from(o: Ptr<NoCopy>) -> Self {
        let __this: Value<NoCopy> = Rc::new(RefCell::new(Self {
            v: Rc::new(RefCell::new((*(*o.upgrade().deref()).v.borrow()))),
        }));
        let this: Ptr<NoCopy> = __this.as_pointer();
        (*(*o.upgrade().deref()).v.borrow_mut()) = 0;
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for NoCopy {
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
pub struct PrivateCopy {
    pub v: Value<i32>,
}
impl PrivateCopy {
    pub fn PrivateCopy() -> Self {
        let __this: Value<PrivateCopy> = Rc::new(RefCell::new(Self {
            v: Rc::new(RefCell::new(0)),
        }));
        let this: Ptr<PrivateCopy> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn move_from(o: Ptr<PrivateCopy>) -> Self {
        let __this: Value<PrivateCopy> = Rc::new(RefCell::new(Self {
            v: Rc::new(RefCell::new((*(*o.upgrade().deref()).v.borrow()))),
        }));
        let this: Ptr<PrivateCopy> = __this.as_pointer();
        (*(*o.upgrade().deref()).v.borrow_mut()) = 0;
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for PrivateCopy {
    fn default() -> Self {
        { PrivateCopy::PrivateCopy() }
    }
}
impl ByteRepr for PrivateCopy {
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
pub struct Immovable {
    pub v: Value<i32>,
}
impl Immovable {
    pub fn Immovable() -> Self {
        let __this: Value<Immovable> = Rc::new(RefCell::new(Self {
            v: Rc::new(RefCell::new(0)),
        }));
        let this: Ptr<Immovable> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for Immovable {
    fn default() -> Self {
        { Immovable::Immovable() }
    }
}
impl ByteRepr for Immovable {
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
#[derive(Default)]
pub struct Container {
    pub inner: Value<NoCopy>,
    pub tag: Value<i32>,
}
impl Container {
    pub fn move_from(_a0: Ptr<Container>) -> Self {
        let __this: Value<Container> = Rc::new(RefCell::new(Self {
            inner: Rc::new(RefCell::new(NoCopy::move_from({
                (*_a0.upgrade().deref()).inner.as_pointer()
            }))),
            tag: Rc::new(RefCell::new((*(*_a0.upgrade().deref()).tag.borrow()))),
        }));
        let this: Ptr<Container> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for Container {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.inner.borrow()).to_bytes(&mut buf[0..4]);
        (*self.tag.borrow()).to_bytes(&mut buf[4..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            inner: Rc::new(RefCell::new(<NoCopy>::from_bytes(&buf[0..4]))),
            tag: Rc::new(RefCell::new(<i32>::from_bytes(&buf[4..8]))),
        }
    }
}
pub fn bump_0(p: Ptr<NoCopy>) {
    let p: Value<Ptr<NoCopy>> = Rc::new(RefCell::new(p));
    (*(*(*p.borrow()).upgrade().deref()).v.borrow_mut()).postfix_inc();
}
pub fn bump_ref_1(r: Ptr<Immovable>) {
    (*(*r.upgrade().deref()).v.borrow_mut()).postfix_inc();
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let a: Value<NoCopy> = Rc::new(RefCell::new(NoCopy::NoCopy({ 1 })));
    let b: Value<NoCopy> = Rc::new(RefCell::new(NoCopy::move_from({ a.as_pointer() })));
    assert!(((*(*b.borrow()).v.borrow()) == 1) && ((*(*a.borrow()).v.borrow()) == 0));
    ({ NoCopyImpl::move_assign(&a.as_pointer(), b.as_pointer()) });
    assert!(((*(*a.borrow()).v.borrow()) == 1) && ((*(*b.borrow()).v.borrow()) == 0));
    ({ bump_0((a.as_pointer())) });
    assert!(((*(*a.borrow()).v.borrow()) == 2));
    let p: Value<PrivateCopy> = Rc::new(RefCell::new(PrivateCopy::PrivateCopy()));
    (*(*p.borrow()).v.borrow_mut()) = 3;
    let q: Value<PrivateCopy> = Rc::new(RefCell::new(PrivateCopy::move_from({ p.as_pointer() })));
    assert!(((*(*q.borrow()).v.borrow()) == 3) && ((*(*p.borrow()).v.borrow()) == 0));
    ({ PrivateCopyImpl::move_assign(&p.as_pointer(), q.as_pointer()) });
    assert!(((*(*p.borrow()).v.borrow()) == 3) && ((*(*q.borrow()).v.borrow()) == 0));
    let im: Value<Immovable> = Rc::new(RefCell::new(Immovable::Immovable()));
    (*(*im.borrow()).v.borrow_mut()) = 4;
    ({ bump_ref_1(im.as_pointer()) });
    let pim: Value<Ptr<Immovable>> = Rc::new(RefCell::new((im.as_pointer())));
    assert!(((*(*(*pim.borrow()).upgrade().deref()).v.borrow()) == 5));
    let c: Value<Container> = Rc::new(RefCell::new(Container {
        inner: Rc::new(RefCell::new(NoCopy::NoCopy({ 6 }))),
        tag: Rc::new(RefCell::new(7)),
    }));
    let d: Value<Container> = Rc::new(RefCell::new(Container::move_from({ c.as_pointer() })));
    assert!(
        (((*(*(*d.borrow()).inner.borrow()).v.borrow()) == 6)
            && ((*(*d.borrow()).tag.borrow()) == 7))
            && ((*(*(*c.borrow()).inner.borrow()).v.borrow()) == 0)
    );
    return 0;
}
pub trait ContainerImpl {
    fn move_assign(&self, _a0: Ptr<Container>) -> Ptr<Container>;
}
impl ContainerImpl for Ptr<Container> {
    fn move_assign(&self, _a0: Ptr<Container>) -> Ptr<Container> {
        ({
            let _o: Ptr<NoCopy> = (*_a0.upgrade().deref()).inner.as_pointer();
            NoCopyImpl::move_assign(&(*(*self).upgrade().deref()).inner.as_pointer(), _o)
        });
        let __rhs = (*(*_a0.upgrade().deref()).tag.borrow());
        (*(*(*self).upgrade().deref()).tag.borrow_mut()) = __rhs;
        return (*self).clone();
    }
}
pub trait NoCopyImpl {
    fn move_assign(&self, o: Ptr<NoCopy>) -> Ptr<NoCopy>;
}
impl NoCopyImpl for Ptr<NoCopy> {
    fn move_assign(&self, o: Ptr<NoCopy>) -> Ptr<NoCopy> {
        let __rhs = (*(*o.upgrade().deref()).v.borrow());
        (*(*(*self).upgrade().deref()).v.borrow_mut()) = __rhs;
        (*(*o.upgrade().deref()).v.borrow_mut()) = 0;
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
        let __rhs = (*(*o.upgrade().deref()).v.borrow());
        (*(*(*self).upgrade().deref()).v.borrow_mut()) = __rhs;
        (*(*o.upgrade().deref()).v.borrow_mut()) = 0;
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
