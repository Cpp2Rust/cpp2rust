extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn early_0(mut n: i32) -> i32 {
    let mut ret: i32 = 0_i32;
    let mut intentionally_const_var: i32 = 0_i32;
    goto_block!({
        '__entry: {
            ret = 0;
            if (((n < 0) as i32) != 0) {
                ret = -1_i32;
                goto!('out);
            }
            ret = 100;
            intentionally_const_var = 22;
        }
        'out: {
            return ((ret + intentionally_const_var) - intentionally_const_var);
        }
    });
    panic!("ub: non-void function does not return a value")
}
pub fn from_loop_1(mut n: i32) -> i32 {
    let mut ret: i32 = 0_i32;
    goto_block!({
        '__entry: {
            ret = 0;
            let mut i: i32 = 0;
            'loop_: while (((i < n) as i32) != 0) {
                if (((i == 3) as i32) != 0) {
                    ret = 7;
                    goto!('out);
                }
                ret += i;
                i.postfix_inc();
            }
            ret = 999;
        }
        'out: {
            return ret;
        }
    });
    panic!("ub: non-void function does not return a value")
}
pub fn from_switch_2(mut n: i32) -> i32 {
    let mut ret: i32 = 0_i32;
    goto_block!({
        '__entry: {
            ret = 0;
            'switch: {
                match { n } {
                    __v if __v == 1 => {
                        ret = 10;
                        goto!('out);
                    }
                    _ => {
                        ret = 20;
                        break 'switch;
                    }
                }
            };
            ret = 999;
        }
        'out: {
            return ret;
        }
    });
    panic!("ub: non-void function does not return a value")
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct wrapper {
    #[offset(0)]
    #[byte_size(8)]
    pub item: Ptr<i32>,
}
pub fn via_pointer_3(mut w: Ptr<wrapper>, mut fail: i32) -> i32 {
    let mut ret: i32 = 0_i32;
    let mut item: Ptr<i32> = Ptr::<i32>::null();
    goto_block!({
        '__entry: {
            ret = 0;
            item = w.with(|__s| __s.item.clone());
            if (fail != 0) {
                ret = -1_i32;
                goto!('out);
            }
            ret = { (item.read()) };
        }
        'out: {
            return ret;
        }
    });
    panic!("ub: non-void function does not return a value")
}
pub fn via_arrays_4(mut fail: i32) -> i32 {
    let mut ret: i32 = 0_i32;
    let mut remain: [u8; 4] = [0_u8; 4];
    let mut name: [i8; 5] = [0_i8; 5];
    goto_block!({
        '__entry: {
            ret = 0;
            remain = [0_u8, 0_u8, 0_u8, 0_u8];
            name = b"wxyz\0".map(i8::from_byte);
            if (fail != 0) {
                ret = -1_i32;
                goto!('out);
            }
            remain[(1) as usize] = 9_u8;
            ret = ((((remain[(0) as usize] as i32) + (remain[(1) as usize] as i32))
                + (((name[(0) as usize] as i32) == ('w' as i32)) as i32))
                + (((name[(4) as usize] as i32) == ('\0' as i32)) as i32));
        }
        'out: {
            return ret;
        }
    });
    panic!("ub: non-void function does not return a value")
}
pub fn main() {
    __cpp2rust_init_globals();
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
        item: (value.as_pointer()),
    }));
    assert!((((({ via_pointer_3((w.as_pointer()), 0,) }) == 42) as i32) != 0));
    assert!((((({ via_pointer_3((w.as_pointer()), 1,) }) == -1_i32) as i32) != 0));
    assert!((((({ via_arrays_4(0,) }) == 11) as i32) != 0));
    assert!((((({ via_arrays_4(1,) }) == -1_i32) as i32) != 0));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
