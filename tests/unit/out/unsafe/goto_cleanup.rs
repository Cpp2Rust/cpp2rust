extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn early_0(mut n: i32) -> i32 {
    let mut ret: i32 = 0_i32;
    let mut intentionally_const_var: i32 = 0_i32;
    goto_block!({
        '__entry: {
            ret = 0;
            if ((((n) < (0)) as i32) != 0) {
                ret = -1_i32;
                goto!('out);
            }
            ret = 100;
            intentionally_const_var = 22;
        }
        'out: {
            return (((ret) + (intentionally_const_var)) - (intentionally_const_var));
        }
    });
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn from_loop_1(mut n: i32) -> i32 {
    let mut ret: i32 = 0_i32;
    goto_block!({
        '__entry: {
            ret = 0;
            let mut i: i32 = 0;
            'loop_: while ((((i) < (n)) as i32) != 0) {
                if ((((i) == (3)) as i32) != 0) {
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
pub unsafe fn from_switch_2(mut n: i32) -> i32 {
    let mut ret: i32 = 0_i32;
    goto_block!({
        '__entry: {
            ret = 0;
            'switch: {
                let __match_cond = n;
                match __match_cond {
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
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct wrapper {
    pub item: *mut i32,
}
pub unsafe fn via_pointer_3(mut w: *mut wrapper, mut fail: i32) -> i32 {
    let mut ret: i32 = 0_i32;
    let mut item: *mut i32 = std::ptr::null_mut();
    goto_block!({
        '__entry: {
            ret = 0;
            item = (*w).item;
            if (fail != 0) {
                ret = -1_i32;
                goto!('out);
            }
            ret = (*item);
        }
        'out: {
            return ret;
        }
    });
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn via_arrays_4(mut fail: i32) -> i32 {
    let mut ret: i32 = 0_i32;
    let mut remain: [u8; 4] = [0_u8; 4];
    let mut name: [libc::c_char; 5] = [(0 as libc::c_char); 5];
    goto_block!({
        '__entry: {
            ret = 0;
            remain = [0_u8, 0_u8, 0_u8, 0_u8];
            name = std::mem::transmute(*b"wxyz\0");
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
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(((((unsafe { early_0(-1_i32,) }) == (-1_i32)) as i32) != 0));
    assert!(((((unsafe { early_0(5,) }) == (100)) as i32) != 0));
    assert!(((((unsafe { from_loop_1(2,) }) == (999)) as i32) != 0));
    assert!(((((unsafe { from_loop_1(10,) }) == (7)) as i32) != 0));
    assert!(((((unsafe { from_switch_2(1,) }) == (10)) as i32) != 0));
    assert!(((((unsafe { from_switch_2(2,) }) == (999)) as i32) != 0));
    let mut value: i32 = 42;
    let mut w: wrapper = wrapper {
        item: (&mut value as *mut i32),
    };
    assert!(((((unsafe { via_pointer_3((&mut w as *mut wrapper), 0,) }) == (42)) as i32) != 0));
    assert!(((((unsafe { via_pointer_3((&mut w as *mut wrapper), 1,) }) == (-1_i32)) as i32) != 0));
    assert!(((((unsafe { via_arrays_4(0,) }) == (11)) as i32) != 0));
    assert!(((((unsafe { via_arrays_4(1,) }) == (-1_i32)) as i32) != 0));
    return 0;
}
