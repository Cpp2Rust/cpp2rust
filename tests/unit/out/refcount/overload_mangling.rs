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
    assert!((({ SImpl::width_i32__char_const(&s.as_pointer(), 3,) }) == 103));
    assert!((({ SImpl::width_i32__int_const(&s.as_pointer(), 3,) }) == 112));
    assert!((({ SImpl::scale_i32__2_const(&s.as_pointer(), 5,) }) == 110));
    assert!((({ SImpl::scale_i32__3_const(&s.as_pointer(), 5,) }) == 115));
    assert!((({ SImpl::count_i32_const(&s.as_pointer(), 1,) }) == 101));
    assert!((({ SImpl::count_i32__int_long_const(&s.as_pointer(), 1,) }) == 103));
    assert!((({ SImpl::plain_i32_const(&s.as_pointer(), 1,) }) == 101));
    assert!((({ SImpl::plain_i64_const(&s.as_pointer(), 1_i64,) }) == 102));
    let y: Value<i32> = Rc::new(RefCell::new(1));
    assert!((({ SImpl::take_pmuti32_const(&s.as_pointer(), y.as_pointer(),) }) == 102));
    assert!(
        (({
            let _x: Value<i32> = Rc::new(RefCell::new(5));
            SImpl::take_pmuti32_rv_const(&s.as_pointer(), _x.as_pointer())
        }) == 107)
    );
    assert!(
        (({
            SImpl::pick_Valuei32_Valuei32_const(
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
            SImpl::pick_Valuei32_Valuei64_const(
                &s.as_pointer(),
                (
                    Rc::new(RefCell::new(1.try_into().expect("failed conversion"))),
                    Rc::new(RefCell::new(2_i64.try_into().expect("failed conversion"))),
                ),
            )
        }) == 102)
    );
    assert!(
        (({
            SImpl::apply_OptionunsafefnPtri32__i32_const(
                &s.as_pointer(),
                FnPtr::<fn(Ptr<i32>)>::new(inc_0),
                1,
            )
        }) == 102)
    );
    assert!(
        (({
            SImpl::apply_OptionunsafefnPtri32_i32__i32_const(
                &s.as_pointer(),
                FnPtr::<fn(Ptr<i32>, i32)>::new(add_1),
                1,
            )
        }) == 111)
    );
    assert!(
        (({
            SImpl::apply_Optionunsafefni32_i32_i32_const(
                &s.as_pointer(),
                FnPtr::<fn(i32) -> i32>::new(twice_2),
                3,
            )
        }) == 106)
    );
    let b: Value<Box> = Rc::new(RefCell::new(Box {
        v: Rc::new(RefCell::new(4)),
    }));
    assert!(((*(*b.borrow()).v.borrow()) == 4));
    return 0;
}
pub trait SImpl {
    fn plain_i32_const(&self, x: i32) -> i32;
    fn plain_i64_const(&self, x: i64) -> i32;
    fn take_pmuti32_const(&self, x: Ptr<i32>) -> i32;
    fn take_pmuti32_rv_const(&self, x: Ptr<i32>) -> i32;
    fn pick_Valuei32_Valuei32_const(&self, p: (Value<i32>, Value<i32>)) -> i32;
    fn pick_Valuei32_Valuei64_const(&self, p: (Value<i32>, Value<i64>)) -> i32;
    fn apply_OptionunsafefnPtri32__i32_const(&self, f: FnPtr<fn(Ptr<i32>)>, x: i32) -> i32;
    fn apply_OptionunsafefnPtri32_i32__i32_const(&self, f: FnPtr<fn(Ptr<i32>, i32)>, x: i32)
    -> i32;
    fn apply_Optionunsafefni32_i32_i32_const(&self, f: FnPtr<fn(i32) -> i32>, x: i32) -> i32;
    fn width_i32__char_const(&self, x: i32) -> i32;
    fn width_i32__int_const(&self, x: i32) -> i32;
    fn scale_i32__2_const(&self, x: i32) -> i32;
    fn scale_i32__3_const(&self, x: i32) -> i32;
    fn count_i32_const(&self, x: i32) -> i32;
    fn count_i32__int_long_const(&self, x: i32) -> i32;
}
impl SImpl for Ptr<S> {
    fn plain_i32_const(&self, x: i32) -> i32 {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        return ((*(*(*self).upgrade().deref()).base.borrow()) + (*x.borrow()));
    }
    fn plain_i64_const(&self, x: i64) -> i32 {
        let x: Value<i64> = Rc::new(RefCell::new(x));
        return (((*(*(*self).upgrade().deref()).base.borrow()) + ((*x.borrow()) as i32)) + 1);
    }
    fn take_pmuti32_const(&self, x: Ptr<i32>) -> i32 {
        return ({
            let _lhs = (*(*(*self).upgrade().deref()).base.borrow());
            _lhs + (x.read())
        } + 1);
    }
    fn take_pmuti32_rv_const(&self, x: Ptr<i32>) -> i32 {
        return ({
            let _lhs = (*(*(*self).upgrade().deref()).base.borrow());
            _lhs + (x.read())
        } + 2);
    }
    fn pick_Valuei32_Valuei32_const(&self, p: (Value<i32>, Value<i32>)) -> i32 {
        let p: Value<(Value<i32>, Value<i32>)> = Rc::new(RefCell::new(p));
        return ((*(*(*self).upgrade().deref()).base.borrow()) + (*(*p.borrow()).0.borrow()));
    }
    fn pick_Valuei32_Valuei64_const(&self, p: (Value<i32>, Value<i64>)) -> i32 {
        let p: Value<(Value<i32>, Value<i64>)> = Rc::new(RefCell::new(p));
        return ((*(*(*self).upgrade().deref()).base.borrow())
            + ((*(*p.borrow()).1.borrow()) as i32));
    }
    fn apply_OptionunsafefnPtri32__i32_const(&self, f: FnPtr<fn(Ptr<i32>)>, x: i32) -> i32 {
        let f: Value<FnPtr<fn(Ptr<i32>)>> = Rc::new(RefCell::new(f));
        let x: Value<i32> = Rc::new(RefCell::new(x));
        ({ (*(*f.borrow()))((x.as_pointer())) });
        return ((*(*(*self).upgrade().deref()).base.borrow()) + (*x.borrow()));
    }
    fn apply_OptionunsafefnPtri32_i32__i32_const(
        &self,
        f: FnPtr<fn(Ptr<i32>, i32)>,
        x: i32,
    ) -> i32 {
        let f: Value<FnPtr<fn(Ptr<i32>, i32)>> = Rc::new(RefCell::new(f));
        let x: Value<i32> = Rc::new(RefCell::new(x));
        ({ (*(*f.borrow()))((x.as_pointer()), 10) });
        return ((*(*(*self).upgrade().deref()).base.borrow()) + (*x.borrow()));
    }
    fn apply_Optionunsafefni32_i32_i32_const(&self, f: FnPtr<fn(i32) -> i32>, x: i32) -> i32 {
        let f: Value<FnPtr<fn(i32) -> i32>> = Rc::new(RefCell::new(f));
        let x: Value<i32> = Rc::new(RefCell::new(x));
        return {
            let _lhs = (*(*(*self).upgrade().deref()).base.borrow());
            _lhs + ({ (*(*f.borrow()))((*x.borrow())) })
        };
    }
    fn width_i32__char_const(&self, x: i32) -> i32 {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        return ((*(*(*self).upgrade().deref()).base.borrow())
            + ((*x.borrow()) * (::std::mem::size_of::<u8>() as i32)));
    }
    fn width_i32__int_const(&self, x: i32) -> i32 {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        return ((*(*(*self).upgrade().deref()).base.borrow())
            + ((*x.borrow()) * (::std::mem::size_of::<i32>() as i32)));
    }
    fn scale_i32__2_const(&self, x: i32) -> i32 {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        return ((*(*(*self).upgrade().deref()).base.borrow()) + ((*x.borrow()) * 2));
    }
    fn scale_i32__3_const(&self, x: i32) -> i32 {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        return ((*(*(*self).upgrade().deref()).base.borrow()) + ((*x.borrow()) * 3));
    }
    fn count_i32_const(&self, x: i32) -> i32 {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        return (((*(*(*self).upgrade().deref()).base.borrow()) + (*x.borrow())) + (0 as i32));
    }
    fn count_i32__int_long_const(&self, x: i32) -> i32 {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        return (((*(*(*self).upgrade().deref()).base.borrow()) + (*x.borrow())) + (2 as i32));
    }
}
pub fn __cpp2rust_init_globals() {}
