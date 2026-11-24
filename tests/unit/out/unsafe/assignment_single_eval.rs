extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_0() {
    let mut a: [u32; 3] = [10_u32, 20_u32, 30_u32];
    let mut i: i32 = 0;
    match ({ 5_u32 }, &mut (a[(i.postfix_inc()) as usize])) {
        (__tmp_0, __tmp_1) => {
            (*__tmp_1) = (((*__tmp_1) as u32).wrapping_add((__tmp_0 as u32))) as u32;
        }
    };
    (unsafe {
        libc::printf(
            c"void %d %d %d\n".as_ptr() as *const libc::c_char,
            (i),
            (a[(0) as usize] as i32),
            (a[(1) as usize] as i32),
        )
    });
    i = 0;
    let mut sum: u32 = (match ({ 2_u32 }, &mut (a[(i.postfix_inc()) as usize])) {
        (__tmp_2, __tmp_3) => {
            (*__tmp_3) = (((*__tmp_3) as u32).wrapping_add((__tmp_2 as u32))) as u32;
            (*__tmp_3).clone()
        }
    });
    (unsafe {
        libc::printf(
            c"compound %d %d %d %d\n".as_ptr() as *const libc::c_char,
            (i),
            (sum as i32),
            (a[(0) as usize] as i32),
            (a[(1) as usize] as i32),
        )
    });
    i = 0;
    let mut assigned: u32 = (match ({ 99_u32 }, &mut (a[(i.postfix_inc()) as usize])) {
        (__tmp_4, __tmp_5) => {
            (*__tmp_5) = __tmp_4;
            (*__tmp_5).clone()
        }
    });
    (unsafe {
        libc::printf(
            c"simple %d %d %d %d\n".as_ptr() as *const libc::c_char,
            (i),
            (assigned as i32),
            (a[(0) as usize] as i32),
            (a[(1) as usize] as i32),
        )
    });
}
pub unsafe fn test_1() {
    let mut a: [i16; 3] = [10_i16, 20_i16, 30_i16];
    let mut i: i32 = 0;
    match ({ 5 }, &mut (a[(i.postfix_inc()) as usize])) {
        (__tmp_6, __tmp_7) => {
            (*__tmp_7) = (((*__tmp_7) as i32) + (__tmp_6 as i32)) as i16;
        }
    };
    (unsafe {
        libc::printf(
            c"void %d %d %d\n".as_ptr() as *const libc::c_char,
            (i),
            (a[(0) as usize] as i32),
            (a[(1) as usize] as i32),
        )
    });
    i = 0;
    let mut sum: i16 = (match ({ 2 }, &mut (a[(i.postfix_inc()) as usize])) {
        (__tmp_8, __tmp_9) => {
            (*__tmp_9) = (((*__tmp_9) as i32) + (__tmp_8 as i32)) as i16;
            (*__tmp_9).clone()
        }
    });
    (unsafe {
        libc::printf(
            c"compound %d %d %d %d\n".as_ptr() as *const libc::c_char,
            (i),
            (sum as i32),
            (a[(0) as usize] as i32),
            (a[(1) as usize] as i32),
        )
    });
    i = 0;
    let mut assigned: i16 = (match ({ 99_i16 }, &mut (a[(i.postfix_inc()) as usize])) {
        (__tmp_10, __tmp_11) => {
            (*__tmp_11) = __tmp_10;
            (*__tmp_11).clone()
        }
    });
    (unsafe {
        libc::printf(
            c"simple %d %d %d %d\n".as_ptr() as *const libc::c_char,
            (i),
            (assigned as i32),
            (a[(0) as usize] as i32),
            (a[(1) as usize] as i32),
        )
    });
}
pub unsafe fn test_2() {
    let mut a: [i32; 3] = [10, 20, 30];
    let mut i: i32 = 0;
    a[(i.postfix_inc()) as usize] += 5;
    (unsafe {
        libc::printf(
            c"void %d %d %d\n".as_ptr() as *const libc::c_char,
            (i),
            (a[(0) as usize]),
            (a[(1) as usize]),
        )
    });
    i = 0;
    let mut sum: i32 = (match ({ 2 }, &mut (a[(i.postfix_inc()) as usize])) {
        (__tmp_12, __tmp_13) => {
            (*__tmp_13) = (((*__tmp_13) as i32) + (__tmp_12 as i32)) as i32;
            (*__tmp_13).clone()
        }
    });
    (unsafe {
        libc::printf(
            c"compound %d %d %d %d\n".as_ptr() as *const libc::c_char,
            (i),
            (sum),
            (a[(0) as usize]),
            (a[(1) as usize]),
        )
    });
    i = 0;
    let mut assigned: i32 = (match ({ 99 }, &mut (a[(i.postfix_inc()) as usize])) {
        (__tmp_14, __tmp_15) => {
            (*__tmp_15) = __tmp_14;
            (*__tmp_15).clone()
        }
    });
    (unsafe {
        libc::printf(
            c"simple %d %d %d %d\n".as_ptr() as *const libc::c_char,
            (i),
            (assigned),
            (a[(0) as usize]),
            (a[(1) as usize]),
        )
    });
}
pub static mut global_3: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 10 });
pub unsafe fn observe_global_4(mut assigned: i32) -> i32 {
    return ((assigned) + (*std::cell::LazyCell::force_mut(&mut *&raw mut global_3)));
}
pub unsafe fn observe_5(mut assigned: i32, mut stored: *const i32) -> i32 {
    return ((assigned) + (*stored));
}
pub unsafe fn test_borrows_6() {
    (unsafe {
        libc::printf(
            c"%d\n".as_ptr() as *const libc::c_char,
            (unsafe {
                observe_global_4(
                    match (
                        { 99 },
                        &mut (*std::cell::LazyCell::force_mut(&mut *&raw mut global_3)),
                    ) {
                        (__tmp_16, __tmp_17) => {
                            (*__tmp_17) = __tmp_16;
                            (*__tmp_17).clone()
                        }
                    },
                )
            }),
        )
    });
    (unsafe {
        libc::printf(
            c"%d\n".as_ptr() as *const libc::c_char,
            (unsafe {
                observe_global_4(
                    match (
                        { 5 },
                        &mut (*std::cell::LazyCell::force_mut(&mut *&raw mut global_3)),
                    ) {
                        (__tmp_18, __tmp_19) => {
                            (*__tmp_19) = (((*__tmp_19) as i32) + (__tmp_18 as i32)) as i32;
                            (*__tmp_19).clone()
                        }
                    },
                )
            }),
        )
    });
    let mut local: i32 = 10;
    (unsafe {
        libc::printf(
            c"%d\n".as_ptr() as *const libc::c_char,
            (unsafe {
                let _assigned: i32 = match ({ 99 }, &mut (local)) {
                    (__tmp_20, __tmp_21) => {
                        (*__tmp_21) = __tmp_20;
                        (*__tmp_21).clone()
                    }
                };
                let _stored: *const i32 = (&mut local as *mut i32).cast_const();
                observe_5(_assigned, _stored)
            }),
        )
    });
    (unsafe {
        libc::printf(
            c"%d\n".as_ptr() as *const libc::c_char,
            (unsafe {
                let _assigned: i32 = match ({ 5 }, &mut (local)) {
                    (__tmp_22, __tmp_23) => {
                        (*__tmp_23) = (((*__tmp_23) as i32) + (__tmp_22 as i32)) as i32;
                        (*__tmp_23).clone()
                    }
                };
                let _stored: *const i32 = (&mut local as *mut i32).cast_const();
                observe_5(_assigned, _stored)
            }),
        )
    });
    let mut record: Record_7 = Record_7 { value: 10 };
    (unsafe {
        libc::printf(
            c"%d\n".as_ptr() as *const libc::c_char,
            (unsafe {
                let _assigned: i32 = match ({ 99 }, &mut (record.value)) {
                    (__tmp_24, __tmp_25) => {
                        (*__tmp_25) = __tmp_24;
                        (*__tmp_25).clone()
                    }
                };
                let _stored: *const i32 = (&mut record.value as *mut i32).cast_const();
                observe_5(_assigned, _stored)
            }),
        )
    });
    (unsafe {
        libc::printf(
            c"%d\n".as_ptr() as *const libc::c_char,
            (unsafe {
                let _assigned: i32 = match ({ 5 }, &mut (record.value)) {
                    (__tmp_26, __tmp_27) => {
                        (*__tmp_27) = (((*__tmp_27) as i32) + (__tmp_26 as i32)) as i32;
                        (*__tmp_27).clone()
                    }
                };
                let _stored: *const i32 = (&mut record.value as *mut i32).cast_const();
                observe_5(_assigned, _stored)
            }),
        )
    });
    let mut values: Vec<i32> = vec![10; 1_usize];
    (unsafe {
        libc::printf(
            c"%d\n".as_ptr() as *const libc::c_char,
            (unsafe {
                let _assigned: i32 = match ({ 99 }, &mut (values[(0_usize)])) {
                    (__tmp_28, __tmp_29) => {
                        (*__tmp_29) = __tmp_28;
                        (*__tmp_29).clone()
                    }
                };
                let _stored: *const i32 = (&mut values[(0_usize)] as *mut i32).cast_const();
                observe_5(_assigned, _stored)
            }),
        )
    });
    (unsafe {
        libc::printf(
            c"%d\n".as_ptr() as *const libc::c_char,
            (unsafe {
                let _assigned: i32 = match ({ 5 }, &mut (values[(0_usize)])) {
                    (__tmp_30, __tmp_31) => {
                        (*__tmp_31) = (((*__tmp_31) as i32) + (__tmp_30 as i32)) as i32;
                        (*__tmp_31).clone()
                    }
                };
                let _stored: *const i32 = (&mut values[(0_usize)] as *mut i32).cast_const();
                observe_5(_assigned, _stored)
            }),
        )
    });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Record_7 {
    pub value: i32,
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_0() });
    (unsafe { test_1() });
    (unsafe { test_2() });
    (unsafe { test_borrows_6() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const global_3);
}
