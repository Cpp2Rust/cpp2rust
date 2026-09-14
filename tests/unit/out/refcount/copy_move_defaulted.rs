extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct Inner {
    pub x: Value<i32>,
}
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
#[derive()]
pub struct Explicit {
    pub v: Value<i32>,
    pub inner: Value<Inner>,
    pub arr: Value<Box<[i32]>>,
}
impl Explicit {
    pub fn Explicit(v: i32) -> Self {
        let v: Value<i32> = Rc::new(RefCell::new(v));
        let __this: Value<Explicit> = Rc::new(RefCell::new(Self {
            v: Rc::new(RefCell::new((*v.borrow()))),
            inner: Rc::new(RefCell::new(Inner {
                x: Rc::new(RefCell::new(((*v.borrow()) * 10))),
            })),
            arr: Rc::new(RefCell::new(Box::new([(*v.borrow()), ((*v.borrow()) + 1)]))),
        }));
        let this: Ptr<Explicit> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for Explicit {
    fn clone(&self) -> Self {
        let __this: Value<Explicit> = Rc::new(RefCell::new(Self {
            v: Rc::new(RefCell::new((*self.v.borrow()))),
            inner: Rc::new(RefCell::new((*self.inner.borrow()).clone())),
            arr: Rc::new(RefCell::new(Box::new(std::array::from_fn::<_, 2, _>(
                |__i: usize| (*self.arr.borrow())[(__i) as usize],
            )))),
        }));
        let this: Ptr<Explicit> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for Explicit {
    fn default() -> Self {
        Explicit {
            v: <Value<i32>>::default(),
            inner: <Value<Inner>>::default(),
            arr: Rc::new(RefCell::new(
                (0..2).map(|_| <i32>::default()).collect::<Box<[i32]>>(),
            )),
        }
    }
}
impl ByteRepr for Explicit {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.v.borrow()).to_bytes(&mut buf[0..4]);
        (*self.inner.borrow()).to_bytes(&mut buf[4..8]);
        (*self.arr.borrow()).to_bytes(&mut buf[8..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            v: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
            inner: Rc::new(RefCell::new(<Inner>::from_bytes(&buf[4..8]))),
            arr: Rc::new(RefCell::new(<Box<[i32]>>::from_bytes(&buf[8..16]))),
        }
    }
}
#[derive()]
pub struct Implicit {
    pub v: Value<i32>,
    pub inner: Value<Inner>,
    pub arr: Value<Box<[i32]>>,
}
impl Clone for Implicit {
    fn clone(&self) -> Self {
        let __this: Value<Implicit> = Rc::new(RefCell::new(Self {
            v: Rc::new(RefCell::new((*self.v.borrow()))),
            inner: Rc::new(RefCell::new((*self.inner.borrow()).clone())),
            arr: Rc::new(RefCell::new(Box::new(std::array::from_fn::<_, 2, _>(
                |__i: usize| (*self.arr.borrow())[(__i) as usize],
            )))),
        }));
        let this: Ptr<Implicit> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for Implicit {
    fn default() -> Self {
        Implicit {
            v: <Value<i32>>::default(),
            inner: <Value<Inner>>::default(),
            arr: Rc::new(RefCell::new(
                (0..2).map(|_| <i32>::default()).collect::<Box<[i32]>>(),
            )),
        }
    }
}
impl ByteRepr for Implicit {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.v.borrow()).to_bytes(&mut buf[0..4]);
        (*self.inner.borrow()).to_bytes(&mut buf[4..8]);
        (*self.arr.borrow()).to_bytes(&mut buf[8..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            v: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
            inner: Rc::new(RefCell::new(<Inner>::from_bytes(&buf[4..8]))),
            arr: Rc::new(RefCell::new(<Box<[i32]>>::from_bytes(&buf[8..16]))),
        }
    }
}
#[derive(Default)]
pub struct DefaultCopyUserMove {
    pub v: Value<i32>,
}
impl DefaultCopyUserMove {
    pub fn DefaultCopyUserMove(v: i32) -> Self {
        let v: Value<i32> = Rc::new(RefCell::new(v));
        let __this: Value<DefaultCopyUserMove> = Rc::new(RefCell::new(Self {
            v: Rc::new(RefCell::new((*v.borrow()))),
        }));
        let this: Ptr<DefaultCopyUserMove> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn DefaultCopyUserMove_pmutDefaultCopyUserMove(o: Ptr<DefaultCopyUserMove>) -> Self {
        let __this: Value<DefaultCopyUserMove> = Rc::new(RefCell::new(Self {
            v: Rc::new(RefCell::new((*(*o.upgrade().deref()).v.borrow()))),
        }));
        let this: Ptr<DefaultCopyUserMove> = __this.as_pointer();
        (*(*o.upgrade().deref()).v.borrow_mut()) = 0;
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for DefaultCopyUserMove {
    fn clone(&self) -> Self {
        let __this: Value<DefaultCopyUserMove> = Rc::new(RefCell::new(Self {
            v: Rc::new(RefCell::new((*self.v.borrow()))),
        }));
        let this: Ptr<DefaultCopyUserMove> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for DefaultCopyUserMove {
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
pub struct UserCopyDefaultMove {
    pub v: Value<i32>,
}
impl UserCopyDefaultMove {
    pub fn UserCopyDefaultMove(v: i32) -> Self {
        let v: Value<i32> = Rc::new(RefCell::new(v));
        let __this: Value<UserCopyDefaultMove> = Rc::new(RefCell::new(Self {
            v: Rc::new(RefCell::new((*v.borrow()))),
        }));
        let this: Ptr<UserCopyDefaultMove> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn UserCopyDefaultMove_pconstUserCopyDefaultMove(o: Ptr<UserCopyDefaultMove>) -> Self {
        let __this: Value<UserCopyDefaultMove> = Rc::new(RefCell::new(Self {
            v: Rc::new(RefCell::new(((*(*o.upgrade().deref()).v.borrow()) + 100))),
        }));
        let this: Ptr<UserCopyDefaultMove> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn UserCopyDefaultMove_pmutUserCopyDefaultMove(_a0: Ptr<UserCopyDefaultMove>) -> Self {
        let __this: Value<UserCopyDefaultMove> = Rc::new(RefCell::new(Self {
            v: Rc::new(RefCell::new((*(*_a0.upgrade().deref()).v.borrow()))),
        }));
        let this: Ptr<UserCopyDefaultMove> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for UserCopyDefaultMove {
    fn clone(&self) -> Self {
        let __src: Value<UserCopyDefaultMove> =
            Rc::new(RefCell::new(UserCopyDefaultMove { v: self.v.clone() }));
        UserCopyDefaultMove::UserCopyDefaultMove_pconstUserCopyDefaultMove(__src.as_pointer())
    }
}
impl ByteRepr for UserCopyDefaultMove {
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
pub struct Buffer {
    pub data: Value<Vec<i32>>,
    pub rows: Value<Vec<Value<Vec<i32>>>>,
    pub n: Value<i32>,
    pub arr: Value<Box<[i32]>>,
}
impl Buffer {
    pub fn Buffer(n: i32) -> Self {
        let n: Value<i32> = Rc::new(RefCell::new(n));
        let __this: Value<Buffer> = Rc::new(RefCell::new(Self {
            data: Rc::new(RefCell::new(vec![
                (*n.borrow());
                ((*n.borrow()) as usize) as usize
            ])),
            rows: Rc::new(RefCell::new(Vec::new())),
            n: Rc::new(RefCell::new((*n.borrow()))),
            arr: Rc::new(RefCell::new(Box::new([(*n.borrow()), ((*n.borrow()) + 1)]))),
        }));
        let this: Ptr<Buffer> = __this.as_pointer();
        ((*this.upgrade().deref()).rows.as_pointer() as Ptr<Vec<Value<Vec<i32>>>>).with_mut(
            |__v: &mut Vec<Value<Vec<i32>>>| {
                __v.push(Rc::new(RefCell::new(
                    (*(*this.upgrade().deref()).data.borrow()).clone(),
                )))
            },
        );
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn Buffer_pmutBuffer(_a0: Ptr<Buffer>) -> Self {
        let __this: Value<Buffer> = Rc::new(RefCell::new(Self {
            data: Rc::new(RefCell::new(std::mem::take(
                &mut (*(*_a0.upgrade().deref()).data.borrow_mut()),
            ))),
            rows: Rc::new(RefCell::new(std::mem::take(
                &mut (*(*_a0.upgrade().deref()).rows.borrow_mut()),
            ))),
            n: Rc::new(RefCell::new((*(*_a0.upgrade().deref()).n.borrow()))),
            arr: Rc::new(RefCell::new(Box::new(std::array::from_fn::<_, 2, _>(
                |__i: usize| (*(*_a0.upgrade().deref()).arr.borrow())[(__i) as usize],
            )))),
        }));
        let this: Ptr<Buffer> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for Buffer {
    fn default() -> Self {
        Buffer {
            data: Rc::new(RefCell::new(Default::default())),
            rows: Rc::new(RefCell::new(Vec::new())),
            n: <Value<i32>>::default(),
            arr: Rc::new(RefCell::new(
                (0..2).map(|_| <i32>::default()).collect::<Box<[i32]>>(),
            )),
        }
    }
}
impl ByteRepr for Buffer {
    fn byte_size() -> usize {
        64
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.data.borrow()).to_bytes(&mut buf[0..24]);
        (*self.rows.borrow()).to_bytes(&mut buf[24..48]);
        (*self.n.borrow()).to_bytes(&mut buf[48..52]);
        (*self.arr.borrow()).to_bytes(&mut buf[52..60]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            data: Rc::new(RefCell::new(<Vec<i32>>::from_bytes(&buf[0..24]))),
            rows: Rc::new(RefCell::new(<Vec<Value<Vec<i32>>>>::from_bytes(
                &buf[24..48],
            ))),
            n: Rc::new(RefCell::new(<i32>::from_bytes(&buf[48..52]))),
            arr: Rc::new(RefCell::new(<Box<[i32]>>::from_bytes(&buf[52..60]))),
        }
    }
}
#[derive()]
pub struct Owner {
    pub data: Value<Vec<i32>>,
    pub n: Value<i32>,
    pub arr: Value<Box<[i32]>>,
    pub p: Value<Option<Value<i32>>>,
}
impl Owner {
    pub fn Owner_pmutOwner(_a0: Ptr<Owner>) -> Self {
        let __this: Value<Owner> = Rc::new(RefCell::new(Self {
            data: Rc::new(RefCell::new(std::mem::take(
                &mut (*(*_a0.upgrade().deref()).data.borrow_mut()),
            ))),
            n: Rc::new(RefCell::new((*(*_a0.upgrade().deref()).n.borrow()))),
            arr: Rc::new(RefCell::new(Box::new(std::array::from_fn::<_, 2, _>(
                |__i: usize| (*(*_a0.upgrade().deref()).arr.borrow())[(__i) as usize],
            )))),
            p: Rc::new(RefCell::new(
                (*(*_a0.upgrade().deref()).p.borrow_mut()).take(),
            )),
        }));
        let this: Ptr<Owner> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for Owner {
    fn default() -> Self {
        Owner {
            data: Rc::new(RefCell::new(Default::default())),
            n: <Value<i32>>::default(),
            arr: Rc::new(RefCell::new(
                (0..2).map(|_| <i32>::default()).collect::<Box<[i32]>>(),
            )),
            p: Rc::new(RefCell::new(None)),
        }
    }
}
impl ByteRepr for Owner {
    fn byte_size() -> usize {
        48
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.data.borrow()).to_bytes(&mut buf[0..24]);
        (*self.n.borrow()).to_bytes(&mut buf[24..28]);
        (*self.arr.borrow()).to_bytes(&mut buf[28..36]);
        (*self.p.borrow()).to_bytes(&mut buf[40..48]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            data: Rc::new(RefCell::new(<Vec<i32>>::from_bytes(&buf[0..24]))),
            n: Rc::new(RefCell::new(<i32>::from_bytes(&buf[24..28]))),
            arr: Rc::new(RefCell::new(<Box<[i32]>>::from_bytes(&buf[28..36]))),
            p: Rc::new(RefCell::new(<Option<Value<i32>>>::from_bytes(&buf[40..48]))),
        }
    }
}
#[derive(Default)]
pub struct Holder {
    pub inner: Value<Inner>,
    pub e: Value<Explicit>,
    pub p: Value<Option<Value<i32>>>,
}
impl Holder {
    pub fn Holder(v: i32) -> Self {
        let v: Value<i32> = Rc::new(RefCell::new(v));
        let __this: Value<Holder> = Rc::new(RefCell::new(Self {
            inner: Rc::new(RefCell::new(Inner {
                x: Rc::new(RefCell::new((*v.borrow()))),
            })),
            e: Rc::new(RefCell::new(Explicit::Explicit({ (*v.borrow()) }))),
            p: Rc::new(RefCell::new(None)),
        }));
        let this: Ptr<Holder> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn Holder_pmutHolder(_a0: Ptr<Holder>) -> Self {
        let __this: Value<Holder> = Rc::new(RefCell::new(Self {
            inner: Rc::new(RefCell::new(
                (*(*_a0.upgrade().deref()).inner.borrow()).clone(),
            )),
            e: Rc::new(RefCell::new((*(*_a0.upgrade().deref()).e.borrow()).clone())),
            p: Rc::new(RefCell::new(
                (*(*_a0.upgrade().deref()).p.borrow_mut()).take(),
            )),
        }));
        let this: Ptr<Holder> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for Holder {
    fn byte_size() -> usize {
        32
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.inner.borrow()).to_bytes(&mut buf[0..4]);
        (*self.e.borrow()).to_bytes(&mut buf[4..20]);
        (*self.p.borrow()).to_bytes(&mut buf[24..32]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            inner: Rc::new(RefCell::new(<Inner>::from_bytes(&buf[0..4]))),
            e: Rc::new(RefCell::new(<Explicit>::from_bytes(&buf[4..20]))),
            p: Rc::new(RefCell::new(<Option<Value<i32>>>::from_bytes(&buf[24..32]))),
        }
    }
}
pub fn same_0(a: Ptr<Explicit>, b: Ptr<Explicit>) -> bool {
    return ((({
        let _lhs = (*(*a.upgrade().deref()).v.borrow());
        _lhs == (*(*b.upgrade().deref()).v.borrow())
    }) && ({
        let _lhs = (*(*(*a.upgrade().deref()).inner.borrow()).x.borrow());
        _lhs == (*(*(*b.upgrade().deref()).inner.borrow()).x.borrow())
    })) && ({
        let _lhs = (*(*a.upgrade().deref()).arr.borrow())[(0) as usize];
        _lhs == (*(*b.upgrade().deref()).arr.borrow())[(0) as usize]
    })) && ({
        let _lhs = (*(*a.upgrade().deref()).arr.borrow())[(1) as usize];
        _lhs == (*(*b.upgrade().deref()).arr.borrow())[(1) as usize]
    });
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let a: Value<Explicit> = Rc::new(RefCell::new(Explicit::Explicit({ 1 })));
    let _dtor_a = ScopedDestructor::new(&a, |__p| __p.destructor());
    let b: Value<Explicit> = Rc::new(RefCell::new((*a.borrow()).clone()));
    let _dtor_b = ScopedDestructor::new(&b, |__p| __p.destructor());
    let c: Value<Explicit> = Rc::new(RefCell::new((*a.borrow()).clone()));
    let _dtor_c = ScopedDestructor::new(&c, |__p| __p.destructor());
    let d: Value<Explicit> = Rc::new(RefCell::new((*a.borrow()).clone()));
    let _dtor_d = ScopedDestructor::new(&d, |__p| __p.destructor());
    assert!(
        (({ same_0(b.as_pointer(), a.as_pointer(),) })
            && ({ same_0(c.as_pointer(), a.as_pointer(),) }))
            && ({ same_0(d.as_pointer(), a.as_pointer(),) })
    );
    let e: Value<Explicit> = Rc::new(RefCell::new(Explicit::Explicit({ 2 })));
    let _dtor_e = ScopedDestructor::new(&e, |__p| __p.destructor());
    let f: Value<Explicit> = Rc::new(RefCell::new(Explicit::Explicit({ 3 })));
    let _dtor_f = ScopedDestructor::new(&f, |__p| __p.destructor());
    (*e.borrow_mut()) = (*b.borrow()).clone();
    (*f.borrow_mut()) = (*c.borrow()).clone();
    assert!(
        ({ same_0(e.as_pointer(), b.as_pointer(),) })
            && ({ same_0(f.as_pointer(), c.as_pointer(),) })
    );
    let g: Value<Explicit> = Rc::new(RefCell::new(Explicit::Explicit({ 4 })));
    let _dtor_g = ScopedDestructor::new(&g, |__p| __p.destructor());
    (*g.borrow_mut()) = {
        (*e.borrow_mut()) = (*f.borrow()).clone();
        (*e.borrow()).clone()
    };
    assert!(
        ({ same_0(g.as_pointer(), f.as_pointer(),) })
            && ({ same_0(e.as_pointer(), f.as_pointer(),) })
    );
    let i: Value<Implicit> = Rc::new(RefCell::new(Implicit {
        v: Rc::new(RefCell::new(5)),
        inner: Rc::new(RefCell::new(Inner {
            x: Rc::new(RefCell::new(50)),
        })),
        arr: Rc::new(RefCell::new(Box::new([5, 6]))),
    }));
    let j: Value<Implicit> = Rc::new(RefCell::new((*i.borrow()).clone()));
    let k: Value<Implicit> = Rc::new(RefCell::new((*i.borrow()).clone()));
    assert!(
        (((*(*j.borrow()).v.borrow()) == 5)
            && ((*(*(*j.borrow()).inner.borrow()).x.borrow()) == 50))
            && ((*(*j.borrow()).arr.borrow())[(1) as usize] == 6)
    );
    assert!(((*(*i.borrow()).v.borrow()) == 5) && ((*(*k.borrow()).v.borrow()) == 5));
    let l: Value<Implicit> = Rc::new(RefCell::new(Implicit {
        v: Rc::new(RefCell::new(0)),
        inner: Rc::new(RefCell::new(Inner {
            x: Rc::new(RefCell::new(0)),
        })),
        arr: Rc::new(RefCell::new(Box::new([0, 0]))),
    }));
    (*l.borrow_mut()) = (*j.borrow()).clone();
    assert!(
        (((*(*l.borrow()).v.borrow()) == 5)
            && ((*(*(*l.borrow()).inner.borrow()).x.borrow()) == 50))
            && ((*(*l.borrow()).arr.borrow())[(0) as usize] == 5)
    );
    let vec_: Value<Vec<Explicit>> = Rc::new(RefCell::new(Vec::new()));
    {
        let a0_clone = (*b.borrow()).clone();
        (*vec_.borrow_mut()).push(a0_clone)
    };
    (*vec_.borrow_mut()).push(Explicit::Explicit({ 9 }));
    assert!(
        ((*(*(vec_.as_pointer() as Ptr<Explicit>)
            .offset(0_usize)
            .upgrade()
            .deref())
        .v
        .borrow())
            == 1)
            && ((*(*(vec_.as_pointer() as Ptr<Explicit>)
                .offset(1_usize)
                .upgrade()
                .deref())
            .v
            .borrow())
                == 9)
    );
    let m: Value<DefaultCopyUserMove> =
        Rc::new(RefCell::new(DefaultCopyUserMove::DefaultCopyUserMove({
            7
        })));
    let m1: Value<DefaultCopyUserMove> = Rc::new(RefCell::new((*m.borrow()).clone()));
    let m2: Value<DefaultCopyUserMove> = Rc::new(RefCell::new(
        DefaultCopyUserMove::DefaultCopyUserMove_pmutDefaultCopyUserMove({ m.as_pointer() }),
    ));
    assert!(
        (((*(*m1.borrow()).v.borrow()) == 7) && ((*(*m2.borrow()).v.borrow()) == 7))
            && ((*(*m.borrow()).v.borrow()) == 0)
    );
    let m3: Value<DefaultCopyUserMove> =
        Rc::new(RefCell::new(DefaultCopyUserMove::DefaultCopyUserMove({
            1
        })));
    let m4: Value<DefaultCopyUserMove> =
        Rc::new(RefCell::new(DefaultCopyUserMove::DefaultCopyUserMove({
            1
        })));
    (*m3.borrow_mut()) = (*m1.borrow()).clone();
    ({
        DefaultCopyUserMoveImpl::operator_assign_pmutDefaultCopyUserMove(
            &m4.as_pointer(),
            m1.as_pointer(),
        )
    });
    assert!(
        (((*(*m3.borrow()).v.borrow()) == 7) && ((*(*m4.borrow()).v.borrow()) == 7))
            && ((*(*m1.borrow()).v.borrow()) == 0)
    );
    let u: Value<UserCopyDefaultMove> =
        Rc::new(RefCell::new(UserCopyDefaultMove::UserCopyDefaultMove({
            8
        })));
    let u1: Value<UserCopyDefaultMove> = Rc::new(RefCell::new(
        UserCopyDefaultMove::UserCopyDefaultMove_pconstUserCopyDefaultMove({ u.as_pointer() }),
    ));
    let u2: Value<UserCopyDefaultMove> = Rc::new(RefCell::new(
        UserCopyDefaultMove::UserCopyDefaultMove_pmutUserCopyDefaultMove({ u.as_pointer() }),
    ));
    assert!(
        (((*(*u1.borrow()).v.borrow()) == 108) && ((*(*u2.borrow()).v.borrow()) == 8))
            && ((*(*u.borrow()).v.borrow()) == 8)
    );
    let u3: Value<UserCopyDefaultMove> =
        Rc::new(RefCell::new(UserCopyDefaultMove::UserCopyDefaultMove({
            1
        })));
    let u4: Value<UserCopyDefaultMove> =
        Rc::new(RefCell::new(UserCopyDefaultMove::UserCopyDefaultMove({
            1
        })));
    ({
        UserCopyDefaultMoveImpl::operator_assign_pconstUserCopyDefaultMove(
            &u3.as_pointer(),
            u2.as_pointer(),
        )
    });
    ({
        UserCopyDefaultMoveImpl::operator_assign_pmutUserCopyDefaultMove(
            &u4.as_pointer(),
            u2.as_pointer(),
        )
    });
    assert!(((*(*u3.borrow()).v.borrow()) == 108) && ((*(*u4.borrow()).v.borrow()) == 8));
    let p: Value<Buffer> = Rc::new(RefCell::new(Buffer::Buffer({ 3 })));
    let q: Value<Buffer> = Rc::new(RefCell::new(Buffer::Buffer_pmutBuffer({ p.as_pointer() })));
    assert!(
        ((((*(*q.borrow()).n.borrow()) == 3) && ((*(*q.borrow()).data.borrow()).len() == 3_usize))
            && ((((*q.borrow()).data.as_pointer() as Ptr<i32>)
                .offset(2_usize)
                .read())
                == 3))
            && ((*(*p.borrow()).data.borrow()).is_empty())
    );
    let r: Value<Buffer> = Rc::new(RefCell::new(Buffer::Buffer({ 1 })));
    ({ BufferImpl::operator_assign_pmutBuffer(&r.as_pointer(), q.as_pointer()) });
    assert!(
        ((((*(*r.borrow()).n.borrow()) == 3) && ((*(*r.borrow()).data.borrow()).len() == 3_usize))
            && ((*(*r.borrow()).arr.borrow())[(1) as usize] == 4))
            && ((*(*q.borrow()).data.borrow()).is_empty())
    );
    assert!(
        (((*(*r.borrow()).rows.borrow()).len() == 1_usize)
            && ((*(((*r.borrow()).rows.as_pointer() as Ptr<Value<Vec<i32>>>)
                .offset(0_usize)
                .upgrade()
                .deref()
                .as_pointer() as Ptr<Vec<i32>>)
                .upgrade()
                .deref())
            .len()
                == 3_usize))
            && ((*(*q.borrow()).rows.borrow()).is_empty())
    );
    let bufs: Value<Vec<Buffer>> = Rc::new(RefCell::new(Vec::new()));
    (*bufs.borrow_mut()).push(Buffer::Buffer_pmutBuffer({ r.as_pointer() }));
    {
        let __arg =
            Buffer::Buffer_pmutBuffer({ (bufs.as_pointer() as Ptr<Buffer>).offset(0_usize) });
        bufs.as_pointer()
            .with_mut(|__v: &mut Vec<Buffer>| __v.push(__arg))
    };
    assert!(
        (((*(*(bufs.as_pointer() as Ptr<Buffer>)
            .offset(1_usize)
            .upgrade()
            .deref())
        .n
        .borrow())
            == 3)
            && ((*(*(bufs.as_pointer() as Ptr<Buffer>)
                .offset(1_usize)
                .upgrade()
                .deref())
            .data
            .borrow())
            .len()
                == 3_usize))
            && ((*(*(bufs.as_pointer() as Ptr<Buffer>)
                .offset(0_usize)
                .upgrade()
                .deref())
            .data
            .borrow())
            .is_empty())
    );
    let o1: Value<Owner> = Rc::new(RefCell::new(<Owner>::default()));
    (*(*o1.borrow()).data.borrow_mut()).push(5);
    (*(*o1.borrow()).n.borrow_mut()) = 5;
    (*(*o1.borrow()).arr.borrow_mut())[(0) as usize] = 5;
    (*(*o1.borrow()).arr.borrow_mut())[(1) as usize] = 6;
    {
        let _p: Ptr<_> = Ptr::alloc(7);
        (*(*o1.borrow()).p.borrow_mut()) = _p.to_owned_opt()
    };
    let o2: Value<Owner> = Rc::new(RefCell::new(Owner::Owner_pmutOwner({ o1.as_pointer() })));
    assert!(
        ((((*(*o2.borrow()).n.borrow()) == 5)
            && ((*(*o2.borrow()).data.borrow()).len() == 1_usize))
            && ((*(*o2.borrow()).arr.borrow())[(1) as usize] == 6))
            && ((*(*(*o2.borrow()).p.borrow()).as_ref().unwrap().borrow()) == 7)
    );
    assert!(
        ((*(*o1.borrow()).data.borrow()).is_empty())
            && (((*(*o1.borrow()).p.borrow()).as_pointer()).is_null())
    );
    let o3: Value<Owner> = Rc::new(RefCell::new(<Owner>::default()));
    ({ OwnerImpl::operator_assign_pmutOwner(&o3.as_pointer(), o2.as_pointer()) });
    assert!(
        ((((*(*o3.borrow()).n.borrow()) == 5)
            && ((((*o3.borrow()).data.as_pointer() as Ptr<i32>)
                .offset(0_usize)
                .read())
                == 5))
            && ((*(*o3.borrow()).arr.borrow())[(0) as usize] == 5))
            && ((*(*(*o3.borrow()).p.borrow()).as_ref().unwrap().borrow()) == 7)
    );
    assert!(
        ((*(*o2.borrow()).data.borrow()).is_empty())
            && (((*(*o2.borrow()).p.borrow()).as_pointer()).is_null())
    );
    let h1: Value<Holder> = Rc::new(RefCell::new(Holder::Holder({ 4 })));
    let _dtor_h1 = ScopedDestructor::new(&h1, |__p| __p.destructor());
    {
        let _p: Ptr<_> = Ptr::alloc(9);
        (*(*h1.borrow()).p.borrow_mut()) = _p.to_owned_opt()
    };
    let h2: Value<Holder> = Rc::new(RefCell::new(Holder::Holder_pmutHolder({ h1.as_pointer() })));
    let _dtor_h2 = ScopedDestructor::new(&h2, |__p| __p.destructor());
    assert!(
        ((((*(*(*h2.borrow()).inner.borrow()).x.borrow()) == 4)
            && ((*(*(*h2.borrow()).e.borrow()).v.borrow()) == 4))
            && ((*(*(*h2.borrow()).p.borrow()).as_ref().unwrap().borrow()) == 9))
            && (((*(*h1.borrow()).p.borrow()).as_pointer()).is_null())
    );
    let h3: Value<Holder> = Rc::new(RefCell::new(Holder::Holder({ 1 })));
    let _dtor_h3 = ScopedDestructor::new(&h3, |__p| __p.destructor());
    ({ HolderImpl::operator_assign_pmutHolder(&h3.as_pointer(), h2.as_pointer()) });
    assert!(
        ((((*(*(*h3.borrow()).inner.borrow()).x.borrow()) == 4)
            && ((*(*(*h3.borrow()).e.borrow()).arr.borrow())[(1) as usize] == 5))
            && ((*(*(*h3.borrow()).p.borrow()).as_ref().unwrap().borrow()) == 9))
            && (((*(*h2.borrow()).p.borrow()).as_pointer()).is_null())
    );
    return 0;
}
pub trait BufferImpl {
    fn operator_assign_pmutBuffer(&self, _a0: Ptr<Buffer>) -> Ptr<Buffer>;
}
impl BufferImpl for Ptr<Buffer> {
    fn operator_assign_pmutBuffer(&self, _a0: Ptr<Buffer>) -> Ptr<Buffer> {
        ((*(*self).upgrade().deref()).data.as_pointer() as Ptr<Vec<i32>>).write(std::mem::take(
            &mut (*(*_a0.upgrade().deref()).data.borrow_mut()),
        ));
        ((*(*self).upgrade().deref()).rows.as_pointer() as Ptr<Vec<Value<Vec<i32>>>>).write(
            std::mem::take(&mut (*(*_a0.upgrade().deref()).rows.borrow_mut())),
        );
        let __rhs = (*(*_a0.upgrade().deref()).n.borrow());
        (*(*(*self).upgrade().deref()).n.borrow_mut()) = __rhs;
        {
            (((*(*self).upgrade().deref()).arr.as_pointer()) as Ptr<i32>)
                .to_any()
                .memcpy(
                    &(((*_a0.upgrade().deref()).arr.as_pointer()) as Ptr<i32>).to_any(),
                    8_usize as usize,
                );
            (((*(*self).upgrade().deref()).arr.as_pointer()) as Ptr<i32>)
                .to_any()
                .clone()
        };
        return (*self).clone();
    }
}
pub trait DefaultCopyUserMoveImpl {
    fn operator_assign_pmutDefaultCopyUserMove(
        &self,
        o: Ptr<DefaultCopyUserMove>,
    ) -> Ptr<DefaultCopyUserMove>;
}
impl DefaultCopyUserMoveImpl for Ptr<DefaultCopyUserMove> {
    fn operator_assign_pmutDefaultCopyUserMove(
        &self,
        o: Ptr<DefaultCopyUserMove>,
    ) -> Ptr<DefaultCopyUserMove> {
        let __rhs = (*(*o.upgrade().deref()).v.borrow());
        (*(*(*self).upgrade().deref()).v.borrow_mut()) = __rhs;
        (*(*o.upgrade().deref()).v.borrow_mut()) = 0;
        return (*self).clone();
    }
}
pub trait ExplicitImpl {
    fn destructor(&self);
}
impl ExplicitImpl for Ptr<Explicit> {
    fn destructor(&self) {}
}
pub trait HolderImpl {
    fn operator_assign_pmutHolder(&self, _a0: Ptr<Holder>) -> Ptr<Holder>;
    fn destructor(&self);
}
impl HolderImpl for Ptr<Holder> {
    fn operator_assign_pmutHolder(&self, _a0: Ptr<Holder>) -> Ptr<Holder> {
        let __rhs = (*(*_a0.upgrade().deref()).inner.borrow()).clone();
        (*(*(*self).upgrade().deref()).inner.borrow_mut()) = __rhs;
        let __rhs = (*(*_a0.upgrade().deref()).e.borrow()).clone();
        (*(*(*self).upgrade().deref()).e.borrow_mut()) = __rhs;
        ((*(*self).upgrade().deref()).p.as_pointer() as Ptr<Option<Value<i32>>>)
            .write((*(*_a0.upgrade().deref()).p.borrow_mut()).take());
        return (*self).clone();
    }
    fn destructor(&self) {
        (*self.upgrade().deref()).e.as_pointer().destructor();
    }
}
pub trait OwnerImpl {
    fn operator_assign_pmutOwner(&self, _a0: Ptr<Owner>) -> Ptr<Owner>;
}
impl OwnerImpl for Ptr<Owner> {
    fn operator_assign_pmutOwner(&self, _a0: Ptr<Owner>) -> Ptr<Owner> {
        ((*(*self).upgrade().deref()).data.as_pointer() as Ptr<Vec<i32>>).write(std::mem::take(
            &mut (*(*_a0.upgrade().deref()).data.borrow_mut()),
        ));
        let __rhs = (*(*_a0.upgrade().deref()).n.borrow());
        (*(*(*self).upgrade().deref()).n.borrow_mut()) = __rhs;
        {
            (((*(*self).upgrade().deref()).arr.as_pointer()) as Ptr<i32>)
                .to_any()
                .memcpy(
                    &(((*_a0.upgrade().deref()).arr.as_pointer()) as Ptr<i32>).to_any(),
                    8_usize as usize,
                );
            (((*(*self).upgrade().deref()).arr.as_pointer()) as Ptr<i32>)
                .to_any()
                .clone()
        };
        ((*(*self).upgrade().deref()).p.as_pointer() as Ptr<Option<Value<i32>>>)
            .write((*(*_a0.upgrade().deref()).p.borrow_mut()).take());
        return (*self).clone();
    }
}
pub trait UserCopyDefaultMoveImpl {
    fn operator_assign_pconstUserCopyDefaultMove(
        &self,
        o: Ptr<UserCopyDefaultMove>,
    ) -> Ptr<UserCopyDefaultMove>;
    fn operator_assign_pmutUserCopyDefaultMove(
        &self,
        _a0: Ptr<UserCopyDefaultMove>,
    ) -> Ptr<UserCopyDefaultMove>;
}
impl UserCopyDefaultMoveImpl for Ptr<UserCopyDefaultMove> {
    fn operator_assign_pconstUserCopyDefaultMove(
        &self,
        o: Ptr<UserCopyDefaultMove>,
    ) -> Ptr<UserCopyDefaultMove> {
        let __rhs = ((*(*o.upgrade().deref()).v.borrow()) + 100);
        (*(*(*self).upgrade().deref()).v.borrow_mut()) = __rhs;
        return (*self).clone();
    }
    fn operator_assign_pmutUserCopyDefaultMove(
        &self,
        _a0: Ptr<UserCopyDefaultMove>,
    ) -> Ptr<UserCopyDefaultMove> {
        let __rhs = (*(*_a0.upgrade().deref()).v.borrow());
        (*(*(*self).upgrade().deref()).v.borrow_mut()) = __rhs;
        return (*self).clone();
    }
}
