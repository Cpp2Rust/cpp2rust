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
#[derive(Record, VaArg, FnPtrArg, Default)]
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
            v: { o.with(|__s: &Partial| __s.v) },
            keep: { o.with(|__s: &Partial| __s.keep) },
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
<<<<<<< HEAD
impl ByteRepr for Partial {}
#[derive(VaArg, FnPtrArg)]
=======
impl ByteRepr for Partial {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.v.to_bytes(&mut buf[0..4]);
        self.keep.to_bytes(&mut buf[4..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            v: <i32>::from_bytes(&buf[0..4]),
            keep: <i32>::from_bytes(&buf[4..8]),
        }
    }
}
#[derive(Clone, Record, VaArg, FnPtrArg)]
>>>>>>> 3ed38b58 (Remove Value<> boxing from struct fields)
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
<<<<<<< HEAD
impl ByteRepr for NonConstAssign {}
#[derive(VaArg, FnPtrArg)]
=======
impl ByteRepr for NonConstAssign {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.mark.to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            mark: <i32>::from_bytes(&buf[0..4]),
        }
    }
}
#[derive(Clone, Record, VaArg, FnPtrArg)]
>>>>>>> 3ed38b58 (Remove Value<> boxing from struct fields)
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
<<<<<<< HEAD
impl ByteRepr for RefQualified {}
#[derive(VaArg, FnPtrArg)]
=======
impl ByteRepr for RefQualified {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.mark.to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            mark: <i32>::from_bytes(&buf[0..4]),
        }
    }
}
#[derive(Clone, Record, VaArg, FnPtrArg)]
>>>>>>> 3ed38b58 (Remove Value<> boxing from struct fields)
pub struct Holder {
    #[offset(0)]
    pub p: Partial,
    #[offset(8)]
    pub arr: Box<[Partial]>,
}
impl Default for Holder {
    fn default() -> Self {
        Holder {
            p: <Partial>::default(),
            arr: (0..2)
                .map(|_| <Partial>::default())
                .collect::<Box<[Partial]>>(),
        }
    }
}
<<<<<<< HEAD
impl ByteRepr for Holder {}
=======
impl ByteRepr for Holder {
    fn byte_size() -> usize {
        24
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.p.to_bytes(&mut buf[0..8]);
        self.arr.to_bytes(&mut buf[8..24]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            p: <Partial>::from_bytes(&buf[0..8]),
            arr: <Box<[Partial]>>::from_bytes(&buf[8..24]),
        }
    }
}
>>>>>>> 3ed38b58 (Remove Value<> boxing from struct fields)
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
        arr: Box::new([Partial::new({ 5 }, { 50 }), Partial::new({ 6 }, { 60 })]),
    }));
    ({ PartialImpl::copy_assign(&{ field_ptr!(h, p) }, b.as_pointer()) });
    ({
        PartialImpl::copy_assign(
            &{ (field_ptr!(h, arr) as Ptr<Partial>).offset(1) },
            c.as_pointer(),
        )
    });
    assert!(({ (*h.borrow()).p.v } == 2) && ({ (*h.borrow()).p.keep } == 40));
    assert!(
        ({ (*h.borrow()).arr[(1) as usize].v } == 2)
            && ({ (*h.borrow()).arr[(1) as usize].keep } == 60)
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
        let __rhs = (o.with(|__s: &NonConstAssign| __s.mark) + 1);
        (*self).with_mut(|__s: &mut NonConstAssign| __s.mark = __rhs);
        return (*self).clone();
    }
    fn operator_assign_3(&self, o: Ptr<NonConstAssign>) -> Ptr<NonConstAssign> {
        let __rhs = (o.with(|__s: &NonConstAssign| __s.mark) + 10);
        (*self).with_mut(|__s: &mut NonConstAssign| __s.mark = __rhs);
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
        let __rhs = o.with(|__s: &Partial| __s.v);
        (*self).with_mut(|__s: &mut Partial| __s.v = __rhs);
        (*assigns_0.with(Value::clone).borrow_mut()).prefix_inc();
        return (*self).clone();
    }
}
pub trait RefQualifiedImpl {
    fn copy_assign(&self, o: Ptr<RefQualified>) -> Ptr<RefQualified>;
}
impl RefQualifiedImpl for Ptr<RefQualified> {
    fn copy_assign(&self, o: Ptr<RefQualified>) -> Ptr<RefQualified> {
        let __rhs = (o.with(|__s: &RefQualified| __s.mark) + 1);
        (*self).with_mut(|__s: &mut RefQualified| __s.mark = __rhs);
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = assigns_0.with(|_| ());
}
