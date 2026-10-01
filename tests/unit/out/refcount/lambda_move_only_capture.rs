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
    pub fn new(v: i32) -> Self {
        let v: Value<i32> = Rc::new(RefCell::new(v));
        let __this: Value<Owner> = Rc::new(RefCell::new(Self {
            p: Ptr::alloc((*v.borrow())),
        }));
        let this: Ptr<Owner> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn move_from(o: Ptr<Owner>) -> Self {
        let __this: Value<Owner> = Rc::new(RefCell::new(Self {
            p: o.with(|__s| __s.p.clone()),
        }));
        let this: Ptr<Owner> = __this.as_pointer();
        field!(o, p).write(Ptr::<i32>::null());
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let o: Value<Owner> = Rc::new(RefCell::new(Owner::new({ 5 })));
    let _dtor_o = ScopedDestructor::new(&o, |__p| __p.destructor());
    let f: Value<FnPtr<fn() -> i32>> = Rc::new(RefCell::new({
        #[derive(Record, ByteRepr)]
        #[byte_size(8)]
        struct Captures {
            #[offset(0)]
            #[byte_size(8)]
            h: Owner,
        }
        FnPtr::<fn() -> i32>::with_captures(
            Captures {
                h: Owner::move_from({ o.as_pointer() }),
            },
            (|this: Ptr<Captures>| {
                return ({ (*this.upgrade().deref()).h.p.clone() }.read());
            }),
        )
    }));
    assert!(({ (*o.borrow()).p.clone() }).is_null());
    assert!((({ (*f.borrow()).call() }) == 5));
    let g: Value<FnPtr<fn() -> i32>> = Rc::new(RefCell::new((*f.borrow_mut()).clone()));
    assert!((({ (*g.borrow()).call() }) == 5));
    let total: Value<i32> = Rc::new(RefCell::new(0));
    let consume: Value<FnPtr<fn()>> = Rc::new(RefCell::new({
        #[derive(Record, ByteRepr)]
        #[byte_size(16)]
        struct Captures {
            #[offset(0)]
            #[byte_size(8)]
            h: Owner,
            #[offset(8)]
            #[byte_size(8)]
            total: Ptr<i32>,
        }
        FnPtr::<fn()>::with_captures(
            Captures {
                h: Owner::new({ 7 }),
                total: total.as_pointer(),
            },
            (|this: Ptr<Captures>| {
                {
                    let _ptr = this.with(|__s| __s.total.clone()).clone();
                    _ptr.write(_ptr.read() + { ({ (*this.upgrade().deref()).h.p.clone() }.read()) })
                };
                { (*this.upgrade().deref()).h.p.clone() }.write(0);
            }),
        )
    }));
    ({ (*consume.borrow()).call() });
    ({ (*consume.borrow()).call() });
    assert!(((*total.borrow()) == 7));
    let o2: Value<Owner> = Rc::new(RefCell::new(Owner::new({ 9 })));
    let _dtor_o2 = ScopedDestructor::new(&o2, |__p| __p.destructor());
    let t: Value<FnPtr<fn() -> i32>> =
        Rc::new(RefCell::new(({ OwnerImpl::take(&o2.as_pointer()) })));
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
        return {
            #[derive(Record, ByteRepr)]
            #[byte_size(8)]
            struct Captures {
                #[offset(0)]
                #[byte_size(8)]
                self_: Owner,
            }
            FnPtr::<fn() -> i32>::with_captures(
                Captures {
                    self_: Owner::move_from({ (*self).clone() }),
                },
                (|this: Ptr<Captures>| {
                    return ({ (*this.upgrade().deref()).self_.p.clone() }.read());
                }),
            )
        };
    }
}
pub fn __cpp2rust_init_globals() {}
