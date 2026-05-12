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
    if (((((*n.borrow()) != 0) && (!(*p.borrow()).is_null())) as i32) != 0) {
        assert!((1 != 0));
    }
    if (((((*n.borrow()) != 0) && (!(*np.borrow()).is_null())) as i32) != 0) {
        assert!((0 != 0));
    }
    if (((((*zero.borrow()) != 0) || (!(*p.borrow()).is_null())) as i32) != 0) {
        assert!((1 != 0));
    }
    if (((((*zero.borrow()) != 0) || (!(*np.borrow()).is_null())) as i32) != 0) {
        assert!((0 != 0));
    }
    if (((((((((((*n.borrow()) != 0) && ((*u.borrow()) != 0)) as i32) != 0)
        && (!(*p.borrow()).is_null())) as i32)
        != 0)
        && (((((*code.borrow()) as u32) == ((Code::CODE_OK as i32) as u32)) as i32) != 0))
        as i32)
        != 0)
    {
        assert!((1 != 0));
    }
    (*side_effect.with(Value::clone).borrow_mut()) = 0;
    if (((((*zero.borrow()) != 0)
        && (({
            let _v: i32 = 1;
            observe_0(_v)
        }) != 0)) as i32)
        != 0)
    {
        assert!((0 != 0));
    }
    assert!(((((*side_effect.with(Value::clone).borrow()) == 0) as i32) != 0));
    if (((((*n.borrow()) != 0)
        || (({
            let _v: i32 = 1;
            observe_0(_v)
        }) != 0)) as i32)
        != 0)
    {
        assert!((1 != 0));
    }
    assert!(((((*side_effect.with(Value::clone).borrow()) == 0) as i32) != 0));
    let x: Value<i32> = Rc::new(RefCell::new(5));
    let y: Value<i32> = Rc::new(RefCell::new(3));
    let flags: Value<u32> = Rc::new(RefCell::new(2_u32));
    if (((((((*x.borrow()) > (*y.borrow())) as i32) != 0) || (((*flags.borrow()) & 1_u32) != 0))
        as i32)
        != 0)
    {
        assert!((1 != 0));
    }
    if (((((((*x.borrow()) < (*y.borrow())) as i32) != 0) || (((*flags.borrow()) & 1_u32) != 0))
        as i32)
        != 0)
    {
        assert!((0 != 0));
    }
    let a: Value<u32> = Rc::new(RefCell::new(1_u32));
    let b: Value<u32> = Rc::new(RefCell::new(2_u32));
    let c: Value<u32> = Rc::new(RefCell::new(3_u32));
    if (((((((*a.borrow()) != (*c.borrow())) as i32) != 0)
        && ((((*b.borrow()) != (*c.borrow())) as i32) != 0)) as i32)
        != 0)
    {
        assert!((1 != 0));
    }
    let s: Value<i32> = Rc::new(RefCell::new(-1_i32));
    if ((((((!((*p.borrow()).is_null())) as i32) != 0) && ((((*s.borrow()) < 0) as i32) != 0))
        as i32)
        != 0)
    {
        assert!((1 != 0));
    }
    let k: Value<u32> = Rc::new(RefCell::new(2_u32));
    let done: Value<bool> = Rc::new(RefCell::new((0 != 0)));
    if (((((((*k.borrow()) > 1_u32) as i32) != 0) || (!(*done.borrow()))) as i32) != 0) {
        assert!((1 != 0));
    }
    if (((((((*x.borrow()) > (*y.borrow())) as i32) != 0) || (((*flags.borrow()) & 4_u32) != 0))
        as i32)
        != 0)
    {
        assert!((1 != 0));
    }
    let ull: Value<u64> = Rc::new(RefCell::new(7_u64));
    if ((((((!((*p.borrow()).is_null())) as i32) != 0) && ((*ull.borrow()) != 0)) as i32) != 0) {
        assert!((1 != 0));
    }
    if (((((((*x.borrow()) > (*y.borrow())) as i32) != 0) && ((*ull.borrow()) != 0)) as i32) != 0) {
        assert!((1 != 0));
    }
    let mask: Value<i64> = Rc::new(RefCell::new(((1_i64 << 4) | (1_i64 << 5))));
    let bits: Value<i64> = Rc::new(RefCell::new((1_i64 << 4)));
    if (((((((*n.borrow()) != 0) as i32) != 0) && (((*bits.borrow()) & (*mask.borrow())) != 0))
        as i32)
        != 0)
    {
        assert!((1 != 0));
    }
    if (((((((*n.borrow()) != 0) as i32) != 0) || (((*bits.borrow()) & 256_i64) != 0)) as i32) != 0)
    {
        assert!((1 != 0));
    }
    let cp: Value<Ptr<u8>> = Rc::new(RefCell::new(Ptr::from_string_literal("hi")));
    let cnp: Value<Ptr<u8>> = Rc::new(RefCell::new(Ptr::<u8>::null()));
    if (((((((*x.borrow()) > (*y.borrow())) as i32) != 0) && (!(*cp.borrow()).is_null())) as i32)
        != 0)
    {
        assert!((1 != 0));
    }
    if (((((((*x.borrow()) < (*y.borrow())) as i32) != 0) || (!(*cnp.borrow()).is_null())) as i32)
        != 0)
    {
        assert!((0 != 0));
    }
    if (((((((*x.borrow()) > (*y.borrow())) as i32) != 0)
        && (((((*n.borrow()) != 0) && (!(*cp.borrow()).is_null())) as i32) != 0)) as i32)
        != 0)
    {
        assert!((1 != 0));
    }
    if (((((((*x.borrow()) > (*y.borrow())) as i32) != 0) && (({ returns_one_1() }) != 0)) as i32)
        != 0)
    {
        assert!((1 != 0));
    }
    if (((((((*x.borrow()) > (*y.borrow())) as i32) != 0) && (!(({ returns_zero_2() }) != 0)))
        as i32)
        != 0)
    {
        assert!((1 != 0));
    }
    if (((((((*x.borrow()) < (*y.borrow())) as i32) != 0) || (({ returns_one_1() }) != 0)) as i32)
        != 0)
    {
        assert!((1 != 0));
    }
    if (((((((*x.borrow()) < (*y.borrow())) as i32) != 0) || (!(({ returns_one_1() }) != 0)))
        as i32)
        != 0)
    {
        assert!((0 != 0));
    }
    if (((((((((!((*p.borrow()).is_null())) as i32) != 0) && (({ returns_one_1() }) != 0)) as i32)
        != 0)
        && ((((*n.borrow()) != 0) as i32) != 0)) as i32)
        != 0)
    {
        assert!((1 != 0));
    }
    return 0;
}
