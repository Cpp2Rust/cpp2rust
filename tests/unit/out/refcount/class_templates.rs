extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct MyContainer_int_ {
    vec_: Value<Vec<i32>>,
}
impl Clone for MyContainer_int_ {
    fn clone(&self) -> Self {
        let __this: Value<MyContainer_int_> = Rc::new(RefCell::new(Self {
            vec_: Rc::new(RefCell::new((*self.vec_.borrow()).clone())),
        }));
        let this: Ptr<MyContainer_int_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for MyContainer_int_ {
    fn byte_size() -> usize {
        24
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.vec_.borrow()).to_bytes(&mut buf[0..24]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            vec_: Rc::new(RefCell::new(<Vec<i32>>::from_bytes(&buf[0..24]))),
        }
    }
}
#[derive(Default)]
pub struct MyContainer_char_ {
    vec_: Value<Vec<u8>>,
}
impl Clone for MyContainer_char_ {
    fn clone(&self) -> Self {
        let __this: Value<MyContainer_char_> = Rc::new(RefCell::new(Self {
            vec_: Rc::new(RefCell::new((*self.vec_.borrow()).clone())),
        }));
        let this: Ptr<MyContainer_char_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for MyContainer_char_ {
    fn byte_size() -> usize {
        24
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.vec_.borrow()).to_bytes(&mut buf[0..24]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            vec_: Rc::new(RefCell::new(<Vec<u8>>::from_bytes(&buf[0..24]))),
        }
    }
}
#[derive(Default)]
pub struct MyContainer_float_ {
    vec_: Value<Vec<f32>>,
}
impl Clone for MyContainer_float_ {
    fn clone(&self) -> Self {
        let __this: Value<MyContainer_float_> = Rc::new(RefCell::new(Self {
            vec_: Rc::new(RefCell::new((*self.vec_.borrow()).clone())),
        }));
        let this: Ptr<MyContainer_float_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for MyContainer_float_ {
    fn byte_size() -> usize {
        24
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.vec_.borrow()).to_bytes(&mut buf[0..24]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            vec_: Rc::new(RefCell::new(<Vec<f32>>::from_bytes(&buf[0..24]))),
        }
    }
}
#[derive(Default)]
pub struct Boxed_int_ {
    pub value: Value<i32>,
}
impl Boxed_int_ {
    pub fn twice(v: i32) -> i32 {
        let v: Value<i32> = Rc::new(RefCell::new(v));
        return ((*v.borrow()) + (*v.borrow()));
    }
}
impl Clone for Boxed_int_ {
    fn clone(&self) -> Self {
        let __this: Value<Boxed_int_> = Rc::new(RefCell::new(Self {
            value: Rc::new(RefCell::new((*self.value.borrow()))),
        }));
        let this: Ptr<Boxed_int_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for Boxed_int_ {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.value.borrow()).to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            value: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
        }
    }
}
#[derive(Default)]
pub struct Boxed_long_ {
    pub value: Value<i64>,
}
impl Boxed_long_ {
    pub fn twice(v: i64) -> i64 {
        let v: Value<i64> = Rc::new(RefCell::new(v));
        return ((*v.borrow()) + (*v.borrow()));
    }
}
impl Clone for Boxed_long_ {
    fn clone(&self) -> Self {
        let __this: Value<Boxed_long_> = Rc::new(RefCell::new(Self {
            value: Rc::new(RefCell::new((*self.value.borrow()))),
        }));
        let this: Ptr<Boxed_long_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for Boxed_long_ {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.value.borrow()).to_bytes(&mut buf[0..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            value: Rc::new(RefCell::new(<i64>::from_bytes(&buf[0..8]))),
        }
    }
}
#[derive(Default)]
pub struct Outer_int__Inner_int_ {
    pub t: Value<i32>,
    pub u: Value<i32>,
}
impl Clone for Outer_int__Inner_int_ {
    fn clone(&self) -> Self {
        let __this: Value<Outer_int__Inner_int_> = Rc::new(RefCell::new(Self {
            t: Rc::new(RefCell::new((*self.t.borrow()))),
            u: Rc::new(RefCell::new((*self.u.borrow()))),
        }));
        let this: Ptr<Outer_int__Inner_int_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for Outer_int__Inner_int_ {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.t.borrow()).to_bytes(&mut buf[0..4]);
        (*self.u.borrow()).to_bytes(&mut buf[4..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            t: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
            u: Rc::new(RefCell::new(<i32>::from_bytes(&buf[4..8]))),
        }
    }
}
#[derive(Default)]
pub struct Outer_int_ {
    pub v: Value<i32>,
}
impl Clone for Outer_int_ {
    fn clone(&self) -> Self {
        let __this: Value<Outer_int_> = Rc::new(RefCell::new(Self {
            v: Rc::new(RefCell::new((*self.v.borrow()))),
        }));
        let this: Ptr<Outer_int_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for Outer_int_ {
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
#[derive(Default)]
pub struct Outer_long__Inner_int_ {
    pub t: Value<i64>,
    pub u: Value<i32>,
}
impl Clone for Outer_long__Inner_int_ {
    fn clone(&self) -> Self {
        let __this: Value<Outer_long__Inner_int_> = Rc::new(RefCell::new(Self {
            t: Rc::new(RefCell::new((*self.t.borrow()))),
            u: Rc::new(RefCell::new((*self.u.borrow()))),
        }));
        let this: Ptr<Outer_long__Inner_int_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for Outer_long__Inner_int_ {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.t.borrow()).to_bytes(&mut buf[0..8]);
        (*self.u.borrow()).to_bytes(&mut buf[8..12]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            t: Rc::new(RefCell::new(<i64>::from_bytes(&buf[0..8]))),
            u: Rc::new(RefCell::new(<i32>::from_bytes(&buf[8..12]))),
        }
    }
}
#[derive(Default)]
pub struct Outer_long__Inner_char_ {
    pub t: Value<i64>,
    pub u: Value<u8>,
}
impl Clone for Outer_long__Inner_char_ {
    fn clone(&self) -> Self {
        let __this: Value<Outer_long__Inner_char_> = Rc::new(RefCell::new(Self {
            t: Rc::new(RefCell::new((*self.t.borrow()))),
            u: Rc::new(RefCell::new((*self.u.borrow()))),
        }));
        let this: Ptr<Outer_long__Inner_char_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for Outer_long__Inner_char_ {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.t.borrow()).to_bytes(&mut buf[0..8]);
        (*self.u.borrow()).to_bytes(&mut buf[8..9]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            t: Rc::new(RefCell::new(<i64>::from_bytes(&buf[0..8]))),
            u: Rc::new(RefCell::new(<u8>::from_bytes(&buf[8..9]))),
        }
    }
}
#[derive(Default)]
pub struct Outer_long_ {
    pub v: Value<i64>,
}
impl Clone for Outer_long_ {
    fn clone(&self) -> Self {
        let __this: Value<Outer_long_> = Rc::new(RefCell::new(Self {
            v: Rc::new(RefCell::new((*self.v.borrow()))),
        }));
        let this: Ptr<Outer_long_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for Outer_long_ {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.v.borrow()).to_bytes(&mut buf[0..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            v: Rc::new(RefCell::new(<i64>::from_bytes(&buf[0..8]))),
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let oi: Value<Outer_int_> = Rc::new(RefCell::new(Outer_int_ {
        v: Rc::new(RefCell::new(3)),
    }));
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
    let ol: Value<Outer_long_> = Rc::new(RefCell::new(Outer_long_ {
        v: Rc::new(RefCell::new(5_i64)),
    }));
    let ic: Value<Outer_long__Inner_char_> = Rc::new(RefCell::new(Outer_long__Inner_char_ {
        t: Rc::new(RefCell::new(6_i64)),
        u: Rc::new(RefCell::new(('a' as u8))),
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
    let bi: Value<Boxed_int_> = Rc::new(RefCell::new(Boxed_int_ {
        value: Rc::new(RefCell::new(4)),
    }));
    assert!((({ Boxed_int_Impl::plus(&bi.as_pointer(), 5,) }) == 9));
    assert!((({ Boxed_long_::twice(10_i64,) }) == 20_i64));
    let bl: Value<Boxed_long_> = Rc::new(RefCell::new(Boxed_long_ {
        value: Rc::new(RefCell::new(7_i64)),
    }));
    assert!((({ Boxed_long_Impl::plus(&bl.as_pointer(), 1_i64,) }) == 8_i64));
    let imc: Value<MyContainer_int_> = Rc::new(RefCell::new(MyContainer_int_ {
        vec_: Rc::new(RefCell::new(Default::default())),
    }));
    assert!(({ MyContainer_int_Impl::empty(&imc.as_pointer(),) }));
    ({
        let _item: Value<i32> = Rc::new(RefCell::new(1));
        MyContainer_int_Impl::push_back(&imc.as_pointer(), _item.as_pointer())
    });
    assert!(
        (({ MyContainer_int_Impl::size(&imc.as_pointer(),) }) == 1_usize)
            && ((({ MyContainer_int_Impl::back(&imc.as_pointer(),) }).read()) == 1)
    );
    ({ MyContainer_int_Impl::pop_back(&imc.as_pointer()) });
    assert!(({ MyContainer_int_Impl::empty(&imc.as_pointer(),) }));
    let cmc: Value<MyContainer_char_> = Rc::new(RefCell::new(MyContainer_char_ {
        vec_: Rc::new(RefCell::new(Default::default())),
    }));
    assert!(({ MyContainer_char_Impl::empty(&cmc.as_pointer(),) }));
    ({
        let _item: Value<u8> = Rc::new(RefCell::new(('a' as u8)));
        MyContainer_char_Impl::push_back(&cmc.as_pointer(), _item.as_pointer())
    });
    assert!(
        (({ MyContainer_char_Impl::size(&cmc.as_pointer(),) }) == 1_usize)
            && (((({ MyContainer_char_Impl::back(&cmc.as_pointer(),) }).read()) as i32)
                == (('a' as u8) as i32))
    );
    ({ MyContainer_char_Impl::pop_back(&cmc.as_pointer()) });
    assert!(({ MyContainer_char_Impl::empty(&cmc.as_pointer(),) }));
    let fmc: Value<MyContainer_float_> = Rc::new(RefCell::new(MyContainer_float_ {
        vec_: Rc::new(RefCell::new(Default::default())),
    }));
    assert!(({ MyContainer_float_Impl::empty(&fmc.as_pointer(),) }));
    ({
        let _item: Value<f32> = Rc::new(RefCell::new((1.0E+0 as f32)));
        MyContainer_float_Impl::push_back(&fmc.as_pointer(), _item.as_pointer())
    });
    assert!(
        (({ MyContainer_float_Impl::size(&fmc.as_pointer(),) }) == 1_usize)
            && (((({ MyContainer_float_Impl::back(&fmc.as_pointer(),) }).read()) as f64) == 1.0E+0)
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
        return ((*(*(*self).upgrade().deref()).value.borrow()) + (*other.borrow()));
    }
}
pub trait Boxed_long_Impl {
    fn plus(&self, other: i64) -> i64;
}
impl Boxed_long_Impl for Ptr<Boxed_long_> {
    fn plus(&self, other: i64) -> i64 {
        let other: Value<i64> = Rc::new(RefCell::new(other));
        return ((*(*(*self).upgrade().deref()).value.borrow()) + (*other.borrow()));
    }
}
pub trait MyContainer_char_Impl {
    fn empty(&self) -> bool;
    fn size(&self) -> usize;
    fn back_const(&self) -> Ptr<u8> {
        unimplemented!()
    }
    fn back(&self) -> Ptr<u8>;
    fn pop_back(&self);
    fn push_back(&self, item: Ptr<u8>);
}
impl MyContainer_char_Impl for Ptr<MyContainer_char_> {
    fn empty(&self) -> bool {
        return (*(*(*self).upgrade().deref()).vec_.borrow()).is_empty();
    }
    fn size(&self) -> usize {
        return (*(*(*self).upgrade().deref()).vec_.borrow()).len();
    }
    fn back(&self) -> Ptr<u8> {
        return ((*(*self).upgrade().deref()).vec_.as_pointer() as Ptr<u8>).to_last();
    }
    fn pop_back(&self) {
        (*(*(*self).upgrade().deref()).vec_.borrow_mut()).pop();
        return;
    }
    fn push_back(&self, item: Ptr<u8>) {
        {
            let a0_clone = (item.read()).clone();
            (*(*(*self).upgrade().deref()).vec_.borrow_mut()).push(a0_clone)
        };
    }
}
pub trait MyContainer_float_Impl {
    fn empty(&self) -> bool;
    fn size(&self) -> usize;
    fn back_const(&self) -> Ptr<f32> {
        unimplemented!()
    }
    fn back(&self) -> Ptr<f32>;
    fn pop_back(&self);
    fn push_back(&self, item: Ptr<f32>);
}
impl MyContainer_float_Impl for Ptr<MyContainer_float_> {
    fn empty(&self) -> bool {
        return (*(*(*self).upgrade().deref()).vec_.borrow()).is_empty();
    }
    fn size(&self) -> usize {
        return (*(*(*self).upgrade().deref()).vec_.borrow()).len();
    }
    fn back(&self) -> Ptr<f32> {
        return ((*(*self).upgrade().deref()).vec_.as_pointer() as Ptr<f32>).to_last();
    }
    fn pop_back(&self) {
        (*(*(*self).upgrade().deref()).vec_.borrow_mut()).pop();
        return;
    }
    fn push_back(&self, item: Ptr<f32>) {
        {
            let a0_clone = (item.read()).clone();
            (*(*(*self).upgrade().deref()).vec_.borrow_mut()).push(a0_clone)
        };
    }
}
pub trait MyContainer_int_Impl {
    fn empty(&self) -> bool;
    fn size(&self) -> usize;
    fn back_const(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn back(&self) -> Ptr<i32>;
    fn pop_back(&self);
    fn push_back(&self, item: Ptr<i32>);
}
impl MyContainer_int_Impl for Ptr<MyContainer_int_> {
    fn empty(&self) -> bool {
        return (*(*(*self).upgrade().deref()).vec_.borrow()).is_empty();
    }
    fn size(&self) -> usize {
        return (*(*(*self).upgrade().deref()).vec_.borrow()).len();
    }
    fn back(&self) -> Ptr<i32> {
        return ((*(*self).upgrade().deref()).vec_.as_pointer() as Ptr<i32>).to_last();
    }
    fn pop_back(&self) {
        (*(*(*self).upgrade().deref()).vec_.borrow_mut()).pop();
        return;
    }
    fn push_back(&self, item: Ptr<i32>) {
        {
            let a0_clone = (item.read()).clone();
            (*(*(*self).upgrade().deref()).vec_.borrow_mut()).push(a0_clone)
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
            t: Rc::new(RefCell::new((*(*(*self).upgrade().deref()).v.borrow()))),
            u: Rc::new(RefCell::new((*n.borrow()))),
        };
    }
}
pub trait Outer_int__Inner_int_Impl {
    fn sum(&self) -> i32;
}
impl Outer_int__Inner_int_Impl for Ptr<Outer_int__Inner_int_> {
    fn sum(&self) -> i32 {
        return (((*(*(*self).upgrade().deref()).t.borrow()) as i32)
            + ((*(*(*self).upgrade().deref()).u.borrow()) as i32));
    }
}
pub trait Outer_long_Impl {
    fn with(&self, n: i32) -> Outer_long__Inner_int_;
}
impl Outer_long_Impl for Ptr<Outer_long_> {
    fn with(&self, n: i32) -> Outer_long__Inner_int_ {
        let n: Value<i32> = Rc::new(RefCell::new(n));
        return Outer_long__Inner_int_ {
            t: Rc::new(RefCell::new((*(*(*self).upgrade().deref()).v.borrow()))),
            u: Rc::new(RefCell::new((*n.borrow()))),
        };
    }
}
pub trait Outer_long__Inner_char_Impl {
    fn sum(&self) -> i32;
}
impl Outer_long__Inner_char_Impl for Ptr<Outer_long__Inner_char_> {
    fn sum(&self) -> i32 {
        return (((*(*(*self).upgrade().deref()).t.borrow()) as i32)
            + ((*(*(*self).upgrade().deref()).u.borrow()) as i32));
    }
}
pub trait Outer_long__Inner_int_Impl {
    fn sum(&self) -> i32;
}
impl Outer_long__Inner_int_Impl for Ptr<Outer_long__Inner_int_> {
    fn sum(&self) -> i32 {
        return (((*(*(*self).upgrade().deref()).t.borrow()) as i32)
            + ((*(*(*self).upgrade().deref()).u.borrow()) as i32));
    }
}
pub fn __cpp2rust_init_globals() {}
