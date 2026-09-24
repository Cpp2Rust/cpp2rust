extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct Owner {
    pub p: Value<Ptr<i32>>,
}
impl Owner {
    pub fn new(v: i32) -> Self {
        let v: Value<i32> = Rc::new(RefCell::new(v));
        let __this: Value<Owner> = Rc::new(RefCell::new(Self {
            p: Rc::new(RefCell::new(Ptr::alloc((*v.borrow())))),
        }));
        let this: Ptr<Owner> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn move_from(o: Ptr<Owner>) -> Self {
        let __this: Value<Owner> = Rc::new(RefCell::new(Self {
            p: Rc::new(RefCell::new((*(*o.upgrade().deref()).p.borrow()).clone())),
        }));
        let this: Ptr<Owner> = __this.as_pointer();
        (*(*o.upgrade().deref()).p.borrow_mut()) = Ptr::<i32>::null();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for Owner {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.p.borrow()).to_bytes(&mut buf[0..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            p: Rc::new(RefCell::new(<Ptr<i32>>::from_bytes(&buf[0..8]))),
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let o: Value<Owner> = Rc::new(RefCell::new(Owner::new({ 5 })));
    let _dtor_o = ScopedDestructor::new(&o, |__p| __p.destructor());
    let f: Value<_> = Rc::new(RefCell::new(
        (|| {
            return ((*(*h.borrow()).p.borrow()).read());
        }),
    ));
    assert!((*(*o.borrow()).p.borrow()).is_null());
    assert!((({ (*f.borrow_mut())() }) == 5));
    let g: Value<_> = Rc::new(RefCell::new((*f.borrow_mut()).clone()));
    assert!((({ (*g.borrow_mut())() }) == 5));
    let total: Value<i32> = Rc::new(RefCell::new(0));
    let consume: Value<_> = Rc::new(RefCell::new(
        (|| {
            let __rhs = ((*(*h.borrow()).p.borrow()).read());
            (*total.borrow_mut()) += __rhs;
            (*(*h.borrow()).p.borrow()).write(0);
        }),
    ));
    ({ (*consume.borrow_mut())() });
    ({ (*consume.borrow_mut())() });
    assert!(((*total.borrow()) == 7));
    return 0;
}
pub trait OwnerImpl {
    fn destructor(&self);
}
impl OwnerImpl for Ptr<Owner> {
    fn destructor(&self) {
        (*(*(*self).upgrade().deref()).p.borrow()).delete();
    }
}
pub fn __cpp2rust_init_globals() {}
