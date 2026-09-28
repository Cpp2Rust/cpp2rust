extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn inc_0(p: Ptr<i32>) {
    let p: Value<Ptr<i32>> = Rc::new(RefCell::new(p));
    {
        let _ptr = (*p.borrow()).clone();
        _ptr.write(_ptr.read() + 1)
    };
}
pub fn add_1(p: Ptr<i32>, n: i32) {
    let p: Value<Ptr<i32>> = Rc::new(RefCell::new(p));
    let n: Value<i32> = Rc::new(RefCell::new(n));
    let __rhs = (*n.borrow());
    {
        let _ptr = (*p.borrow()).clone();
        _ptr.write(_ptr.read() + __rhs)
    };
}
pub fn twice_2(n: i32) -> i32 {
    let n: Value<i32> = Rc::new(RefCell::new(n));
    return ((*n.borrow()) * 2);
}
#[derive(Clone, ByteRepr, Default)]
pub struct Access_S_ {}
#[derive(Default)]
pub struct S {
    pub base: Value<i32>,
}
impl Clone for S {
    fn clone(&self) -> Self {
        let __this: Value<S> = Rc::new(RefCell::new(Self {
            base: Rc::new(RefCell::new((*self.base.borrow()))),
        }));
        let this: Ptr<S> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for S {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.base.borrow()).to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            base: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
        }
    }
}
#[derive(Default)]
pub struct Box {
    pub v: Value<i32>,
}
impl Clone for Box {
    fn clone(&self) -> Self {
        let __this: Value<Box> = Rc::new(RefCell::new(Self {
            v: Rc::new(RefCell::new((*self.v.borrow()))),
        }));
        let this: Ptr<Box> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for Box {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.v.borrow()).to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            v: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let s: Value<S> = Rc::new(RefCell::new(S {
        base: Rc::new(RefCell::new(100)),
    }));
    assert!((({ SImpl::width_1_char(&s.as_pointer(), 3,) }) == 103));
    assert!((({ SImpl::width_1_int(&s.as_pointer(), 3,) }) == 112));
    assert!((({ SImpl::scale_2_2(&s.as_pointer(), 5,) }) == 110));
    assert!((({ SImpl::scale_2_3(&s.as_pointer(), 5,) }) == 115));
    assert!((({ SImpl::count_3(&s.as_pointer(), 1,) }) == 101));
    assert!((({ SImpl::count_3_int_long(&s.as_pointer(), 1,) }) == 103));
    assert!((({ SImpl::plain_4(&s.as_pointer(), 1,) }) == 101));
    assert!((({ SImpl::plain_5(&s.as_pointer(), 1_i64,) }) == 102));
    let y: Value<i32> = Rc::new(RefCell::new(1));
    assert!((({ SImpl::take_6(&s.as_pointer(), y.as_pointer(),) }) == 102));
    assert!(
        (({
            let _x: Value<i32> = Rc::new(RefCell::new(5));
            SImpl::take_7(&s.as_pointer(), _x.as_pointer())
        }) == 107)
    );
    assert!(
        (({
            SImpl::pick_8(
                &s.as_pointer(),
                (
                    Rc::new(RefCell::new(1.try_into().expect("failed conversion"))),
                    Rc::new(RefCell::new(2.try_into().expect("failed conversion"))),
                ),
            )
        }) == 101)
    );
    assert!(
        (({
            SImpl::pick_9(
                &s.as_pointer(),
                (
                    Rc::new(RefCell::new(1.try_into().expect("failed conversion"))),
                    Rc::new(RefCell::new(2_i64.try_into().expect("failed conversion"))),
                ),
            )
        }) == 102)
    );
    assert!(
        (({ SImpl::apply_10(&s.as_pointer(), FnPtr::<fn(Ptr::<i32>)>::new(inc_0), 1,) }) == 102)
    );
    assert!(
        (({ SImpl::apply_11(&s.as_pointer(), FnPtr::<fn(Ptr::<i32>, i32)>::new(add_1), 1,) })
            == 111)
    );
    assert!(
        (({ SImpl::apply_12(&s.as_pointer(), FnPtr::<fn(i32) -> i32>::new(twice_2), 3,) }) == 106)
    );
    let c: Value<i32> = Rc::new(RefCell::new(3));
    assert!(
        (({
            let _p: Value<(Value<i32>, Value<i64>)> = Rc::new(RefCell::new((
                Rc::new(RefCell::new(1.try_into().expect("failed conversion"))),
                Rc::new(RefCell::new(2_i64.try_into().expect("failed conversion"))),
            )));
            SImpl::combine_13(
                &s.as_pointer(),
                _p.as_pointer(),
                FnPtr::<fn(i32) -> i32>::new(twice_2),
                (c.as_pointer()),
                4_u64,
            )
        }) == 112)
    );
    let z: Value<i32> = Rc::new(RefCell::new(1));
    assert!(
        (({
            let _p: Value<(Value<i32>, Value<i32>)> = Rc::new(RefCell::new((
                Rc::new(RefCell::new(1.try_into().expect("failed conversion"))),
                Rc::new(RefCell::new(2.try_into().expect("failed conversion"))),
            )));
            SImpl::combine_14(
                &s.as_pointer(),
                _p.as_pointer(),
                FnPtr::<fn(Ptr<i32>, i32)>::new(add_1),
                (z.as_pointer()),
                5_u64,
            )
        }) == 107)
    );
    assert!(((*z.borrow()) == 6));
    let a: Value<Access_S_> = Rc::new(RefCell::new(<Access_S_>::default()));
    let cs: Value<Ptr<S>> = Rc::new(RefCell::new((s.as_pointer())));
    let cr: Ptr<S> = s.as_pointer();
    assert!((({ Access_S_Impl::get_1(&a.as_pointer(), (s.as_pointer()),) }) == 100));
    assert!((({ Access_S_Impl::get_2(&a.as_pointer(), (*cs.borrow()).clone(),) }) == 101));
    assert!((({ Access_S_Impl::ref_3(&a.as_pointer(), s.as_pointer(),) }) == 102));
    assert!(
        (({
            let _r: Ptr<S> = (cr).clone();
            Access_S_Impl::ref_4(&a.as_pointer(), _r)
        }) == 103)
    );
    let b: Value<Box> = Rc::new(RefCell::new(Box {
        v: Rc::new(RefCell::new(4)),
    }));
    assert!(((*(*b.borrow()).v.borrow()) == 4));
    return 0;
}
pub trait Access_S_Impl {
    fn get_1(&self, p: Ptr<S>) -> i32;
    fn get_2(&self, p: Ptr<S>) -> i32;
    fn ref_3(&self, r: Ptr<S>) -> i32;
    fn ref_4(&self, r: Ptr<S>) -> i32;
}
impl Access_S_Impl for Ptr<Access_S_> {
    fn get_1(&self, p: Ptr<S>) -> i32 {
        let p: Value<Ptr<S>> = Rc::new(RefCell::new(p));
        return (*(*(*p.borrow()).upgrade().deref()).base.borrow());
    }
    fn get_2(&self, p: Ptr<S>) -> i32 {
        let p: Value<Ptr<S>> = Rc::new(RefCell::new(p));
        return ((*(*(*p.borrow()).upgrade().deref()).base.borrow()) + 1);
    }
    fn ref_3(&self, r: Ptr<S>) -> i32 {
        return ((*(*r.upgrade().deref()).base.borrow()) + 2);
    }
    fn ref_4(&self, r: Ptr<S>) -> i32 {
        return ((*(*r.upgrade().deref()).base.borrow()) + 3);
    }
}
pub trait SImpl {
    fn plain_4(&self, x: i32) -> i32;
    fn plain_5(&self, x: i64) -> i32;
    fn take_6(&self, x: Ptr<i32>) -> i32;
    fn take_7(&self, x: Ptr<i32>) -> i32;
    fn pick_8(&self, p: (Value<i32>, Value<i32>)) -> i32;
    fn pick_9(&self, p: (Value<i32>, Value<i64>)) -> i32;
    fn apply_10(&self, f: FnPtr<fn(Ptr<i32>)>, x: i32) -> i32;
    fn apply_11(&self, f: FnPtr<fn(Ptr<i32>, i32)>, x: i32) -> i32;
    fn apply_12(&self, f: FnPtr<fn(i32) -> i32>, x: i32) -> i32;
    fn combine_13(
        &self,
        p: Ptr<(Value<i32>, Value<i64>)>,
        f: FnPtr<fn(i32) -> i32>,
        q: Ptr<i32>,
        n: u64,
    ) -> i32;
    fn combine_14(
        &self,
        p: Ptr<(Value<i32>, Value<i32>)>,
        f: FnPtr<fn(Ptr<i32>, i32)>,
        q: Ptr<i32>,
        n: u64,
    ) -> i32;
    fn width_1_char(&self, x: i32) -> i32;
    fn width_1_int(&self, x: i32) -> i32;
    fn scale_2_2(&self, x: i32) -> i32;
    fn scale_2_3(&self, x: i32) -> i32;
    fn count_3(&self, x: i32) -> i32;
    fn count_3_int_long(&self, x: i32) -> i32;
}
impl SImpl for Ptr<S> {
    fn plain_4(&self, x: i32) -> i32 {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        return ((*(*(*self).upgrade().deref()).base.borrow()) + (*x.borrow()));
    }
    fn plain_5(&self, x: i64) -> i32 {
        let x: Value<i64> = Rc::new(RefCell::new(x));
        return (((*(*(*self).upgrade().deref()).base.borrow()) + ((*x.borrow()) as i32)) + 1);
    }
    fn take_6(&self, x: Ptr<i32>) -> i32 {
        return ({
            let _lhs = (*(*(*self).upgrade().deref()).base.borrow());
            _lhs + (x.read())
        } + 1);
    }
    fn take_7(&self, x: Ptr<i32>) -> i32 {
        return ({
            let _lhs = (*(*(*self).upgrade().deref()).base.borrow());
            _lhs + (x.read())
        } + 2);
    }
    fn pick_8(&self, p: (Value<i32>, Value<i32>)) -> i32 {
        let p: Value<(Value<i32>, Value<i32>)> = Rc::new(RefCell::new(p));
        return ((*(*(*self).upgrade().deref()).base.borrow()) + (*(*p.borrow()).0.borrow()));
    }
    fn pick_9(&self, p: (Value<i32>, Value<i64>)) -> i32 {
        let p: Value<(Value<i32>, Value<i64>)> = Rc::new(RefCell::new(p));
        return ((*(*(*self).upgrade().deref()).base.borrow())
            + ((*(*p.borrow()).1.borrow()) as i32));
    }
    fn apply_10(&self, f: FnPtr<fn(Ptr<i32>)>, x: i32) -> i32 {
        let f: Value<FnPtr<fn(Ptr<i32>)>> = Rc::new(RefCell::new(f));
        let x: Value<i32> = Rc::new(RefCell::new(x));
        ({ (*f.borrow()).call((x.as_pointer())) });
        return ((*(*(*self).upgrade().deref()).base.borrow()) + (*x.borrow()));
    }
    fn apply_11(&self, f: FnPtr<fn(Ptr<i32>, i32)>, x: i32) -> i32 {
        let f: Value<FnPtr<fn(Ptr<i32>, i32)>> = Rc::new(RefCell::new(f));
        let x: Value<i32> = Rc::new(RefCell::new(x));
        ({ (*f.borrow()).call((x.as_pointer()), 10) });
        return ((*(*(*self).upgrade().deref()).base.borrow()) + (*x.borrow()));
    }
    fn apply_12(&self, f: FnPtr<fn(i32) -> i32>, x: i32) -> i32 {
        let f: Value<FnPtr<fn(i32) -> i32>> = Rc::new(RefCell::new(f));
        let x: Value<i32> = Rc::new(RefCell::new(x));
        return {
            let _lhs = (*(*(*self).upgrade().deref()).base.borrow());
            _lhs + ({ (*f.borrow()).call((*x.borrow())) })
        };
    }
    fn combine_13(
        &self,
        p: Ptr<(Value<i32>, Value<i64>)>,
        f: FnPtr<fn(i32) -> i32>,
        q: Ptr<i32>,
        n: u64,
    ) -> i32 {
        let f: Value<FnPtr<fn(i32) -> i32>> = Rc::new(RefCell::new(f));
        let q: Value<Ptr<i32>> = Rc::new(RefCell::new(q));
        let n: Value<u64> = Rc::new(RefCell::new(n));
        return {
            let _lhs = {
                let _lhs = {
                    let _lhs = (*(*(*self).upgrade().deref()).base.borrow());
                    _lhs + ((*(*p.upgrade().deref()).1.borrow()) as i32)
                };
                _lhs + ({ (*f.borrow()).call(((*q.borrow()).read())) })
            };
            _lhs + ((*n.borrow()) as i32)
        };
    }
    fn combine_14(
        &self,
        p: Ptr<(Value<i32>, Value<i32>)>,
        f: FnPtr<fn(Ptr<i32>, i32)>,
        q: Ptr<i32>,
        n: u64,
    ) -> i32 {
        let f: Value<FnPtr<fn(Ptr<i32>, i32)>> = Rc::new(RefCell::new(f));
        let q: Value<Ptr<i32>> = Rc::new(RefCell::new(q));
        let n: Value<u64> = Rc::new(RefCell::new(n));
        ({
            let _arg0: Ptr<i32> = (*q.borrow()).clone();
            let _arg1: i32 = ((*n.borrow()) as i32);
            (*f.borrow()).call(_arg0, _arg1)
        });
        return {
            let _lhs = {
                let _lhs = (*(*(*self).upgrade().deref()).base.borrow());
                _lhs + (*(*p.upgrade().deref()).0.borrow())
            };
            _lhs + ((*q.borrow()).read())
        };
    }
    fn width_1_char(&self, x: i32) -> i32 {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        return ((*(*(*self).upgrade().deref()).base.borrow())
            + ((*x.borrow()) * (::std::mem::size_of::<u8>() as i32)));
    }
    fn width_1_int(&self, x: i32) -> i32 {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        return ((*(*(*self).upgrade().deref()).base.borrow())
            + ((*x.borrow()) * (::std::mem::size_of::<i32>() as i32)));
    }
    fn scale_2_2(&self, x: i32) -> i32 {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        return ((*(*(*self).upgrade().deref()).base.borrow()) + ((*x.borrow()) * 2));
    }
    fn scale_2_3(&self, x: i32) -> i32 {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        return ((*(*(*self).upgrade().deref()).base.borrow()) + ((*x.borrow()) * 3));
    }
    fn count_3(&self, x: i32) -> i32 {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        return (((*(*(*self).upgrade().deref()).base.borrow()) + (*x.borrow())) + (0 as i32));
    }
    fn count_3_int_long(&self, x: i32) -> i32 {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        return (((*(*(*self).upgrade().deref()).base.borrow()) + (*x.borrow())) + (2 as i32));
    }
}
pub fn __cpp2rust_init_globals() {}
