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
#[derive(Default)]
pub struct Partial {
    pub v: Value<i32>,
    pub keep: Value<i32>,
}
impl Partial {
    pub fn Partial(v: i32, keep: i32) -> Self {
        let v: Value<i32> = Rc::new(RefCell::new(v));
        let keep: Value<i32> = Rc::new(RefCell::new(keep));
        let __this: Value<Partial> = Rc::new(RefCell::new(Self {
            v: Rc::new(RefCell::new((*v.borrow()))),
            keep: Rc::new(RefCell::new((*keep.borrow()))),
        }));
        let this: Ptr<Partial> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn Partial_pconstPartial(o: Ptr<Partial>) -> Self {
        let __this: Value<Partial> = Rc::new(RefCell::new(Self {
            v: Rc::new(RefCell::new((*(*o.upgrade().deref()).v.borrow()))),
            keep: Rc::new(RefCell::new((*(*o.upgrade().deref()).keep.borrow()))),
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
        Partial::Partial_pconstPartial(__src.as_pointer())
    }
}
impl ByteRepr for Partial {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.v.borrow()).to_bytes(&mut buf[0..4]);
        (*self.keep.borrow()).to_bytes(&mut buf[4..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            v: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
            keep: Rc::new(RefCell::new(<i32>::from_bytes(&buf[4..8]))),
        }
    }
}
#[derive()]
pub struct NonConstAssign {
    pub mark: Value<i32>,
}
impl NonConstAssign {
    pub fn NonConstAssign() -> Self {
        let __this: Value<NonConstAssign> = Rc::new(RefCell::new(Self {
            mark: Rc::new(RefCell::new(0)),
        }));
        let this: Ptr<NonConstAssign> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for NonConstAssign {
    fn clone(&self) -> Self {
        let __this: Value<NonConstAssign> = Rc::new(RefCell::new(Self {
            mark: Rc::new(RefCell::new((*self.mark.borrow()))),
        }));
        let this: Ptr<NonConstAssign> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for NonConstAssign {
    fn default() -> Self {
        { NonConstAssign::NonConstAssign() }
    }
}
impl ByteRepr for NonConstAssign {
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
#[derive()]
pub struct RefQualified {
    pub mark: Value<i32>,
}
impl RefQualified {
    pub fn RefQualified() -> Self {
        let __this: Value<RefQualified> = Rc::new(RefCell::new(Self {
            mark: Rc::new(RefCell::new(0)),
        }));
        let this: Ptr<RefQualified> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for RefQualified {
    fn clone(&self) -> Self {
        let __this: Value<RefQualified> = Rc::new(RefCell::new(Self {
            mark: Rc::new(RefCell::new((*self.mark.borrow()))),
        }));
        let this: Ptr<RefQualified> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for RefQualified {
    fn default() -> Self {
        { RefQualified::RefQualified() }
    }
}
impl ByteRepr for RefQualified {
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
#[derive()]
pub struct Holder {
    pub p: Value<Partial>,
    pub arr: Value<Box<[Partial]>>,
}
impl Clone for Holder {
    fn clone(&self) -> Self {
        let __this: Value<Holder> = Rc::new(RefCell::new(Self {
            p: Rc::new(RefCell::new(Partial::Partial_pconstPartial({
                self.p.as_pointer()
            }))),
            arr: Rc::new(RefCell::new((*self.arr.borrow()).clone())),
        }));
        let this: Ptr<Holder> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for Holder {
    fn default() -> Self {
        Holder {
            p: <Value<Partial>>::default(),
            arr: Rc::new(RefCell::new(
                (0..2)
                    .map(|_| <Partial>::default())
                    .collect::<Box<[Partial]>>(),
            )),
        }
    }
}
impl ByteRepr for Holder {
    fn byte_size() -> usize {
        24
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.p.borrow()).to_bytes(&mut buf[0..8]);
        (*self.arr.borrow()).to_bytes(&mut buf[8..24]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            p: Rc::new(RefCell::new(<Partial>::from_bytes(&buf[0..8]))),
            arr: Rc::new(RefCell::new(<Box<[Partial]>>::from_bytes(&buf[8..24]))),
        }
    }
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let a: Value<Partial> = Rc::new(RefCell::new(Partial::Partial({ 1 }, { 100 })));
    let b: Value<Partial> = Rc::new(RefCell::new(Partial::Partial({ 2 }, { 200 })));
    let c: Value<Partial> = Rc::new(RefCell::new(Partial::Partial({ 3 }, { 300 })));
    ({ PartialImpl::operator_assign(&a.as_pointer(), b.as_pointer()) });
    assert!(((*(*a.borrow()).v.borrow()) == 2) && ((*(*a.borrow()).keep.borrow()) == 100));
    assert!(((*assigns_0.with(Value::clone).borrow()) == 1));
    ({
        PartialImpl::operator_assign(
            &c.as_pointer(),
            ({ PartialImpl::operator_assign(&a.as_pointer(), b.as_pointer()) }),
        )
    });
    assert!(((*(*c.borrow()).v.borrow()) == 2) && ((*(*c.borrow()).keep.borrow()) == 300));
    assert!(((*assigns_0.with(Value::clone).borrow()) == 3));
    ({
        let _o: Ptr<Partial> = a.as_pointer();
        PartialImpl::operator_assign(&a.as_pointer(), _o)
    });
    assert!(((*assigns_0.with(Value::clone).borrow()) == 3));
    ({
        let _o: Value<Partial> = Rc::new(RefCell::new(Partial::Partial({ 9 }, { 900 })));
        PartialImpl::operator_assign(&a.as_pointer(), _o.as_pointer())
    });
    assert!(((*(*a.borrow()).v.borrow()) == 9) && ((*(*a.borrow()).keep.borrow()) == 100));
    assert!(((*assigns_0.with(Value::clone).borrow()) == 4));
    let ra: Ptr<Partial> = a.as_pointer();
    ({
        let _o: Ptr<Partial> = c.as_pointer();
        PartialImpl::operator_assign(&ra, _o)
    });
    assert!(((*(*a.borrow()).v.borrow()) == 2));
    let pa: Value<Ptr<Partial>> = Rc::new(RefCell::new((a.as_pointer())));
    ({
        let _o: Ptr<Partial> = b.as_pointer();
        PartialImpl::operator_assign(&(*pa.borrow()), _o)
    });
    assert!(((*(*a.borrow()).v.borrow()) == 2));
    assert!(((*assigns_0.with(Value::clone).borrow()) == 6));
    let h: Value<Holder> = Rc::new(RefCell::new(Holder {
        p: Rc::new(RefCell::new(Partial::Partial({ 4 }, { 40 }))),
        arr: Rc::new(RefCell::new(Box::new([
            Partial::Partial({ 5 }, { 50 }),
            Partial::Partial({ 6 }, { 60 }),
        ]))),
    }));
    ({ PartialImpl::operator_assign(&(*h.borrow()).p.as_pointer(), b.as_pointer()) });
    ({
        PartialImpl::operator_assign(
            &((*h.borrow()).arr.as_pointer() as Ptr<Partial>).offset(1),
            c.as_pointer(),
        )
    });
    assert!(
        ((*(*(*h.borrow()).p.borrow()).v.borrow()) == 2)
            && ((*(*(*h.borrow()).p.borrow()).keep.borrow()) == 40)
    );
    assert!(
        ((*(*(*h.borrow()).arr.borrow())[(1) as usize].v.borrow()) == 2)
            && ((*(*(*h.borrow()).arr.borrow())[(1) as usize].keep.borrow()) == 60)
    );
    assert!(((*assigns_0.with(Value::clone).borrow()) == 8));
    let n: Value<NonConstAssign> = Rc::new(RefCell::new(NonConstAssign::NonConstAssign()));
    let n1: Value<NonConstAssign> = Rc::new(RefCell::new(NonConstAssign::NonConstAssign()));
    let n2: Value<NonConstAssign> = Rc::new(RefCell::new(NonConstAssign::NonConstAssign()));
    let cn: Value<NonConstAssign> = Rc::new(RefCell::new(NonConstAssign::NonConstAssign()));
    ({ NonConstAssignImpl::operator_assign_pmutNonConstAssign(&n1.as_pointer(), n.as_pointer()) });
    ({
        NonConstAssignImpl::operator_assign_pconstNonConstAssign(&n2.as_pointer(), cn.as_pointer())
    });
    assert!(((*(*n1.borrow()).mark.borrow()) == 1));
    assert!(((*(*n2.borrow()).mark.borrow()) == 10));
    let r: Value<RefQualified> = Rc::new(RefCell::new(RefQualified::RefQualified()));
    let r1: Value<RefQualified> = Rc::new(RefCell::new(RefQualified::RefQualified()));
    ({ RefQualifiedImpl::operator_assign(&r1.as_pointer(), r.as_pointer()) });
    assert!(((*(*r1.borrow()).mark.borrow()) == 1));
    return 0;
}
pub trait NonConstAssignImpl {
    fn operator_assign_pmutNonConstAssign(&self, o: Ptr<NonConstAssign>) -> Ptr<NonConstAssign>;
    fn operator_assign_pconstNonConstAssign(&self, o: Ptr<NonConstAssign>) -> Ptr<NonConstAssign>;
}
impl NonConstAssignImpl for Ptr<NonConstAssign> {
    fn operator_assign_pmutNonConstAssign(&self, o: Ptr<NonConstAssign>) -> Ptr<NonConstAssign> {
        let __rhs = ((*(*o.upgrade().deref()).mark.borrow()) + 1);
        (*(*(*self).upgrade().deref()).mark.borrow_mut()) = __rhs;
        return (*self).clone();
    }
    fn operator_assign_pconstNonConstAssign(&self, o: Ptr<NonConstAssign>) -> Ptr<NonConstAssign> {
        let __rhs = ((*(*o.upgrade().deref()).mark.borrow()) + 10);
        (*(*(*self).upgrade().deref()).mark.borrow_mut()) = __rhs;
        return (*self).clone();
    }
}
pub trait PartialImpl {
    fn operator_assign(&self, o: Ptr<Partial>) -> Ptr<Partial>;
}
impl PartialImpl for Ptr<Partial> {
    fn operator_assign(&self, o: Ptr<Partial>) -> Ptr<Partial> {
        if ((*self) == (o)) {
            return (*self).clone();
        }
        let __rhs = (*(*o.upgrade().deref()).v.borrow());
        (*(*(*self).upgrade().deref()).v.borrow_mut()) = __rhs;
        (*assigns_0.with(Value::clone).borrow_mut()).prefix_inc();
        return (*self).clone();
    }
}
pub trait RefQualifiedImpl {
    fn operator_assign(&self, o: Ptr<RefQualified>) -> Ptr<RefQualified>;
}
impl RefQualifiedImpl for Ptr<RefQualified> {
    fn operator_assign(&self, o: Ptr<RefQualified>) -> Ptr<RefQualified> {
        let __rhs = ((*(*o.upgrade().deref()).mark.borrow()) + 1);
        (*(*(*self).upgrade().deref()).mark.borrow_mut()) = __rhs;
        return (*self).clone();
    }
}
