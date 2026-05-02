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
    let code: Value<Code> = Rc::new(RefCell::new(Code::CODE_OK));
    if {
        let _lhs = ((*n.borrow()) != 0);
        _lhs && (!(*p.borrow()).is_null()).clone()
    } {
        assert!(true);
    }
    if {
        let _lhs = ((*n.borrow()) != 0);
        _lhs && (!(*np.borrow()).is_null()).clone()
    } {
        assert!(false);
    }
    if {
        let _lhs = ((*zero.borrow()) != 0);
        _lhs || (!(*p.borrow()).is_null()).clone()
    } {
        assert!(true);
    }
    if {
        let _lhs = ((*zero.borrow()) != 0);
        _lhs || (!(*np.borrow()).is_null()).clone()
    } {
        assert!(false);
    }
    if {
        let _lhs = {
            let _lhs = (((*n.borrow()) != 0) && ((*u.borrow()) != 0));
            _lhs && (!(*p.borrow()).is_null()).clone()
        };
        _lhs && (((*code.borrow()) as i32) == (Code::CODE_OK as i32)).clone()
    } {
        assert!(true);
    }
    (*side_effect.with(Value::clone).borrow_mut()) = 0;
    if (((*zero.borrow()) != 0)
        && (({
            let _v: i32 = 1;
            observe_0(_v)
        }) != 0))
    {
        assert!(false);
    }
    assert!(((*side_effect.with(Value::clone).borrow()) == 0));
    if (((*n.borrow()) != 0)
        || (({
            let _v: i32 = 1;
            observe_0(_v)
        }) != 0))
    {
        assert!(true);
    }
    assert!(((*side_effect.with(Value::clone).borrow()) == 0));
    return 0;
}
