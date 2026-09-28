// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

fn t0() -> *mut ::libc::c_void {
    std::ptr::null_mut::<::libc::c_void>()
}

fn t10() -> bool {
    false
}

fn t11() -> *mut bool {
    std::ptr::null_mut()
}

fn t12() -> *const bool {
    std::ptr::null()
}

fn t13() -> *mut bool {
    std::ptr::null_mut()
}

fn t14() -> *const bool {
    std::ptr::null()
}

fn t20() -> libc::c_char {
    (0 as libc::c_char)
}

fn t21() -> *mut libc::c_char {
    std::ptr::null_mut()
}

fn t22() -> *const libc::c_char {
    std::ptr::null()
}

fn t23() -> *mut libc::c_char {
    std::ptr::null_mut()
}

fn t24() -> *const libc::c_char {
    std::ptr::null()
}

fn t30() -> i8 {
    0_i8
}

fn t31() -> *mut i8 {
    std::ptr::null_mut()
}

fn t32() -> *const i8 {
    std::ptr::null()
}

fn t33() -> *mut i8 {
    std::ptr::null_mut()
}

fn t34() -> *const i8 {
    std::ptr::null()
}

fn t40() -> u8 {
    0_u8
}

fn t41() -> *mut u8 {
    std::ptr::null_mut()
}

fn t42() -> *const u8 {
    std::ptr::null()
}

fn t43() -> *mut u8 {
    std::ptr::null_mut()
}

fn t44() -> *const u8 {
    std::ptr::null()
}

fn t50() -> u8 {
    0_u8
}

fn t51() -> *mut u8 {
    std::ptr::null_mut()
}

fn t52() -> *const u8 {
    std::ptr::null()
}

fn t53() -> *mut u8 {
    std::ptr::null_mut()
}

fn t54() -> *const u8 {
    std::ptr::null()
}

fn t60() -> i16 {
    0_i16
}

fn t61() -> *mut i16 {
    std::ptr::null_mut()
}

fn t62() -> *const i16 {
    std::ptr::null()
}

fn t63() -> *mut i16 {
    std::ptr::null_mut()
}

fn t64() -> *const i16 {
    std::ptr::null()
}

fn t70() -> u16 {
    0_u16
}

fn t71() -> *mut u16 {
    std::ptr::null_mut()
}

fn t72() -> *const u16 {
    std::ptr::null()
}

fn t73() -> *mut u16 {
    std::ptr::null_mut()
}

fn t74() -> *const u16 {
    std::ptr::null()
}

fn t80() -> u16 {
    0_u16
}

fn t81() -> *mut u16 {
    std::ptr::null_mut()
}

fn t82() -> *const u16 {
    std::ptr::null()
}

fn t83() -> *mut u16 {
    std::ptr::null_mut()
}

fn t84() -> *const u16 {
    std::ptr::null()
}

fn t90() -> i32 {
    0_i32
}

fn t91() -> *mut i32 {
    std::ptr::null_mut()
}

fn t92() -> *const i32 {
    std::ptr::null()
}

fn t93() -> *mut i32 {
    std::ptr::null_mut()
}

fn t94() -> *const i32 {
    std::ptr::null()
}

fn t100() -> u32 {
    0_u32
}

fn t101() -> *mut u32 {
    std::ptr::null_mut()
}

fn t102() -> *const u32 {
    std::ptr::null()
}

fn t103() -> *mut u32 {
    std::ptr::null_mut()
}

fn t104() -> *const u32 {
    std::ptr::null()
}

fn t110() -> i32 {
    0_i32
}

fn t111() -> *mut i32 {
    std::ptr::null_mut()
}

fn t112() -> *const i32 {
    std::ptr::null()
}

fn t113() -> *mut i32 {
    std::ptr::null_mut()
}

fn t114() -> *const i32 {
    std::ptr::null()
}

fn t120() -> u32 {
    0_u32
}

fn t121() -> *mut u32 {
    std::ptr::null_mut()
}

fn t122() -> *const u32 {
    std::ptr::null()
}

fn t123() -> *mut u32 {
    std::ptr::null_mut()
}

fn t124() -> *const u32 {
    std::ptr::null()
}

fn t130() -> f32 {
    0_f32
}

fn t131() -> *mut f32 {
    std::ptr::null_mut()
}

fn t132() -> *const f32 {
    std::ptr::null()
}

fn t133() -> *mut f32 {
    std::ptr::null_mut()
}

fn t134() -> *const f32 {
    std::ptr::null()
}

fn t140() -> i64 {
    0_i64
}

fn t141() -> *mut i64 {
    std::ptr::null_mut()
}

fn t142() -> *const i64 {
    std::ptr::null()
}

fn t143() -> *mut i64 {
    std::ptr::null_mut()
}

fn t144() -> *const i64 {
    std::ptr::null()
}

fn t150() -> u64 {
    0_u64
}

fn t151() -> *mut u64 {
    std::ptr::null_mut()
}

fn t152() -> *const u64 {
    std::ptr::null()
}

fn t153() -> *mut u64 {
    std::ptr::null_mut()
}

fn t154() -> *const u64 {
    std::ptr::null()
}

fn t160() -> i64 {
    0_i64
}

fn t161() -> *mut i64 {
    std::ptr::null_mut()
}

fn t162() -> *const i64 {
    std::ptr::null()
}

fn t163() -> *mut i64 {
    std::ptr::null_mut()
}

fn t164() -> *const i64 {
    std::ptr::null()
}

fn t170() -> u64 {
    0_u64
}

fn t171() -> *mut u64 {
    std::ptr::null_mut()
}

fn t172() -> *const u64 {
    std::ptr::null()
}

fn t173() -> *mut u64 {
    std::ptr::null_mut()
}

fn t174() -> *const u64 {
    std::ptr::null()
}

fn t180() -> f64 {
    0_f64
}

fn t181() -> *mut f64 {
    std::ptr::null_mut()
}

fn t182() -> *const f64 {
    std::ptr::null()
}

fn t183() -> *mut f64 {
    std::ptr::null_mut()
}

fn t184() -> *const f64 {
    std::ptr::null()
}

fn t190() -> f64 {
    0_f64
}

fn t191() -> *mut f64 {
    std::ptr::null_mut()
}

fn t192() -> *const f64 {
    std::ptr::null()
}

fn t193() -> *mut f64 {
    std::ptr::null_mut()
}

fn t194() -> *const f64 {
    std::ptr::null()
}

fn t200() -> i128 {
    0_i128
}

fn t201() -> *mut i128 {
    std::ptr::null_mut()
}

fn t202() -> *const i128 {
    std::ptr::null()
}

fn t203() -> *mut i128 {
    std::ptr::null_mut()
}

fn t204() -> *const i128 {
    std::ptr::null()
}

fn t210() -> u128 {
    0_u128
}

fn t211() -> *mut u128 {
    std::ptr::null_mut()
}

fn t212() -> *const u128 {
    std::ptr::null()
}

fn t213() -> *mut u128 {
    std::ptr::null_mut()
}

fn t214() -> *const u128 {
    std::ptr::null()
}

fn t220() -> () {
    ()
}

fn t221() -> *mut ::libc::c_void {
    std::ptr::null_mut()
}

fn t222() -> *const ::libc::c_void {
    std::ptr::null()
}

fn t223() -> *mut ::libc::c_void {
    std::ptr::null_mut()
}

fn t224() -> *const ::libc::c_void {
    std::ptr::null()
}
