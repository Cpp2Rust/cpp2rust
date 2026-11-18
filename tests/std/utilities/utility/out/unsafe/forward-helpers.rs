extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_0() {
    let mut value: i32 = 1;
    let mut forwarded: i32 = value;
    assert!(((forwarded) == (1)));
    let lvalue: *mut i32 = &mut value;
    (*lvalue) = 2;
    assert!(((value) == (2)));
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, CppFromUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct p6_forward_like_Movable {
    pub value: i32,
}
impl p6_forward_like_Movable {
    pub unsafe fn new(mut value: i32) -> Self {
        let mut this = Self { value: value };
        this
    }
    pub unsafe fn copy_from(other: *const p6_forward_like_Movable) -> Self {
        let mut this = Self {
            value: (*other).value,
        };
        this
    }
    pub unsafe fn move_from(other: *mut p6_forward_like_Movable) -> Self {
        let mut this = Self {
            value: (*other).value,
        };
        (*other).value = 0;
        this
    }
}
impl Clone for p6_forward_like_Movable {
    fn clone(&self) -> Self {
        unsafe { p6_forward_like_Movable::copy_from(self as *const p6_forward_like_Movable) }
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, MoveCtorUnsafe)]
pub struct p6_forward_like_Holder {
    pub member: p6_forward_like_Movable,
}
impl p6_forward_like_Holder {
    pub unsafe fn move_from(_a0: *mut p6_forward_like_Holder) -> Self {
        let mut this = Self {
            member: p6_forward_like_Movable::move_from({ &mut (*_a0).member }),
        };
        this
    }
}
impl Default for p6_forward_like_Holder {
    fn default() -> Self {
        unsafe {
            p6_forward_like_Holder {
                member: p6_forward_like_Movable::new({ 1 }),
            }
        }
    }
}
pub unsafe fn take_1(self_: *mut p6_forward_like_Holder) -> p6_forward_like_Movable {
    return p6_forward_like_Movable::copy_from({ &(*self_).member });
}
pub unsafe fn take_2(self_: *mut p6_forward_like_Holder) -> p6_forward_like_Movable {
    return p6_forward_like_Movable::move_from({ &mut (*self_).member });
}
pub unsafe fn test_3() {
    let mut holder: p6_forward_like_Holder = <p6_forward_like_Holder>::default();
    let mut copied: p6_forward_like_Movable = (unsafe { take_1(&mut holder) });
    assert!(((copied.value) == (1)) && ((holder.member.value) == (1)));
    let mut moved: p6_forward_like_Movable = (unsafe { take_2(&mut holder) });
    assert!(((moved.value) == (1)) && ((holder.member.value) == (0)));
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, CppFromUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct p9_move_Movable {
    pub value: i32,
}
impl p9_move_Movable {
    pub unsafe fn new(mut value: i32) -> Self {
        let mut this = Self { value: value };
        this
    }
    pub unsafe fn move_from(other: *mut p9_move_Movable) -> Self {
        let mut this = Self {
            value: (*other).value,
        };
        (*other).value = 0;
        this
    }
}
pub unsafe fn test_4() {
    let mut number: i32 = 1;
    let mut moved: i32 = number;
    assert!(((moved) == (1)) && ((number) == (1)));
    let mut target: i32 = 2;
    let mut pointer: *mut i32 = (&mut target as *mut i32);
    let mut moved_pointer: *mut i32 = pointer;
    assert!(
        ((moved_pointer) == (&mut target as *mut i32)) && ((pointer) == (&mut target as *mut i32))
    );
    let mut source: p9_move_Movable = p9_move_Movable::new({ 3 });
    let mut destination: p9_move_Movable = p9_move_Movable::move_from({ &mut source });
    assert!(((destination.value) == (3)) && ((source.value) == (0)));
    let mut strings: Vec<Vec<libc::c_char>> = Vec::new();
    let mut text: Vec<libc::c_char> = std::slice::from_raw_parts(
        c"text".as_ptr(),
        (0..).take_while(|&i| *c"text".as_ptr().add(i) != 0).count() + 1,
    )
    .to_vec();
    {
        let __a1 = std::mem::replace(&mut text, vec![0]);
        strings.push(__a1)
    };
    assert!(
        strings[(0_usize)] == {
            std::slice::from_raw_parts(
                c"text".as_ptr(),
                (0..).take_while(|&i| *c"text".as_ptr().add(i) != 0).count() + 1,
            )
            .to_vec()
        }
    );
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, CppFromUnsafe)]
#[cpp_from(i32, new, value)]
pub struct p10_forward_in_template_A {
    pub value: i32,
    pub copies: i32,
    pub moves: i32,
}
impl p10_forward_in_template_A {
    pub unsafe fn new(mut value: i32) -> Self {
        let mut this = Self {
            value: value,
            copies: 0,
            moves: 0,
        };
        this
    }
    pub unsafe fn copy_from(other: *const p10_forward_in_template_A) -> Self {
        let mut this = Self {
            value: (*other).value,
            copies: (((*other).copies) + (1)),
            moves: 0,
        };
        this
    }
    pub unsafe fn move_from(other: *mut p10_forward_in_template_A) -> Self {
        let mut this = Self {
            value: (*other).value,
            copies: 0,
            moves: (((*other).moves) + (1)),
        };
        (*other).value = 0;
        this
    }
}
impl Clone for p10_forward_in_template_A {
    fn clone(&self) -> Self {
        unsafe { p10_forward_in_template_A::copy_from(self as *const p10_forward_in_template_A) }
    }
}
impl Default for p10_forward_in_template_A {
    fn default() -> Self {
        unsafe {
            p10_forward_in_template_A {
                value: 0_i32,
                copies: 0,
                moves: 0,
            }
        }
    }
}
pub unsafe fn make_5(source: *mut p10_forward_in_template_A) -> p10_forward_in_template_A {
    return p10_forward_in_template_A::copy_from({ source });
}
pub unsafe fn make_6(source: *mut p10_forward_in_template_A) -> p10_forward_in_template_A {
    return p10_forward_in_template_A::move_from({ source });
}
pub unsafe fn test_7() {
    let mut a: p10_forward_in_template_A = p10_forward_in_template_A::new({ 1 });
    let mut copied: p10_forward_in_template_A = (unsafe { make_5(&mut a) });
    assert!((((copied.value) == (1)) && ((copied.copies) == (1))) && ((copied.moves) == (0)));
    assert!(((a.value) == (1)));
    let mut moved: p10_forward_in_template_A = (unsafe { make_6(&mut a) });
    assert!((((moved.value) == (1)) && ((moved.copies) == (0))) && ((moved.moves) == (1)));
    assert!(((a.value) == (0)));
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, CppFromUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct p11_move_if_noexcept_NoexceptMove {
    pub value: i32,
}
impl p11_move_if_noexcept_NoexceptMove {
    pub unsafe fn new(mut value: i32) -> Self {
        let mut this = Self { value: value };
        this
    }
    pub unsafe fn copy_from(other: *const p11_move_if_noexcept_NoexceptMove) -> Self {
        let mut this = Self {
            value: (*other).value,
        };
        this
    }
    pub unsafe fn move_from(other: *mut p11_move_if_noexcept_NoexceptMove) -> Self {
        let mut this = Self {
            value: (*other).value,
        };
        (*other).value = 0;
        this
    }
}
impl Clone for p11_move_if_noexcept_NoexceptMove {
    fn clone(&self) -> Self {
        unsafe {
            p11_move_if_noexcept_NoexceptMove::copy_from(
                self as *const p11_move_if_noexcept_NoexceptMove,
            )
        }
    }
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, CppFromUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct p11_move_if_noexcept_ThrowingMove {
    pub value: i32,
}
impl p11_move_if_noexcept_ThrowingMove {
    pub unsafe fn new(mut value: i32) -> Self {
        let mut this = Self { value: value };
        this
    }
    pub unsafe fn copy_from(other: *const p11_move_if_noexcept_ThrowingMove) -> Self {
        let mut this = Self {
            value: (*other).value,
        };
        this
    }
    pub unsafe fn move_from(other: *mut p11_move_if_noexcept_ThrowingMove) -> Self {
        let mut this = Self {
            value: (*other).value,
        };
        (*other).value = 0;
        this
    }
}
impl Clone for p11_move_if_noexcept_ThrowingMove {
    fn clone(&self) -> Self {
        unsafe {
            p11_move_if_noexcept_ThrowingMove::copy_from(
                self as *const p11_move_if_noexcept_ThrowingMove,
            )
        }
    }
}
pub unsafe fn test_8() {
    let mut moved_from: p11_move_if_noexcept_NoexceptMove =
        p11_move_if_noexcept_NoexceptMove::new({ 1 });
    let mut moved: p11_move_if_noexcept_NoexceptMove =
        p11_move_if_noexcept_NoexceptMove::move_from({ &mut moved_from });
    assert!(((moved.value) == (1)) && ((moved_from.value) == (0)));
    let mut copied_from: p11_move_if_noexcept_ThrowingMove =
        p11_move_if_noexcept_ThrowingMove::new({ 2 });
    let mut copied: p11_move_if_noexcept_ThrowingMove =
        p11_move_if_noexcept_ThrowingMove::copy_from({ &copied_from });
    assert!(((copied.value) == (2)) && ((copied_from.value) == (2)));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_0() });
    (unsafe { test_3() });
    (unsafe { test_4() });
    (unsafe { test_7() });
    (unsafe { test_8() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
