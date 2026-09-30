extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(VaArg, FnPtrArg, Default)]
pub struct Counter {
    pub n: Value<i32>,
}
impl Clone for Counter {
    fn clone(&self) -> Self {
        let __this: Value<Counter> = Rc::new(RefCell::new(Self {
            n: Rc::new(RefCell::new((*self.n.borrow()))),
        }));
        let this: Ptr<Counter> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for Counter {
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
#[derive(VaArg, FnPtrArg)]
pub struct S {
    pub tag: Value<i32>,
    pub c: Value<Counter>,
    pub arr: Value<Box<[Counter]>>,
    pub v: Value<Vec<i32>>,
}
impl Clone for S {
    fn clone(&self) -> Self {
        let __this: Value<S> = Rc::new(RefCell::new(Self {
            tag: Rc::new(RefCell::new((*self.tag.borrow()))),
            c: Rc::new(RefCell::new((*self.c.borrow()).clone())),
            arr: Rc::new(RefCell::new(Box::new(std::array::from_fn::<_, 2, _>(
                |__i: usize| ((*self.arr.borrow())[(__i) as usize]).clone(),
            )))),
            v: Rc::new(RefCell::new((*self.v.borrow()).clone())),
        }));
        let this: Ptr<S> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for S {
    fn default() -> Self {
        S {
            tag: Rc::new(RefCell::new(0_i32)),
            c: <Value<Counter>>::default(),
            arr: Rc::new(RefCell::new(
                (0..2)
                    .map(|_| <Counter>::default())
                    .collect::<Box<[Counter]>>(),
            )),
            v: Rc::new(RefCell::new(Default::default())),
        }
    }
}
impl ByteRepr for S {
    fn byte_size() -> usize {
        40
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.tag.borrow()).to_bytes(&mut buf[0..4]);
        (*self.c.borrow()).to_bytes(&mut buf[4..8]);
        (*self.arr.borrow()).to_bytes(&mut buf[8..16]);
        (*self.v.borrow()).to_bytes(&mut buf[16..40]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            tag: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
            c: Rc::new(RefCell::new(<Counter>::from_bytes(&buf[4..8]))),
            arr: Rc::new(RefCell::new(<Box<[Counter]>>::from_bytes(&buf[8..16]))),
            v: Rc::new(RefCell::new(<Vec<i32>>::from_bytes(&buf[16..40]))),
        }
    }
}
pub fn run_0(o: Ptr<S>) {
    let o: Value<Ptr<S>> = Rc::new(RefCell::new(o));
    ({ CounterImpl::add(&(*(*o.borrow()).upgrade().deref()).c.as_pointer(), 2) });
    assert!((({ CounterImpl::get(&(*(*o.borrow()).upgrade().deref()).c.as_pointer(),) }) == 2));
    ({
        CounterImpl::add(
            &((*(*o.borrow()).upgrade().deref()).arr.as_pointer() as Ptr<Counter>).offset(1),
            5,
        )
    });
    assert!(
        (({
            CounterImpl::get(
                &((*(*o.borrow()).upgrade().deref()).arr.as_pointer() as Ptr<Counter>).offset(1),
            )
        }) == 5)
    );
    ({
        CounterImpl::add(
            &({ CounterImpl::self_(&(*(*o.borrow()).upgrade().deref()).c.as_pointer()) }),
            1,
        )
    });
    assert!((({ CounterImpl::get(&(*(*o.borrow()).upgrade().deref()).c.as_pointer(),) }) == 3));
    assert!({
        let _lhs = ({ CounterImpl::self_(&(*(*o.borrow()).upgrade().deref()).c.as_pointer()) });
        _lhs == ((*(*o.borrow()).upgrade().deref()).c.as_pointer())
    });
    ({
        let _other: Ptr<Counter> =
            (((*(*o.borrow()).upgrade().deref()).arr.as_pointer() as Ptr<Counter>).offset(1));
        CounterImpl::take(
            &((*(*o.borrow()).upgrade().deref()).arr.as_pointer() as Ptr<Counter>).offset(0),
            _other,
        )
    });
    assert!(
        (({
            CounterImpl::get(
                &((*(*o.borrow()).upgrade().deref()).arr.as_pointer() as Ptr<Counter>).offset(0),
            )
        }) == 5)
            && (({
                CounterImpl::get(
                    &((*(*o.borrow()).upgrade().deref()).arr.as_pointer() as Ptr<Counter>)
                        .offset(1),
                )
            }) == 0)
    );
    ({
        let _other: Ptr<Counter> = ((*(*o.borrow()).upgrade().deref()).c.as_pointer());
        CounterImpl::take(&(*(*o.borrow()).upgrade().deref()).c.as_pointer(), _other)
    });
    assert!((({ CounterImpl::get(&(*(*o.borrow()).upgrade().deref()).c.as_pointer(),) }) == 0));
    ({ SImpl::bump(&(*o.borrow())) });
    assert!((({ CounterImpl::get(&(*(*o.borrow()).upgrade().deref()).c.as_pointer(),) }) == 1));
    (*(*(*o.borrow()).upgrade().deref()).v.borrow_mut())
        .push(({ CounterImpl::get(&(*(*o.borrow()).upgrade().deref()).c.as_pointer()) }));
    assert!(
        ((*(*(*o.borrow()).upgrade().deref()).v.borrow()).len() == 1_usize)
            && ((((*(*o.borrow()).upgrade().deref()).v.as_pointer() as Ptr<i32>)
                .offset(0_usize)
                .read())
                == 1)
    );
    assert!(((*(*(*o.borrow()).upgrade().deref()).tag.borrow()) == 1));
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let local: Value<S> = Rc::new(RefCell::new(<S>::default()));
    (*(*local.borrow()).tag.borrow_mut()) = 1;
    ({ run_0((local.as_pointer())) });
    let heap: Value<Ptr<S>> = Rc::new(RefCell::new(Ptr::alloc(<S>::default())));
    (*(*(*heap.borrow()).upgrade().deref()).tag.borrow_mut()) = 1;
    ({ run_0((*heap.borrow()).clone()) });
    (*heap.borrow()).delete();
    return 0;
}
pub trait CounterImpl {
    fn get(&self) -> i32;
    fn add(&self, k: i32);
    fn self_(&self) -> Ptr<Counter>;
    fn take(&self, other: Ptr<Counter>);
}
impl CounterImpl for Ptr<Counter> {
    fn get(&self) -> i32 {
        return (*(*(*self).upgrade().deref()).n.borrow());
    }
    fn add(&self, k: i32) {
        let k: Value<i32> = Rc::new(RefCell::new(k));
        (*(*(*self).upgrade().deref()).n.borrow_mut()) += (*k.borrow());
    }
    fn self_(&self) -> Ptr<Counter> {
        return (*self).clone();
    }
    fn take(&self, other: Ptr<Counter>) {
        let other: Value<Ptr<Counter>> = Rc::new(RefCell::new(other));
        let __rhs = (*(*(*other.borrow()).upgrade().deref()).n.borrow());
        (*(*(*self).upgrade().deref()).n.borrow_mut()) += __rhs;
        (*(*(*other.borrow()).upgrade().deref()).n.borrow_mut()) = 0;
    }
}
pub trait SImpl {
    fn bump(&self);
}
impl SImpl for Ptr<S> {
    fn bump(&self) {
        ({
            let _k: i32 = (*(*(*self).upgrade().deref()).tag.borrow());
            CounterImpl::add(&(*(*self).upgrade().deref()).c.as_pointer(), _k)
        });
    }
}
pub fn __cpp2rust_init_globals() {}
