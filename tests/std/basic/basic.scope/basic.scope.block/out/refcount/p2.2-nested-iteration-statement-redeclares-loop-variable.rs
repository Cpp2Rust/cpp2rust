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
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut outer_iterations: i32 = 0;
    let mut inner_iterations: i32 = 0;
    let mut sum: i32 = 0;
    {
        let mut index: i32 = 0;
        'loop_: while (index < 2) {
            {
                let mut index: i32 = 10;
                'loop_: while (index < 13) {
                    inner_iterations.prefix_inc();
                    sum += index;
                    index.prefix_inc();
                }
            }
            {
                index.prefix_inc();
                outer_iterations.prefix_inc()
            };
        }
    }
    assert!((outer_iterations == 2));
    assert!((inner_iterations == 6));
    assert!((sum == 66));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
