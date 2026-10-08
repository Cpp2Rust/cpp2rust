extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Inner {
    #[offset(0)]
    pub x: i32,
}
#[derive(DeepClone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(16)]
pub struct Explicit {
    #[offset(0)]
    pub v: i32,
    #[offset(4)]
    #[byte_size(4)]
    pub inner: Inner,
    #[offset(8)]
    #[byte_size(8)]
    pub arr: Value<Box<[i32]>>,
}
impl Explicit {
    pub fn new(mut v: i32) -> Self {
        Self {
            v: v,
            inner: Inner { x: (v * 10) },
            arr: Rc::new(RefCell::new(Box::new([v, (v + 1)]))),
        }
    }
}
impl Default for Explicit {
    fn default() -> Self {
        Explicit {
            v: 0_i32,
            inner: <Inner>::default(),
            arr: Rc::new(RefCell::new((0..2).map(|_| 0_i32).collect::<Box<[i32]>>())),
        }
    }
}
#[derive(DeepClone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(16)]
pub struct Implicit {
    #[offset(0)]
    pub v: i32,
    #[offset(4)]
    #[byte_size(4)]
    pub inner: Inner,
    #[offset(8)]
    #[byte_size(8)]
    pub arr: Value<Box<[i32]>>,
}
impl Default for Implicit {
    fn default() -> Self {
        Implicit {
            v: 0_i32,
            inner: <Inner>::default(),
            arr: Rc::new(RefCell::new((0..2).map(|_| 0_i32).collect::<Box<[i32]>>())),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct DefaultCopyUserMove {
    #[offset(0)]
    pub v: i32,
}
impl DefaultCopyUserMove {
    pub fn new(mut v: i32) -> Self {
        Self { v: v }
    }
    pub fn move_from(o: Ptr<DefaultCopyUserMove>) -> Self {
        let __this: DefaultCopyUserMove = Self {
            v: o.with(|__s| __s.v),
        };
        field!(o, v).write(0);
        __this
    }
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct UserCopyDefaultMove {
    #[offset(0)]
    pub v: i32,
}
impl UserCopyDefaultMove {
    pub fn new(mut v: i32) -> Self {
        Self { v: v }
    }
    pub fn copy_from(o: Ptr<UserCopyDefaultMove>) -> Self {
        Self {
            v: (o.with(|__s| __s.v) + 100),
        }
    }
    pub fn move_from(_a0: Ptr<UserCopyDefaultMove>) -> Self {
        Self {
            v: { (*_a0.upgrade().deref()).v },
        }
    }
}
impl Clone for UserCopyDefaultMove {
    fn clone(&self) -> Self {
        let __src: Value<UserCopyDefaultMove> =
            Rc::new(RefCell::new(UserCopyDefaultMove { v: self.v.clone() }));
        UserCopyDefaultMove::copy_from(__src.as_pointer())
    }
}
#[derive(Record, ByteRepr)]
#[byte_size(64)]
pub struct Buffer {
    #[offset(0)]
    #[byte_size(24)]
    pub data: Value<Vec<i32>>,
    #[offset(24)]
    #[byte_size(24)]
    pub rows: Value<Vec<Value<Vec<i32>>>>,
    #[offset(48)]
    pub n: i32,
    #[offset(52)]
    #[byte_size(8)]
    pub arr: Value<Box<[i32]>>,
}
impl Buffer {
    pub fn new(mut n: i32) -> Self {
        let __this: Value<Buffer> = Rc::new(RefCell::new(Self {
            data: Rc::new(RefCell::new(vec![n; (n as usize) as usize])),
            rows: Rc::new(RefCell::new(Vec::new())),
            n: n,
            arr: Rc::new(RefCell::new(Box::new([n, (n + 1)]))),
        }));
        let this: Ptr<Buffer> = __this.as_pointer();
        (this.with(|__s| __s.rows.as_pointer()) as Ptr<Vec<Value<Vec<i32>>>>).with_mut(
            |__v: &mut Vec<Value<Vec<i32>>>| {
                __v.push(Rc::new(RefCell::new(
                    (*this.with(|__s| __s.data.clone()).borrow()).clone(),
                )))
            },
        );
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn move_from(_a0: Ptr<Buffer>) -> Self {
        Self {
            data: Rc::new(RefCell::new(std::mem::take(
                &mut (*{ (*_a0.upgrade().deref()).data.clone() }.borrow_mut()),
            ))),
            rows: Rc::new(RefCell::new(std::mem::take(
                &mut (*{ (*_a0.upgrade().deref()).rows.clone() }.borrow_mut()),
            ))),
            n: { (*_a0.upgrade().deref()).n },
            arr: Rc::new(RefCell::new(Box::new(std::array::from_fn::<_, 2, _>(
                |__i: usize| (*{ (*_a0.upgrade().deref()).arr.clone() }.borrow())[(__i) as usize],
            )))),
        }
    }
}
impl Default for Buffer {
    fn default() -> Self {
        Buffer {
            data: Rc::new(RefCell::new(Default::default())),
            rows: Rc::new(RefCell::new(Vec::new())),
            n: 0_i32,
            arr: Rc::new(RefCell::new((0..2).map(|_| 0_i32).collect::<Box<[i32]>>())),
        }
    }
}
#[derive(Record, ByteRepr)]
#[byte_size(48)]
pub struct Owner {
    #[offset(0)]
    #[byte_size(24)]
    pub data: Value<Vec<i32>>,
    #[offset(24)]
    pub n: i32,
    #[offset(28)]
    #[byte_size(8)]
    pub arr: Value<Box<[i32]>>,
    #[offset(40)]
    #[byte_size(8)]
    pub p: Option<Value<i32>>,
}
impl Owner {
    pub fn move_from(_a0: Ptr<Owner>) -> Self {
        Self {
            data: Rc::new(RefCell::new(std::mem::take(
                &mut (*{ (*_a0.upgrade().deref()).data.clone() }.borrow_mut()),
            ))),
            n: { (*_a0.upgrade().deref()).n },
            arr: Rc::new(RefCell::new(Box::new(std::array::from_fn::<_, 2, _>(
                |__i: usize| (*{ (*_a0.upgrade().deref()).arr.clone() }.borrow())[(__i) as usize],
            )))),
            p: field!(_a0, p).with_mut(|__v: &mut Option<Value<i32>>| __v.take()),
        }
    }
}
impl Default for Owner {
    fn default() -> Self {
        Owner {
            data: Rc::new(RefCell::new(Default::default())),
            n: 0_i32,
            arr: Rc::new(RefCell::new((0..2).map(|_| 0_i32).collect::<Box<[i32]>>())),
            p: None,
        }
    }
}
#[derive(Record, ByteRepr, Default)]
#[byte_size(32)]
pub struct Holder {
    #[offset(0)]
    #[byte_size(4)]
    pub inner: Inner,
    #[offset(4)]
    #[byte_size(16)]
    pub e: Explicit,
    #[offset(24)]
    #[byte_size(8)]
    pub p: Option<Value<i32>>,
}
impl Holder {
    pub fn new(mut v: i32) -> Self {
        Self {
            inner: Inner { x: v },
            e: Explicit::new({ v }),
            p: None,
        }
    }
    pub fn move_from(_a0: Ptr<Holder>) -> Self {
        Self {
            inner: { (*_a0.upgrade().deref()).inner.clone() },
            e: { (*_a0.upgrade().deref()).e.clone() },
            p: field!(_a0, p).with_mut(|__v: &mut Option<Value<i32>>| __v.take()),
        }
    }
}
pub fn same_0(a: Ptr<Explicit>, b: Ptr<Explicit>) -> bool {
    return ((({ a.with(|__s| __s.v) } == { b.with(|__s| __s.v) })
        && ({ a.with(|__s| __s.inner.x) } == { b.with(|__s| __s.inner.x) }))
        && ({ (elem!((array_field_ptr!(a, arr) as Ptr::<i32>), 0).read()) } == {
            (elem!((array_field_ptr!(b, arr) as Ptr::<i32>), 0).read())
        }))
        && ({ (elem!((array_field_ptr!(a, arr) as Ptr::<i32>), 1).read()) } == {
            (elem!((array_field_ptr!(b, arr) as Ptr::<i32>), 1).read())
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
        arr: Rc::new(RefCell::new(Box::new([5, 6]))),
    }));
    let mut j: Implicit = (*i.borrow()).clone();
    let mut k: Implicit = (*i.borrow()).clone();
    assert!(
        ((j.v == 5) && (j.inner.x == 50))
            && ((elem!((j.arr.as_pointer() as Ptr::<i32>), 1).read()) == 6)
    );
    assert!(({ (*i.borrow()).v } == 5) && (k.v == 5));
    let mut l: Implicit = Implicit {
        v: 0,
        inner: Inner { x: 0 },
        arr: Rc::new(RefCell::new(Box::new([0, 0]))),
    };
    l = (j).clone();
    assert!(
        ((l.v == 5) && (l.inner.x == 50))
            && ((elem!((l.arr.as_pointer() as Ptr::<i32>), 0).read()) == 5)
    );
    let mut vec_: Vec<Explicit> = Vec::new();
    {
        let a0_clone = (*b.borrow()).clone();
        vec_.push(a0_clone)
    };
    {
        let __a1 = Explicit::new({ 9 });
        vec_.push(__a1)
    };
    assert!(({ vec_[0_usize].v } == 1) && ({ vec_[1_usize].v } == 9));
    let m: Value<DefaultCopyUserMove> = Rc::new(RefCell::new(DefaultCopyUserMove::new({ 7 })));
    let m1: Value<DefaultCopyUserMove> = Rc::new(RefCell::new((*m.borrow()).clone()));
    let mut m2: DefaultCopyUserMove = DefaultCopyUserMove::move_from({ m.as_pointer() });
    assert!((({ (*m1.borrow()).v } == 7) && (m2.v == 7)) && ({ (*m.borrow()).v } == 0));
    let mut m3: DefaultCopyUserMove = DefaultCopyUserMove::new({ 1 });
    let m4: Value<DefaultCopyUserMove> = Rc::new(RefCell::new(DefaultCopyUserMove::new({ 1 })));
    m3 = (*m1.borrow()).clone();
    ({ DefaultCopyUserMoveImpl::move_assign(&m4.as_pointer(), m1.as_pointer()) });
    assert!(((m3.v == 7) && ({ (*m4.borrow()).v } == 7)) && ({ (*m1.borrow()).v } == 0));
    let u: Value<UserCopyDefaultMove> = Rc::new(RefCell::new(UserCopyDefaultMove::new({ 8 })));
    let mut u1: UserCopyDefaultMove = UserCopyDefaultMove::copy_from({ u.as_pointer() });
    let u2: Value<UserCopyDefaultMove> = Rc::new(RefCell::new(UserCopyDefaultMove::move_from({
        u.as_pointer()
    })));
    assert!(((u1.v == 108) && ({ (*u2.borrow()).v } == 8)) && ({ (*u.borrow()).v } == 8));
    let u3: Value<UserCopyDefaultMove> = Rc::new(RefCell::new(UserCopyDefaultMove::new({ 1 })));
    let u4: Value<UserCopyDefaultMove> = Rc::new(RefCell::new(UserCopyDefaultMove::new({ 1 })));
    ({ UserCopyDefaultMoveImpl::copy_assign(&u3.as_pointer(), u2.as_pointer()) });
    ({ UserCopyDefaultMoveImpl::move_assign(&u4.as_pointer(), u2.as_pointer()) });
    assert!(({ (*u3.borrow()).v } == 108) && ({ (*u4.borrow()).v } == 8));
    let p: Value<Buffer> = Rc::new(RefCell::new(Buffer::new({ 3 })));
    let q: Value<Buffer> = Rc::new(RefCell::new(Buffer::move_from({ p.as_pointer() })));
    assert!(
        ((({ (*q.borrow()).n } == 3)
            && ((*{ (*q.borrow()).data.clone() }.borrow()).len() == 3_usize))
            && ((elem!(({ (*q.borrow()).data.as_pointer() } as Ptr<i32>), 2_usize).read()) == 3))
            && ((*{ (*p.borrow()).data.clone() }.borrow()).is_empty())
    );
    let r: Value<Buffer> = Rc::new(RefCell::new(Buffer::new({ 1 })));
    ({ BufferImpl::move_assign(&r.as_pointer(), q.as_pointer()) });
    assert!(
        ((({ (*r.borrow()).n } == 3)
            && ((*{ (*r.borrow()).data.clone() }.borrow()).len() == 3_usize))
            && ((elem!((array_field_ptr!(r.as_pointer(), arr) as Ptr::<i32>), 1).read()) == 4))
            && ((*{ (*q.borrow()).data.clone() }.borrow()).is_empty())
    );
    assert!(
        (((*{ (*r.borrow()).rows.clone() }.borrow()).len() == 1_usize)
            && ((*(({ (*r.borrow()).rows.as_pointer() } as Ptr<Value<Vec<i32>>>)
                .offset(0_usize)
                .upgrade()
                .deref()
                .as_pointer() as Ptr<Vec<i32>>)
                .upgrade()
                .deref())
            .len()
                == 3_usize))
            && ((*{ (*q.borrow()).rows.clone() }.borrow()).is_empty())
    );
    let bufs: Value<Vec<Buffer>> = Rc::new(RefCell::new(Vec::new()));
    {
        let __a1 = Buffer::move_from({ r.as_pointer() });
        (*bufs.borrow_mut()).push(__a1)
    };
    {
        let __init = Buffer::move_from({ (bufs.as_pointer() as Ptr<Buffer>).offset(0_usize) });
        (*bufs.borrow_mut()).push(__init)
    };
    assert!(
        (({ (*bufs.borrow())[1_usize].n } == 3)
            && ((*{ (*bufs.borrow())[1_usize].data.clone() }.borrow()).len() == 3_usize))
            && ((*{ (*bufs.borrow())[0_usize].data.clone() }.borrow()).is_empty())
    );
    let o1: Value<Owner> = Rc::new(RefCell::new(<Owner>::default()));
    {
        let __a1 = 5;
        (*{ (*o1.borrow()).data.clone() }.borrow_mut()).push(__a1)
    };
    (*o1.borrow_mut()).n = 5;
    elem!((array_field_ptr!(o1.as_pointer(), arr) as Ptr::<i32>), 0).write(5);
    elem!((array_field_ptr!(o1.as_pointer(), arr) as Ptr::<i32>), 1).write(6);
    {
        let _p: Ptr<_> = Ptr::alloc(7);
        (field_ptr!(o1.as_pointer(), p) as Ptr<Option<Value<i32>>>).write(_p.to_owned_opt())
    };
    let o2: Value<Owner> = Rc::new(RefCell::new(Owner::move_from({ o1.as_pointer() })));
    assert!(
        ((({ (*o2.borrow()).n } == 5)
            && ((*{ (*o2.borrow()).data.clone() }.borrow()).len() == 1_usize))
            && ((elem!((array_field_ptr!(o2.as_pointer(), arr) as Ptr::<i32>), 1).read()) == 6))
            && ((*{ (*o2.borrow()).p.clone() }.as_ref().unwrap().borrow()) == 7)
    );
    assert!(
        ((*{ (*o1.borrow()).data.clone() }.borrow()).is_empty())
            && (({ (*o1.borrow()).p.clone() }.as_pointer()).is_null())
    );
    let o3: Value<Owner> = Rc::new(RefCell::new(<Owner>::default()));
    ({ OwnerImpl::move_assign(&o3.as_pointer(), o2.as_pointer()) });
    assert!(
        ((({ (*o3.borrow()).n } == 5)
            && ((elem!(({ (*o3.borrow()).data.as_pointer() } as Ptr<i32>), 0_usize).read()) == 5))
            && ((elem!((array_field_ptr!(o3.as_pointer(), arr) as Ptr::<i32>), 0).read()) == 5))
            && ((*{ (*o3.borrow()).p.clone() }.as_ref().unwrap().borrow()) == 7)
    );
    assert!(
        ((*{ (*o2.borrow()).data.clone() }.borrow()).is_empty())
            && (({ (*o2.borrow()).p.clone() }.as_pointer()).is_null())
    );
    let h1: Value<Holder> = Rc::new(RefCell::new(Holder::new({ 4 })));
    let _dtor_h1 = ScopedDestructor::new(&h1, |__p| __p.destructor());
    {
        let _p: Ptr<_> = Ptr::alloc(9);
        (field_ptr!(h1.as_pointer(), p) as Ptr<Option<Value<i32>>>).write(_p.to_owned_opt())
    };
    let h2: Value<Holder> = Rc::new(RefCell::new(Holder::move_from({ h1.as_pointer() })));
    let _dtor_h2 = ScopedDestructor::new(&h2, |__p| __p.destructor());
    assert!(
        ((({ (*h2.borrow()).inner.x } == 4) && ({ (*h2.borrow()).e.v } == 4))
            && ((*{ (*h2.borrow()).p.clone() }.as_ref().unwrap().borrow()) == 9))
            && (({ (*h1.borrow()).p.clone() }.as_pointer()).is_null())
    );
    let h3: Value<Holder> = Rc::new(RefCell::new(Holder::new({ 1 })));
    let _dtor_h3 = ScopedDestructor::new(&h3, |__p| __p.destructor());
    ({ HolderImpl::move_assign(&h3.as_pointer(), h2.as_pointer()) });
    assert!(
        ((({ (*h3.borrow()).inner.x } == 4)
            && ((elem!(
                (array_field_ptr!(field_ptr!(h3.as_pointer(), e), arr) as Ptr::<i32>),
                1
            )
            .read())
                == 5))
            && ((*{ (*h3.borrow()).p.clone() }.as_ref().unwrap().borrow()) == 9))
            && (({ (*h2.borrow()).p.clone() }.as_pointer()).is_null())
    );
    return 0;
}
pub trait BufferImpl {
    fn move_assign(&self, _a0: Ptr<Buffer>) -> Ptr<Buffer>;
}
impl BufferImpl for Ptr<Buffer> {
    fn move_assign(&self, _a0: Ptr<Buffer>) -> Ptr<Buffer> {
        ((*self).with(|__s| __s.data.as_pointer()) as Ptr<Vec<i32>>).write(std::mem::take(
            &mut (*{ (*_a0.upgrade().deref()).data.clone() }.borrow_mut()),
        ));
        ((*self).with(|__s| __s.rows.as_pointer()) as Ptr<Vec<Value<Vec<i32>>>>).write(
            std::mem::take(&mut (*{ (*_a0.upgrade().deref()).rows.clone() }.borrow_mut())),
        );
        field!((*self), n).write({ { (*_a0.upgrade().deref()).n } });
        {
            ((array_field_ptr!((*self), arr)) as Ptr<i32>)
                .to_any()
                .memcpy(
                    &((array_field_ptr!(_a0, arr)) as Ptr<i32>).to_any(),
                    8_usize as usize,
                );
            ((array_field_ptr!((*self), arr)) as Ptr<i32>).to_any()
        };
        return (*self).clone();
    }
}
pub trait DefaultCopyUserMoveImpl {
    fn move_assign(&self, o: Ptr<DefaultCopyUserMove>) -> Ptr<DefaultCopyUserMove>;
}
impl DefaultCopyUserMoveImpl for Ptr<DefaultCopyUserMove> {
    fn move_assign(&self, o: Ptr<DefaultCopyUserMove>) -> Ptr<DefaultCopyUserMove> {
        field!((*self), v).write({ o.with(|__s| __s.v) });
        field!(o, v).write(0);
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
        field!((*self), inner).write({ { (*_a0.upgrade().deref()).inner.clone() } });
        field!((*self), e).write({ { (*_a0.upgrade().deref()).e.clone() } });
        (field_ptr!((*self), p) as Ptr<Option<Value<i32>>>)
            .write(field!(_a0, p).with_mut(|__v: &mut Option<Value<i32>>| __v.take()));
        return (*self).clone();
    }
    fn destructor(&self) {
        field_ptr!(self, e).destructor();
    }
}
pub trait OwnerImpl {
    fn move_assign(&self, _a0: Ptr<Owner>) -> Ptr<Owner>;
}
impl OwnerImpl for Ptr<Owner> {
    fn move_assign(&self, _a0: Ptr<Owner>) -> Ptr<Owner> {
        ((*self).with(|__s| __s.data.as_pointer()) as Ptr<Vec<i32>>).write(std::mem::take(
            &mut (*{ (*_a0.upgrade().deref()).data.clone() }.borrow_mut()),
        ));
        field!((*self), n).write({ { (*_a0.upgrade().deref()).n } });
        {
            ((array_field_ptr!((*self), arr)) as Ptr<i32>)
                .to_any()
                .memcpy(
                    &((array_field_ptr!(_a0, arr)) as Ptr<i32>).to_any(),
                    8_usize as usize,
                );
            ((array_field_ptr!((*self), arr)) as Ptr<i32>).to_any()
        };
        (field_ptr!((*self), p) as Ptr<Option<Value<i32>>>)
            .write(field!(_a0, p).with_mut(|__v: &mut Option<Value<i32>>| __v.take()));
        return (*self).clone();
    }
}
pub trait UserCopyDefaultMoveImpl {
    fn copy_assign(&self, o: Ptr<UserCopyDefaultMove>) -> Ptr<UserCopyDefaultMove>;
    fn move_assign(&self, _a0: Ptr<UserCopyDefaultMove>) -> Ptr<UserCopyDefaultMove>;
}
impl UserCopyDefaultMoveImpl for Ptr<UserCopyDefaultMove> {
    fn copy_assign(&self, o: Ptr<UserCopyDefaultMove>) -> Ptr<UserCopyDefaultMove> {
        field!((*self), v).write({ (o.with(|__s| __s.v) + 100) });
        return (*self).clone();
    }
    fn move_assign(&self, _a0: Ptr<UserCopyDefaultMove>) -> Ptr<UserCopyDefaultMove> {
        field!((*self), v).write({ { (*_a0.upgrade().deref()).v } });
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
