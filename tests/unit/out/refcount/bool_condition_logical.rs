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
pub fn returns_one_1() -> i32 {
    return 1;
}
pub fn returns_zero_2() -> i32 {
    return 0;
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let n: Value<i32> = Rc::new(RefCell::new(3));
    let zero: Value<i32> = Rc::new(RefCell::new(0));
    let storage: Value<i32> = Rc::new(RefCell::new(7));
    let p: Value<Ptr<i32>> = Rc::new(RefCell::new((storage.as_pointer())));
    let np: Value<Ptr<i32>> = Rc::new(RefCell::new(Ptr::<i32>::null()));
    let u: Value<u32> = Rc::new(RefCell::new(4_u32));
    let code: Value<Code> = Rc::new(RefCell::new(Code::CODE_OK));
    if ((*n.borrow()) != 0) && (!(*p.borrow()).is_null()) {
        assert!(true);
    }
    if ((*n.borrow()) != 0) && (!(*np.borrow()).is_null()) {
        assert!(false);
    }
    if ((*zero.borrow()) != 0) || (!(*p.borrow()).is_null()) {
        assert!(true);
    }
    if ((*zero.borrow()) != 0) || (!(*np.borrow()).is_null()) {
        assert!(false);
    }
    if ((((*n.borrow()) != 0) && ((*u.borrow()) != 0)) && (!(*p.borrow()).is_null()))
        && (((*code.borrow()) as i32) == (Code::CODE_OK as i32))
    {
        assert!(true);
    }
    (*side_effect.with(Value::clone).borrow_mut()) = 0;
    if ((*zero.borrow()) != 0)
        && (({
            let _v: i32 = 1;
            observe_0(_v)
        }) != 0)
    {
        assert!(false);
    }
    assert!(((*side_effect.with(Value::clone).borrow()) == 0));
    if ((*n.borrow()) != 0)
        || (({
            let _v: i32 = 1;
            observe_0(_v)
        }) != 0)
    {
        assert!(true);
    }
    assert!(((*side_effect.with(Value::clone).borrow()) == 0));
    let x: Value<i32> = Rc::new(RefCell::new(5));
    let y: Value<i32> = Rc::new(RefCell::new(3));
    let flags: Value<u32> = Rc::new(RefCell::new(2));
    if ((*x.borrow()) > (*y.borrow())) || (((*flags.borrow()) & 1) != 0) {
        assert!(true);
    }
    if ((*x.borrow()) < (*y.borrow())) || (((*flags.borrow()) & 1) != 0) {
        assert!(false);
    }
    let a: Value<u32> = Rc::new(RefCell::new(1));
    let b: Value<u32> = Rc::new(RefCell::new(2));
    let c: Value<u32> = Rc::new(RefCell::new(3));
    if ((*a.borrow()) != (*c.borrow())) && ((*b.borrow()) != (*c.borrow())) {
        assert!(true);
    }
    let s: Value<i32> = Rc::new(RefCell::new(-1_i32));
    if (!((*p.borrow()).is_null())) && ((*s.borrow()) < 0) {
        assert!(true);
    }
    let k: Value<u32> = Rc::new(RefCell::new(2));
    let done: Value<bool> = Rc::new(RefCell::new(false));
    if ((*k.borrow()) > 1) || (!(*done.borrow())) {
        assert!(true);
    }
    if ((*x.borrow()) > (*y.borrow())) || (((*flags.borrow()) & 4) != 0) {
        assert!(true);
    }
    let ull: Value<u64> = Rc::new(RefCell::new(7));
    if (!((*p.borrow()).is_null())) && ((*ull.borrow()) != 0) {
        assert!(true);
    }
    if ((*x.borrow()) > (*y.borrow())) && ((*ull.borrow()) != 0) {
        assert!(true);
    }
    let mask: Value<i64> = Rc::new(RefCell::new(((1 << 4) | (1 << 5))));
    let bits: Value<i64> = Rc::new(RefCell::new((1 << 4)));
    if ((*n.borrow()) != 0) && (((*bits.borrow()) & (*mask.borrow())) != 0) {
        assert!(true);
    }
    if ((*n.borrow()) != 0) || (((*bits.borrow()) & 256) != 0) {
        assert!(true);
    }
    let cp: Value<Ptr<u8>> = Rc::new(RefCell::new(Ptr::from_string_literal("hi")));
    let cnp: Value<Ptr<u8>> = Rc::new(RefCell::new(Ptr::<u8>::null()));
    if ((*x.borrow()) > (*y.borrow())) && (!(*cp.borrow()).is_null()) {
        assert!(true);
    }
    if ((*x.borrow()) < (*y.borrow())) || (!(*cnp.borrow()).is_null()) {
        assert!(false);
    }
    if ((*x.borrow()) > (*y.borrow())) && (((*n.borrow()) != 0) && (!(*cp.borrow()).is_null())) {
        assert!(true);
    }
    if ((*x.borrow()) > (*y.borrow())) && (({ returns_one_1() }) != 0) {
        assert!(true);
    }
    if ((*x.borrow()) > (*y.borrow())) && (!(({ returns_zero_2() }) != 0)) {
        assert!(true);
    }
    if ((*x.borrow()) < (*y.borrow())) || (({ returns_one_1() }) != 0) {
        assert!(true);
    }
    if ((*x.borrow()) < (*y.borrow())) || (!(({ returns_one_1() }) != 0)) {
        assert!(false);
    }
    if ((!((*p.borrow()).is_null())) && (({ returns_one_1() }) != 0)) && ((*n.borrow()) != 0) {
        assert!(true);
    }
    return 0;
}
