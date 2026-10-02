extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Counter {
    #[offset(0)]
    pub n: i32,
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(40)]
pub struct S {
    #[offset(0)]
    pub tag: i32,
    #[offset(4)]
    #[byte_size(4)]
    pub c: Counter,
    #[offset(8)]
    #[byte_size(8)]
    pub arr: Value<Box<[Counter]>>,
    #[offset(16)]
    #[byte_size(24)]
    pub v: Value<Vec<i32>>,
}
impl Clone for S {
    fn clone(&self) -> Self {
        Self {
            tag: self.tag.clone(),
            c: self.c.clone(),
            arr: Rc::new(RefCell::new((*self.arr.borrow()).clone())),
            v: Rc::new(RefCell::new((*self.v.borrow()).clone())),
        }
    }
}
impl Default for S {
    fn default() -> Self {
        S {
            tag: 0_i32,
            c: <Counter>::default(),
            arr: Rc::new(RefCell::new(
                (0..2)
                    .map(|_| <Counter>::default())
                    .collect::<Box<[Counter]>>(),
            )),
            v: Rc::new(RefCell::new(Default::default())),
        }
    }
}
pub fn run_0(o: Ptr<S>) {
    let o: Value<Ptr<S>> = Rc::new(RefCell::new(o));
    ({ CounterImpl::add(&field_ptr!((*o.borrow()), c), 2) });
    assert!((({ CounterImpl::get(&field_ptr!((*o.borrow()), c),) }) == 2));
    ({
        CounterImpl::add(
            &(array_field_ptr!((*o.borrow()), arr) as Ptr<Counter>).offset((1) as isize),
            5,
        )
    });
    assert!(
        (({
            CounterImpl::get(
                &(array_field_ptr!((*o.borrow()), arr) as Ptr<Counter>).offset((1) as isize),
            )
        }) == 5)
    );
    ({ CounterImpl::add(&({ CounterImpl::self_(&field_ptr!((*o.borrow()), c)) }), 1) });
    assert!((({ CounterImpl::get(&field_ptr!((*o.borrow()), c),) }) == 3));
    assert!(
        ({ ({ CounterImpl::self_(&field_ptr!((*o.borrow()), c),) }) } == {
            (field_ptr!((*o.borrow()), c))
        })
    );
    ({
        let _other: Ptr<Counter> =
            ((array_field_ptr!((*o.borrow()), arr) as Ptr<Counter>).offset((1) as isize));
        CounterImpl::take(
            &(array_field_ptr!((*o.borrow()), arr) as Ptr<Counter>).offset((0) as isize),
            _other,
        )
    });
    assert!(
        (({
            CounterImpl::get(
                &(array_field_ptr!((*o.borrow()), arr) as Ptr<Counter>).offset((0) as isize),
            )
        }) == 5)
            && (({
                CounterImpl::get(
                    &(array_field_ptr!((*o.borrow()), arr) as Ptr<Counter>).offset((1) as isize),
                )
            }) == 0)
    );
    ({
        let _other: Ptr<Counter> = (field_ptr!((*o.borrow()), c));
        CounterImpl::take(&field_ptr!((*o.borrow()), c), _other)
    });
    assert!((({ CounterImpl::get(&field_ptr!((*o.borrow()), c),) }) == 0));
    ({ SImpl::bump(&(*o.borrow())) });
    assert!((({ CounterImpl::get(&field_ptr!((*o.borrow()), c),) }) == 1));
    {
        let __a1 = ({ CounterImpl::get(&field_ptr!((*o.borrow()), c)) });
        (*(*o.borrow()).with(|__s| __s.v.clone()).borrow_mut()).push(__a1)
    };
    assert!(
        ((*(*o.borrow()).with(|__s| __s.v.clone()).borrow()).len() == 1_usize)
            && ((elem!(
                ((*o.borrow()).with(|__s| __s.v.as_pointer()) as Ptr<i32>),
                0_usize
            )
            .read())
                == 1)
    );
    assert!(((*o.borrow()).with(|__s| __s.tag) == 1));
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let local: Value<S> = Rc::new(RefCell::new(<S>::default()));
    (*local.borrow_mut()).tag = 1;
    ({ run_0((local.as_pointer())) });
    let heap: Value<Ptr<S>> = Rc::new(RefCell::new(Ptr::alloc(<S>::default())));
    field!((*heap.borrow()), tag).write(1);
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
        return (*self).with(|__s| __s.n);
    }
    fn add(&self, k: i32) {
        let k: Value<i32> = Rc::new(RefCell::new(k));
        {
            let _ptr = field!((*self), n);
            _ptr.write(_ptr.read() + (*k.borrow()))
        };
    }
    fn self_(&self) -> Ptr<Counter> {
        return (*self).clone();
    }
    fn take(&self, other: Ptr<Counter>) {
        let other: Value<Ptr<Counter>> = Rc::new(RefCell::new(other));
        {
            let _ptr = field!((*self), n);
            _ptr.write(_ptr.read() + { (*other.borrow()).with(|__s| __s.n) })
        };
        field!((*other.borrow()), n).write(0);
    }
}
pub trait SImpl {
    fn bump(&self);
}
impl SImpl for Ptr<S> {
    fn bump(&self) {
        ({
            let _k: i32 = (*self).with(|__s| __s.tag);
            CounterImpl::add(&field_ptr!((*self), c), _k)
        });
    }
}
pub fn __cpp2rust_init_globals() {}
