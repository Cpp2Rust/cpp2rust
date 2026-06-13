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
impl MyContainer_int_ {
    pub fn empty(&self) -> bool {
        return (*self.vec_.borrow()).is_empty();
    }
    pub fn size(&self) -> usize {
        return (*self.vec_.borrow()).len();
    }
    pub fn back_const(&self) -> Ptr<i32> {
        return (self.vec_.as_pointer() as Ptr<i32>).to_last();
    }
    pub fn back(&self) -> Ptr<i32> {
        return (self.vec_.as_pointer() as Ptr<i32>).to_last();
    }
    pub fn pop_back(&self) {
        (*self.vec_.borrow_mut()).pop();
        return;
    }
    pub fn push_back(&self, item: Ptr<i32>) {
        {
            let a0_clone = (item.read()).clone();
            (*self.vec_.borrow_mut()).push(a0_clone)
        };
    }
}
impl Clone for MyContainer_int_ {
    fn clone(&self) -> Self {
        let mut this = Self {
            vec_: Rc::new(RefCell::new((*self.vec_.borrow()).clone())),
        };
        this
    }
}
impl ByteRepr for MyContainer_int_ {}
#[derive(Default)]
pub struct MyContainer_char_ {
    vec_: Value<Vec<u8>>,
}
impl MyContainer_char_ {
    pub fn empty(&self) -> bool {
        return (*self.vec_.borrow()).is_empty();
    }
    pub fn size(&self) -> usize {
        return (*self.vec_.borrow()).len();
    }
    pub fn back_const(&self) -> Ptr<u8> {
        return (self.vec_.as_pointer() as Ptr<u8>).to_last();
    }
    pub fn back(&self) -> Ptr<u8> {
        return (self.vec_.as_pointer() as Ptr<u8>).to_last();
    }
    pub fn pop_back(&self) {
        (*self.vec_.borrow_mut()).pop();
        return;
    }
    pub fn push_back(&self, item: Ptr<u8>) {
        {
            let a0_clone = (item.read()).clone();
            (*self.vec_.borrow_mut()).push(a0_clone)
        };
    }
}
impl Clone for MyContainer_char_ {
    fn clone(&self) -> Self {
        let mut this = Self {
            vec_: Rc::new(RefCell::new((*self.vec_.borrow()).clone())),
        };
        this
    }
}
impl ByteRepr for MyContainer_char_ {}
#[derive(Default)]
pub struct MyContainer_float_ {
    vec_: Value<Vec<f32>>,
}
impl MyContainer_float_ {
    pub fn empty(&self) -> bool {
        return (*self.vec_.borrow()).is_empty();
    }
    pub fn size(&self) -> usize {
        return (*self.vec_.borrow()).len();
    }
    pub fn back_const(&self) -> Ptr<f32> {
        return (self.vec_.as_pointer() as Ptr<f32>).to_last();
    }
    pub fn back(&self) -> Ptr<f32> {
        return (self.vec_.as_pointer() as Ptr<f32>).to_last();
    }
    pub fn pop_back(&self) {
        (*self.vec_.borrow_mut()).pop();
        return;
    }
    pub fn push_back(&self, item: Ptr<f32>) {
        {
            let a0_clone = (item.read()).clone();
            (*self.vec_.borrow_mut()).push(a0_clone)
        };
    }
}
impl Clone for MyContainer_float_ {
    fn clone(&self) -> Self {
        let mut this = Self {
            vec_: Rc::new(RefCell::new((*self.vec_.borrow()).clone())),
        };
        this
    }
}
impl ByteRepr for MyContainer_float_ {}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let imc: Value<MyContainer_int_> = Rc::new(RefCell::new(<MyContainer_int_>::default()));
    assert!(({ (*imc.borrow()).empty() }));
    ({
        let _item: Value<i32> = Rc::new(RefCell::new(1));
        (*imc.borrow()).push_back(_item.as_pointer())
    });
    assert!(
        (({ (*imc.borrow()).size() }) == 1_usize) && ((({ (*imc.borrow()).back() }).read()) == 1)
    );
    ({ (*imc.borrow()).pop_back() });
    assert!(({ (*imc.borrow()).empty() }));
    let cmc: Value<MyContainer_char_> = Rc::new(RefCell::new(<MyContainer_char_>::default()));
    assert!(({ (*cmc.borrow()).empty() }));
    ({
        let _item: Value<u8> = Rc::new(RefCell::new(('a' as u8)));
        (*cmc.borrow()).push_back(_item.as_pointer())
    });
    assert!(
        (({ (*cmc.borrow()).size() }) == 1_usize)
            && (((({ (*cmc.borrow()).back() }).read()) as i32) == (('a' as u8) as i32))
    );
    ({ (*cmc.borrow()).pop_back() });
    assert!(({ (*cmc.borrow()).empty() }));
    let fmc: Value<MyContainer_float_> = Rc::new(RefCell::new(<MyContainer_float_>::default()));
    assert!(({ (*fmc.borrow()).empty() }));
    ({
        let _item: Value<f32> = Rc::new(RefCell::new((('a' as u8) as f32)));
        (*fmc.borrow()).push_back(_item.as_pointer())
    });
    assert!(
        (({ (*fmc.borrow()).size() }) == 1_usize)
            && ((({ (*fmc.borrow()).back() }).read()) == ((('a' as u8) as i32) as f32))
    );
    ({ (*fmc.borrow()).pop_back() });
    assert!(({ (*fmc.borrow()).empty() }));
    return 0;
}
