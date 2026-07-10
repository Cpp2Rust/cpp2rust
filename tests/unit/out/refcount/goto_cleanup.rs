extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn early_0(n: i32) -> i32 {
    let n: Value<i32> = Rc::new(RefCell::new(n));
    let ret: Value<i32> = <Value<i32>>::default();
    let intentionally_const_var: Value<i32> = <Value<i32>>::default();
    goto_block!({
        '__entry: {
            *ret.borrow_mut() = 0;
            if ((((*n.borrow()) < 0) as i32) != 0) {
                (*ret.borrow_mut()) = -1_i32;
                goto!('out);
            }
            (*ret.borrow_mut()) = 100;
            *intentionally_const_var.borrow_mut() = 22;
        }
        'out: {
            return (((*ret.borrow()) + (*intentionally_const_var.borrow()))
                - (*intentionally_const_var.borrow()));
        }
    });
    panic!("ub: non-void function does not return a value")
}
pub fn from_loop_1(n: i32) -> i32 {
    let n: Value<i32> = Rc::new(RefCell::new(n));
    let ret: Value<i32> = <Value<i32>>::default();
    goto_block!({
        '__entry: {
            *ret.borrow_mut() = 0;
            let i: Value<i32> = Rc::new(RefCell::new(0));
            'loop_: while ((((*i.borrow()) < (*n.borrow())) as i32) != 0) {
                if ((((*i.borrow()) == 3) as i32) != 0) {
                    (*ret.borrow_mut()) = 7;
                    goto!('out);
                }
                (*ret.borrow_mut()) += (*i.borrow());
                (*i.borrow_mut()).postfix_inc();
            }
            (*ret.borrow_mut()) = 999;
        }
        'out: {
            return (*ret.borrow());
        }
    });
    panic!("ub: non-void function does not return a value")
}
pub fn from_switch_2(n: i32) -> i32 {
    let n: Value<i32> = Rc::new(RefCell::new(n));
    let ret: Value<i32> = <Value<i32>>::default();
    goto_block!({
        '__entry: {
            *ret.borrow_mut() = 0;
            'switch: {
                let __match_cond = (*n.borrow());
                match __match_cond {
                    __v if __v == 1 => {
                        (*ret.borrow_mut()) = 10;
                        goto!('out);
                    }
                    _ => {
                        (*ret.borrow_mut()) = 20;
                        break 'switch;
                    }
                }
            };
            (*ret.borrow_mut()) = 999;
        }
        'out: {
            return (*ret.borrow());
        }
    });
    panic!("ub: non-void function does not return a value")
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!((((({ early_0(-1_i32,) }) == -1_i32) as i32) != 0));
    assert!((((({ early_0(5,) }) == 100) as i32) != 0));
    assert!((((({ from_loop_1(2,) }) == 999) as i32) != 0));
    assert!((((({ from_loop_1(10,) }) == 7) as i32) != 0));
    assert!((((({ from_switch_2(1,) }) == 10) as i32) != 0));
    assert!((((({ from_switch_2(2,) }) == 999) as i32) != 0));
    return 0;
}
