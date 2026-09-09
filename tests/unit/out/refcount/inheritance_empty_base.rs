extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct Tag {}
impl Clone for Tag {
    fn clone(&self) -> Self {
        let __this: Value<Tag> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<Tag> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for Tag {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
#[derive(Default)]
pub struct View {
    pub base_Tag: Value<Tag>,
    pub i: Value<i32>,
}
impl Clone for View {
    fn clone(&self) -> Self {
        let __this: Value<View> = Rc::new(RefCell::new(Self {
            base_Tag: Rc::new(RefCell::new((*self.base_Tag.borrow()).clone())),
            i: Rc::new(RefCell::new((*self.i.borrow()))),
        }));
        let this: Ptr<View> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for View {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.base_Tag.borrow()).to_bytes(&mut buf[0..1]);
        (*self.i.borrow()).to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            base_Tag: Rc::new(RefCell::new(<Tag>::from_bytes(&buf[0..1]))),
            i: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
        }
    }
}
#[derive(Default)]
pub struct Base {}
impl Clone for Base {
    fn clone(&self) -> Self {
        let __this: Value<Base> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<Base> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for Base {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
#[derive(Default)]
pub struct Derived {
    pub base_Base: Value<Base>,
}
impl Clone for Derived {
    fn clone(&self) -> Self {
        let __this: Value<Derived> = Rc::new(RefCell::new(Self {
            base_Base: Rc::new(RefCell::new((*self.base_Base.borrow()).clone())),
        }));
        let this: Ptr<Derived> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for Derived {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.base_Base.borrow()).to_bytes(&mut buf[0..1]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            base_Base: Rc::new(RefCell::new(<Base>::from_bytes(&buf[0..1]))),
        }
    }
}
pub fn as_base_0(d: Ptr<Derived>) -> Ptr<Base> {
    let d: Value<Ptr<Derived>> = Rc::new(RefCell::new(d));
    return ((*(*d.borrow()).upgrade().deref()).base_Base.as_pointer());
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let v: Value<View> = Rc::new(RefCell::new(<View>::default()));
    (*(*v.borrow()).i.borrow_mut()) = 5;
    assert!(((*(*v.borrow()).i.borrow()) == 5));
    let d: Value<Derived> = Rc::new(RefCell::new(<Derived>::default()));
    let b: Value<Ptr<Base>> = Rc::new(RefCell::new(({ as_base_0((d.as_pointer())) })));
    assert!({
        let _lhs = (*b.borrow()).clone();
        _lhs == ((*(d.as_pointer()).upgrade().deref()).base_Base.as_pointer())
    });
    return 0;
}
