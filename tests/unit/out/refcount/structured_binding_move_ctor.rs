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
#[derive(Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Movable {
    #[offset(0)]
    pub x: i32,
    #[offset(4)]
    pub y: i32,
}
impl Movable {
    pub fn new(x: i32, y: i32) -> Self {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        let y: Value<i32> = Rc::new(RefCell::new(y));
        let __this: Value<Movable> = Rc::new(RefCell::new(Self {
            x: (*x.borrow()),
            y: (*y.borrow()),
        }));
        let this: Ptr<Movable> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn copy_from(other: Ptr<Movable>) -> Self {
        let __this: Value<Movable> = Rc::new(RefCell::new(Self {
            x: other.with(|__s| __s.x),
            y: other.with(|__s| __s.y),
        }));
        let this: Ptr<Movable> = __this.as_pointer();
        (*copies_0.with(Value::clone).borrow_mut()).prefix_inc();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn move_from(other: Ptr<Movable>) -> Self {
        let __this: Value<Movable> = Rc::new(RefCell::new(Self {
            x: other.with(|__s| __s.x),
            y: other.with(|__s| __s.y),
        }));
        let this: Ptr<Movable> = __this.as_pointer();
        field!(other, x).write(0);
        field!(other, y).write(0);
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
#[derive(DeepClone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(48)]
pub struct Holder {
    #[offset(0)]
    #[byte_size(24)]
    pub xs: Value<Vec<i32>>,
    #[offset(24)]
    #[byte_size(24)]
    pub ys: Value<Vec<i32>>,
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
        xs: Rc::new(RefCell::new(vec![1, 2, 3])),
        ys: Rc::new(RefCell::new(vec![4, 5])),
    }));
    let __decomp_3: Value<Holder> =
        Rc::new(RefCell::new(({ HolderImpl::extract(&h.as_pointer()) })));
    assert!(((*{ (*__decomp_3.borrow()).xs.clone() }.borrow()).len() == 3_usize));
    assert!(((*{ (*__decomp_3.borrow()).ys.clone() }.borrow()).len() == 2_usize));
    assert!(
        ((elem!(
            ({ (*__decomp_3.borrow()).xs.as_pointer() } as Ptr<i32>),
            2_usize
        )
        .read())
            == 3)
    );
    assert!(
        ((elem!(
            ({ (*__decomp_3.borrow()).ys.as_pointer() } as Ptr<i32>),
            0_usize
        )
        .read())
            == 4)
    );
    assert!((*{ (*h.borrow()).xs.clone() }.borrow()).is_empty());
    assert!((*{ (*h.borrow()).ys.clone() }.borrow()).is_empty());
    return 0;
}
pub trait HolderImpl {
    fn extract(&self) -> Holder;
}
impl HolderImpl for Ptr<Holder> {
    fn extract(&self) -> Holder {
        return Holder {
            xs: Rc::new(RefCell::new(std::mem::take(
                &mut (*(*self).with(|__s| __s.xs.clone()).borrow_mut()),
            ))),
            ys: Rc::new(RefCell::new(std::mem::take(
                &mut (*(*self).with(|__s| __s.ys.clone()).borrow_mut()),
            ))),
        };
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = copies_0.with(|_| ());
    let _ = moves_1.with(|_| ());
}
