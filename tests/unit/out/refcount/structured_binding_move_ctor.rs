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
#[derive(Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, Default)]
#[byte_size(8)]
pub struct Movable {
    #[offset(0)]
    pub x: i32,
    #[offset(4)]
    pub y: i32,
}
impl Movable {
    pub fn new(mut x: i32, mut y: i32) -> Self {
        Self { x: x, y: y }
    }
    pub fn copy_from(other: Ptr<Movable>) -> Self {
        let __this: Movable = Self {
            x: other.with(|__s| __s.x),
            y: other.with(|__s| __s.y),
        };
        (*copies_0.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
    pub fn move_from(other: Ptr<Movable>) -> Self {
        let __this: Movable = Self {
            x: other.with(|__s| __s.x),
            y: other.with(|__s| __s.y),
        };
        field!(other, x).write(0);
        field!(other, y).write(0);
        (*moves_1.with(Value::clone).borrow_mut()).prefix_inc();
        __this
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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(48)]
pub struct Holder {
    #[offset(0)]
    #[byte_size(24)]
    pub xs: Vec<i32>,
    #[offset(24)]
    #[byte_size(24)]
    pub ys: Vec<i32>,
}
impl Holder {
    pub fn move_from(_a0: Ptr<Holder>) -> Self {
        Self {
            xs: field!(_a0, xs).with_mut(std::mem::take),
            ys: field!(_a0, ys).with_mut(std::mem::take),
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
    assert!(({ (*__decomp_2.borrow()).x } == 3));
    assert!(({ (*__decomp_2.borrow()).y } == 4));
    assert!(({ (*m.borrow()).x } == 0));
    assert!(({ (*m.borrow()).y } == 0));
    let h: Value<Holder> = Rc::new(RefCell::new(Holder {
        xs: Vec::from([1, 2, 3]),
        ys: Vec::from([4, 5]),
    }));
    let __decomp_3: Value<Holder> =
        Rc::new(RefCell::new(({ HolderImpl::extract(&h.as_pointer()) })));
    assert!(((*__decomp_3.borrow()).xs.len() == 3_usize));
    assert!(((*__decomp_3.borrow()).ys.len() == 2_usize));
    assert!(((elem!(field_ref!(__decomp_3.as_pointer(), xs).decay(), 2_usize).read()) == 3));
    assert!(((elem!(field_ref!(__decomp_3.as_pointer(), ys).decay(), 0_usize).read()) == 4));
    assert!((*h.borrow()).xs.is_empty());
    assert!((*h.borrow()).ys.is_empty());
    return 0;
}
pub trait HolderImpl {
    fn extract(&self) -> Holder;
    fn move_assign(&self, _a0: Ptr<Holder>) -> Ptr<Holder>;
}
impl HolderImpl for Ptr<Holder> {
    fn extract(&self) -> Holder {
        return Holder {
            xs: field!((*self), xs).with_mut(std::mem::take),
            ys: field!((*self), ys).with_mut(std::mem::take),
        };
    }
    fn move_assign(&self, _a0: Ptr<Holder>) -> Ptr<Holder> {
        field_ref!((*self), xs)
            .as_pointer()
            .write(field!(_a0, xs).with_mut(std::mem::take));
        field_ref!((*self), ys)
            .as_pointer()
            .write(field!(_a0, ys).with_mut(std::mem::take));
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = copies_0.with(|_| ());
    let _ = moves_1.with(|_| ());
}
