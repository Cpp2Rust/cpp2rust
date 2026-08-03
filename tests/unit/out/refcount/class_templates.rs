extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[repr(C)]
#[derive(Default)]
pub struct MyContainer_int_ {
    vec_: Vec<i32>,
}
pub trait MyContainer_int_Methods {
    fn empty(&self) -> bool;
    fn size(&self) -> usize;
    fn back_const(&self) -> Ptr<i32>;
    fn back(&self) -> Ptr<i32>;
    fn pop_back(&self);
    fn push_back(&self, item: Ptr<i32>);
}
impl MyContainer_int_Methods for Ptr<MyContainer_int_> {
    fn empty(&self) -> bool {
        return self.with(|__v| (*__v).vec_.clone()).is_empty();
    }
    fn size(&self) -> usize {
        return self.with(|__v| (*__v).vec_.clone()).len();
    }
    fn back_const(&self) -> Ptr<i32> {
        return (self.field_ptr(
            0,
            |__v: &MyContainer_int_| &__v.vec_[..],
            |__v: &mut MyContainer_int_| &mut __v.vec_[..],
        ) as Ptr<i32>)
            .to_last();
    }
    fn back(&self) -> Ptr<i32> {
        return (self.field_ptr(
            0,
            |__v: &MyContainer_int_| &__v.vec_[..],
            |__v: &mut MyContainer_int_| &mut __v.vec_[..],
        ) as Ptr<i32>)
            .to_last();
    }
    fn pop_back(&self) {
        self.with_mut(|__v| __v.vec_.pop());
        return;
    }
    fn push_back(&self, item: Ptr<i32>) {
        {
            let a0_clone = (item.read()).clone();
            self.with_mut(|__v| __v.vec_.push(a0_clone))
        };
    }
}
impl Clone for MyContainer_int_ {
    fn clone(&self) -> Self {
        let mut this = Self {
            vec_: (self.vec_).clone(),
        };
        this
    }
}
impl ByteRepr for MyContainer_int_ {
    fn byte_size() -> usize {
        24
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.vec_.to_bytes(&mut buf[0..24]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            vec_: <Vec<i32>>::from_bytes(&buf[0..24]),
        }
    }
}
#[repr(C)]
#[derive(Default)]
pub struct MyContainer_char_ {
    vec_: Vec<u8>,
}
pub trait MyContainer_char_Methods {
    fn empty(&self) -> bool;
    fn size(&self) -> usize;
    fn back_const(&self) -> Ptr<u8>;
    fn back(&self) -> Ptr<u8>;
    fn pop_back(&self);
    fn push_back(&self, item: Ptr<u8>);
}
impl MyContainer_char_Methods for Ptr<MyContainer_char_> {
    fn empty(&self) -> bool {
        return self.with(|__v| (*__v).vec_.clone()).is_empty();
    }
    fn size(&self) -> usize {
        return self.with(|__v| (*__v).vec_.clone()).len();
    }
    fn back_const(&self) -> Ptr<u8> {
        return (self.field_ptr(
            0,
            |__v: &MyContainer_char_| &__v.vec_[..],
            |__v: &mut MyContainer_char_| &mut __v.vec_[..],
        ) as Ptr<u8>)
            .to_last();
    }
    fn back(&self) -> Ptr<u8> {
        return (self.field_ptr(
            0,
            |__v: &MyContainer_char_| &__v.vec_[..],
            |__v: &mut MyContainer_char_| &mut __v.vec_[..],
        ) as Ptr<u8>)
            .to_last();
    }
    fn pop_back(&self) {
        self.with_mut(|__v| __v.vec_.pop());
        return;
    }
    fn push_back(&self, item: Ptr<u8>) {
        {
            let a0_clone = (item.read()).clone();
            self.with_mut(|__v| __v.vec_.push(a0_clone))
        };
    }
}
impl Clone for MyContainer_char_ {
    fn clone(&self) -> Self {
        let mut this = Self {
            vec_: (self.vec_).clone(),
        };
        this
    }
}
impl ByteRepr for MyContainer_char_ {
    fn byte_size() -> usize {
        24
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.vec_.to_bytes(&mut buf[0..24]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            vec_: <Vec<u8>>::from_bytes(&buf[0..24]),
        }
    }
}
#[repr(C)]
#[derive(Default)]
pub struct MyContainer_float_ {
    vec_: Vec<f32>,
}
pub trait MyContainer_float_Methods {
    fn empty(&self) -> bool;
    fn size(&self) -> usize;
    fn back_const(&self) -> Ptr<f32>;
    fn back(&self) -> Ptr<f32>;
    fn pop_back(&self);
    fn push_back(&self, item: Ptr<f32>);
}
impl MyContainer_float_Methods for Ptr<MyContainer_float_> {
    fn empty(&self) -> bool {
        return self.with(|__v| (*__v).vec_.clone()).is_empty();
    }
    fn size(&self) -> usize {
        return self.with(|__v| (*__v).vec_.clone()).len();
    }
    fn back_const(&self) -> Ptr<f32> {
        return (self.field_ptr(
            0,
            |__v: &MyContainer_float_| &__v.vec_[..],
            |__v: &mut MyContainer_float_| &mut __v.vec_[..],
        ) as Ptr<f32>)
            .to_last();
    }
    fn back(&self) -> Ptr<f32> {
        return (self.field_ptr(
            0,
            |__v: &MyContainer_float_| &__v.vec_[..],
            |__v: &mut MyContainer_float_| &mut __v.vec_[..],
        ) as Ptr<f32>)
            .to_last();
    }
    fn pop_back(&self) {
        self.with_mut(|__v| __v.vec_.pop());
        return;
    }
    fn push_back(&self, item: Ptr<f32>) {
        {
            let a0_clone = (item.read()).clone();
            self.with_mut(|__v| __v.vec_.push(a0_clone))
        };
    }
}
impl Clone for MyContainer_float_ {
    fn clone(&self) -> Self {
        let mut this = Self {
            vec_: (self.vec_).clone(),
        };
        this
    }
}
impl ByteRepr for MyContainer_float_ {
    fn byte_size() -> usize {
        24
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.vec_.to_bytes(&mut buf[0..24]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            vec_: <Vec<f32>>::from_bytes(&buf[0..24]),
        }
    }
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let imc: Value<MyContainer_int_> = Rc::new(RefCell::new(<MyContainer_int_>::default()));
    assert!(({ imc.as_pointer().empty() }));
    ({
        let _item: Value<i32> = Rc::new(RefCell::new(1));
        imc.as_pointer().push_back(_item.as_pointer())
    });
    assert!(
        (({ imc.as_pointer().size() }) == 1_usize) && ((({ imc.as_pointer().back() }).read()) == 1)
    );
    ({ imc.as_pointer().pop_back() });
    assert!(({ imc.as_pointer().empty() }));
    let cmc: Value<MyContainer_char_> = Rc::new(RefCell::new(<MyContainer_char_>::default()));
    assert!(({ cmc.as_pointer().empty() }));
    ({
        let _item: Value<u8> = Rc::new(RefCell::new(('a' as u8)));
        cmc.as_pointer().push_back(_item.as_pointer())
    });
    assert!(
        (({ cmc.as_pointer().size() }) == 1_usize)
            && (((({ cmc.as_pointer().back() }).read()) as i32) == (('a' as u8) as i32))
    );
    ({ cmc.as_pointer().pop_back() });
    assert!(({ cmc.as_pointer().empty() }));
    let fmc: Value<MyContainer_float_> = Rc::new(RefCell::new(<MyContainer_float_>::default()));
    assert!(({ fmc.as_pointer().empty() }));
    ({
        let _item: Value<f32> = Rc::new(RefCell::new((1.0E+0 as f32)));
        fmc.as_pointer().push_back(_item.as_pointer())
    });
    assert!(
        (({ fmc.as_pointer().size() }) == 1_usize)
            && (((({ fmc.as_pointer().back() }).read()) as f64) == 1.0E+0)
    );
    ({ fmc.as_pointer().pop_back() });
    assert!(({ fmc.as_pointer().empty() }));
    return 0;
}
