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
pub struct p2_pointer_to_array_member_HasIntArray {
    pub before: i32,
    pub elements: [i32; 3],
}
pub unsafe fn test_0() {
    let mut pm: isize =
        (::std::mem::offset_of!(p2_pointer_to_array_member_HasIntArray, elements) as isize);
    let mut obj: p2_pointer_to_array_member_HasIntArray = p2_pointer_to_array_member_HasIntArray {
        before: 1,
        elements: [2, 3, 4],
    };
    assert!(
        (((*(&mut obj as *mut p2_pointer_to_array_member_HasIntArray)
            .wrapping_byte_offset(pm)
            .cast::<[i32; 3]>())[(0) as usize])
            == (2))
    );
    assert!(
        (((*(&mut obj as *mut p2_pointer_to_array_member_HasIntArray)
            .wrapping_byte_offset(pm)
            .cast::<[i32; 3]>())[(2) as usize])
            == (4))
    );
    (*(&mut obj as *mut p2_pointer_to_array_member_HasIntArray)
        .wrapping_byte_offset(pm)
        .cast::<[i32; 3]>())[(1) as usize] = 7;
    assert!(((obj.elements[(1) as usize]) == (7)));
    assert!(((obj.before) == (1)));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct p2_pointer_to_data_member_HasTwoInts {
    pub first: i32,
    pub second: i32,
}
pub unsafe fn test_1() {
    let mut pm: isize =
        (::std::mem::offset_of!(p2_pointer_to_data_member_HasTwoInts, first) as isize);
    let mut obj: p2_pointer_to_data_member_HasTwoInts = p2_pointer_to_data_member_HasTwoInts {
        first: 3,
        second: 4,
    };
    assert!(
        ((*(&mut obj as *mut p2_pointer_to_data_member_HasTwoInts)
            .wrapping_byte_offset(pm)
            .cast::<i32>())
            == (3))
    );
    (*(&mut obj as *mut p2_pointer_to_data_member_HasTwoInts)
        .wrapping_byte_offset(pm)
        .cast::<i32>()) = 5;
    assert!(((obj.first) == (5)));
    assert!(((obj.second) == (4)));
    pm = (::std::mem::offset_of!(p2_pointer_to_data_member_HasTwoInts, second) as isize);
    assert!(
        ((*(&mut obj as *mut p2_pointer_to_data_member_HasTwoInts)
            .wrapping_byte_offset(pm)
            .cast::<i32>())
            == (4))
    );
    (*(&mut obj as *mut p2_pointer_to_data_member_HasTwoInts)
        .wrapping_byte_offset(pm)
        .cast::<i32>()) = 6;
    assert!(((obj.first) == (5)));
    assert!(((obj.second) == (6)));
}
pub static mut pm_2: std::cell::LazyCell<isize> = std::cell::LazyCell::new(|| unsafe { -1_isize });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct p2_pointer_to_member_of_incomplete_class_CompletedLater {
    pub first: i32,
    pub second: i32,
}
pub unsafe fn test_3() {
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut pm_2)) == -1_isize));
    (*std::cell::LazyCell::force_mut(&mut *&raw mut pm_2)) = (::std::mem::offset_of!(
        p2_pointer_to_member_of_incomplete_class_CompletedLater,
        second
    ) as isize);
    let mut obj: p2_pointer_to_member_of_incomplete_class_CompletedLater =
        p2_pointer_to_member_of_incomplete_class_CompletedLater {
            first: 3,
            second: 4,
        };
    assert!(
        ((*(&mut obj as *mut p2_pointer_to_member_of_incomplete_class_CompletedLater)
            .wrapping_byte_offset((*std::cell::LazyCell::force_mut(&mut *&raw mut pm_2)))
            .cast::<i32>())
            == (4))
    );
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct p2_pointer_to_member_of_pointer_to_member_type_HasTwoInts {
    pub first: i32,
    pub second: i32,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct p2_pointer_to_member_of_pointer_to_member_type_HoldsSelector {
    pub selector: isize,
}
impl Default for p2_pointer_to_member_of_pointer_to_member_type_HoldsSelector {
    fn default() -> Self {
        unsafe {
            p2_pointer_to_member_of_pointer_to_member_type_HoldsSelector { selector: -1_isize }
        }
    }
}
pub unsafe fn test_4() {
    let mut pm: isize = (::std::mem::offset_of!(
        p2_pointer_to_member_of_pointer_to_member_type_HoldsSelector,
        selector
    ) as isize);
    let mut holder: p2_pointer_to_member_of_pointer_to_member_type_HoldsSelector =
        p2_pointer_to_member_of_pointer_to_member_type_HoldsSelector {
            selector: (::std::mem::offset_of!(
                p2_pointer_to_member_of_pointer_to_member_type_HasTwoInts,
                second
            ) as isize),
        };
    let mut obj: p2_pointer_to_member_of_pointer_to_member_type_HasTwoInts =
        p2_pointer_to_member_of_pointer_to_member_type_HasTwoInts {
            first: 3,
            second: 4,
        };
    assert!(
        ((*(&mut obj as *mut p2_pointer_to_member_of_pointer_to_member_type_HasTwoInts)
            .wrapping_byte_offset(
                (*(&mut holder
                    as *mut p2_pointer_to_member_of_pointer_to_member_type_HoldsSelector)
                    .wrapping_byte_offset(pm)
                    .cast::<isize>())
            )
            .cast::<i32>())
            == (4))
    );
    (*(&mut holder as *mut p2_pointer_to_member_of_pointer_to_member_type_HoldsSelector)
        .wrapping_byte_offset(pm)
        .cast::<isize>()) = (::std::mem::offset_of!(
        p2_pointer_to_member_of_pointer_to_member_type_HasTwoInts,
        first
    ) as isize);
    assert!(
        ((*(&mut obj as *mut p2_pointer_to_member_of_pointer_to_member_type_HasTwoInts)
            .wrapping_byte_offset((holder.selector))
            .cast::<i32>())
            == (3))
    );
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_0() });
    (unsafe { test_1() });
    (unsafe { test_3() });
    (unsafe { test_4() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const pm_2);
}
