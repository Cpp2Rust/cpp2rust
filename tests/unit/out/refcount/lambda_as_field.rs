extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct Guard__lambda_at_lambda_as_field_cpp______ {
    pub f: Value<_>,
}
impl Clone for Guard__lambda_at_lambda_as_field_cpp______ {
    fn clone(&self) -> Self {
        let __this: Value<Guard__lambda_at_lambda_as_field_cpp______> =
            Rc::new(RefCell::new(Self {
                f: Rc::new(RefCell::new((*self.f.borrow()).clone())),
            }));
        let this: Ptr<Guard__lambda_at_lambda_as_field_cpp______> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for Guard__lambda_at_lambda_as_field_cpp______ {}
#[derive(Default)]
pub struct Holder__lambda_at_lambda_as_field_cpp______ {
    pub f: Value<_>,
    pub calls: Value<i32>,
}
impl Clone for Holder__lambda_at_lambda_as_field_cpp______ {
    fn clone(&self) -> Self {
        let __this: Value<Holder__lambda_at_lambda_as_field_cpp______> =
            Rc::new(RefCell::new(Self {
                f: Rc::new(RefCell::new((*self.f.borrow()).clone())),
                calls: Rc::new(RefCell::new((*self.calls.borrow()))),
            }));
        let this: Ptr<Holder__lambda_at_lambda_as_field_cpp______> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for Holder__lambda_at_lambda_as_field_cpp______ {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.f.borrow()).to_bytes(&mut buf[0..4]);
        (*self.calls.borrow()).to_bytes(&mut buf[4..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            f: Rc::new(RefCell::new(<_>::from_bytes(&buf[0..4]))),
            calls: Rc::new(RefCell::new(<i32>::from_bytes(&buf[4..8]))),
        }
    }
}
pub fn wrap_0(fn_: impl Fn(i32) -> i32) -> _ {
    let fn_: Value<_> = Rc::new(RefCell::new(fn_));
    return (|x: i32| {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        return (({ (*fn_.borrow_mut())((*x.borrow())) }) + 1);
    });
}
pub fn wrap_1(fn_: impl Fn(i32) -> i32) -> _ {
    let fn_: Value<_> = Rc::new(RefCell::new(fn_));
    return (|x: i32| {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        return (({ (*fn_.borrow_mut())((*x.borrow())) }) + 1);
    });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let cleaned: Value<i32> = Rc::new(RefCell::new(0));
    {
        let g: Value<Guard__lambda_at_lambda_as_field_cpp______> =
            Rc::new(RefCell::new(Guard__lambda_at_lambda_as_field_cpp______ {
                f: Rc::new(RefCell::new(
                    (|| {
                        (*cleaned.borrow_mut()).postfix_inc();
                    }),
                )),
            }));
        let _dtor_g = ScopedDestructor::new(&g, |__p| __p.destructor());
        assert!(((*cleaned.borrow()) == 0));
    }
    assert!(((*cleaned.borrow()) == 1));
    let factor: Value<i32> = Rc::new(RefCell::new(3));
    let h: Value<Holder__lambda_at_lambda_as_field_cpp______> =
        Rc::new(RefCell::new(Holder__lambda_at_lambda_as_field_cpp______ {
            f: Rc::new(RefCell::new(
                (|x: i32| {
                    let x: Value<i32> = Rc::new(RefCell::new(x));
                    return ((*x.borrow()) * (*factor.borrow()));
                }),
            )),
            calls: Rc::new(RefCell::new(0)),
        }));
    (*factor.borrow_mut()) = 100;
    assert!(
        (({ Holder__lambda_at_lambda_as_field_cpp______Impl::call(&h.as_pointer(), 2,) }) == 6)
    );
    assert!(
        (({ Holder__lambda_at_lambda_as_field_cpp______Impl::call(&h.as_pointer(), 5,) }) == 15)
    );
    assert!(((*(*h.borrow()).calls.borrow()) == 2));
    let w: Value<_> = Rc::new(RefCell::new(
        ({
            wrap_0(
                (|x: i32| {
                    let x: Value<i32> = Rc::new(RefCell::new(x));
                    return ((*x.borrow()) * (*factor.borrow()));
                }),
            )
        }),
    ));
    (*factor.borrow_mut()) = 7;
    assert!((({ (*w.borrow_mut())(2,) }) == 201));
    let ww: Value<_> = Rc::new(RefCell::new(({ wrap_1((*w.borrow()).clone()) })));
    assert!((({ (*ww.borrow_mut())(2,) }) == 202));
    return 0;
}
pub trait Guard__lambda_at_lambda_as_field_cpp______Impl {
    fn destructor(&self);
}
impl Guard__lambda_at_lambda_as_field_cpp______Impl
    for Ptr<Guard__lambda_at_lambda_as_field_cpp______>
{
    fn destructor(&self) {
        ({ (*(*(*self).upgrade().deref()).f.borrow_mut())() });
    }
}
pub trait Holder__lambda_at_lambda_as_field_cpp______Impl {
    fn call(&self, x: i32) -> i32;
}
impl Holder__lambda_at_lambda_as_field_cpp______Impl
    for Ptr<Holder__lambda_at_lambda_as_field_cpp______>
{
    fn call(&self, x: i32) -> i32 {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        (*(*(*self).upgrade().deref()).calls.borrow_mut()).postfix_inc();
        return ({ (*(*(*self).upgrade().deref()).f.borrow_mut())((*x.borrow())) });
    }
}
pub fn __cpp2rust_init_globals() {}
