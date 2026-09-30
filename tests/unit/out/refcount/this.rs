extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct S {
    #[offset(0)]
    pub a_: i32,
    #[offset(8)]
    #[byte_size(8)]
    pub self__: Ptr<S>,
}
impl S {
    pub fn new_1(a: i32) -> Self {
        let a: Value<i32> = Rc::new(RefCell::new(a));
        let __this: Value<S> = Rc::new(RefCell::new(Self {
            a_: (*a.borrow()),
            self__: Ptr::<S>::null(),
        }));
        let this: Ptr<S> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn new_2(a: i32, other: Ptr<S>) -> Self {
        let a: Value<i32> = Rc::new(RefCell::new(a));
        let other: Value<Ptr<S>> = Rc::new(RefCell::new(other));
        let __this: Value<S> = Rc::new(RefCell::new(Self {
            a_: (*a.borrow()),
            self__: Ptr::<S>::null(),
        }));
        let this: Ptr<S> = __this.as_pointer();
        if (this == (*other.borrow())) {
            field!(this, self__).write(Ptr::<S>::null());
        } else {
            field!(this, self__).write((*other.borrow()).clone());
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn new_3(a: i32, other: Ptr<S>) -> Self {
        let a: Value<i32> = Rc::new(RefCell::new(a));
        let other: Value<Ptr<S>> = Rc::new(RefCell::new(other));
        let __this: Value<S> = Rc::new(RefCell::new(Self {
            a_: (*a.borrow()),
            self__: Ptr::<S>::null(),
        }));
        let this: Ptr<S> = __this.as_pointer();
        if (this == (*other.borrow())) {
            field!(this, self__).write(Ptr::<S>::null());
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
pub fn bump_0(p: Ptr<S>) {
    let p: Value<Ptr<S>> = Rc::new(RefCell::new(p));
    field!((*p.borrow()), a_).with_mut(|__v| __v.postfix_inc());
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct D {
    #[offset(0)]
    pub a_: i32,
}
impl D {
    pub fn new(a: i32) -> Self {
        let a: Value<i32> = Rc::new(RefCell::new(a));
        let __this: Value<D> = Rc::new(RefCell::new(Self { a_: (*a.borrow()) }));
        let this: Ptr<D> = __this.as_pointer();
        {
            let _ptr = field!(this, a_);
            _ptr.write(_ptr.read() * 2)
        };
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let s: Value<S> = Rc::new(RefCell::new(S::new_1({ 1 })));
    let ref_: Ptr<S> = ({ SImpl::returns_this_reference(&s.as_pointer()) });
    field!(ref_, a_).with_mut(|__v| __v.postfix_inc());
    assert!(({ (*s.borrow()).a_ } == 2));
    let ptr: Value<Ptr<S>> = Rc::new(RefCell::new(
        ({ SImpl::returns_this_pointer(&s.as_pointer()) }),
    ));
    field!((*ptr.borrow()), a_).with_mut(|__v| __v.postfix_inc());
    assert!(({ (*s.borrow()).a_ } == 3));
    ({ SImpl::inc(&({ SImpl::inc(&({ SImpl::inc(&s.as_pointer()) })) })) });
    assert!(({ (*s.borrow()).a_ } == 6));
    ({ SImpl::set_from_this(&s.as_pointer()) });
    assert!(({ (*s.borrow()).a_ } == 7));
    assert!((({ SImpl::twice(&s.as_pointer(),) }) == 14));
    ({ SImpl::link(&s.as_pointer()) });
    assert!(({ { (*s.borrow()).self__.clone() } } == { (s.as_pointer()) }));
    field!({ (*s.borrow()).self__.clone() }, a_).with_mut(|__v| __v.postfix_inc());
    assert!(({ (*s.borrow()).a_ } == 8));
    ({ SImpl::bump_me(&s.as_pointer()) });
    assert!(({ (*s.borrow()).a_ } == 9));
    let d: Value<D> = Rc::new(RefCell::new(D::new({ 3 })));
    assert!(({ (*d.borrow()).a_ } == 6));
    let cr: Ptr<S> = ({ SImpl::cref(&s.as_pointer()) });
    assert!((cr.with(|__s| __s.a_) == 9));
    let t: Value<S> = Rc::new(RefCell::new(S::new_1({ 0 })));
    assert!(
        ({
            let _o: Ptr<S> = (s.as_pointer());
            SImpl::is(&s.as_pointer(), _o)
        })
    );
    assert!(!({ SImpl::is(&s.as_pointer(), (t.as_pointer()),) }));
    let p: Value<Ptr<S>> = Rc::new(RefCell::new(Ptr::alloc(S::new_1({ 1 }))));
    let q: Value<Ptr<S>> = Rc::new(RefCell::new(
        ({ SImpl::returns_this_pointer(&(*p.borrow())) }),
    ));
    field!((*q.borrow()), a_).with_mut(|__v| __v.postfix_inc());
    assert!(((*p.borrow()).with(|__s| __s.a_) == 2));
    (*p.borrow()).delete();
    let h: Value<Ptr<S>> = Rc::new(RefCell::new(Ptr::alloc(S::new_1({ 5 }))));
    ({ SImpl::destroy(&(*h.borrow())) });
    ({ SImpl::reset(&s.as_pointer()) });
    assert!(({ (*s.borrow()).a_ } == 0));
    assert!(({ (*s.borrow()).self__.clone() }).is_null());
    assert!(
        ((({
            let _other: Ptr<S> = (s.as_pointer());
            SImpl::copy_if_different(&s.as_pointer(), _other)
        }) as i32)
            == (false as i32))
    );
    assert!(
        ((({
            let _other: Ptr<S> = (s.as_pointer());
            SImpl::copy_if_different_const(&s.as_pointer(), _other)
        }) as i32)
            == (false as i32))
    );
    let other: Value<S> = Rc::new(RefCell::new(S::new_1({ 22 })));
    assert!(
        ((({ SImpl::copy_if_different(&s.as_pointer(), (other.as_pointer()),) }) as i32)
            == (true as i32))
    );
    assert!(({ (*s.borrow()).a_ } == { (*other.borrow()).a_ }));
    assert!(({ { (*s.borrow()).self__.clone() } } == { { (*other.borrow()).self__.clone() } }));
    let u: Value<S> = Rc::new(RefCell::new(S::new_2({ 1 }, { (s.as_pointer()) })));
    assert!(({ { (*u.borrow()).self__.clone() } } == { (s.as_pointer()) }));
    let s_const: Value<S> = Rc::new(RefCell::new(S::new_1({ 100 })));
    let u1: Value<S> = Rc::new(RefCell::new(S::new_3({ 1 }, { (s_const.as_pointer()) })));
    assert!(({ (*u1.borrow()).self__.clone() }).is_null());
    return 0;
}
pub trait SImpl {
    fn returns_this_reference(&self) -> Ptr<S>;
    fn returns_this_pointer(&self) -> Ptr<S>;
    fn inc(&self) -> Ptr<S>;
    fn set_from_this(&self);
    fn get(&self) -> i32;
    fn twice(&self) -> i32;
    fn link(&self);
    fn bump_me(&self);
    fn cref(&self) -> Ptr<S>;
    fn is(&self, o: Ptr<S>) -> bool;
    fn destroy(&self);
    fn reset(&self);
    fn copy_if_different_const(&self, other: Ptr<S>) -> bool;
    fn copy_if_different(&self, other: Ptr<S>) -> bool;
}
impl SImpl for Ptr<S> {
    fn returns_this_reference(&self) -> Ptr<S> {
        return (*self).clone();
    }
    fn returns_this_pointer(&self) -> Ptr<S> {
        return (*self).clone();
    }
    fn inc(&self) -> Ptr<S> {
        field!((*self), a_).with_mut(|__v| __v.postfix_inc());
        return (*self).clone();
    }
    fn set_from_this(&self) {
        field!((*self), a_).write({ ((*self).with(|__s| __s.a_) + 1) });
    }
    fn get(&self) -> i32 {
        return (*self).with(|__s| __s.a_);
    }
    fn twice(&self) -> i32 {
        return (({ SImpl::get(self) }) * 2);
    }
    fn link(&self) {
        field!((*self), self__).write((*self).clone());
    }
    fn bump_me(&self) {
        ({ bump_0((*self).clone()) });
    }
    fn cref(&self) -> Ptr<S> {
        return (*self).clone();
    }
    fn is(&self, o: Ptr<S>) -> bool {
        let o: Value<Ptr<S>> = Rc::new(RefCell::new(o));
        return ((*o.borrow()) == (*self));
    }
    fn destroy(&self) {
        (*self).delete();
    }
    fn reset(&self) {
        (*self).write(S::new_1({ 0 }));
    }
    fn copy_if_different_const(&self, other: Ptr<S>) -> bool {
        let other: Value<Ptr<S>> = Rc::new(RefCell::new(other));
        if ((*self) == (*other.borrow())) {
            return false;
        }
        field!((*self), a_).write({ (*other.borrow()).with(|__s| __s.a_) });
        field!((*self), self__).write({ (*other.borrow()).with(|__s| __s.self__.clone()) });
        return true;
    }
    fn copy_if_different(&self, other: Ptr<S>) -> bool {
        let other: Value<Ptr<S>> = Rc::new(RefCell::new(other));
        if ((*self) == (*other.borrow())) {
            return false;
        }
        field!((*self), a_).write({ (*other.borrow()).with(|__s| __s.a_) });
        field!((*self), self__).write({ (*other.borrow()).with(|__s| __s.self__.clone()) });
        return true;
    }
}
pub fn __cpp2rust_init_globals() {}
