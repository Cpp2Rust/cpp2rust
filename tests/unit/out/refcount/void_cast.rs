extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn unused_param_0(x: i32) {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    (*x.borrow_mut());
}
thread_local!(
    pub static side_effect_counter: Value<i32> = Rc::new(RefCell::new(0));
);
pub fn bump_and_return_1() -> i32 {
    (*side_effect_counter.with(Value::clone).borrow_mut()).prefix_inc();
    return (*side_effect_counter.with(Value::clone).borrow());
}
#[derive(Default)]
pub struct Holder {
    pub field: Value<i32>,
}
impl Clone for Holder {
    fn clone(&self) -> Self {
        let mut this = Self {
            field: Rc::new(RefCell::new((*self.field.borrow()))),
        };
        this
    }
}
impl ByteRepr for Holder {}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    ({
        let _x: i32 = 42;
        unused_param_0(_x)
    });
    let y: Value<i32> = Rc::new(RefCell::new(5));
    (*y.borrow_mut());
    let z: Value<i32> = Rc::new(RefCell::new({
        (*y.borrow_mut());
        7
    }));
    assert!(((*z.borrow()) == 7));
    let counter: Value<i32> = Rc::new(RefCell::new(0));
    let w: Value<i32> = Rc::new(RefCell::new({
        (*counter.borrow_mut());
        (*counter.borrow_mut()) = 3;
        (*counter.borrow())
    }));
    assert!(((*w.borrow()) == 3));
    assert!(((*counter.borrow()) == 3));
    ({ bump_and_return_1() });
    assert!(((*side_effect_counter.with(Value::clone).borrow()) == 1));
    let v: Value<i32> = Rc::new(RefCell::new({
        ({ bump_and_return_1() });
        99
    }));
    assert!(((*side_effect_counter.with(Value::clone).borrow()) == 2));
    assert!(((*v.borrow()) == 99));
    0;
    (0);
    (*y.borrow_mut());
    (0);
    (*y.borrow_mut());
    let err: Value<i32> = Rc::new(RefCell::new(0));
    ((*err.borrow_mut()) = 42);
    assert!(((*err.borrow()) == 42));
    let chosen: Value<i32> = Rc::new(RefCell::new({
        ((*err.borrow_mut()) = 7);
        123
    }));
    assert!(((*err.borrow()) == 7));
    assert!(((*chosen.borrow()) == 123));
    bump_and_return_1;
    assert!(((*side_effect_counter.with(Value::clone).borrow()) == 2));
    (FnPtr::<fn() -> i32>::new(bump_and_return_1));
    assert!(((*side_effect_counter.with(Value::clone).borrow()) == 2));
    ((FnPtr::<fn() -> i32>::new(bump_and_return_1)).cast::<fn() -> i32>(None));
    assert!(((*side_effect_counter.with(Value::clone).borrow()) == 2));
    let storage: Value<i32> = Rc::new(RefCell::new(11));
    let p: Value<Ptr<i32>> = Rc::new(RefCell::new((storage.as_pointer())));
    ((*p.borrow()).read());
    (*p.borrow_mut()).clone();
    let arr: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3])));
    ((*arr.borrow_mut())[(1) as usize]);
    let h: Value<Holder> = Rc::new(RefCell::new(Holder {
        field: Rc::new(RefCell::new(17)),
    }));
    (*(*h.borrow()).field.borrow_mut());
    let hp: Value<Ptr<Holder>> = Rc::new(RefCell::new((h.as_pointer())));
    (*(*(*hp.borrow()).upgrade().deref()).field.borrow_mut());
    return 0;
}
