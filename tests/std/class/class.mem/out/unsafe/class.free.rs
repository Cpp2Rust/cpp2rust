extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut allocations_0: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct p1_allocation_function_is_static_Allocates {}
impl p1_allocation_function_is_static_Allocates {
    pub unsafe fn operator_new(mut size: usize) -> *mut ::libc::c_void {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut allocations_0)).prefix_inc();
        return libcc2rs::malloc_unsafe(size);
    }
    pub unsafe fn operator_new_arr(mut size: usize) -> *mut ::libc::c_void {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut allocations_0)) += 10;
        return libcc2rs::malloc_unsafe(size);
    }
}
pub unsafe fn test_1() {
    let mut one: *mut i32 = ((unsafe {
        p1_allocation_function_is_static_Allocates::operator_new(::std::mem::size_of::<i32>())
    }) as *mut i32);
    (*one) = 7;
    assert!(((*one) == (7)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut allocations_0)) == (1)));
    libcc2rs::free_unsafe((one as *mut ::libc::c_void));
    let mut many: *mut i32 = ((unsafe {
        p1_allocation_function_is_static_Allocates::operator_new_arr(
            (::std::mem::size_of::<i32>() as usize).wrapping_mul(2_usize),
        )
    }) as *mut i32);
    (*many.offset((0) as isize)) = 1;
    (*many.offset((1) as isize)) = 2;
    assert!((((*many.offset((0) as isize)) + (*many.offset((1) as isize))) == (3)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut allocations_0)) == (11)));
    libcc2rs::free_unsafe((many as *mut ::libc::c_void));
}
pub static mut deallocations_2: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0 });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct p3_deallocation_function_is_static_Deallocates {}
impl p3_deallocation_function_is_static_Deallocates {
    pub unsafe fn operator_delete(mut ptr: *mut ::libc::c_void) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut deallocations_2)).prefix_inc();
        libcc2rs::free_unsafe(ptr);
    }
    pub unsafe fn operator_delete_arr(mut ptr: *mut ::libc::c_void) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut deallocations_2)) += 10;
        libcc2rs::free_unsafe(ptr);
    }
}
pub unsafe fn test_3() {
    let mut one: *mut i32 = ((libcc2rs::malloc_unsafe(::std::mem::size_of::<i32>())) as *mut i32);
    (unsafe {
        p3_deallocation_function_is_static_Deallocates::operator_delete(
            (one as *mut ::libc::c_void),
        )
    });
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut deallocations_2)) == (1)));
    let mut many: *mut i32 =
        ((libcc2rs::malloc_unsafe((::std::mem::size_of::<i32>() as usize).wrapping_mul(2_usize)))
            as *mut i32);
    (unsafe {
        p3_deallocation_function_is_static_Deallocates::operator_delete_arr(
            (many as *mut ::libc::c_void),
        )
    });
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut deallocations_2)) == (11)));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_1() });
    (unsafe { test_3() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const allocations_0);
    std::cell::LazyCell::force(&*&raw const deallocations_2);
}
