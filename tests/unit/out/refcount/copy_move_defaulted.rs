extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, VaArg, FnPtrArg, Default)]
pub struct Inner {
    #[offset(0)]
    pub x: i32,
}
impl ByteRepr for Inner {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.x.to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            x: <i32>::from_bytes(&buf[0..4]),
        }
    }
}
#[derive(Clone, Record, VaArg, FnPtrArg)]
pub struct Explicit {
    #[offset(0)]
    pub v: i32,
    #[offset(4)]
    pub inner: Inner,
    #[offset(8)]
    pub arr: Box<[i32]>,
}
impl Explicit {
    pub fn new(v: i32) -> Self {
        let v: Value<i32> = Rc::new(RefCell::new(v));
        let __this: Value<Explicit> = Rc::new(RefCell::new(Self {
            v: (*v.borrow()),
            inner: Inner {
                x: ((*v.borrow()) * 10),
            },
            arr: Box::new([(*v.borrow()), ((*v.borrow()) + 1)]),
        }));
        let this: Ptr<Explicit> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for Explicit {
    fn default() -> Self {
        Explicit {
            v: 0_i32,
            inner: <Inner>::default(),
            arr: (0..2).map(|_| 0_i32).collect::<Box<[i32]>>(),
        }
    }
}
<<<<<<< HEAD
impl ByteRepr for Explicit {}
#[derive(VaArg, FnPtrArg)]
=======
impl ByteRepr for Explicit {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.v.to_bytes(&mut buf[0..4]);
        self.inner.to_bytes(&mut buf[4..8]);
        self.arr.to_bytes(&mut buf[8..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            v: <i32>::from_bytes(&buf[0..4]),
            inner: <Inner>::from_bytes(&buf[4..8]),
            arr: <Box<[i32]>>::from_bytes(&buf[8..16]),
        }
    }
}
#[derive(Clone, Record, VaArg, FnPtrArg)]
>>>>>>> 3ed38b58 (Remove Value<> boxing from struct fields)
pub struct Implicit {
    #[offset(0)]
    pub v: i32,
    #[offset(4)]
    pub inner: Inner,
    #[offset(8)]
    pub arr: Box<[i32]>,
}
impl Default for Implicit {
    fn default() -> Self {
        Implicit {
            v: 0_i32,
            inner: <Inner>::default(),
            arr: (0..2).map(|_| 0_i32).collect::<Box<[i32]>>(),
        }
    }
}
impl ByteRepr for Implicit {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.v.to_bytes(&mut buf[0..4]);
        self.inner.to_bytes(&mut buf[4..8]);
        self.arr.to_bytes(&mut buf[8..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            v: <i32>::from_bytes(&buf[0..4]),
            inner: <Inner>::from_bytes(&buf[4..8]),
            arr: <Box<[i32]>>::from_bytes(&buf[8..16]),
        }
    }
}
#[derive(Clone, Record, VaArg, FnPtrArg, Default)]
pub struct DefaultCopyUserMove {
    #[offset(0)]
    pub v: i32,
}
impl DefaultCopyUserMove {
    pub fn new(v: i32) -> Self {
        let v: Value<i32> = Rc::new(RefCell::new(v));
        let __this: Value<DefaultCopyUserMove> = Rc::new(RefCell::new(Self { v: (*v.borrow()) }));
        let this: Ptr<DefaultCopyUserMove> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn move_from(o: Ptr<DefaultCopyUserMove>) -> Self {
        let __this: Value<DefaultCopyUserMove> = Rc::new(RefCell::new(Self {
            v: { o.with(|__s: &DefaultCopyUserMove| __s.v) },
        }));
        let this: Ptr<DefaultCopyUserMove> = __this.as_pointer();
        o.with_mut(|__s: &mut DefaultCopyUserMove| __s.v = 0);
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
<<<<<<< HEAD
impl ByteRepr for DefaultCopyUserMove {}
#[derive(VaArg, FnPtrArg, Default)]
=======
impl ByteRepr for DefaultCopyUserMove {
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
#[derive(Record, VaArg, FnPtrArg, Default)]
>>>>>>> 3ed38b58 (Remove Value<> boxing from struct fields)
pub struct UserCopyDefaultMove {
    #[offset(0)]
    pub v: i32,
}
impl UserCopyDefaultMove {
    pub fn new(v: i32) -> Self {
        let v: Value<i32> = Rc::new(RefCell::new(v));
        let __this: Value<UserCopyDefaultMove> = Rc::new(RefCell::new(Self { v: (*v.borrow()) }));
        let this: Ptr<UserCopyDefaultMove> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn copy_from(o: Ptr<UserCopyDefaultMove>) -> Self {
        let __this: Value<UserCopyDefaultMove> = Rc::new(RefCell::new(Self {
            v: { (o.with(|__s: &UserCopyDefaultMove| __s.v) + 100) },
        }));
        let this: Ptr<UserCopyDefaultMove> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn move_from(_a0: Ptr<UserCopyDefaultMove>) -> Self {
        let __this: Value<UserCopyDefaultMove> = Rc::new(RefCell::new(Self {
            v: { _a0.with(|__s: &UserCopyDefaultMove| __s.v) },
        }));
        let this: Ptr<UserCopyDefaultMove> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for UserCopyDefaultMove {
    fn clone(&self) -> Self {
        let __src: Value<UserCopyDefaultMove> =
            Rc::new(RefCell::new(UserCopyDefaultMove { v: self.v.clone() }));
        UserCopyDefaultMove::copy_from(__src.as_pointer())
    }
}
<<<<<<< HEAD
impl ByteRepr for UserCopyDefaultMove {}
#[derive()]
=======
impl ByteRepr for UserCopyDefaultMove {
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
pub struct Buffer {
    #[offset(0)]
    pub data: Value<Vec<i32>>,
    #[offset(24)]
    pub rows: Value<Vec<Value<Vec<i32>>>>,
    #[offset(48)]
    pub n: i32,
    #[offset(52)]
    pub arr: Box<[i32]>,
}
impl Buffer {
    pub fn new(n: i32) -> Self {
        let n: Value<i32> = Rc::new(RefCell::new(n));
        let __this: Value<Buffer> = Rc::new(RefCell::new(Self {
            data: Rc::new(RefCell::new(vec![
                (*n.borrow());
                ((*n.borrow()) as usize) as usize
            ])),
            rows: Rc::new(RefCell::new(Vec::new())),
            n: (*n.borrow()),
            arr: Box::new([(*n.borrow()), ((*n.borrow()) + 1)]),
        }));
        let this: Ptr<Buffer> = __this.as_pointer();
        {
            let __a1 = (*this.with(|__s: &Buffer| __s.data.clone()).borrow()).clone();
            (this.with(|__s: &Buffer| __s.rows.as_pointer()) as Ptr<Vec<Value<Vec<i32>>>>)
                .with_mut(|__v: &mut Vec<Value<Vec<i32>>>| __v.push(Rc::new(RefCell::new(__a1))))
        };
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn move_from(_a0: Ptr<Buffer>) -> Self {
        let __this: Value<Buffer> = Rc::new(RefCell::new(Self {
            data: {
                Rc::new(RefCell::new(_a0.with(|__s: &Buffer| {
                    std::mem::take(&mut (*__s.data.borrow_mut()))
                })))
            },
            rows: {
                Rc::new(RefCell::new(_a0.with(|__s: &Buffer| {
                    std::mem::take(&mut (*__s.rows.borrow_mut()))
                })))
            },
            n: { _a0.with(|__s: &Buffer| __s.n) },
            arr: Box::new(std::array::from_fn::<_, 2, _>(|__i: usize| {
                (*_a0.upgrade().deref()).arr[(__i) as usize]
            })),
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
            n: 0_i32,
            arr: (0..2).map(|_| 0_i32).collect::<Box<[i32]>>(),
        }
    }
}
<<<<<<< HEAD
impl ByteRepr for Buffer {}
#[derive()]
=======
impl ByteRepr for Buffer {
    fn byte_size() -> usize {
        64
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.data.to_bytes(&mut buf[0..24]);
        self.rows.to_bytes(&mut buf[24..48]);
        self.n.to_bytes(&mut buf[48..52]);
        self.arr.to_bytes(&mut buf[52..60]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            data: <Value<Vec<i32>>>::from_bytes(&buf[0..24]),
            rows: <Value<Vec<Value<Vec<i32>>>>>::from_bytes(&buf[24..48]),
            n: <i32>::from_bytes(&buf[48..52]),
            arr: <Box<[i32]>>::from_bytes(&buf[52..60]),
        }
    }
}
#[derive(Record)]
>>>>>>> 3ed38b58 (Remove Value<> boxing from struct fields)
pub struct Owner {
    #[offset(0)]
    pub data: Value<Vec<i32>>,
    #[offset(24)]
    pub n: i32,
    #[offset(28)]
    pub arr: Box<[i32]>,
    #[offset(40)]
    pub p: Option<Value<i32>>,
}
impl Owner {
    pub fn move_from(_a0: Ptr<Owner>) -> Self {
        let __this: Value<Owner> = Rc::new(RefCell::new(Self {
            data: {
                Rc::new(RefCell::new(_a0.with(|__s: &Owner| {
                    std::mem::take(&mut (*__s.data.borrow_mut()))
                })))
            },
            n: { _a0.with(|__s: &Owner| __s.n) },
            arr: Box::new(std::array::from_fn::<_, 2, _>(|__i: usize| {
                (*_a0.upgrade().deref()).arr[(__i) as usize]
            })),
            p: { _a0.with_mut(|__s: &mut Owner| __s.p.take()) },
        }));
        let this: Ptr<Owner> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for Owner {
    fn default() -> Self {
        Owner {
            data: Rc::new(RefCell::new(Default::default())),
            n: 0_i32,
            arr: (0..2).map(|_| 0_i32).collect::<Box<[i32]>>(),
            p: None,
        }
    }
}
<<<<<<< HEAD
impl ByteRepr for Owner {}
#[derive(Default)]
=======
impl ByteRepr for Owner {
    fn byte_size() -> usize {
        48
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.data.to_bytes(&mut buf[0..24]);
        self.n.to_bytes(&mut buf[24..28]);
        self.arr.to_bytes(&mut buf[28..36]);
        self.p.to_bytes(&mut buf[40..48]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            data: <Value<Vec<i32>>>::from_bytes(&buf[0..24]),
            n: <i32>::from_bytes(&buf[24..28]),
            arr: <Box<[i32]>>::from_bytes(&buf[28..36]),
            p: <Option<Value<i32>>>::from_bytes(&buf[40..48]),
        }
    }
}
#[derive(Record, Default)]
>>>>>>> 3ed38b58 (Remove Value<> boxing from struct fields)
pub struct Holder {
    #[offset(0)]
    pub inner: Inner,
    #[offset(4)]
    pub e: Explicit,
    #[offset(24)]
    pub p: Option<Value<i32>>,
}
impl Holder {
    pub fn new(v: i32) -> Self {
        let v: Value<i32> = Rc::new(RefCell::new(v));
        let __this: Value<Holder> = Rc::new(RefCell::new(Self {
            inner: Inner { x: (*v.borrow()) },
            e: Explicit::new({ (*v.borrow()) }),
            p: None,
        }));
        let this: Ptr<Holder> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn move_from(_a0: Ptr<Holder>) -> Self {
        let __this: Value<Holder> = Rc::new(RefCell::new(Self {
            inner: { _a0.with(|__s: &Holder| (__s.inner).clone()) },
            e: { _a0.with(|__s: &Holder| (__s.e).clone()) },
            p: { _a0.with_mut(|__s: &mut Holder| __s.p.take()) },
        }));
        let this: Ptr<Holder> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
<<<<<<< HEAD
impl ByteRepr for Holder {}
=======
impl ByteRepr for Holder {
    fn byte_size() -> usize {
        32
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.inner.to_bytes(&mut buf[0..4]);
        self.e.to_bytes(&mut buf[4..20]);
        self.p.to_bytes(&mut buf[24..32]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            inner: <Inner>::from_bytes(&buf[0..4]),
            e: <Explicit>::from_bytes(&buf[4..20]),
            p: <Option<Value<i32>>>::from_bytes(&buf[24..32]),
        }
    }
}
>>>>>>> 3ed38b58 (Remove Value<> boxing from struct fields)
pub fn same_0(a: Ptr<Explicit>, b: Ptr<Explicit>) -> bool {
    return ((({
        let _lhs = a.with(|__s: &Explicit| __s.v);
        _lhs == b.with(|__s: &Explicit| __s.v)
    }) && ({
        let _lhs = a.with(|__s: &Explicit| __s.inner.x);
        _lhs == b.with(|__s: &Explicit| __s.inner.x)
    })) && ({
        let _lhs = a.with(|__s: &Explicit| __s.arr[(0) as usize]);
        _lhs == b.with(|__s: &Explicit| __s.arr[(0) as usize])
    })) && ({
        let _lhs = a.with(|__s: &Explicit| __s.arr[(1) as usize]);
        _lhs == b.with(|__s: &Explicit| __s.arr[(1) as usize])
    });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let a: Value<Explicit> = Rc::new(RefCell::new(Explicit::new({ 1 })));
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
    let e: Value<Explicit> = Rc::new(RefCell::new(Explicit::new({ 2 })));
    let _dtor_e = ScopedDestructor::new(&e, |__p| __p.destructor());
    let f: Value<Explicit> = Rc::new(RefCell::new(Explicit::new({ 3 })));
    let _dtor_f = ScopedDestructor::new(&f, |__p| __p.destructor());
    (*e.borrow_mut()) = (*b.borrow()).clone();
    (*f.borrow_mut()) = (*c.borrow()).clone();
    assert!(
        ({ same_0(e.as_pointer(), b.as_pointer(),) })
            && ({ same_0(f.as_pointer(), c.as_pointer(),) })
    );
    let g: Value<Explicit> = Rc::new(RefCell::new(Explicit::new({ 4 })));
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
        v: 5,
        inner: Inner { x: 50 },
        arr: Box::new([5, 6]),
    }));
    let j: Value<Implicit> = Rc::new(RefCell::new((*i.borrow()).clone()));
    let k: Value<Implicit> = Rc::new(RefCell::new((*i.borrow()).clone()));
    assert!(
        (({ (*j.borrow()).v } == 5) && ({ (*j.borrow()).inner.x } == 50))
            && ((*j.borrow()).arr[(1) as usize] == 6)
    );
    assert!(({ (*i.borrow()).v } == 5) && ({ (*k.borrow()).v } == 5));
    let l: Value<Implicit> = Rc::new(RefCell::new(Implicit {
        v: 0,
        inner: Inner { x: 0 },
        arr: Box::new([0, 0]),
    }));
    (*l.borrow_mut()) = (*j.borrow()).clone();
    assert!(
        (({ (*l.borrow()).v } == 5) && ({ (*l.borrow()).inner.x } == 50))
            && ((*l.borrow()).arr[(0) as usize] == 5)
    );
    let vec_: Value<Vec<Explicit>> = Rc::new(RefCell::new(Vec::new()));
    {
        let a0_clone = (*b.borrow()).clone();
        (*vec_.borrow_mut()).push(a0_clone)
    };
    (*vec_.borrow_mut()).push(Explicit::new({ 9 }));
    assert!(
        ((vec_.as_pointer() as Ptr<Explicit>)
            .offset(0_usize)
            .with(|__s: &Explicit| __s.v)
            == 1)
            && ((vec_.as_pointer() as Ptr<Explicit>)
                .offset(1_usize)
                .with(|__s: &Explicit| __s.v)
                == 9)
    );
    let m: Value<DefaultCopyUserMove> = Rc::new(RefCell::new(DefaultCopyUserMove::new({ 7 })));
    let m1: Value<DefaultCopyUserMove> = Rc::new(RefCell::new((*m.borrow()).clone()));
    let m2: Value<DefaultCopyUserMove> = Rc::new(RefCell::new(DefaultCopyUserMove::move_from({
        m.as_pointer()
    })));
    assert!(
        (({ (*m1.borrow()).v } == 7) && ({ (*m2.borrow()).v } == 7)) && ({ (*m.borrow()).v } == 0)
    );
    let m3: Value<DefaultCopyUserMove> = Rc::new(RefCell::new(DefaultCopyUserMove::new({ 1 })));
    let m4: Value<DefaultCopyUserMove> = Rc::new(RefCell::new(DefaultCopyUserMove::new({ 1 })));
    (*m3.borrow_mut()) = (*m1.borrow()).clone();
    ({ DefaultCopyUserMoveImpl::move_assign(&m4.as_pointer(), m1.as_pointer()) });
    assert!(
        (({ (*m3.borrow()).v } == 7) && ({ (*m4.borrow()).v } == 7)) && ({ (*m1.borrow()).v } == 0)
    );
    let u: Value<UserCopyDefaultMove> = Rc::new(RefCell::new(UserCopyDefaultMove::new({ 8 })));
    let u1: Value<UserCopyDefaultMove> = Rc::new(RefCell::new(UserCopyDefaultMove::copy_from({
        u.as_pointer()
    })));
    let u2: Value<UserCopyDefaultMove> = Rc::new(RefCell::new(UserCopyDefaultMove::move_from({
        u.as_pointer()
    })));
    assert!(
        (({ (*u1.borrow()).v } == 108) && ({ (*u2.borrow()).v } == 8))
            && ({ (*u.borrow()).v } == 8)
    );
    let u3: Value<UserCopyDefaultMove> = Rc::new(RefCell::new(UserCopyDefaultMove::new({ 1 })));
    let u4: Value<UserCopyDefaultMove> = Rc::new(RefCell::new(UserCopyDefaultMove::new({ 1 })));
    ({ UserCopyDefaultMoveImpl::copy_assign(&u3.as_pointer(), u2.as_pointer()) });
    ({ UserCopyDefaultMoveImpl::move_assign(&u4.as_pointer(), u2.as_pointer()) });
    assert!(({ (*u3.borrow()).v } == 108) && ({ (*u4.borrow()).v } == 8));
    let p: Value<Buffer> = Rc::new(RefCell::new(Buffer::new({ 3 })));
    let q: Value<Buffer> = Rc::new(RefCell::new(Buffer::move_from({ p.as_pointer() })));
    assert!(
        ((({ (*q.borrow()).n } == 3) && ((*(*q.borrow()).data.borrow()).len() == 3_usize))
            && ({ (*(*q.borrow()).data.borrow())[(2_usize) as usize] } == 3))
            && ((*(*p.borrow()).data.borrow()).is_empty())
    );
    let r: Value<Buffer> = Rc::new(RefCell::new(Buffer::new({ 1 })));
    ({ BufferImpl::move_assign(&r.as_pointer(), q.as_pointer()) });
    assert!(
        ((({ (*r.borrow()).n } == 3) && ((*(*r.borrow()).data.borrow()).len() == 3_usize))
            && ((*r.borrow()).arr[(1) as usize] == 4))
            && ((*(*q.borrow()).data.borrow()).is_empty())
    );
    assert!(
        (((*(*r.borrow()).rows.borrow()).len() == 1_usize)
            && ((*(({ (*r.borrow()).rows.as_pointer() } as Ptr<Value<Vec<i32>>>)
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
    (*bufs.borrow_mut()).push(Buffer::move_from({ r.as_pointer() }));
    {
        let __init = Buffer::move_from({ (bufs.as_pointer() as Ptr<Buffer>).offset(0_usize) });
        (*bufs.borrow_mut()).push(__init)
    };
    assert!(
        (((bufs.as_pointer() as Ptr<Buffer>)
            .offset(1_usize)
            .with(|__s: &Buffer| __s.n)
            == 3)
            && ((*(bufs.as_pointer() as Ptr<Buffer>)
                .offset(1_usize)
                .with(|__s: &Buffer| __s.data.clone())
                .borrow())
            .len()
                == 3_usize))
            && ((*(bufs.as_pointer() as Ptr<Buffer>)
                .offset(0_usize)
                .with(|__s: &Buffer| __s.data.clone())
                .borrow())
            .is_empty())
    );
    let o1: Value<Owner> = Rc::new(RefCell::new(<Owner>::default()));
    (*(*o1.borrow()).data.borrow_mut()).push(5);
    (*o1.borrow_mut()).n = 5;
    (*o1.borrow_mut()).arr[(0) as usize] = 5;
    (*o1.borrow_mut()).arr[(1) as usize] = 6;
    {
        let _p: Ptr<_> = Ptr::alloc(7);
        (*o1.borrow_mut()).p = _p.to_owned_opt()
    };
    let o2: Value<Owner> = Rc::new(RefCell::new(Owner::move_from({ o1.as_pointer() })));
    assert!(
        ((({ (*o2.borrow()).n } == 5) && ((*(*o2.borrow()).data.borrow()).len() == 1_usize))
            && ((*o2.borrow()).arr[(1) as usize] == 6))
            && ((*(*o2.borrow()).p.as_ref().unwrap().borrow()) == 7)
    );
    assert!(
        ((*(*o1.borrow()).data.borrow()).is_empty()) && (((*o1.borrow()).p.as_pointer()).is_null())
    );
    let o3: Value<Owner> = Rc::new(RefCell::new(<Owner>::default()));
    ({ OwnerImpl::move_assign(&o3.as_pointer(), o2.as_pointer()) });
    assert!(
        ((({ (*o3.borrow()).n } == 5)
            && ({ (*(*o3.borrow()).data.borrow())[(0_usize) as usize] } == 5))
            && ((*o3.borrow()).arr[(0) as usize] == 5))
            && ((*(*o3.borrow()).p.as_ref().unwrap().borrow()) == 7)
    );
    assert!(
        ((*(*o2.borrow()).data.borrow()).is_empty()) && (((*o2.borrow()).p.as_pointer()).is_null())
    );
    let h1: Value<Holder> = Rc::new(RefCell::new(Holder::new({ 4 })));
    let _dtor_h1 = ScopedDestructor::new(&h1, |__p| __p.destructor());
    {
        let _p: Ptr<_> = Ptr::alloc(9);
        (*h1.borrow_mut()).p = _p.to_owned_opt()
    };
    let h2: Value<Holder> = Rc::new(RefCell::new(Holder::move_from({ h1.as_pointer() })));
    let _dtor_h2 = ScopedDestructor::new(&h2, |__p| __p.destructor());
    assert!(
        ((({ (*h2.borrow()).inner.x } == 4) && ({ (*h2.borrow()).e.v } == 4))
            && ((*(*h2.borrow()).p.as_ref().unwrap().borrow()) == 9))
            && (((*h1.borrow()).p.as_pointer()).is_null())
    );
    let h3: Value<Holder> = Rc::new(RefCell::new(Holder::new({ 1 })));
    let _dtor_h3 = ScopedDestructor::new(&h3, |__p| __p.destructor());
    ({ HolderImpl::move_assign(&h3.as_pointer(), h2.as_pointer()) });
    assert!(
        ((({ (*h3.borrow()).inner.x } == 4) && ((*h3.borrow()).e.arr[(1) as usize] == 5))
            && ((*(*h3.borrow()).p.as_ref().unwrap().borrow()) == 9))
            && (((*h2.borrow()).p.as_pointer()).is_null())
    );
    return 0;
}
pub trait BufferImpl {
    fn move_assign(&self, _a0: Ptr<Buffer>) -> Ptr<Buffer>;
}
impl BufferImpl for Ptr<Buffer> {
    fn move_assign(&self, _a0: Ptr<Buffer>) -> Ptr<Buffer> {
        ((*self).with(|__s: &Buffer| __s.data.as_pointer()) as Ptr<Vec<i32>>)
            .write(_a0.with(|__s: &Buffer| std::mem::take(&mut (*__s.data.borrow_mut()))));
        ((*self).with(|__s: &Buffer| __s.rows.as_pointer()) as Ptr<Vec<Value<Vec<i32>>>>)
            .write(_a0.with(|__s: &Buffer| std::mem::take(&mut (*__s.rows.borrow_mut()))));
        let __rhs = _a0.with(|__s: &Buffer| __s.n);
        (*self).with_mut(|__s: &mut Buffer| __s.n = __rhs);
        {
            ((field_ptr!((*self), arr)) as Ptr<i32>).to_any().memcpy(
                &((field_ptr!(_a0, arr)) as Ptr<i32>).to_any(),
                8_usize as usize,
            );
            ((field_ptr!((*self), arr)) as Ptr<i32>).to_any()
        };
        return (*self).clone();
    }
}
pub trait DefaultCopyUserMoveImpl {
    fn move_assign(&self, o: Ptr<DefaultCopyUserMove>) -> Ptr<DefaultCopyUserMove>;
}
impl DefaultCopyUserMoveImpl for Ptr<DefaultCopyUserMove> {
    fn move_assign(&self, o: Ptr<DefaultCopyUserMove>) -> Ptr<DefaultCopyUserMove> {
        let __rhs = o.with(|__s: &DefaultCopyUserMove| __s.v);
        (*self).with_mut(|__s: &mut DefaultCopyUserMove| __s.v = __rhs);
        o.with_mut(|__s: &mut DefaultCopyUserMove| __s.v = 0);
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
    fn move_assign(&self, _a0: Ptr<Holder>) -> Ptr<Holder>;
    fn destructor(&self);
}
impl HolderImpl for Ptr<Holder> {
    fn move_assign(&self, _a0: Ptr<Holder>) -> Ptr<Holder> {
        let __rhs = _a0.with(|__s: &Holder| (__s.inner).clone());
        (*self).with_mut(|__s: &mut Holder| __s.inner = __rhs);
        let __rhs = _a0.with(|__s: &Holder| (__s.e).clone());
        (*self).with_mut(|__s: &mut Holder| __s.e = __rhs);
        ((field_ptr!((*self), p) as Ptr<Option<Value<i32>>>) as Ptr<Option<Value<i32>>>)
            .write(_a0.with_mut(|__s: &mut Holder| __s.p.take()));
        return (*self).clone();
    }
    fn destructor(&self) {
        ExplicitImpl::destructor(&field_ptr!(self, e));
    }
}
pub trait OwnerImpl {
    fn move_assign(&self, _a0: Ptr<Owner>) -> Ptr<Owner>;
}
impl OwnerImpl for Ptr<Owner> {
    fn move_assign(&self, _a0: Ptr<Owner>) -> Ptr<Owner> {
        ((*self).with(|__s: &Owner| __s.data.as_pointer()) as Ptr<Vec<i32>>)
            .write(_a0.with(|__s: &Owner| std::mem::take(&mut (*__s.data.borrow_mut()))));
        let __rhs = _a0.with(|__s: &Owner| __s.n);
        (*self).with_mut(|__s: &mut Owner| __s.n = __rhs);
        {
            ((field_ptr!((*self), arr)) as Ptr<i32>).to_any().memcpy(
                &((field_ptr!(_a0, arr)) as Ptr<i32>).to_any(),
                8_usize as usize,
            );
            ((field_ptr!((*self), arr)) as Ptr<i32>).to_any()
        };
        ((field_ptr!((*self), p) as Ptr<Option<Value<i32>>>) as Ptr<Option<Value<i32>>>)
            .write(_a0.with_mut(|__s: &mut Owner| __s.p.take()));
        return (*self).clone();
    }
}
pub trait UserCopyDefaultMoveImpl {
    fn copy_assign(&self, o: Ptr<UserCopyDefaultMove>) -> Ptr<UserCopyDefaultMove>;
    fn move_assign(&self, _a0: Ptr<UserCopyDefaultMove>) -> Ptr<UserCopyDefaultMove>;
}
impl UserCopyDefaultMoveImpl for Ptr<UserCopyDefaultMove> {
    fn copy_assign(&self, o: Ptr<UserCopyDefaultMove>) -> Ptr<UserCopyDefaultMove> {
        let __rhs = (o.with(|__s: &UserCopyDefaultMove| __s.v) + 100);
        (*self).with_mut(|__s: &mut UserCopyDefaultMove| __s.v = __rhs);
        return (*self).clone();
    }
    fn move_assign(&self, _a0: Ptr<UserCopyDefaultMove>) -> Ptr<UserCopyDefaultMove> {
        let __rhs = _a0.with(|__s: &UserCopyDefaultMove| __s.v);
        (*self).with_mut(|__s: &mut UserCopyDefaultMove| __s.v = __rhs);
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
