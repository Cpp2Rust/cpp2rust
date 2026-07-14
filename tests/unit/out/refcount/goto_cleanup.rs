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
#[derive(Default)]
pub struct wrapper {
    pub item: Value<Ptr<i32>>,
}
impl Clone for wrapper {
    fn clone(&self) -> Self {
        Self {
            item: Rc::new(RefCell::new((*self.item.borrow()).clone())),
        }
    }
}
impl ByteRepr for wrapper {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.item.borrow()).to_bytes(&mut buf[0..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            item: Rc::new(RefCell::new(<Ptr<i32>>::from_bytes(&buf[0..8]))),
        }
    }
}
pub fn via_pointer_3(w: Ptr<wrapper>, fail: i32) -> i32 {
    let w: Value<Ptr<wrapper>> = Rc::new(RefCell::new(w));
    let fail: Value<i32> = Rc::new(RefCell::new(fail));
    let ret: Value<i32> = <Value<i32>>::default();
    let item: Value<Ptr<i32>> = Rc::new(RefCell::new(Ptr::<i32>::null()));
    goto_block!({
        '__entry: {
            *ret.borrow_mut() = 0;
            *item.borrow_mut() = (*(*(*w.borrow()).upgrade().deref()).item.borrow()).clone();
            if ((*fail.borrow()) != 0) {
                (*ret.borrow_mut()) = -1_i32;
                goto!('out);
            }
            let __rhs = ((*item.borrow()).read());
            (*ret.borrow_mut()) = __rhs;
        }
        'out: {
            return (*ret.borrow());
        }
    });
    panic!("ub: non-void function does not return a value")
}
pub fn via_arrays_4(fail: i32) -> i32 {
    let fail: Value<i32> = Rc::new(RefCell::new(fail));
    let ret: Value<i32> = <Value<i32>>::default();
    let remain: Value<Box<[u8]>> = Rc::new(RefCell::new(
        (0..4).map(|_| <u8>::default()).collect::<Box<[u8]>>(),
    ));
    let name: Value<Box<[u8]>> = Rc::new(RefCell::new(
        (0..5).map(|_| <u8>::default()).collect::<Box<[u8]>>(),
    ));
    goto_block!({
        '__entry: {
            *ret.borrow_mut() = 0;
            *remain.borrow_mut() =
                Box::new([0_u8, <u8>::default(), <u8>::default(), <u8>::default()]);
            *name.borrow_mut() = Box::from(*b"wxyz\0");
            if ((*fail.borrow()) != 0) {
                (*ret.borrow_mut()) = -1_i32;
                goto!('out);
            }
            (*remain.borrow_mut())[(1) as usize] = 9_u8;
            (*ret.borrow_mut()) = (((((*remain.borrow())[(0) as usize] as i32)
                + ((*remain.borrow())[(1) as usize] as i32))
                + ((((*name.borrow())[(0) as usize] as i32) == ('w' as i32)) as i32))
                + ((((*name.borrow())[(4) as usize] as i32) == ('\0' as i32)) as i32));
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
    let value: Value<i32> = Rc::new(RefCell::new(42));
    let w: Value<wrapper> = Rc::new(RefCell::new(wrapper {
        item: Rc::new(RefCell::new((value.as_pointer()))),
    }));
    assert!((((({ via_pointer_3((w.as_pointer()), 0,) }) == 42) as i32) != 0));
    assert!((((({ via_pointer_3((w.as_pointer()), 1,) }) == -1_i32) as i32) != 0));
    assert!((((({ via_arrays_4(0,) }) == 11) as i32) != 0));
    assert!((((({ via_arrays_4(1,) }) == -1_i32) as i32) != 0));
    return 0;
}
