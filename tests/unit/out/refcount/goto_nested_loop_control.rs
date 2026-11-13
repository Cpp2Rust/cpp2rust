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
    let mut visits: i32 = 0;
    let mut tail: i32 = 0;
    let mut i: i32 = 0;
    'loop_: while (i < 3) {
        goto_block!({
            '__entry: {}
            'outer: {
                visits.postfix_inc();
                {
                    goto_block!({
                        '__entry: {}
                        'inner: {
                            if (i == 0) {
                                break;
                            }
                        }
                    });
                }
                tail.postfix_inc();
            }
        });
        i.postfix_inc();
    }
    assert!((visits == 1));
    assert!((tail == 0));
    visits = 0;
    tail = 0;
    let mut i: i32 = 0;
    'loop_: while (i < 3) {
        switch!(match i {
            __v if __v == 0 => {
                {
                    goto_block!({
                        '__entry: {}
                        'switch_outer: {
                            visits.postfix_inc();
                            {
                                goto_block!({
                                    '__entry: {}
                                    'switch_inner: {
                                        if (i == 0) {
                                            break;
                                        }
                                    }
                                });
                            }
                            tail.postfix_inc();
                        }
                    });
                }
            }
            __v if __v == 1 => {
                tail += 10;
                break;
            }
            _ => {
                tail += 100;
                break;
            }
        });
        i.postfix_inc();
    }
    assert!((visits == 1));
    assert!((tail == 110));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
