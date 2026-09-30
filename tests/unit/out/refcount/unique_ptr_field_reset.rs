extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, VaArg, FnPtrArg, Default)]
pub struct Data {
    #[offset(0)]
    pub v: i32,
}
impl ByteRepr for Data {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.v.to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            v: <i32>::from_bytes(&buf[0..4]),
        }
    }
}
#[derive(Record, Default)]
pub struct Holder {
    #[offset(0)]
    pub data: Option<Value<Data>>,
    #[offset(8)]
    pub n: i32,
}
impl Holder {
    pub fn move_from(_a0: Ptr<Holder>) -> Self {
        let __this: Value<Holder> = Rc::new(RefCell::new(Self {
            data: field!(_a0, data).with_mut(|__v: &mut Option<Value<Data>>| __v.take()),
            n: { (*_a0.upgrade().deref()).n },
        }));
        let this: Ptr<Holder> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for Holder {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.data.to_bytes(&mut buf[0..8]);
        self.n.to_bytes(&mut buf[8..12]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            data: <Option<Value<Data>>>::from_bytes(&buf[0..8]),
            n: <i32>::from_bytes(&buf[8..12]),
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let h: Value<Holder> = Rc::new(RefCell::new(<Holder>::default()));
    (*h.borrow_mut()).n = 1;
    let hp: Value<Ptr<Holder>> = Rc::new(RefCell::new((h.as_pointer())));
    {
        let _p: Ptr<_> = Ptr::alloc(Data { v: 3 });
        (field_ptr!((*hp.borrow()), data) as Ptr<Option<Value<Data>>>).write(_p.to_owned_opt())
    };
    assert!(({ (*{ (*h.borrow()).data.clone() }.as_ref().unwrap().borrow()).v } == 3));
    ({ HolderImpl::set(&h.as_pointer(), Ptr::alloc(Data { v: 4 })) });
    assert!(
        ({
            (*(*hp.borrow())
                .with(|__s| __s.data.clone())
                .as_ref()
                .unwrap()
                .borrow())
            .v
        } == 4)
    );
    let __rhs = (*hp.borrow()).with(|__s| __s.n);
    (*(*hp.borrow())
        .with(|__s| __s.data.clone())
        .as_ref()
        .unwrap()
        .borrow_mut())
    .v += __rhs;
    assert!(({ (*{ (*h.borrow()).data.clone() }.as_ref().unwrap().borrow()).v } == 5));
    return 0;
}
pub trait HolderImpl {
    fn set(&self, p: Ptr<Data>);
    fn move_assign(&self, _a0: Ptr<Holder>) -> Ptr<Holder>;
}
impl HolderImpl for Ptr<Holder> {
    fn set(&self, p: Ptr<Data>) {
        let p: Value<Ptr<Data>> = Rc::new(RefCell::new(p));
        {
            let _p: Ptr<_> = (*p.borrow()).clone();
            (field_ptr!((*self), data) as Ptr<Option<Value<Data>>>).write(_p.to_owned_opt())
        };
    }
    fn move_assign(&self, _a0: Ptr<Holder>) -> Ptr<Holder> {
        (field_ptr!((*self), data) as Ptr<Option<Value<Data>>>)
            .write(field!(_a0, data).with_mut(|__v: &mut Option<Value<Data>>| __v.take()));
        let __rhs = { (*_a0.upgrade().deref()).n };
        field!((*self), n).write(__rhs);
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
