extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct Guard {
    pub digit: i32,
}
impl Guard {
    pub unsafe fn destructor(&mut self) {
        (unsafe { libc::printf(c"%d ".as_ptr() as *const libc::c_char, (self.digit)) });
    }
    pub unsafe fn to_bool(&self) -> bool {
        return ((self.digit) != (0));
    }
}
pub unsafe fn step_0(i: *mut i32) {
    (unsafe { libc::printf(c"9 ".as_ptr() as *const libc::c_char) });
    (*i).prefix_inc();
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { libc::printf(c"normal/continue: ".as_ptr() as *const libc::c_char) });
    {
        let mut i: i32 = 0;
        'loop_: while ((i) < (2)) {
            'for_body: {
                let mut first: Guard = Guard { digit: 1 };
                let _dtor_first =
                    std::mem::DropGuard::new(&raw mut first, |__p| unsafe { (*__p).destructor() });
                let mut second: Guard = Guard { digit: 2 };
                let _dtor_second =
                    std::mem::DropGuard::new(&raw mut second, |__p| unsafe { (*__p).destructor() });
                if ((i) == (0)) {
                    break 'for_body;
                }
            }
            (unsafe { step_0(&mut i) });
        }
    }
    (unsafe { libc::printf(c"\nbreak: ".as_ptr() as *const libc::c_char) });
    {
        let mut i: i32 = 0;
        'loop_: while ((i) < (2)) {
            'for_body: {
                let mut g: Guard = Guard { digit: 3 };
                let _dtor_g =
                    std::mem::DropGuard::new(&raw mut g, |__p| unsafe { (*__p).destructor() });
                break 'loop_;
            }
            (unsafe { step_0(&mut i) });
        }
    }
    (unsafe { libc::printf(c"\nnested for: ".as_ptr() as *const libc::c_char) });
    {
        let mut i: i32 = 0;
        'loop_: while ((i) < (1)) {
            'for_body: {
                let mut outer: Guard = Guard { digit: 4 };
                let _dtor_outer =
                    std::mem::DropGuard::new(&raw mut outer, |__p| unsafe { (*__p).destructor() });
                {
                    let mut j: i32 = 0;
                    'loop_: while ((j) < (2)) {
                        'for_body: {
                            let mut inner: Guard = Guard { digit: 5 };
                            let _dtor_inner =
                                std::mem::DropGuard::new(&raw mut inner, |__p| unsafe {
                                    (*__p).destructor()
                                });
                            if ((j) == (0)) {
                                break 'for_body;
                            }
                            break 'loop_;
                        }
                        (unsafe { step_0(&mut j) });
                    }
                }
                break 'for_body;
            }
            (unsafe { step_0(&mut i) });
        }
    }
    (unsafe { libc::printf(c"\ninner while/do/range: ".as_ptr() as *const libc::c_char) });
    {
        let mut i: i32 = 0;
        'loop_: while ((i) < (1)) {
            'for_body: {
                let mut outer: Guard = Guard { digit: 4 };
                let _dtor_outer =
                    std::mem::DropGuard::new(&raw mut outer, |__p| unsafe { (*__p).destructor() });
                let mut j: i32 = 0;
                'loop_: while ((*j.prefix_inc()) < (3)) {
                    let mut inner: Guard = Guard { digit: 5 };
                    let _dtor_inner = std::mem::DropGuard::new(&raw mut inner, |__p| unsafe {
                        (*__p).destructor()
                    });
                    if ((j) == (1)) {
                        continue 'loop_;
                    }
                    break;
                }
                let mut __do_while = true;
                'loop_: while __do_while || (false) {
                    __do_while = false;
                    let mut inner: Guard = Guard { digit: 6 };
                    let _dtor_inner = std::mem::DropGuard::new(&raw mut inner, |__p| unsafe {
                        (*__p).destructor()
                    });
                    continue 'loop_;
                }
                let mut values: [i32; 2] = [1, 2];
                'loop_: for value in IterRange::new(
                    values.as_mut_ptr(),
                    values.as_mut_ptr().offset((2_i64) as isize),
                ) {
                    let mut value: i32 = (*value);
                    let mut inner: Guard = Guard { digit: value };
                    let _dtor_inner = std::mem::DropGuard::new(&raw mut inner, |__p| unsafe {
                        (*__p).destructor()
                    });
                    if ((value) == (1)) {
                        continue 'loop_;
                    }
                    break;
                }
                break 'for_body;
            }
            (unsafe { step_0(&mut i) });
        }
    }
    (unsafe { libc::printf(c"\ncondition: ".as_ptr() as *const libc::c_char) });
    {
        let mut i: i32 = 2;
        'loop_: loop {
            let mut condition: Guard = Guard { digit: i };
            let _dtor_condition =
                std::mem::DropGuard::new(&raw mut condition, |__p| unsafe { (*__p).destructor() });
            if !(unsafe { Guard::to_bool(&condition) }) {
                break;
            }
            'for_body: {
                let mut body: Guard = Guard { digit: 4 };
                let _dtor_body =
                    std::mem::DropGuard::new(&raw mut body, |__p| unsafe { (*__p).destructor() });
                if ((i) == (2)) {
                    break 'for_body;
                }
            }
            {
                i.prefix_dec();
                (unsafe {
                    libc::printf(c"inc%d ".as_ptr() as *const libc::c_char, (condition.digit))
                })
            };
        }
    }
    (unsafe { libc::printf(c"\ninit/noncompound: ".as_ptr() as *const libc::c_char) });
    let mut i: i32 = 0;
    {
        let mut init: Guard = Guard { digit: 5 };
        let _dtor_init =
            std::mem::DropGuard::new(&raw mut init, |__p| unsafe { (*__p).destructor() });
        'loop_: while ((i) < (2)) {
            'for_body: {
                let mut body: Guard = Guard { digit: 6 };
                let _dtor_body =
                    std::mem::DropGuard::new(&raw mut body, |__p| unsafe { (*__p).destructor() });
            }
            (unsafe { step_0(&mut i) });
        }
    }
    (unsafe { libc::printf(c"\nno increment: ".as_ptr() as *const libc::c_char) });
    {
        let mut j: i32 = 0;
        'loop_: while ((j) < (2)) {
            let mut g: Guard = Guard {
                digit: (*j.prefix_inc()),
            };
            let _dtor_g =
                std::mem::DropGuard::new(&raw mut g, |__p| unsafe { (*__p).destructor() });
            continue 'loop_;
        }
    }
    (unsafe { libc::printf(c"\n".as_ptr() as *const libc::c_char) });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
