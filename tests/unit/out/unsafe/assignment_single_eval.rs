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
pub unsafe fn test_comma_3() {
    let mut x: u32 = 10_u32;
    let mut i: i32 = 0;
    match (
        { 5_u32 },
        &mut (*({
            i.prefix_inc();
            &mut x
        })),
    ) {
        (__tmp_16, __tmp_17) => {
            (*__tmp_17) = (((*__tmp_17) as u32).wrapping_add((__tmp_16 as u32))) as u32;
        }
    };
    (unsafe {
        libc::printf(
            c"direct void %d %d\n".as_ptr() as *const libc::c_char,
            (i),
            (x as i32),
        )
    });
    let mut result: u32 = (match (
        { 2_u32 },
        &mut (*({
            i.prefix_inc();
            &mut x
        })),
    ) {
        (__tmp_18, __tmp_19) => {
            (*__tmp_19) = (((*__tmp_19) as u32).wrapping_add((__tmp_18 as u32))) as u32;
            (*__tmp_19).clone()
        }
    });
    (unsafe {
        libc::printf(
            c"direct compound %d %d %d\n".as_ptr() as *const libc::c_char,
            (i),
            (result as i32),
            (x as i32),
        )
    });
    result = (match (
        { 99_u32 },
        &mut (*({
            i.prefix_inc();
            &mut x
        })),
    ) {
        (__tmp_20, __tmp_21) => {
            (*__tmp_21) = __tmp_20;
            (*__tmp_21).clone()
        }
    });
    (unsafe {
        libc::printf(
            c"direct simple %d %d %d\n".as_ptr() as *const libc::c_char,
            (i),
            (result as i32),
            (x as i32),
        )
    });
    let mut p: *mut u32 = (&mut x as *mut u32);
    match (
        { 5_u32 },
        &mut (*({
            i.prefix_inc();
            (p).clone()
        })),
    ) {
        (__tmp_22, __tmp_23) => {
            (*__tmp_23) = (((*__tmp_23) as u32).wrapping_add((__tmp_22 as u32))) as u32;
        }
    };
    (unsafe {
        libc::printf(
            c"indirect void %d %d\n".as_ptr() as *const libc::c_char,
            (i),
            ((*p) as i32),
        )
    });
    result = (match (
        { 2_u32 },
        &mut (*({
            i.prefix_inc();
            (p).clone()
        })),
    ) {
        (__tmp_24, __tmp_25) => {
            (*__tmp_25) = (((*__tmp_25) as u32).wrapping_add((__tmp_24 as u32))) as u32;
            (*__tmp_25).clone()
        }
    });
    (unsafe {
        libc::printf(
            c"indirect compound %d %d %d\n".as_ptr() as *const libc::c_char,
            (i),
            (result as i32),
            ((*p) as i32),
        )
    });
    result = (match (
        { 99_u32 },
        &mut (*({
            i.prefix_inc();
            (p).clone()
        })),
    ) {
        (__tmp_26, __tmp_27) => {
            (*__tmp_27) = __tmp_26;
            (*__tmp_27).clone()
        }
    });
    (unsafe {
        libc::printf(
            c"indirect simple %d %d %d\n".as_ptr() as *const libc::c_char,
            (i),
            (result as i32),
            ((*p) as i32),
        )
    });
}
pub unsafe fn test_comma_4() {
    let mut x: i16 = 10_i16;
    let mut i: i32 = 0;
    match (
        { 5 },
        &mut (*({
            i.prefix_inc();
            &mut x
        })),
    ) {
        (__tmp_28, __tmp_29) => {
            (*__tmp_29) = (((*__tmp_29) as i32) + (__tmp_28 as i32)) as i16;
        }
    };
    (unsafe {
        libc::printf(
            c"direct void %d %d\n".as_ptr() as *const libc::c_char,
            (i),
            (x as i32),
        )
    });
    let mut result: i16 = (match (
        { 2 },
        &mut (*({
            i.prefix_inc();
            &mut x
        })),
    ) {
        (__tmp_30, __tmp_31) => {
            (*__tmp_31) = (((*__tmp_31) as i32) + (__tmp_30 as i32)) as i16;
            (*__tmp_31).clone()
        }
    });
    (unsafe {
        libc::printf(
            c"direct compound %d %d %d\n".as_ptr() as *const libc::c_char,
            (i),
            (result as i32),
            (x as i32),
        )
    });
    result = (match (
        { 99_i16 },
        &mut (*({
            i.prefix_inc();
            &mut x
        })),
    ) {
        (__tmp_32, __tmp_33) => {
            (*__tmp_33) = __tmp_32;
            (*__tmp_33).clone()
        }
    });
    (unsafe {
        libc::printf(
            c"direct simple %d %d %d\n".as_ptr() as *const libc::c_char,
            (i),
            (result as i32),
            (x as i32),
        )
    });
    let mut p: *mut i16 = (&mut x as *mut i16);
    match (
        { 5 },
        &mut (*({
            i.prefix_inc();
            (p).clone()
        })),
    ) {
        (__tmp_34, __tmp_35) => {
            (*__tmp_35) = (((*__tmp_35) as i32) + (__tmp_34 as i32)) as i16;
        }
    };
    (unsafe {
        libc::printf(
            c"indirect void %d %d\n".as_ptr() as *const libc::c_char,
            (i),
            ((*p) as i32),
        )
    });
    result = (match (
        { 2 },
        &mut (*({
            i.prefix_inc();
            (p).clone()
        })),
    ) {
        (__tmp_36, __tmp_37) => {
            (*__tmp_37) = (((*__tmp_37) as i32) + (__tmp_36 as i32)) as i16;
            (*__tmp_37).clone()
        }
    });
    (unsafe {
        libc::printf(
            c"indirect compound %d %d %d\n".as_ptr() as *const libc::c_char,
            (i),
            (result as i32),
            ((*p) as i32),
        )
    });
    result = (match (
        { 99_i16 },
        &mut (*({
            i.prefix_inc();
            (p).clone()
        })),
    ) {
        (__tmp_38, __tmp_39) => {
            (*__tmp_39) = __tmp_38;
            (*__tmp_39).clone()
        }
    });
    (unsafe {
        libc::printf(
            c"indirect simple %d %d %d\n".as_ptr() as *const libc::c_char,
            (i),
            (result as i32),
            ((*p) as i32),
        )
    });
}
pub static mut global_5: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 10 });
pub unsafe fn observe_global_6(mut assigned: i32) -> i32 {
    return ((assigned) + (*std::cell::LazyCell::force_mut(&mut *&raw mut global_5)));
}
pub unsafe fn observe_7(mut assigned: i32, mut stored: *const i32) -> i32 {
    return ((assigned) + (*stored));
}
pub unsafe fn test_borrows_8() {
    (unsafe {
        libc::printf(
            c"%d\n".as_ptr() as *const libc::c_char,
            (unsafe {
                observe_global_6(
                    match (
                        { 99 },
                        &mut (*std::cell::LazyCell::force_mut(&mut *&raw mut global_5)),
                    ) {
                        (__tmp_40, __tmp_41) => {
                            (*__tmp_41) = __tmp_40;
                            (*__tmp_41).clone()
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
                observe_global_6(
                    match (
                        { 5 },
                        &mut (*std::cell::LazyCell::force_mut(&mut *&raw mut global_5)),
                    ) {
                        (__tmp_42, __tmp_43) => {
                            (*__tmp_43) = (((*__tmp_43) as i32) + (__tmp_42 as i32)) as i32;
                            (*__tmp_43).clone()
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
                    (__tmp_44, __tmp_45) => {
                        (*__tmp_45) = __tmp_44;
                        (*__tmp_45).clone()
                    }
                };
                let _stored: *const i32 = (&mut local as *mut i32).cast_const();
                observe_7(_assigned, _stored)
            }),
        )
    });
    (unsafe {
        libc::printf(
            c"%d\n".as_ptr() as *const libc::c_char,
            (unsafe {
                let _assigned: i32 = match ({ 5 }, &mut (local)) {
                    (__tmp_46, __tmp_47) => {
                        (*__tmp_47) = (((*__tmp_47) as i32) + (__tmp_46 as i32)) as i32;
                        (*__tmp_47).clone()
                    }
                };
                let _stored: *const i32 = (&mut local as *mut i32).cast_const();
                observe_7(_assigned, _stored)
            }),
        )
    });
    let mut record: Record_9 = Record_9 { value: 10 };
    (unsafe {
        libc::printf(
            c"%d\n".as_ptr() as *const libc::c_char,
            (unsafe {
                let _assigned: i32 = match ({ 99 }, &mut (record.value)) {
                    (__tmp_48, __tmp_49) => {
                        (*__tmp_49) = __tmp_48;
                        (*__tmp_49).clone()
                    }
                };
                let _stored: *const i32 = (&mut record.value as *mut i32).cast_const();
                observe_7(_assigned, _stored)
            }),
        )
    });
    (unsafe {
        libc::printf(
            c"%d\n".as_ptr() as *const libc::c_char,
            (unsafe {
                let _assigned: i32 = match ({ 5 }, &mut (record.value)) {
                    (__tmp_50, __tmp_51) => {
                        (*__tmp_51) = (((*__tmp_51) as i32) + (__tmp_50 as i32)) as i32;
                        (*__tmp_51).clone()
                    }
                };
                let _stored: *const i32 = (&mut record.value as *mut i32).cast_const();
                observe_7(_assigned, _stored)
            }),
        )
    });
    let mut values: Vec<i32> = vec![10; 1_usize];
    (unsafe {
        libc::printf(
            c"%d\n".as_ptr() as *const libc::c_char,
            (unsafe {
                let _assigned: i32 = match ({ 99 }, &mut (values[(0_usize)])) {
                    (__tmp_52, __tmp_53) => {
                        (*__tmp_53) = __tmp_52;
                        (*__tmp_53).clone()
                    }
                };
                let _stored: *const i32 = (&mut values[(0_usize)] as *mut i32).cast_const();
                observe_7(_assigned, _stored)
            }),
        )
    });
    (unsafe {
        libc::printf(
            c"%d\n".as_ptr() as *const libc::c_char,
            (unsafe {
                let _assigned: i32 = match ({ 5 }, &mut (values[(0_usize)])) {
                    (__tmp_54, __tmp_55) => {
                        (*__tmp_55) = (((*__tmp_55) as i32) + (__tmp_54 as i32)) as i32;
                        (*__tmp_55).clone()
                    }
                };
                let _stored: *const i32 = (&mut values[(0_usize)] as *mut i32).cast_const();
                observe_7(_assigned, _stored)
            }),
        )
    });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Record_9 {
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
    (unsafe { test_comma_3() });
    (unsafe { test_comma_4() });
    let mut values: [u32; 2] = [10_u32, 20_u32];
    let ref_: *mut [u32; 2] = &mut values;
    let mut i: i32 = 0;
    let mut result: u32 = (match (
        { 5_u32 },
        &mut ((*({
            i.prefix_inc();
            (ref_).clone()
        }))[(0) as usize]),
    ) {
        (__tmp_56, __tmp_57) => {
            (*__tmp_57) = (((*__tmp_57) as u32).wrapping_add((__tmp_56 as u32))) as u32;
            (*__tmp_57).clone()
        }
    });
    (unsafe {
        libc::printf(
            c"array reference %d %u %u\n".as_ptr() as *const libc::c_char,
            (i),
            (result),
            (values[(0) as usize]),
        )
    });
    (unsafe { test_borrows_8() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const global_5);
}
