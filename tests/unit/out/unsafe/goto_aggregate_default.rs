extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn agg_0(mut n: i32) -> i32 {
    let mut buf: [u8; 40] = [0_u8; 40];
    let mut total: i32 = 0_i32;
    goto_block!({
        '__entry: {
            total = 0;
            if ((((n) < (0)) as i32) != 0) {
                goto!('out);
            }
            {
                let byte_0 = (buf.as_mut_ptr() as *mut u8 as *mut ::libc::c_void) as *mut u8;
                for offset in 0..::std::mem::size_of::<[u8; 40]>() as u64 {
                    *byte_0.offset(offset as isize) = 1 as u8;
                }
                (buf.as_mut_ptr() as *mut u8 as *mut ::libc::c_void)
            };
            let mut i: i32 = 0;
            'loop_: while ((((i) < (40)) as i32) != 0) {
                total += (buf[(i) as usize] as i32);
                i.postfix_inc();
            }
        }
        'out: {
            return total;
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
    assert!(
        ((((unsafe {
            let _n: i32 = -1_i32;
            agg_0(_n)
        }) == (0)) as i32)
            != 0)
    );
    assert!(
        ((((unsafe {
            let _n: i32 = 2;
            agg_0(_n)
        }) == (40)) as i32)
            != 0)
    );
    return 0;
}
