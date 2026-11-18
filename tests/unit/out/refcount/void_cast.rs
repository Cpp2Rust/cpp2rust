extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn unused_param_0(mut x: i32) {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(24)]
pub struct NonTrivial {
    #[offset(0)]
    #[byte_size(24)]
    pub data: Vec<i32>,
}
impl NonTrivial {
    pub fn move_from(_a0: Ptr<NonTrivial>) -> Self {
        Self {
            data: field!(_a0, data).with_mut(std::mem::take),
        }
    }
}
pub fn unused_ref_param_1(x: Ptr<NonTrivial>) {}
pub fn unused_ptr_param_2(mut p: Ptr<NonTrivial>) {}
thread_local!(
    pub static side_effect_counter_3: Value<i32> = Rc::new(RefCell::new(0));
);
pub fn bump_and_return_4() -> i32 {
    (*side_effect_counter_3.with(Value::clone).borrow_mut()).prefix_inc();
    return side_effect_counter_3.with(|rc| *rc.borrow());
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Holder {
    #[offset(0)]
    pub field: i32,
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(8)]
pub struct NonCopyable {
    #[offset(0)]
    #[byte_size(8)]
    pub value: Option<Value<i32>>,
}
impl NonCopyable {
    pub fn move_from(_a0: Ptr<NonCopyable>) -> Self {
        Self {
            value: field!(_a0, value).with_mut(|__v: &mut Option<Value<i32>>| __v.take()),
        }
    }
}
pub fn unused_noncopyable_param_5(x: Ptr<NonCopyable>) {}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    ({ unused_param_0(42) });
    let mut y: i32 = 5;
    let mut z: i32 = {
        7
    };
    assert!((z == 7));
    let counter: Value<i32> = Rc::new(RefCell::new(0));
    let mut w: i32 = {
        {
            (*counter.borrow_mut()) = 3
        };
        (*counter.borrow())
    };
    assert!((w == 3));
    assert!(({ ((*counter.borrow()) == 3) }));
    &({ bump_and_return_4() });
    assert!((side_effect_counter_3.with(|rc| *rc.borrow()) == 1));
    let mut v: i32 = {
        &({ bump_and_return_4() });
        99
    };
    assert!((side_effect_counter_3.with(|rc| *rc.borrow()) == 2));
    assert!((v == 99));
    ();
    ();
    let mut err: i32 = 0;
    (&(err = 42));
    assert!((err == 42));
    let mut chosen: i32 = {
        &(err = 7);
        123
    };
    assert!((err == 7));
    assert!((chosen == 123));
    assert!((side_effect_counter_3.with(|rc| *rc.borrow()) == 2));
    assert!((side_effect_counter_3.with(|rc| *rc.borrow()) == 2));
    assert!((side_effect_counter_3.with(|rc| *rc.borrow()) == 2));
    let storage: Value<i32> = Rc::new(RefCell::new(11));
    let mut p: Ptr<i32> = (storage.as_pointer());
    let mut arr: [i32; 3] = [1, 2, 3];
    let h: Value<Holder> = Rc::new(RefCell::new(Holder { field: 17 }));
    let mut hp: Ptr<Holder> = (h.as_pointer());
    let nt: Value<NonTrivial> = Rc::new(RefCell::new(<NonTrivial>::default()));
    ({ unused_ref_param_1(nt.as_pointer()) });
    ({ unused_ptr_param_2((nt.as_pointer())) });
    let g: Value<NonCopyable> = Rc::new(RefCell::new(NonCopyable {
        value: Some(Rc::new(RefCell::new(9))),
    }));
    ();
    ({ unused_noncopyable_param_5(g.as_pointer()) });
    assert!(((({ { (*g.borrow()).value.clone() } }).as_pointer().read()) == 9));
    return 0;
}
pub trait NonCopyableImpl {
    fn move_assign(&self, _a0: Ptr<NonCopyable>) -> Ptr<NonCopyable>;
}
impl NonCopyableImpl for Ptr<NonCopyable> {
    fn move_assign(&self, _a0: Ptr<NonCopyable>) -> Ptr<NonCopyable> {
        field_ptr!((*self), value)
            .write(field!(_a0, value).with_mut(|__v: &mut Option<Value<i32>>| __v.take()));
        return (*self).clone();
    }
}
pub trait NonTrivialImpl {
    fn move_assign(&self, _a0: Ptr<NonTrivial>) -> Ptr<NonTrivial>;
}
impl NonTrivialImpl for Ptr<NonTrivial> {
    fn move_assign(&self, _a0: Ptr<NonTrivial>) -> Ptr<NonTrivial> {
        field_ref!((*self), data)
            .as_pointer()
            .write(field!(_a0, data).with_mut(std::mem::take));
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = side_effect_counter_3.with(|_| ());
}
