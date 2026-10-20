// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

// std::swap, for the translation rules: cpp_swap!(a, b), or
// cpp_swap_unsafe!(a, b), swaps the values that a and b point to as C++
// does:
//   T tmp(std::move(a)); a = std::move(b); b = std::move(tmp); ~tmp
// Each step is done with the special member functions of the type that C++
// picks, which may have side effects. The macros call
// `(&&&Construct(..)).construct()`, `(&&&&&Assign(..)).assign()` and
// `(&&Destroy(..)).destroy()`, and autoref specialization picks, among the
// impls below, the one with the most `&`s whose bounds the type satisfies:
//   - the temporary is made with the move constructor (MoveCtor), or else
//     with the copy constructor (Clone);
//   - a value is assigned with the move assignment operator (MoveAssign), or
//     else with the copy assignment operator (CopyAssign), as C++ copies the
//     types without one; the other types are assigned as a whole, by moving
//     the value with the move constructor (MoveCtor), or else by copying it
//     (Clone);
//   - the temporary is destroyed with the destructor, if any (Destructor).
// The values are accessed one at a time, so they may be in the same
// allocation, e.g., two fields of a struct.
//
// The types without any of those special members are only copied by C++, and
// so their values (or bytes, if reinterpreted) are swapped instead, without a
// temporary, as `(&&&&&Trivial(..)).is_trivial()` tells. The other steps are
// still compiled for them, and are unreachable if they can't be copied.

#![allow(private_bounds, clippy::missing_safety_doc)]

use std::cell::RefCell;
use std::rc::Rc;

use crate::capture::{
    CopyAssign, CopyAssignUnsafe, Destructor, DestructorUnsafe, MoveAssign, MoveAssignUnsafe,
    MoveCtor, MoveCtorUnsafe,
};
use crate::rc::{AsPointer, Ptr, Value};
use crate::reinterpret::ByteRepr;

#[macro_export]
macro_rules! cpp_swap {
    ($a:expr, $b:expr) => {{
        #[allow(unused_imports)]
        use $crate::__swap::{
            AssignClone, AssignCopy, AssignMove, AssignMoveCtor, AssignNone, ConstructCopy,
            ConstructMove, ConstructNone, DestroyNone, DestroyWithDtor, HasCopyAssign,
            HasDestructor, HasMoveAssign, HasMoveCtor, HasNone, TakeClone, TakeNone,
        };
        let __a = $a;
        let __b = $b;
        if (&&&&&$crate::__swap::Trivial(&__a)).is_trivial() {
            __a.swap_with(&__b, (&&$crate::__swap::Take(&__a)).take());
        } else {
            let __tmp = $crate::__swap::temporary((&&&$crate::__swap::Construct(&__a)).construct());
            let __tmp_ptr = $crate::__swap::pointer(&__tmp);
            (&&&&&$crate::__swap::Assign(&__a, &__b)).assign();
            (&&&&&$crate::__swap::Assign(&__b, &__tmp_ptr)).assign();
            (&&$crate::__swap::Destroy(&__tmp_ptr)).destroy();
        }
    }};
}

#[macro_export]
macro_rules! cpp_swap_unsafe {
    ($a:expr, $b:expr) => {{
        #[allow(unused_imports)]
        use $crate::__swap::{
            AssignCloneUnsafe, AssignCopyUnsafe, AssignMoveCtorUnsafe, AssignMoveUnsafe,
            AssignNoneUnsafe, ConstructCopyUnsafe, ConstructMoveUnsafe, ConstructNoneUnsafe,
            DestroyNoneUnsafe, DestroyWithDtorUnsafe, HasCopyAssignUnsafe, HasDestructorUnsafe,
            HasMoveAssignUnsafe, HasMoveCtorUnsafe, HasNoneUnsafe,
        };
        let __a: *mut _ = $a;
        let __b: *mut _ = $b;
        if (&&&&&$crate::__swap::TrivialUnsafe(&__a)).is_trivial() {
            ::std::ptr::swap(__a, __b);
        } else {
            let mut __tmp = (&&&$crate::__swap::ConstructUnsafe(__a)).construct();
            let __tmp_ptr: *mut _ = &mut __tmp;
            (&&&&&$crate::__swap::AssignUnsafe(__a, __b)).assign();
            (&&&&&$crate::__swap::AssignUnsafe(__b, __tmp_ptr)).assign();
            (&&$crate::__swap::DestroyUnsafe(__tmp_ptr)).destroy();
        }
    }};
}

// Whether the type has none of the special members that the swap calls.
macro_rules! trivial {
    ($wrapper:ident, $ptr:ty, $none:ident, $($tr:ident: $bound:ident for $recv:ty),*) => {
        pub struct $wrapper<'a, T>(pub &'a $ptr);
        $(
            pub trait $tr {
                fn is_trivial(self) -> bool;
            }

            impl<T: $bound> $tr for $recv {
                fn is_trivial(self) -> bool {
                    false
                }
            }
        )*

        pub trait $none {
            fn is_trivial(self) -> bool;
        }

        impl<T> $none for &$wrapper<'_, T> {
            fn is_trivial(self) -> bool {
                true
            }
        }
    };
}

trivial!(Trivial, Ptr<T>, HasNone,
    HasMoveCtor: MoveCtor for &&&&&Trivial<'_, T>,
    HasMoveAssign: MoveAssign for &&&&Trivial<'_, T>,
    HasCopyAssign: CopyAssign for &&&Trivial<'_, T>,
    HasDestructor: Destructor for &&Trivial<'_, T>
);

trivial!(TrivialUnsafe, *mut T, HasNoneUnsafe,
    HasMoveCtorUnsafe: MoveCtorUnsafe for &&&&&TrivialUnsafe<'_, T>,
    HasMoveAssignUnsafe: MoveAssignUnsafe for &&&&TrivialUnsafe<'_, T>,
    HasCopyAssignUnsafe: CopyAssignUnsafe for &&&TrivialUnsafe<'_, T>,
    HasDestructorUnsafe: DestructorUnsafe for &&TrivialUnsafe<'_, T>
);

#[cold]
#[inline(never)]
fn not_swappable() -> ! {
    panic!("std::swap of a type that can't be copied nor moved")
}

// How Ptr::swap_with copies a value of a trivial type into a temporary, if
// it needs one.
pub struct Take<'a, T>(pub &'a Ptr<T>);

pub trait TakeClone<T> {
    fn take(self) -> fn(&Ptr<T>) -> T;
}

pub trait TakeNone<T> {
    fn take(self) -> fn(&Ptr<T>) -> T;
}

impl<T: Clone + ByteRepr> TakeClone<T> for &&Take<'_, T> {
    fn take(self) -> fn(&Ptr<T>) -> T {
        Ptr::read
    }
}

impl<T> TakeNone<T> for &Take<'_, T> {
    fn take(self) -> fn(&Ptr<T>) -> T {
        |_| not_swappable()
    }
}

// The temporary of cpp_swap!, and a pointer to it.
pub fn temporary<T>(value: T) -> Value<T> {
    Rc::new(RefCell::new(value))
}

pub fn pointer<T>(value: &Value<T>) -> Ptr<T> {
    <Value<T> as AsPointer<T>>::as_pointer(value)
}

pub struct Construct<'a, T>(pub &'a Ptr<T>);

pub trait ConstructMove<T> {
    fn construct(self) -> T;
}

pub trait ConstructCopy<T> {
    fn construct(self) -> T;
}

pub trait ConstructNone<T> {
    fn construct(self) -> T;
}

impl<T: MoveCtor> ConstructMove<T> for &&&Construct<'_, T> {
    fn construct(self) -> T {
        T::move_ctor(self.0.clone())
    }
}

impl<T: Clone + ByteRepr> ConstructCopy<T> for &&Construct<'_, T> {
    fn construct(self) -> T {
        self.0.read()
    }
}

impl<T> ConstructNone<T> for &Construct<'_, T> {
    fn construct(self) -> T {
        not_swappable()
    }
}

// Assigns the value that the second pointer points to to the first one.
pub struct Assign<'a, T>(pub &'a Ptr<T>, pub &'a Ptr<T>);

pub trait AssignMove {
    fn assign(self);
}

pub trait AssignCopy {
    fn assign(self);
}

pub trait AssignMoveCtor {
    fn assign(self);
}

pub trait AssignClone {
    fn assign(self);
}

pub trait AssignNone {
    fn assign(self);
}

impl<T: MoveAssign> AssignMove for &&&&&Assign<'_, T> {
    fn assign(self) {
        T::move_assign(self.0, self.1.clone())
    }
}

impl<T: CopyAssign> AssignCopy for &&&&Assign<'_, T> {
    fn assign(self) {
        T::copy_assign(self.0, self.1.clone())
    }
}

impl<T: MoveCtor + ByteRepr> AssignMoveCtor for &&&Assign<'_, T> {
    fn assign(self) {
        self.0.write(T::move_ctor(self.1.clone()))
    }
}

impl<T: Clone + ByteRepr> AssignClone for &&Assign<'_, T> {
    fn assign(self) {
        self.0.write(self.1.read())
    }
}

impl<T> AssignNone for &Assign<'_, T> {
    fn assign(self) {
        not_swappable()
    }
}

pub struct Destroy<'a, T>(pub &'a Ptr<T>);

pub trait DestroyWithDtor {
    fn destroy(self);
}

pub trait DestroyNone {
    fn destroy(self);
}

impl<T: Destructor> DestroyWithDtor for &&Destroy<'_, T> {
    fn destroy(self) {
        T::destroy(self.0.clone())
    }
}

impl<T> DestroyNone for &Destroy<'_, T> {
    fn destroy(self) {}
}

pub struct ConstructUnsafe<T>(pub *mut T);

pub trait ConstructMoveUnsafe<T> {
    unsafe fn construct(self) -> T;
}

pub trait ConstructCopyUnsafe<T> {
    unsafe fn construct(self) -> T;
}

pub trait ConstructNoneUnsafe<T> {
    unsafe fn construct(self) -> T;
}

impl<T: MoveCtorUnsafe> ConstructMoveUnsafe<T> for &&&ConstructUnsafe<T> {
    unsafe fn construct(self) -> T {
        unsafe { T::move_ctor(self.0) }
    }
}

impl<T: Clone> ConstructCopyUnsafe<T> for &&ConstructUnsafe<T> {
    unsafe fn construct(self) -> T {
        unsafe { (*self.0).clone() }
    }
}

impl<T> ConstructNoneUnsafe<T> for &ConstructUnsafe<T> {
    unsafe fn construct(self) -> T {
        not_swappable()
    }
}

pub struct AssignUnsafe<T>(pub *mut T, pub *mut T);

pub trait AssignMoveUnsafe {
    unsafe fn assign(self);
}

pub trait AssignCopyUnsafe {
    unsafe fn assign(self);
}

pub trait AssignMoveCtorUnsafe {
    unsafe fn assign(self);
}

pub trait AssignCloneUnsafe {
    unsafe fn assign(self);
}

pub trait AssignNoneUnsafe {
    unsafe fn assign(self);
}

impl<T: MoveAssignUnsafe> AssignMoveUnsafe for &&&&&AssignUnsafe<T> {
    unsafe fn assign(self) {
        unsafe { T::move_assign(self.0, self.1) }
    }
}

impl<T: CopyAssignUnsafe> AssignCopyUnsafe for &&&&AssignUnsafe<T> {
    unsafe fn assign(self) {
        unsafe { T::copy_assign(self.0, self.1) }
    }
}

impl<T: MoveCtorUnsafe> AssignMoveCtorUnsafe for &&&AssignUnsafe<T> {
    unsafe fn assign(self) {
        unsafe { *self.0 = T::move_ctor(self.1) }
    }
}

impl<T: Clone> AssignCloneUnsafe for &&AssignUnsafe<T> {
    unsafe fn assign(self) {
        unsafe { *self.0 = (*self.1).clone() }
    }
}

impl<T> AssignNoneUnsafe for &AssignUnsafe<T> {
    unsafe fn assign(self) {
        not_swappable()
    }
}

pub struct DestroyUnsafe<T>(pub *mut T);

pub trait DestroyWithDtorUnsafe {
    unsafe fn destroy(self);
}

pub trait DestroyNoneUnsafe {
    unsafe fn destroy(self);
}

impl<T: DestructorUnsafe> DestroyWithDtorUnsafe for &&DestroyUnsafe<T> {
    unsafe fn destroy(self) {
        unsafe { T::destroy(self.0) }
    }
}

impl<T> DestroyNoneUnsafe for &DestroyUnsafe<T> {
    unsafe fn destroy(self) {}
}
