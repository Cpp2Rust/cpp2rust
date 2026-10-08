extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Record, ByteRepr, Default)]
#[byte_size(8)]
pub struct Owner {
    #[offset(0)]
    #[byte_size(8)]
    pub p: Ptr<i32>,
}
impl Owner {
    pub fn new(mut v: i32) -> Self {
        Self { p: Ptr::alloc(v) }
    }
    pub fn move_from(o: Ptr<Owner>) -> Self {
        let __this: Owner = Self {
            p: o.with(|__s| __s.p.clone()),
        };
        field!(o, p).write(Ptr::<i32>::null());
        __this
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let o: Value<Owner> = Rc::new(RefCell::new(Owner::new({ 5 })));
    let _dtor_o = ScopedDestructor::new(&o, |__p| __p.destructor());
    let f: Value<FnPtr<fn() -> i32>> = Rc::new(RefCell::new(lambda!(
        {
            let h: Value<Owner> = Rc::new(RefCell::new(Owner::move_from({ o.as_pointer() })));
        },
        || -> i32 {
            return ({ (*h.borrow()).p.clone() }.read());
        },
        move_from {
            h: Rc::new(RefCell::new(Owner::move_from({ h.as_pointer() },))),
        },
        destroy = {
            self.h.as_pointer().destructor();
        }
    )));
    let _dtor_f = ScopedDestructor::new(&f, |__p| __p.with(|__f| __f.destroy()));
    assert!(({ (*o.borrow()).p.clone() }).is_null());
    assert!((({ (*f.borrow()).call() }) == 5));
    let g: Value<FnPtr<fn() -> i32>> = Rc::new(RefCell::new((*f.borrow()).move_from()));
    let _dtor_g = ScopedDestructor::new(&g, |__p| __p.with(|__f| __f.destroy()));
    assert!((({ (*g.borrow()).call() }) == 5));
    let total: Value<i32> = Rc::new(RefCell::new(0));
    let consume: Value<FnPtr<fn()>> = Rc::new(RefCell::new(lambda!(
        {
            let h: Value<Owner> = Rc::new(RefCell::new(Owner::new({ 7 })));
            let total: Ptr<i32> = total.as_pointer();
        },
        || {
            {
                let __rhs = { ({ (*h.borrow()).p.clone() }.read()) };
                total.with_mut(|__v| *__v = *__v + __rhs)
            };
            { (*h.borrow()).p.clone() }.write(0);
        },
        move_from {
            h: Rc::new(RefCell::new(Owner::move_from({ h.as_pointer() },))),
            total: (total).clone(),
        },
        destroy = {
            self.h.as_pointer().destructor();
        }
    )));
    let _dtor_consume = ScopedDestructor::new(&consume, |__p| __p.with(|__f| __f.destroy()));
    ({ (*consume.borrow()).call() });
    ({ (*consume.borrow()).call() });
    assert!(((*total.borrow()) == 7));
    let o2: Value<Owner> = Rc::new(RefCell::new(Owner::new({ 9 })));
    let _dtor_o2 = ScopedDestructor::new(&o2, |__p| __p.destructor());
    let t: Value<FnPtr<fn() -> i32>> =
        Rc::new(RefCell::new(({ OwnerImpl::take(&o2.as_pointer()) })));
    let _dtor_t = ScopedDestructor::new(&t, |__p| __p.with(|__f| __f.destroy()));
    assert!(({ (*o2.borrow()).p.clone() }).is_null());
    assert!((({ (*t.borrow()).call() }) == 9));
    return 0;
}
pub trait OwnerImpl {
    fn destructor(&self);
    fn take(&self) -> FnPtr<fn() -> i32>;
}
impl OwnerImpl for Ptr<Owner> {
    fn destructor(&self) {
        (*self).with(|__s| __s.p.clone()).delete();
    }
    fn take(&self) -> FnPtr<fn() -> i32> {
        return lambda!(
            {
                let self_: Value<Owner> =
                    Rc::new(RefCell::new(Owner::move_from({ (*self).clone() })));
            },
            || -> i32 {
                return ({ (*self_.borrow()).p.clone() }.read());
            },
            move_from {
                self_: Rc::new(RefCell::new(Owner::move_from({ self_.as_pointer() },))),
            },
            destroy = {
                self.self_.as_pointer().destructor();
            }
        );
    }
}
pub fn __cpp2rust_init_globals() {}
