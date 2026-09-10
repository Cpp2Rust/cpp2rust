extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static order_0: Value<Box<[i32]>> = Rc::new(RefCell::new(
        (0..8).map(|_| <i32>::default()).collect::<Box<[i32]>>(),
    ));
);
thread_local!(
    pub static n_1: Value<i32> = Rc::new(RefCell::new(0));
);
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
pub struct Member {}
impl Clone for Member {
    fn clone(&self) -> Self {
        let __this: Value<Member> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<Member> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for Member {
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
    pub m: Value<Member>,
}
impl Clone for Derived {
    fn clone(&self) -> Self {
        let __this: Value<Derived> = Rc::new(RefCell::new(Self {
            base_Base: Rc::new(RefCell::new((*self.base_Base.borrow()).clone())),
            m: Rc::new(RefCell::new((*self.m.borrow()).clone())),
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
        (*self.m.borrow()).to_bytes(&mut buf[0..1]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            base_Base: Rc::new(RefCell::new(<Base>::from_bytes(&buf[0..1]))),
            m: Rc::new(RefCell::new(<Member>::from_bytes(&buf[0..1]))),
        }
    }
}
#[derive(Default)]
pub struct Implicit {
    pub base_Base: Value<Base>,
    pub m: Value<Member>,
}
impl Clone for Implicit {
    fn clone(&self) -> Self {
        let __this: Value<Implicit> = Rc::new(RefCell::new(Self {
            base_Base: Rc::new(RefCell::new((*self.base_Base.borrow()).clone())),
            m: Rc::new(RefCell::new((*self.m.borrow()).clone())),
        }));
        let this: Ptr<Implicit> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for Implicit {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.base_Base.borrow()).to_bytes(&mut buf[0..1]);
        (*self.m.borrow()).to_bytes(&mut buf[0..1]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            base_Base: Rc::new(RefCell::new(<Base>::from_bytes(&buf[0..1]))),
            m: Rc::new(RefCell::new(<Member>::from_bytes(&buf[0..1]))),
        }
    }
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    {
        let d: Value<Derived> = Rc::new(RefCell::new(<Derived>::default()));
        let _dtor_d = ScopedDestructor::new(&d, |__p| __p.destructor());
    }
    assert!(((*n_1.with(Value::clone).borrow()) == 3));
    assert!(
        (((*order_0.with(Value::clone).borrow())[(0) as usize] == 3)
            && ((*order_0.with(Value::clone).borrow())[(1) as usize] == 2))
            && ((*order_0.with(Value::clone).borrow())[(2) as usize] == 1)
    );
    (*n_1.with(Value::clone).borrow_mut()) = 0;
    {
        let i: Value<Implicit> = Rc::new(RefCell::new(<Implicit>::default()));
        let _dtor_i = ScopedDestructor::new(&i, |__p| __p.destructor());
    }
    assert!(((*n_1.with(Value::clone).borrow()) == 2));
    assert!(
        ((*order_0.with(Value::clone).borrow())[(0) as usize] == 2)
            && ((*order_0.with(Value::clone).borrow())[(1) as usize] == 1)
    );
    (*n_1.with(Value::clone).borrow_mut()) = 0;
    let p: Value<Ptr<Derived>> = Rc::new(RefCell::new(Ptr::alloc(<Derived>::default())));
    {
        let __p = (*p.borrow()).clone();
        __p.destructor();
        __p.delete();
    };
    assert!(((*n_1.with(Value::clone).borrow()) == 3));
    assert!(
        (((*order_0.with(Value::clone).borrow())[(0) as usize] == 3)
            && ((*order_0.with(Value::clone).borrow())[(1) as usize] == 2))
            && ((*order_0.with(Value::clone).borrow())[(2) as usize] == 1)
    );
    return 0;
}
pub trait BaseImpl {
    fn destructor(&self);
}
impl BaseImpl for Ptr<Base> {
    fn destructor(&self) {
        (*order_0.with(Value::clone).borrow_mut())
            [((*n_1.with(Value::clone).borrow_mut()).postfix_inc()) as usize] = 1;
    }
}
pub trait DerivedImpl {
    fn destructor(&self);
}
impl DerivedImpl for Ptr<Derived> {
    fn destructor(&self) {
        (*order_0.with(Value::clone).borrow_mut())
            [((*n_1.with(Value::clone).borrow_mut()).postfix_inc()) as usize] = 3;
        (*self.upgrade().deref()).m.as_pointer().destructor();
        (*self.upgrade().deref())
            .base_Base
            .as_pointer()
            .destructor();
    }
}
pub trait ImplicitImpl {
    fn destructor(&self);
}
impl ImplicitImpl for Ptr<Implicit> {
    fn destructor(&self) {
        (*self.upgrade().deref()).m.as_pointer().destructor();
        (*self.upgrade().deref())
            .base_Base
            .as_pointer()
            .destructor();
    }
}
pub trait MemberImpl {
    fn destructor(&self);
}
impl MemberImpl for Ptr<Member> {
    fn destructor(&self) {
        (*order_0.with(Value::clone).borrow_mut())
            [((*n_1.with(Value::clone).borrow_mut()).postfix_inc()) as usize] = 2;
    }
}
