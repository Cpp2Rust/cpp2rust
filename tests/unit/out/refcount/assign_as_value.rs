extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let buf: Value<Box<[i8]>> = Rc::new(RefCell::new((0..2).map(|_| 0_i8).collect::<Box<[i8]>>()));
    let mut p: Ptr<i8> = buf.decay();
    let mut q: Ptr<i8> = Ptr::<i8>::null();
    q = match ({ 1 }, &mut (p)) {
        (__tmp_0, __tmp_1) => {
            (*__tmp_1) = (*__tmp_1).offset((__tmp_0 as isize));
            (*__tmp_1).clone()
        }
    };
    assert!(({ (q).clone() } == { buf.decay().offset((1) as isize) }));
    let mut out: i8 = 0_i8;
    'switch: {
        match {
            ((match ({ (('x' as i32) as i8) }, &mut (out)) {
                (__tmp_2, __tmp_3) => {
                    (*__tmp_3) = __tmp_2;
                    (*__tmp_3).clone()
                }
            }) as i32)
        } {
            __v if __v == ('x' as i32) => {
                assert!(true);
                break 'switch;
            }
            _ => {
                assert!(false);
                break 'switch;
            }
        }
    };
    assert!(((out as i32) == ('x' as i32)));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
