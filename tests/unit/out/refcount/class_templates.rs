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
pub trait MyContainer_int_Impl {
    fn empty(&self) -> bool;
    fn size(&self) -> usize;
    fn back(&self) -> Ptr<i32>;
    fn pop_back(&self);
    fn push_back(&self, item: Ptr<i32>);
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
pub trait MyContainer_char_Impl {
    fn empty(&self) -> bool;
    fn size(&self) -> usize;
    fn back(&self) -> Ptr<u8>;
    fn pop_back(&self);
    fn push_back(&self, item: Ptr<u8>);
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
pub trait MyContainer_float_Impl {
    fn empty(&self) -> bool;
    fn size(&self) -> usize;
    fn back(&self) -> Ptr<f32>;
    fn pop_back(&self);
    fn push_back(&self, item: Ptr<f32>);
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
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let imc: Value<MyContainer_int_> = Rc::new(RefCell::new(<MyContainer_int_>::default()));
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
    let cmc: Value<MyContainer_char_> = Rc::new(RefCell::new(<MyContainer_char_>::default()));
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
    let fmc: Value<MyContainer_float_> = Rc::new(RefCell::new(<MyContainer_float_>::default()));
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
