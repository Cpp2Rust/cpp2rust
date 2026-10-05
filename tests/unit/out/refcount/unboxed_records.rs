extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Inner {
    #[offset(0)]
    pub a: i32,
    #[offset(4)]
    pub b: i32,
}
#[derive(DeepClone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(48)]
pub struct Cookie {
    #[offset(0)]
    pub x: i32,
    #[offset(8)]
    #[byte_size(8)]
    pub data: Ptr<i32>,
    #[offset(16)]
    #[byte_size(8)]
    pub in_: Ptr<i32>,
    #[offset(24)]
    #[byte_size(8)]
    pub inner: Inner,
    #[offset(32)]
    #[byte_size(16)]
    pub arr: Value<Box<[i32]>>,
}
impl Default for Cookie {
    fn default() -> Self {
        Cookie {
            x: 0_i32,
            data: Ptr::<i32>::null(),
            in_: Ptr::<i32>::null(),
            inner: <Inner>::default(),
            arr: Rc::new(RefCell::new((0..4).map(|_| 0_i32).collect::<Box<[i32]>>())),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct Counter {
    #[offset(0)]
    pub n: i32,
}
impl Default for Counter {
    fn default() -> Self {
        Counter { n: 0 }
    }
}
pub fn set_0(mut p: Ptr<i32>, mut v: i32) {
    p.write({ v });
}
pub fn get_1(c: Ptr<Cookie>) -> i32 {
    return c.with(|__s| __s.x);
}
pub fn first_2(mut p: Ptr<i32>) -> i32 {
    return (elem!(p, 0).read());
}
pub fn consume_3(cookie: Ptr<Cookie>) -> i32 {
    let mut c: Cookie = (*cookie.upgrade().deref()).clone();
    let mut sum: i32 = 0;
    let mut i: i32 = 0;
    'loop_: while (i < 4) {
        elem!(c.data, i).write({ ({ (elem!(c.in_, i).read()) } * { c.x }) });
        sum += { (elem!(c.data, i).read()) };
        i.prefix_inc();
    }
    c.x = sum;
    c.inner.a += { c.x };
    elem!((c.arr.as_pointer() as Ptr::<i32>), 1).write({ c.inner.a });
    let __rhs = ({ (elem!((c.arr.as_pointer() as Ptr::<i32>), 1).read()) } + {
        ({ first_2((c.in_).clone()) })
    });
    c.data.write(__rhs);
    ({
        let _p: Ptr<i32> = c.data.offset((1) as isize);
        let _v: i32 = ({ first_2((c.data).clone()) });
        set_0(_p, _v)
    });
    return (c.x + c.inner.b);
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let data: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0, 0, 0])));
    let in_: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4])));
    let cookie: Value<Cookie> = Rc::new(RefCell::new(<Cookie>::default()));
    (*cookie.borrow_mut()).x = 2;
    (*cookie.borrow_mut()).data = (data.as_pointer() as Ptr<i32>);
    (*cookie.borrow_mut()).in_ = (in_.as_pointer() as Ptr<i32>);
    (*cookie.borrow_mut()).inner.a = 1;
    (*cookie.borrow_mut()).inner.b = 3;
    assert!((({ consume_3(cookie.as_pointer(),) }) == 23));
    assert!(
        (((*data.borrow())[(0) as usize] == 22) && ((*data.borrow())[(1) as usize] == 22))
            && ((*data.borrow())[(3) as usize] == 8)
    );
    assert!((({ get_1(cookie.as_pointer(),) }) == 2));
    let mut local: Inner = <Inner>::default();
    local.a = 4;
    local.b = { (local.a * 2) };
    local.a.postfix_inc();
    assert!(((local.a + local.b) == 13));
    let field_addr: Value<Inner> = Rc::new(RefCell::new(Inner { a: 0, b: 0 }));
    ({ set_0((field_ptr!(field_addr.as_pointer(), b)), 7) });
    assert!(({ (*field_addr.borrow()).b } == 7));
    let counter: Value<Counter> = Rc::new(RefCell::new(<Counter>::default()));
    ({ CounterImpl::inc(&counter.as_pointer()) });
    assert!(({ (*counter.borrow()).n } == 1));
    return 0;
}
pub trait CounterImpl {
    fn inc(&self);
}
impl CounterImpl for Ptr<Counter> {
    fn inc(&self) {
        field!((*self), n).with_mut(|__v| __v.prefix_inc());
    }
}
pub fn __cpp2rust_init_globals() {}
