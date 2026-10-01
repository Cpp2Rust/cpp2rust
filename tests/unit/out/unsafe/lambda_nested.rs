extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct S {
    pub v: i32,
}
impl S {
    pub unsafe fn nested_this(&mut self) -> i32 {
        let mut outer: FnPtr<fn(i32) -> i32> = {
            #[repr(C)]
            struct Captures {
                this_: *mut S,
            }
            FnPtr::<fn(i32) -> i32>::with_captures_unsafe(
                Captures {
                    this_: (self as *mut S),
                },
                (|this: &mut Captures, y: i32| unsafe {
                    let mut inner: FnPtr<fn(i32) -> i32> = {
                        #[repr(C)]
                        struct Captures {
                            this_: *mut S,
                            y: i32,
                        }
                        FnPtr::<fn(i32) -> i32>::with_captures_unsafe(
                            Captures {
                                this_: this.this_,
                                y: y,
                            },
                            (|this: &mut Captures, z: i32| unsafe {
                                return ((((*this.this_).v) + (this.y)) + (z));
                            }),
                        )
                    };
                    return (unsafe { inner.call(1) });
                }),
            )
        };
        return (unsafe { outer.call(20) });
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut x: i32 = 10;
    let mut outer: FnPtr<fn(i32) -> i32> = {
        #[repr(C)]
        struct Captures {
            x: *mut i32,
        }
        FnPtr::<fn(i32) -> i32>::with_captures_unsafe(
            Captures { x: &mut x },
            (|this: &mut Captures, y: i32| unsafe {
                let mut inner: FnPtr<fn(i32) -> i32> = {
                    #[repr(C)]
                    struct Captures {
                        x: *mut i32,
                        y: i32,
                    }
                    FnPtr::<fn(i32) -> i32>::with_captures_unsafe(
                        Captures {
                            x: &mut (*this.x),
                            y: y,
                        },
                        (|this: &mut Captures, z: i32| unsafe {
                            return (((*this.x) + (this.y)) + (z));
                        }),
                    )
                };
                return (unsafe { inner.call(1) });
            }),
        )
    };
    assert!(((unsafe { outer.call(20,) }) == (31)));
    x = 100;
    assert!(((unsafe { outer.call(20,) }) == (121)));
    let mut s: S = S { v: 5 };
    assert!(((unsafe { S::nested_this(&mut s,) }) == (26)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
