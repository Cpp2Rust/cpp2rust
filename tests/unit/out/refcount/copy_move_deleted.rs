extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Record, Default)]
pub struct NoCopy {
    #[offset(0)]
    pub v: i32,
}
impl NoCopy {
    pub fn new(v: i32) -> Self {
        let v: Value<i32> = Rc::new(RefCell::new(v));
        let __this: Value<NoCopy> = Rc::new(RefCell::new(Self { v: (*v.borrow()) }));
        let this: Ptr<NoCopy> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn move_from(o: Ptr<NoCopy>) -> Self {
        let __this: Value<NoCopy> = Rc::new(RefCell::new(Self {
            v: o.with(|__s| __s.v),
        }));
        let this: Ptr<NoCopy> = __this.as_pointer();
        field!(o, v).write(0);
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
<<<<<<< HEAD
impl ByteRepr for NoCopy {}
#[derive()]
=======
impl ByteRepr for NoCopy {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.v.to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            v: <i32>::from_bytes(&buf[0..4]),
        }
    }
}
#[derive(Record)]
>>>>>>> 3ed38b58 (Remove Value<> boxing from struct fields)
pub struct PrivateCopy {
    #[offset(0)]
    pub v: i32,
}
impl PrivateCopy {
    pub fn new() -> Self {
        let __this: Value<PrivateCopy> = Rc::new(RefCell::new(Self { v: 0 }));
        let this: Ptr<PrivateCopy> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn move_from(o: Ptr<PrivateCopy>) -> Self {
        let __this: Value<PrivateCopy> = Rc::new(RefCell::new(Self {
            v: o.with(|__s| __s.v),
        }));
        let this: Ptr<PrivateCopy> = __this.as_pointer();
        field!(o, v).write(0);
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for PrivateCopy {
    fn default() -> Self {
        { PrivateCopy::new() }
    }
}
<<<<<<< HEAD
impl ByteRepr for PrivateCopy {}
#[derive()]
=======
impl ByteRepr for PrivateCopy {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.v.to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            v: <i32>::from_bytes(&buf[0..4]),
        }
    }
}
#[derive(Record)]
>>>>>>> 3ed38b58 (Remove Value<> boxing from struct fields)
pub struct Immovable {
    #[offset(0)]
    pub v: i32,
}
impl Immovable {
    pub fn new() -> Self {
        let __this: Value<Immovable> = Rc::new(RefCell::new(Self { v: 0 }));
        let this: Ptr<Immovable> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for Immovable {
    fn default() -> Self {
        { Immovable::new() }
    }
}
impl ByteRepr for Immovable {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.v.to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            v: <i32>::from_bytes(&buf[0..4]),
        }
    }
}
#[derive(Record, Default)]
pub struct Container {
    #[offset(0)]
    pub inner: NoCopy,
    #[offset(4)]
    pub tag: i32,
}
impl Container {
    pub fn move_from(_a0: Ptr<Container>) -> Self {
        let __this: Value<Container> = Rc::new(RefCell::new(Self {
            inner: NoCopy::move_from({ field_ptr!(_a0, inner) }),
            tag: { (*_a0.upgrade().deref()).tag },
        }));
        let this: Ptr<Container> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
<<<<<<< HEAD
impl ByteRepr for Container {}
=======
impl ByteRepr for Container {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.inner.to_bytes(&mut buf[0..4]);
        self.tag.to_bytes(&mut buf[4..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            inner: <NoCopy>::from_bytes(&buf[0..4]),
            tag: <i32>::from_bytes(&buf[4..8]),
        }
    }
}
>>>>>>> 3ed38b58 (Remove Value<> boxing from struct fields)
pub fn bump_0(p: Ptr<NoCopy>) {
    let p: Value<Ptr<NoCopy>> = Rc::new(RefCell::new(p));
    field!((*p.borrow()), v).with_mut(|__v| __v.postfix_inc());
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
    let pim: Value<Ptr<Immovable>> = Rc::new(RefCell::new((im.as_pointer())));
    assert!(((*pim.borrow()).with(|__s| __s.v) == 5));
    let c: Value<Container> = Rc::new(RefCell::new(Container {
        inner: NoCopy::new({ 6 }),
        tag: 7,
    }));
    let d: Value<Container> = Rc::new(RefCell::new(Container::move_from({ c.as_pointer() })));
    assert!(
        (({ (*d.borrow()).inner.v } == 6) && ({ (*d.borrow()).tag } == 7))
            && ({ (*c.borrow()).inner.v } == 0)
    );
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
        let __rhs = { (*_a0.upgrade().deref()).tag };
        field!((*self), tag).write(__rhs);
        return (*self).clone();
    }
}
pub trait NoCopyImpl {
    fn move_assign(&self, o: Ptr<NoCopy>) -> Ptr<NoCopy>;
}
impl NoCopyImpl for Ptr<NoCopy> {
    fn move_assign(&self, o: Ptr<NoCopy>) -> Ptr<NoCopy> {
        let __rhs = o.with(|__s| __s.v);
        field!((*self), v).write(__rhs);
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
        let __rhs = o.with(|__s| __s.v);
        field!((*self), v).write(__rhs);
        field!(o, v).write(0);
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
