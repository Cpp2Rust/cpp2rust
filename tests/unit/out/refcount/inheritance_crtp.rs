extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct Counter_Impl_ {
    pub n: Value<i32>,
}
impl Clone for Counter_Impl_ {
    fn clone(&self) -> Self {
        let __this: Value<Counter_Impl_> = Rc::new(RefCell::new(Self {
            n: Rc::new(RefCell::new((*self.n.borrow()))),
        }));
        let this: Ptr<Counter_Impl_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for Counter_Impl_ {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.n.borrow()).to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            n: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
        }
    }
}
#[derive(Default)]
pub struct Impl {
    pub base_Counter_Impl_: Value<Counter_Impl_>,
}
impl Clone for Impl {
    fn clone(&self) -> Self {
        let __this: Value<Impl> = Rc::new(RefCell::new(Self {
            base_Counter_Impl_: Rc::new(RefCell::new((self as Counter_Impl_).clone())),
        }));
        let this: Ptr<Impl> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for Impl {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.base_Counter_Impl_.borrow()).to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            base_Counter_Impl_: Rc::new(RefCell::new(<Counter_Impl_>::from_bytes(&buf[0..4]))),
        }
    }
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let i: Value<Impl> = Rc::new(RefCell::new(<Impl>::default()));
    ({
        Counter_Impl_Impl::inc(
            &(({ Counter_Impl_Impl::inc(&(i.as_pointer() as Counter_Impl_)) }) as Counter_Impl_),
        )
    });
    assert!((({ ImplImpl::twice(&i.as_pointer(),) }) == 4));
    assert!(
        (({
            ImplImpl::twice(&({ Counter_Impl_Impl::inc(&(i.as_pointer() as Counter_Impl_)) }))
        }) == 6)
    );
    return 0;
}
pub trait Counter_Impl_Impl {
    fn inc(&self) -> Ptr<Impl>;
}
impl Counter_Impl_Impl for Ptr<Counter_Impl_> {
    fn inc(&self) -> Ptr<Impl> {
        (*(*(*self).upgrade().deref()).n.borrow_mut()).prefix_inc();
        return ((*self) as *mut Impl).clone();
    }
}
pub trait ImplImpl {
    fn twice(&self) -> i32;
}
impl ImplImpl for Ptr<Impl> {
    fn twice(&self) -> i32 {
        return ((*(*((*self) as Ptr<Counter_Impl_>).upgrade().deref())
            .n
            .borrow())
            * 2);
    }
}
