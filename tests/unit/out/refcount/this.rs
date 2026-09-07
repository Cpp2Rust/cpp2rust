extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct S {
    pub a_: Value<i32>,
    pub self__: Value<Ptr<S>>,
}
impl S {
    pub fn S(a: i32) -> Self {
        let a: Value<i32> = Rc::new(RefCell::new(a));
        let __this: Value<S> = Rc::new(RefCell::new(Self {
            a_: Rc::new(RefCell::new((*a.borrow()))),
            self__: Rc::new(RefCell::new(Ptr::<S>::null())),
        }));
        let this: Ptr<S> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
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
}
impl Clone for S {
    fn clone(&self) -> Self {
        let __this: Value<S> = Rc::new(RefCell::new(Self {
            a_: Rc::new(RefCell::new((*self.a_.borrow()))),
            self__: Rc::new(RefCell::new((*self.self__.borrow()).clone())),
        }));
        let this: Ptr<S> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for S {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.a_.borrow()).to_bytes(&mut buf[0..4]);
        (*self.self__.borrow()).to_bytes(&mut buf[8..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            a_: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
            self__: Rc::new(RefCell::new(<Ptr<S>>::from_bytes(&buf[8..16]))),
        }
    }
}
pub fn bump_0(p: Ptr<S>) {
    let p: Value<Ptr<S>> = Rc::new(RefCell::new(p));
    (*(*(*p.borrow()).upgrade().deref()).a_.borrow_mut()).postfix_inc();
}
#[derive(Default)]
pub struct D {
    pub a_: Value<i32>,
}
impl D {
    pub fn D(a: i32) -> Self {
        let a: Value<i32> = Rc::new(RefCell::new(a));
        let __this: Value<D> = Rc::new(RefCell::new(Self {
            a_: Rc::new(RefCell::new((*a.borrow()))),
        }));
        let this: Ptr<D> = __this.as_pointer();
        (*(*this.upgrade().deref()).a_.borrow_mut()) *= 2;
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for D {
    fn clone(&self) -> Self {
        let __this: Value<D> = Rc::new(RefCell::new(Self {
            a_: Rc::new(RefCell::new((*self.a_.borrow()))),
        }));
        let this: Ptr<D> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for D {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.a_.borrow()).to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            a_: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
        }
    }
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let s: Value<S> = Rc::new(RefCell::new(S::S({ 1 })));
    let ref_: Ptr<S> = ({ SImpl::returns_this_reference(&s.as_pointer()) });
    (*(*ref_.upgrade().deref()).a_.borrow_mut()).postfix_inc();
    assert!(((*(*s.borrow()).a_.borrow()) == 2));
    let ptr: Value<Ptr<S>> = Rc::new(RefCell::new(
        ({ SImpl::returns_this_pointer(&s.as_pointer()) }),
    ));
    (*(*(*ptr.borrow()).upgrade().deref()).a_.borrow_mut()).postfix_inc();
    assert!(((*(*s.borrow()).a_.borrow()) == 3));
    ({ SImpl::inc(&({ SImpl::inc(&({ SImpl::inc(&s.as_pointer()) })) })) });
    assert!(((*(*s.borrow()).a_.borrow()) == 6));
    ({ SImpl::set_from_this(&s.as_pointer()) });
    assert!(((*(*s.borrow()).a_.borrow()) == 7));
    assert!((({ SImpl::twice(&s.as_pointer(),) }) == 14));
    ({ SImpl::link(&s.as_pointer()) });
    assert!({
        let _lhs = (*(*s.borrow()).self__.borrow()).clone();
        _lhs == (s.as_pointer())
    });
    (*(*(*(*s.borrow()).self__.borrow()).upgrade().deref())
        .a_
        .borrow_mut())
    .postfix_inc();
    assert!(((*(*s.borrow()).a_.borrow()) == 8));
    ({ SImpl::bump_me(&s.as_pointer()) });
    assert!(((*(*s.borrow()).a_.borrow()) == 9));
    let d: Value<D> = Rc::new(RefCell::new(D::D({ 3 })));
    assert!(((*(*d.borrow()).a_.borrow()) == 6));
    let cr: Ptr<S> = ({ SImpl::cref(&s.as_pointer()) });
    assert!(((*(*cr.upgrade().deref()).a_.borrow()) == 9));
    let t: Value<S> = Rc::new(RefCell::new(S::S({ 0 })));
    assert!(
        ({
            let _o: Ptr<S> = (s.as_pointer());
            SImpl::is(&s.as_pointer(), _o)
        })
    );
    assert!(!({ SImpl::is(&s.as_pointer(), (t.as_pointer()),) }));
    let p: Value<Ptr<S>> = Rc::new(RefCell::new(Ptr::alloc(S::S({ 1 }))));
    let q: Value<Ptr<S>> = Rc::new(RefCell::new(
        ({ SImpl::returns_this_pointer(&(*p.borrow())) }),
    ));
    (*(*(*q.borrow()).upgrade().deref()).a_.borrow_mut()).postfix_inc();
    assert!(((*(*(*p.borrow()).upgrade().deref()).a_.borrow()) == 2));
    (*p.borrow()).delete();
    let h: Value<Ptr<S>> = Rc::new(RefCell::new(Ptr::alloc(S::S({ 5 }))));
    ({ SImpl::destroy(&(*h.borrow())) });
    ({ SImpl::reset(&s.as_pointer()) });
    assert!(((*(*s.borrow()).a_.borrow()) == 0));
    assert!((*(*s.borrow()).self__.borrow()).is_null());
    return 0;
}
impl SImpl for Ptr<S> {
    fn returns_this_reference(&self) -> Ptr<S> {
        return (*self).clone();
    }
    fn returns_this_pointer(&self) -> Ptr<S> {
        return (*self).clone();
    }
    fn inc(&self) -> Ptr<S> {
        (*(*(*self).upgrade().deref()).a_.borrow_mut()).postfix_inc();
        return (*self).clone();
    }
    fn set_from_this(&self) {
        let __rhs = ((*(*(*self).upgrade().deref()).a_.borrow()) + 1);
        (*(*(*self).upgrade().deref()).a_.borrow_mut()) = __rhs;
    }
    fn get(&self) -> i32 {
        return (*(*(*self).upgrade().deref()).a_.borrow());
    }
    fn twice(&self) -> i32 {
        return (({ SImpl::get(self) }) * 2);
    }
    fn link(&self) {
        (*(*(*self).upgrade().deref()).self__.borrow_mut()) = (*self).clone();
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
        (*self).write(S::S({ 0 }));
    }
}
