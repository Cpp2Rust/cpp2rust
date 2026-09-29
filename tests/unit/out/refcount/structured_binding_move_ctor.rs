extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static copies_0: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static moves_1: Value<i32> = Rc::new(RefCell::new(0));
);
#[derive(VaArg, FnPtrArg, Default)]
pub struct Movable {
    pub x: Value<i32>,
    pub y: Value<i32>,
}
impl Movable {
    pub fn new(x: i32, y: i32) -> Self {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        let y: Value<i32> = Rc::new(RefCell::new(y));
        let __this: Value<Movable> = Rc::new(RefCell::new(Self {
            x: Rc::new(RefCell::new((*x.borrow()))),
            y: Rc::new(RefCell::new((*y.borrow()))),
        }));
        let this: Ptr<Movable> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn copy_from(other: Ptr<Movable>) -> Self {
        let __this: Value<Movable> = Rc::new(RefCell::new(Self {
            x: Rc::new(RefCell::new((*(*other.upgrade().deref()).x.borrow()))),
            y: Rc::new(RefCell::new((*(*other.upgrade().deref()).y.borrow()))),
        }));
        let this: Ptr<Movable> = __this.as_pointer();
        (*copies_0.with(Value::clone).borrow_mut()).prefix_inc();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn move_from(other: Ptr<Movable>) -> Self {
        let __this: Value<Movable> = Rc::new(RefCell::new(Self {
            x: Rc::new(RefCell::new((*(*other.upgrade().deref()).x.borrow()))),
            y: Rc::new(RefCell::new((*(*other.upgrade().deref()).y.borrow()))),
        }));
        let this: Ptr<Movable> = __this.as_pointer();
        (*(*other.upgrade().deref()).x.borrow_mut()) = 0;
        (*(*other.upgrade().deref()).y.borrow_mut()) = 0;
        (*moves_1.with(Value::clone).borrow_mut()).prefix_inc();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for Movable {
    fn clone(&self) -> Self {
        let __src: Value<Movable> = Rc::new(RefCell::new(Movable {
            x: self.x.clone(),
            y: self.y.clone(),
        }));
        Movable::copy_from(__src.as_pointer())
    }
}
impl ByteRepr for Movable {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.x.borrow()).to_bytes(&mut buf[0..4]);
        (*self.y.borrow()).to_bytes(&mut buf[4..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            x: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
            y: Rc::new(RefCell::new(<i32>::from_bytes(&buf[4..8]))),
        }
    }
}
#[derive(VaArg, FnPtrArg, Default)]
pub struct Holder {
    pub xs: Value<Vec<i32>>,
    pub ys: Value<Vec<i32>>,
}
impl Clone for Holder {
    fn clone(&self) -> Self {
        let __this: Value<Holder> = Rc::new(RefCell::new(Self {
            xs: Rc::new(RefCell::new((*self.xs.borrow()).clone())),
            ys: Rc::new(RefCell::new((*self.ys.borrow()).clone())),
        }));
        let this: Ptr<Holder> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for Holder {
    fn byte_size() -> usize {
        48
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.xs.borrow()).to_bytes(&mut buf[0..24]);
        (*self.ys.borrow()).to_bytes(&mut buf[24..48]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            xs: Rc::new(RefCell::new(<Vec<i32>>::from_bytes(&buf[0..24]))),
            ys: Rc::new(RefCell::new(<Vec<i32>>::from_bytes(&buf[24..48]))),
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let m: Value<Movable> = Rc::new(RefCell::new(Movable::new({ 3 }, { 4 })));
    let __decomp_2: Value<Movable> = Rc::new(RefCell::new(Movable::move_from({ m.as_pointer() })));
    assert!((moves_1.with(|rc| *rc.borrow()) == 1));
    assert!((copies_0.with(|rc| *rc.borrow()) == 0));
    assert!(((*(*__decomp_2.borrow()).x.borrow()) == 3));
    assert!(((*(*__decomp_2.borrow()).y.borrow()) == 4));
    assert!(((*(*m.borrow()).x.borrow()) == 0));
    assert!(((*(*m.borrow()).y.borrow()) == 0));
    let h: Value<Holder> = Rc::new(RefCell::new(Holder {
        xs: Rc::new(RefCell::new(vec![1, 2, 3])),
        ys: Rc::new(RefCell::new(vec![4, 5])),
    }));
    let __decomp_3: Value<Holder> =
        Rc::new(RefCell::new(({ HolderImpl::extract(&h.as_pointer()) })));
    assert!(((*(*__decomp_3.borrow()).xs.borrow()).len() == 3_usize));
    assert!(((*(*__decomp_3.borrow()).ys.borrow()).len() == 2_usize));
    assert!(
        ((((*__decomp_3.borrow()).xs.as_pointer() as Ptr<i32>)
            .offset(2_usize)
            .read())
            == 3)
    );
    assert!(
        ((((*__decomp_3.borrow()).ys.as_pointer() as Ptr<i32>)
            .offset(0_usize)
            .read())
            == 4)
    );
    assert!((*(*h.borrow()).xs.borrow()).is_empty());
    assert!((*(*h.borrow()).ys.borrow()).is_empty());
    return 0;
}
pub trait HolderImpl {
    fn extract(&self) -> Holder;
}
impl HolderImpl for Ptr<Holder> {
    fn extract(&self) -> Holder {
        return Holder {
            xs: Rc::new(RefCell::new(std::mem::take(
                &mut (*(*(*self).upgrade().deref()).xs.borrow_mut()),
            ))),
            ys: Rc::new(RefCell::new(std::mem::take(
                &mut (*(*(*self).upgrade().deref()).ys.borrow_mut()),
            ))),
        };
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = copies_0.with(|_| ());
    let _ = moves_1.with(|_| ());
}
