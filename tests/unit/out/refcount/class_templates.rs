extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(DeepClone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(24)]
pub struct MyContainer_int_ {
    #[offset(0)]
    #[byte_size(24)]
    vec_: Value<Vec<i32>>,
}
#[derive(DeepClone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(24)]
pub struct MyContainer_char_ {
    #[offset(0)]
    #[byte_size(24)]
    vec_: Value<Vec<u8>>,
}
#[derive(DeepClone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(24)]
pub struct MyContainer_float_ {
    #[offset(0)]
    #[byte_size(24)]
    vec_: Value<Vec<f32>>,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Boxed_int_ {
    #[offset(0)]
    pub value: i32,
}
impl Boxed_int_ {
    pub fn twice(v: i32) -> i32 {
        let v: Value<i32> = Rc::new(RefCell::new(v));
        return ((*v.borrow()) + (*v.borrow()));
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Boxed_long_ {
    #[offset(0)]
    pub value: i64,
}
impl Boxed_long_ {
    pub fn twice(v: i64) -> i64 {
        let v: Value<i64> = Rc::new(RefCell::new(v));
        return ((*v.borrow()) + (*v.borrow()));
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Outer_int__Inner_int_ {
    #[offset(0)]
    pub t: i32,
    #[offset(4)]
    pub u: i32,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Outer_int_ {
    #[offset(0)]
    pub v: i32,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct Outer_long__Inner_int_ {
    #[offset(0)]
    pub t: i64,
    #[offset(8)]
    pub u: i32,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct Outer_long__Inner_char_ {
    #[offset(0)]
    pub t: i64,
    #[offset(8)]
    pub u: u8,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Outer_long_ {
    #[offset(0)]
    pub v: i64,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let oi: Value<Outer_int_> = Rc::new(RefCell::new(Outer_int_ { v: 3 }));
    assert!(
        (({
            Outer_int__Inner_int_Impl::sum(
                &Rc::new(RefCell::new(
                    ({ Outer_int_Impl::with(&oi.as_pointer(), 4) }),
                ))
                .as_pointer(),
            )
        }) == 7)
    );
    let ol: Value<Outer_long_> = Rc::new(RefCell::new(Outer_long_ { v: 5_i64 }));
    let ic: Value<Outer_long__Inner_char_> = Rc::new(RefCell::new(Outer_long__Inner_char_ {
        t: 6_i64,
        u: ('a' as u8),
    }));
    assert!(
        (({
            Outer_long__Inner_int_Impl::sum(
                &Rc::new(RefCell::new(
                    ({ Outer_long_Impl::with(&ol.as_pointer(), 2) }),
                ))
                .as_pointer(),
            )
        }) == 7)
    );
    assert!(
        (({ Outer_long__Inner_char_Impl::sum(&ic.as_pointer(),) }) == (6 + (('a' as u8) as i32)))
    );
    assert!((({ Boxed_int_::twice(3,) }) == 6));
    let bi: Value<Boxed_int_> = Rc::new(RefCell::new(Boxed_int_ { value: 4 }));
    assert!((({ Boxed_int_Impl::plus(&bi.as_pointer(), 5,) }) == 9));
    assert!((({ Boxed_long_::twice(10_i64,) }) == 20_i64));
    let bl: Value<Boxed_long_> = Rc::new(RefCell::new(Boxed_long_ { value: 7_i64 }));
    assert!((({ Boxed_long_Impl::plus(&bl.as_pointer(), 1_i64,) }) == 8_i64));
    let imc: Value<MyContainer_int_> = Rc::new(RefCell::new(<MyContainer_int_>::default()));
    assert!(({ MyContainer_int_Impl::empty(&imc.as_pointer(),) }));
    ({
        let _item: Value<i32> = Rc::new(RefCell::new(1));
        MyContainer_int_Impl::push_back(&imc.as_pointer(), _item.as_pointer())
    });
    assert!(
        (({ MyContainer_int_Impl::size(&imc.as_pointer(),) }) == 1_usize)
            && ((({ MyContainer_int_Impl::back_4(&imc.as_pointer(),) }).read()) == 1)
    );
    ({ MyContainer_int_Impl::pop_back(&imc.as_pointer()) });
    assert!(({ MyContainer_int_Impl::empty(&imc.as_pointer(),) }));
    let cmc: Value<MyContainer_char_> = Rc::new(RefCell::new(<MyContainer_char_>::default()));
    assert!(({ MyContainer_char_Impl::empty(&cmc.as_pointer(),) }));
    ({
        let _item: Value<u8> = Rc::new(RefCell::new(('a' as u8)));
        MyContainer_char_Impl::push_back(&cmc.as_pointer(), _item.as_pointer())
    });
    assert!(
        (({ MyContainer_char_Impl::size(&cmc.as_pointer(),) }) == 1_usize)
            && (((({ MyContainer_char_Impl::back_4(&cmc.as_pointer(),) }).read()) as i32)
                == (('a' as u8) as i32))
    );
    ({ MyContainer_char_Impl::pop_back(&cmc.as_pointer()) });
    assert!(({ MyContainer_char_Impl::empty(&cmc.as_pointer(),) }));
    let fmc: Value<MyContainer_float_> = Rc::new(RefCell::new(<MyContainer_float_>::default()));
    assert!(({ MyContainer_float_Impl::empty(&fmc.as_pointer(),) }));
    ({
        let _item: Value<f32> = Rc::new(RefCell::new((1.0E+0 as f32)));
        MyContainer_float_Impl::push_back(&fmc.as_pointer(), _item.as_pointer())
    });
    assert!(
        (({ MyContainer_float_Impl::size(&fmc.as_pointer(),) }) == 1_usize)
            && (((({ MyContainer_float_Impl::back_4(&fmc.as_pointer(),) }).read()) as f64)
                == 1.0E+0)
    );
    ({ MyContainer_float_Impl::pop_back(&fmc.as_pointer()) });
    assert!(({ MyContainer_float_Impl::empty(&fmc.as_pointer(),) }));
    return 0;
}
pub trait Boxed_int_Impl {
    fn plus(&self, other: i32) -> i32;
}
impl Boxed_int_Impl for Ptr<Boxed_int_> {
    fn plus(&self, other: i32) -> i32 {
        let other: Value<i32> = Rc::new(RefCell::new(other));
        return ((*self).with(|__s| __s.value) + (*other.borrow()));
    }
}
pub trait Boxed_long_Impl {
    fn plus(&self, other: i64) -> i64;
}
impl Boxed_long_Impl for Ptr<Boxed_long_> {
    fn plus(&self, other: i64) -> i64 {
        let other: Value<i64> = Rc::new(RefCell::new(other));
        return ((*self).with(|__s| __s.value) + (*other.borrow()));
    }
}
pub trait MyContainer_char_Impl {
    fn empty(&self) -> bool;
    fn size(&self) -> usize;
    fn back_3(&self) -> Ptr<u8> {
        unimplemented!()
    }
    fn back_4(&self) -> Ptr<u8>;
    fn pop_back(&self);
    fn push_back(&self, item: Ptr<u8>);
}
impl MyContainer_char_Impl for Ptr<MyContainer_char_> {
    fn empty(&self) -> bool {
        return (*(*self).with(|__s| __s.vec_.clone()).borrow()).is_empty();
    }
    fn size(&self) -> usize {
        return (*(*self).with(|__s| __s.vec_.clone()).borrow()).len();
    }
    fn back_4(&self) -> Ptr<u8> {
        return ((*self).with(|__s| __s.vec_.as_pointer()) as Ptr<u8>).to_last();
    }
    fn pop_back(&self) {
        (*(*self).with(|__s| __s.vec_.clone()).borrow_mut()).pop();
        return;
    }
    fn push_back(&self, item: Ptr<u8>) {
        {
            let a0_clone = (item.read()).clone();
            (*(*self).with(|__s| __s.vec_.clone()).borrow_mut()).push(a0_clone)
        };
    }
}
pub trait MyContainer_float_Impl {
    fn empty(&self) -> bool;
    fn size(&self) -> usize;
    fn back_3(&self) -> Ptr<f32> {
        unimplemented!()
    }
    fn back_4(&self) -> Ptr<f32>;
    fn pop_back(&self);
    fn push_back(&self, item: Ptr<f32>);
}
impl MyContainer_float_Impl for Ptr<MyContainer_float_> {
    fn empty(&self) -> bool {
        return (*(*self).with(|__s| __s.vec_.clone()).borrow()).is_empty();
    }
    fn size(&self) -> usize {
        return (*(*self).with(|__s| __s.vec_.clone()).borrow()).len();
    }
    fn back_4(&self) -> Ptr<f32> {
        return ((*self).with(|__s| __s.vec_.as_pointer()) as Ptr<f32>).to_last();
    }
    fn pop_back(&self) {
        (*(*self).with(|__s| __s.vec_.clone()).borrow_mut()).pop();
        return;
    }
    fn push_back(&self, item: Ptr<f32>) {
        {
            let a0_clone = (item.read()).clone();
            (*(*self).with(|__s| __s.vec_.clone()).borrow_mut()).push(a0_clone)
        };
    }
}
pub trait MyContainer_int_Impl {
    fn empty(&self) -> bool;
    fn size(&self) -> usize;
    fn back_3(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn back_4(&self) -> Ptr<i32>;
    fn pop_back(&self);
    fn push_back(&self, item: Ptr<i32>);
}
impl MyContainer_int_Impl for Ptr<MyContainer_int_> {
    fn empty(&self) -> bool {
        return (*(*self).with(|__s| __s.vec_.clone()).borrow()).is_empty();
    }
    fn size(&self) -> usize {
        return (*(*self).with(|__s| __s.vec_.clone()).borrow()).len();
    }
    fn back_4(&self) -> Ptr<i32> {
        return ((*self).with(|__s| __s.vec_.as_pointer()) as Ptr<i32>).to_last();
    }
    fn pop_back(&self) {
        (*(*self).with(|__s| __s.vec_.clone()).borrow_mut()).pop();
        return;
    }
    fn push_back(&self, item: Ptr<i32>) {
        {
            let a0_clone = (item.read()).clone();
            (*(*self).with(|__s| __s.vec_.clone()).borrow_mut()).push(a0_clone)
        };
    }
}
pub trait Outer_int_Impl {
    fn with(&self, n: i32) -> Outer_int__Inner_int_;
}
impl Outer_int_Impl for Ptr<Outer_int_> {
    fn with(&self, n: i32) -> Outer_int__Inner_int_ {
        let n: Value<i32> = Rc::new(RefCell::new(n));
        return Outer_int__Inner_int_ {
            t: (*self).with(|__s| __s.v),
            u: (*n.borrow()),
        };
    }
}
pub trait Outer_int__Inner_int_Impl {
    fn sum(&self) -> i32;
}
impl Outer_int__Inner_int_Impl for Ptr<Outer_int__Inner_int_> {
    fn sum(&self) -> i32 {
        return (((*self).with(|__s| __s.t) as i32) + ((*self).with(|__s| __s.u) as i32));
    }
}
pub trait Outer_long_Impl {
    fn with(&self, n: i32) -> Outer_long__Inner_int_;
}
impl Outer_long_Impl for Ptr<Outer_long_> {
    fn with(&self, n: i32) -> Outer_long__Inner_int_ {
        let n: Value<i32> = Rc::new(RefCell::new(n));
        return Outer_long__Inner_int_ {
            t: (*self).with(|__s| __s.v),
            u: (*n.borrow()),
        };
    }
}
pub trait Outer_long__Inner_char_Impl {
    fn sum(&self) -> i32;
}
impl Outer_long__Inner_char_Impl for Ptr<Outer_long__Inner_char_> {
    fn sum(&self) -> i32 {
        return (((*self).with(|__s| __s.t) as i32) + ((*self).with(|__s| __s.u) as i32));
    }
}
pub trait Outer_long__Inner_int_Impl {
    fn sum(&self) -> i32;
}
impl Outer_long__Inner_int_Impl for Ptr<Outer_long__Inner_int_> {
    fn sum(&self) -> i32 {
        return (((*self).with(|__s| __s.t) as i32) + ((*self).with(|__s| __s.u) as i32));
    }
}
pub fn __cpp2rust_init_globals() {}
