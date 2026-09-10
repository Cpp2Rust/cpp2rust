extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct Base {
    pub a: Value<i32>,
    pub copies: Value<i32>,
}
impl Base {
    pub fn Base(x: i32) -> Self {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        let __this: Value<Base> = Rc::new(RefCell::new(Self {
            a: Rc::new(RefCell::new((*x.borrow()))),
            copies: Rc::new(RefCell::new(0)),
        }));
        let this: Ptr<Base> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn Base_pconstBase(o: Ptr<Base>) -> Self {
        let __this: Value<Base> = Rc::new(RefCell::new(Self {
            a: Rc::new(RefCell::new((*(*o.upgrade().deref()).a.borrow()))),
            copies: Rc::new(RefCell::new(
                ((*(*o.upgrade().deref()).copies.borrow()) + 1),
            )),
        }));
        let this: Ptr<Base> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for Base {
    fn clone(&self) -> Self {
        let __src: Value<Base> = Rc::new(RefCell::new(Base {
            a: self.a.clone(),
            copies: self.copies.clone(),
        }));
        Base::Base_pconstBase(__src.as_pointer())
    }
}
impl ByteRepr for Base {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.a.borrow()).to_bytes(&mut buf[0..4]);
        (*self.copies.borrow()).to_bytes(&mut buf[4..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            a: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
            copies: Rc::new(RefCell::new(<i32>::from_bytes(&buf[4..8]))),
        }
    }
}
#[derive(Default)]
pub struct Derived {
    pub base_Base: Value<Base>,
    pub b: Value<i32>,
}
impl Derived {
    pub fn Derived(x: i32) -> Self {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        let __this: Value<Derived> = Rc::new(RefCell::new(Self {
            base_Base: Rc::new(RefCell::new(Base::Base({ (*x.borrow()) }))),
            b: Rc::new(RefCell::new(((*x.borrow()) * 10))),
        }));
        let this: Ptr<Derived> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for Derived {
    fn clone(&self) -> Self {
        let __this: Value<Derived> = Rc::new(RefCell::new(Self {
            base_Base: Rc::new(RefCell::new(Base::Base_pconstBase({
                (self.base_Base.as_pointer())
            }))),
            b: Rc::new(RefCell::new((*self.b.borrow()))),
        }));
        let this: Ptr<Derived> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for Derived {
    fn byte_size() -> usize {
        12
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.base_Base.borrow()).to_bytes(&mut buf[0..8]);
        (*self.b.borrow()).to_bytes(&mut buf[8..12]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            base_Base: Rc::new(RefCell::new(<Base>::from_bytes(&buf[0..8]))),
            b: Rc::new(RefCell::new(<i32>::from_bytes(&buf[8..12]))),
        }
    }
}
pub fn take_0(v: Base) -> i32 {
    let v: Value<Base> = Rc::new(RefCell::new(v));
    return ((*(*v.borrow()).a.borrow()) + (*(*v.borrow()).copies.borrow()));
}
pub fn make_1(x: i32) -> Base {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    let d: Value<Derived> = Rc::new(RefCell::new(Derived::Derived({ (*x.borrow()) })));
    return Base::Base_pconstBase({ ((*d.borrow()).base_Base.as_pointer()) });
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let d: Value<Derived> = Rc::new(RefCell::new(Derived::Derived({ 1 })));
    let init: Value<Base> = Rc::new(RefCell::new(Base::Base_pconstBase({
        ((*d.borrow()).base_Base.as_pointer())
    })));
    assert!(((*(*init.borrow()).a.borrow()) == 1));
    assert!(((*(*init.borrow()).copies.borrow()) == 1));
    let assigned: Value<Base> = Rc::new(RefCell::new(Base::Base({ 9 })));
    ({
        BaseImpl::operator_assign(
            &assigned.as_pointer(),
            ((*d.borrow()).base_Base.as_pointer()),
        )
    });
    assert!(((*(*assigned.borrow()).a.borrow()) == 1));
    assert!(((*(*assigned.borrow()).copies.borrow()) == 1));
    assert!(
        (({
            take_0(Base::Base_pconstBase({
                ((*d.borrow()).base_Base.as_pointer())
            }))
        }) == 2)
    );
    let made: Value<Base> = Rc::new(RefCell::new(({ make_1(3) })));
    assert!(((*(*made.borrow()).a.borrow()) == 3));
    assert!(((*(*made.borrow()).copies.borrow()) >= 1));
    (*(*(*d.borrow()).base_Base.borrow()).a.borrow_mut()) = 7;
    assert!(((*(*init.borrow()).a.borrow()) == 1));
    assert!(((*(*assigned.borrow()).a.borrow()) == 1));
    return 0;
}
pub trait BaseImpl {
    fn operator_assign(&self, o: Ptr<Base>) -> Ptr<Base>;
}
impl BaseImpl for Ptr<Base> {
    fn operator_assign(&self, o: Ptr<Base>) -> Ptr<Base> {
        let __rhs = (*(*o.upgrade().deref()).a.borrow());
        (*(*(*self).upgrade().deref()).a.borrow_mut()) = __rhs;
        let __rhs = ((*(*o.upgrade().deref()).copies.borrow()) + 1);
        (*(*(*self).upgrade().deref()).copies.borrow_mut()) = __rhs;
        return (*self).clone();
    }
}
