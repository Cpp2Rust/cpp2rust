// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

fn t1() -> bool {
    false
}

fn t2() -> *mut bool {
    std::ptr::null_mut()
}

fn t3() -> *const bool {
    std::ptr::null()
}

fn t4() -> f32 {
    0_f32
}

fn t5() -> *mut f32 {
    std::ptr::null_mut()
}

fn t6() -> *const f32 {
    std::ptr::null()
}

fn t7() -> f64 {
    0_f64
}

fn t8() -> *mut f64 {
    std::ptr::null_mut()
}

fn t9() -> *const f64 {
    std::ptr::null()
}

fn t10() -> libc::c_char {
    (0 as libc::c_char)
}

fn t11() -> *mut libc::c_char {
    std::ptr::null_mut()
}

fn t12() -> *const libc::c_char {
    std::ptr::null()
}

fn t13() -> i8 {
    0_i8
}

fn t14() -> *mut i8 {
    std::ptr::null_mut()
}

fn t15() -> *const i8 {
    std::ptr::null()
}

fn t16() -> u8 {
    0_u8
}

fn t17() -> *mut u8 {
    std::ptr::null_mut()
}

fn t18() -> *const u8 {
    std::ptr::null()
}

fn t19() -> i16 {
    0_i16
}

fn t20() -> *mut i16 {
    std::ptr::null_mut()
}

fn t21() -> *const i16 {
    std::ptr::null()
}

fn t22() -> u16 {
    0_u16
}

fn t23() -> *mut u16 {
    std::ptr::null_mut()
}

fn t24() -> *const u16 {
    std::ptr::null()
}

fn t25() -> i32 {
    0_i32
}

fn t26() -> *mut i32 {
    std::ptr::null_mut()
}

fn t27() -> *const i32 {
    std::ptr::null()
}

fn t28() -> u32 {
    0_u32
}

fn t29() -> *mut u32 {
    std::ptr::null_mut()
}

fn t30() -> *const u32 {
    std::ptr::null()
}

fn t31() -> i64 {
    0_i64
}

fn t32() -> *mut i64 {
    std::ptr::null_mut()
}

fn t33() -> *const i64 {
    std::ptr::null()
}

fn t34() -> u64 {
    0_u64
}

fn t35() -> *mut u64 {
    std::ptr::null_mut()
}

fn t36() -> *const u64 {
    std::ptr::null()
}

fn t37() -> i64 {
    0_i64
}

fn t38() -> *mut i64 {
    std::ptr::null_mut()
}

fn t39() -> *const i64 {
    std::ptr::null()
}

fn t40() -> u64 {
    0_u64
}

fn t41() -> *mut u64 {
    std::ptr::null_mut()
}

fn t42() -> *const u64 {
    std::ptr::null()
}

fn t43() -> *mut ::libc::c_void {
    std::ptr::null_mut()
}

fn t44() -> *const ::libc::c_void {
    std::ptr::null()
}

fn t45() -> *mut ::libc::c_void {
    std::ptr::null_mut::<::libc::c_void>()
}

fn t46() -> ::libc::c_void {
    unreachable!()
}

fn t47() -> f64 {
    0_f64
}

fn t48() -> i32 {
    0_i32
}

fn t49() -> u8 {
    0_u8
}

fn t50() -> u16 {
    0_u16
}

fn t51() -> u32 {
    0_u32
}

fn t52() -> i128 {
    0_i128
}

fn t53() -> u128 {
    0_u128
}
