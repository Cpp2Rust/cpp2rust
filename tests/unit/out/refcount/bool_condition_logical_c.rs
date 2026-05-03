extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Copy, PartialEq, Debug, Default)]
enum Code {
    #[default]
    CODE_OK = 0,
    CODE_ERR = 1,
    CODE_FATAL = 2,
}
impl From<i32> for Code {
    fn from(n: i32) -> Code {
        match n {
            0 => Code::CODE_OK,
            1 => Code::CODE_ERR,
            2 => Code::CODE_FATAL,
            _ => panic!("invalid Code value: {}", n),
        }
    }
}
thread_local!(
    pub static side_effect: Value<i32> = Rc::new(RefCell::new(0));
);
pub fn observe_0(v: i32) -> i32 {
    let v: Value<i32> = Rc::new(RefCell::new(v));
    (*side_effect.with(Value::clone).borrow_mut()).prefix_inc();
    return (*v.borrow());
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let n: Value<i32> = Rc::new(RefCell::new(3));
    let zero: Value<i32> = Rc::new(RefCell::new(0));
    let storage: Value<i32> = Rc::new(RefCell::new(7));
    let p: Value<Ptr<i32>> = Rc::new(RefCell::new((storage.as_pointer())));
    let np: Value<Ptr<i32>> = Rc::new(RefCell::new(Default::default()));
    let u: Value<u32> = Rc::new(RefCell::new(4_u32));
    let code: Value<Code> = Rc::new(RefCell::new(Code::from((Code::CODE_OK as i32))));
    if ((*n.borrow()) != 0) && (!(*p.borrow()).is_null()) {
        assert!((1 != 0));
    }
    if ((*n.borrow()) != 0) && (!(*np.borrow()).is_null()) {
        assert!((0 != 0));
    }
    if ((*zero.borrow()) != 0) || (!(*p.borrow()).is_null()) {
        assert!((1 != 0));
    }
    if ((*zero.borrow()) != 0) || (!(*np.borrow()).is_null()) {
        assert!((0 != 0));
    }
    if ((((*n.borrow()) != 0) && ((*u.borrow()) != 0)) && (!(*p.borrow()).is_null()))
        && (((*code.borrow()) as u32) == ((Code::CODE_OK as i32) as u32))
    {
        assert!((1 != 0));
    }
    (*side_effect.with(Value::clone).borrow_mut()) = 0;
    if ((*zero.borrow()) != 0)
        && (({
            let _v: i32 = 1;
            observe_0(_v)
        }) != 0)
    {
        assert!((0 != 0));
    }
    assert!(((*side_effect.with(Value::clone).borrow()) == 0));
    if ((*n.borrow()) != 0)
        || (({
            let _v: i32 = 1;
            observe_0(_v)
        }) != 0)
    {
        assert!((1 != 0));
    }
    assert!(((*side_effect.with(Value::clone).borrow()) == 0));
    let x: Value<i32> = Rc::new(RefCell::new(5));
    let y: Value<i32> = Rc::new(RefCell::new(3));
    let flags: Value<u32> = Rc::new(RefCell::new(2_u32));
    if ((*x.borrow()) > (*y.borrow())) || (((*flags.borrow()) & 1_u32) != 0) {
        assert!((1 != 0));
    }
    if ((*x.borrow()) < (*y.borrow())) || (((*flags.borrow()) & 1_u32) != 0) {
        assert!((0 != 0));
    }
    let a: Value<u32> = Rc::new(RefCell::new(1_u32));
    let b: Value<u32> = Rc::new(RefCell::new(2_u32));
    let c: Value<u32> = Rc::new(RefCell::new(3_u32));
    if ((*a.borrow()) != (*c.borrow())) && ((*b.borrow()) != (*c.borrow())) {
        assert!((1 != 0));
    }
    let s: Value<i32> = Rc::new(RefCell::new(-1_i32));
    if ((*p.borrow()) != (Default::default())) && ((*s.borrow()) < 0) {
        assert!((1 != 0));
    }
    let k: Value<u32> = Rc::new(RefCell::new(2_u32));
    let done: Value<bool> = Rc::new(RefCell::new((0 != 0)));
    if ((*k.borrow()) > 1_u32) || (!(*done.borrow())) {
        assert!((1 != 0));
    }
    if ((*x.borrow()) > (*y.borrow())) || (((*flags.borrow()) & 4_u32) != 0) {
        assert!((1 != 0));
    }
    return 0;
}
