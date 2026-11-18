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
pub struct p3_arrow_star_HasTwoInts {
    pub first: i32,
    pub second: i32,
}
pub unsafe fn test_0() {
    let mut pm: isize = (::std::mem::offset_of!(p3_arrow_star_HasTwoInts, second) as isize);
    let mut obj: p3_arrow_star_HasTwoInts = p3_arrow_star_HasTwoInts {
        first: 3,
        second: 4,
    };
    let mut p: *mut p3_arrow_star_HasTwoInts = (&mut obj as *mut p3_arrow_star_HasTwoInts);
    assert!(((*(p).wrapping_byte_offset(pm).cast::<i32>()) == (4)));
    (*(p).wrapping_byte_offset(pm).cast::<i32>()) = 5;
    assert!(((obj.second) == (5)));
    assert!(((obj.first) == (3)));
    assert!(
        ((&mut (*(p).wrapping_byte_offset(pm).cast::<i32>()) as *mut i32)
            == (&mut (*(&mut (*p) as *mut p3_arrow_star_HasTwoInts)
                .wrapping_byte_offset(pm)
                .cast::<i32>()) as *mut i32))
    );
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct p4_object_expression_sequenced_first_HasValue {
    pub value: i32,
}
pub static mut sequence_1: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub static mut object_stamp_2: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub static mut member_stamp_3: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub static mut object_4: std::cell::LazyCell<p4_object_expression_sequenced_first_HasValue> =
    std::cell::LazyCell::new(|| unsafe {
        p4_object_expression_sequenced_first_HasValue { value: 7 }
    });
pub unsafe fn select_object_5() -> *mut p4_object_expression_sequenced_first_HasValue {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut object_stamp_2)) =
        (*(*std::cell::LazyCell::force_mut(&mut *&raw mut sequence_1)).prefix_inc());
    return &mut (*std::cell::LazyCell::force_mut(&mut *&raw mut object_4));
}
pub unsafe fn select_member_6() -> isize {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut member_stamp_3)) =
        (*(*std::cell::LazyCell::force_mut(&mut *&raw mut sequence_1)).prefix_inc());
    return (::std::mem::offset_of!(p4_object_expression_sequenced_first_HasValue, value) as isize);
}
pub unsafe fn test_7() {
    assert!(
        ((*(unsafe { select_object_5() })
            .wrapping_byte_offset((unsafe { select_member_6() }))
            .cast::<i32>())
            == (7))
    );
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut object_stamp_2)) == (1)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut member_stamp_3)) == (2)));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct p6_data_member_of_lvalue_is_lvalue_HasTwoInts {
    pub first: i32,
    pub second: i32,
}
pub unsafe fn test_8() {
    let mut pm: isize =
        (::std::mem::offset_of!(p6_data_member_of_lvalue_is_lvalue_HasTwoInts, second) as isize);
    let mut obj: p6_data_member_of_lvalue_is_lvalue_HasTwoInts =
        p6_data_member_of_lvalue_is_lvalue_HasTwoInts {
            first: 3,
            second: 4,
        };
    assert!(
        ((&mut (*(&mut obj as *mut p6_data_member_of_lvalue_is_lvalue_HasTwoInts)
            .wrapping_byte_offset(pm)
            .cast::<i32>()) as *mut i32)
            == (&mut obj.second as *mut i32))
    );
    let ref_: *mut i32 = &mut (*(&mut obj as *mut p6_data_member_of_lvalue_is_lvalue_HasTwoInts)
        .wrapping_byte_offset(pm)
        .cast::<i32>());
    (*ref_) = 5;
    assert!(((obj.second) == (5)));
    assert!(((obj.first) == (3)));
}
pub static mut copyCount_9: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
pub static mut moveCount_10: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, Default)]
pub struct p6_data_member_of_rvalue_is_xvalue_CountsCopiesAndMoves {}
impl p6_data_member_of_rvalue_is_xvalue_CountsCopiesAndMoves {
    pub unsafe fn copy_from(
        _a0: *const p6_data_member_of_rvalue_is_xvalue_CountsCopiesAndMoves,
    ) -> Self {
        let mut this = Self {};
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copyCount_9)).prefix_inc();
        this
    }
    pub unsafe fn move_from(
        _a0: *mut p6_data_member_of_rvalue_is_xvalue_CountsCopiesAndMoves,
    ) -> Self {
        let mut this = Self {};
        (*std::cell::LazyCell::force_mut(&mut *&raw mut moveCount_10)).prefix_inc();
        this
    }
}
impl Clone for p6_data_member_of_rvalue_is_xvalue_CountsCopiesAndMoves {
    fn clone(&self) -> Self {
        unsafe {
            p6_data_member_of_rvalue_is_xvalue_CountsCopiesAndMoves::copy_from(
                self as *const p6_data_member_of_rvalue_is_xvalue_CountsCopiesAndMoves,
            )
        }
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, MoveCtorUnsafe, Default)]
pub struct p6_data_member_of_rvalue_is_xvalue_HoldsCounted {
    pub member: p6_data_member_of_rvalue_is_xvalue_CountsCopiesAndMoves,
}
impl p6_data_member_of_rvalue_is_xvalue_HoldsCounted {
    pub unsafe fn move_from(_a0: *mut p6_data_member_of_rvalue_is_xvalue_HoldsCounted) -> Self {
        let mut this = Self {
            member: p6_data_member_of_rvalue_is_xvalue_CountsCopiesAndMoves::move_from({
                &mut (*_a0).member
            }),
        };
        this
    }
}
pub unsafe fn test_11() {
    let mut pm: isize =
        (::std::mem::offset_of!(p6_data_member_of_rvalue_is_xvalue_HoldsCounted, member) as isize);
    let mut holder: p6_data_member_of_rvalue_is_xvalue_HoldsCounted =
        <p6_data_member_of_rvalue_is_xvalue_HoldsCounted>::default();
    let mut from_lvalue: p6_data_member_of_rvalue_is_xvalue_CountsCopiesAndMoves =
        p6_data_member_of_rvalue_is_xvalue_CountsCopiesAndMoves::copy_from({
            &(*(&mut holder as *mut p6_data_member_of_rvalue_is_xvalue_HoldsCounted)
                .wrapping_byte_offset(pm)
                .cast::<p6_data_member_of_rvalue_is_xvalue_CountsCopiesAndMoves>())
        });
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut copyCount_9)) == (1)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut moveCount_10)) == (0)));
    let mut from_xvalue: p6_data_member_of_rvalue_is_xvalue_CountsCopiesAndMoves =
        p6_data_member_of_rvalue_is_xvalue_CountsCopiesAndMoves::move_from({
            &mut (*(&mut holder as *mut p6_data_member_of_rvalue_is_xvalue_HoldsCounted)
                .wrapping_byte_offset(pm)
                .cast::<p6_data_member_of_rvalue_is_xvalue_CountsCopiesAndMoves>())
        });
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut copyCount_9)) == (1)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut moveCount_10)) == (1)));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_0() });
    (unsafe { test_7() });
    (unsafe { test_8() });
    (unsafe { test_11() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const sequence_1);
    std::cell::LazyCell::force(&*&raw const object_stamp_2);
    std::cell::LazyCell::force(&*&raw const member_stamp_3);
    std::cell::LazyCell::force(&*&raw const object_4);
    std::cell::LazyCell::force(&*&raw const copyCount_9);
    std::cell::LazyCell::force(&*&raw const moveCount_10);
}
