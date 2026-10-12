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
#[derive(DeepClone, Record, ByteRepr, VaArg, FnPtrArg)]
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
impl Default for S {
    fn default() -> Self {
        {
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
}
pub fn run_0(mut o: Ptr<S>) {
    ({ CounterImpl::add(&field_ptr!(o, c), 2) });
    assert!((({ CounterImpl::get(&field_ptr!(o, c),) }) == 2));
    ({
        CounterImpl::add(
            &(array_field_ptr!(o, arr) as Ptr<Counter>).offset((1) as isize),
            5,
        )
    });
    assert!(
        (({ CounterImpl::get(&(array_field_ptr!(o, arr) as Ptr<Counter>).offset((1) as isize),) })
            == 5)
    );
    ({ CounterImpl::add(&({ CounterImpl::self_(&field_ptr!(o, c)) }), 1) });
    assert!((({ CounterImpl::get(&field_ptr!(o, c),) }) == 3));
    assert!(({ ({ CounterImpl::self_(&field_ptr!(o, c),) }) } == { (field_ptr!(o, c)) }));
    ({
        let _other: Ptr<Counter> =
            ((array_field_ptr!(o, arr) as Ptr<Counter>).offset((1) as isize));
        CounterImpl::take(
            &(array_field_ptr!(o, arr) as Ptr<Counter>).offset((0) as isize),
            _other,
        )
    });
    assert!(
        (({ CounterImpl::get(&(array_field_ptr!(o, arr) as Ptr<Counter>).offset((0) as isize),) })
            == 5)
            && (({
                CounterImpl::get(&(array_field_ptr!(o, arr) as Ptr<Counter>).offset((1) as isize))
            }) == 0)
    );
    ({
        let _other: Ptr<Counter> = (field_ptr!(o, c));
        CounterImpl::take(&field_ptr!(o, c), _other)
    });
    assert!((({ CounterImpl::get(&field_ptr!(o, c),) }) == 0));
    ({ SImpl::bump(&o) });
    assert!((({ CounterImpl::get(&field_ptr!(o, c),) }) == 1));
    {
        let __a1 = ({ CounterImpl::get(&field_ptr!(o, c)) });
        (*o.with(|__s| __s.v.clone()).borrow_mut()).push(__a1)
    };
    assert!(
        ((*o.with(|__s| __s.v.clone()).borrow()).len() == 1_usize)
            && ((elem!((o.with(|__s| __s.v.as_pointer()) as Ptr<i32>), 0_usize).read()) == 1)
    );
    assert!((o.with(|__s| __s.tag) == 1));
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let local: Value<S> = Rc::new(RefCell::new(<S>::default()));
    (*local.borrow_mut()).tag = 1;
    ({ run_0((local.as_pointer())) });
    let mut heap: Ptr<S> = Ptr::alloc(<S>::default());
    field!(heap, tag).write(1);
    ({ run_0((heap).clone()) });
    heap.delete();
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
    fn add(&self, mut k: i32) {
        {
            let __rhs = k;
            field!((*self), n).with_mut(|__v| *__v = *__v + __rhs)
        };
    }
    fn self_(&self) -> Ptr<Counter> {
        return (*self).clone();
    }
    fn take(&self, mut other: Ptr<Counter>) {
        {
            let __rhs = { other.with(|__s| __s.n) };
            field!((*self), n).with_mut(|__v| *__v = *__v + __rhs)
        };
        field!(other, n).write(0);
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
