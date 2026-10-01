extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut start: i32 = 5;
    let mut next: FnPtr<fn() -> i32> = {
        #[repr(C)]
        struct Captures {
            start: i32,
        }
        FnPtr::<fn() -> i32>::with_captures_unsafe(
            Captures { start: start },
            (|this: &mut Captures| unsafe {
                return this.start.postfix_inc();
            }),
        )
    };
    assert!(((unsafe { next.call() }) == (5)));
    assert!(((unsafe { next.call() }) == (6)));
    assert!(((unsafe { next.call() }) == (7)));
    assert!(((start) == (5)));
    let mut total: i32 = 0;
    let mut accumulate: FnPtr<fn(i32) -> i32> = {
        #[repr(C)]
        struct Captures {
            total: i32,
        }
        FnPtr::<fn(i32) -> i32>::with_captures_unsafe(
            Captures { total: total },
            (|this: &mut Captures, x: i32| unsafe {
                this.total += x;
                return this.total;
            }),
        )
    };
    assert!(((unsafe { accumulate.call(1,) }) == (1)));
    assert!(((unsafe { accumulate.call(2,) }) == (3)));
    assert!(((total) == (0)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
