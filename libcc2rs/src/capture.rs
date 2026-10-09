// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

// The copy and move constructors of the captures of lambda! and
// lambda_unsafe!, synthesized from the type of each capture. The macros call
// `(&&&Capture(..)).copy_capture()` (or `move_capture()`), and autoref
// specialization picks, among the impls below, the one with the most `&`s
// whose bounds the capture's type satisfies:
//   - a lambda (a FnPtr) is copied and moved with its own constructors;
//   - a type with a C++ move constructor (MoveCtor) is moved with it;
//   - any other type is copied with Clone, also when it is moved, as C++
//     copies the types without a move constructor;
//   - a type that is not Clone is not copyable, and neither is the lambda,
//     so its copy constructor is unreachable.
// Likewise, the destructor of the lambda (`destroy_capture()`) destroys the
// captures that have a C++ destructor (Destructor), in reverse order.
// In the refcount model, a capture by value is a Value, which is copied
// into a new Value, and a capture by reference is a Ptr, which is copied.

#![allow(private_bounds, clippy::missing_safety_doc)]

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

use crate::fn_ptr::{FnPtr, FnSig};
use crate::rc::{AsPointer, DeepClone, Ptr, Value};
use crate::reinterpret::ByteRepr;

// The C++ move constructor of a type, in the refcount model, for the types
// whose move isn't a copy. Records implement it with #[derive(MoveCtor)].
pub trait MoveCtor: Sized {
    fn move_ctor(src: Ptr<Self>) -> Self;
}

// The C++ move constructor of a type, in the unsafe model. Records implement
// it with #[derive(MoveCtorUnsafe)].
pub trait MoveCtorUnsafe: Sized {
    /// # Safety
    /// `src` must point to a live value.
    unsafe fn move_ctor(src: *mut Self) -> Self;
}

impl<T: FnSig> MoveCtor for FnPtr<T> {
    fn move_ctor(src: Ptr<Self>) -> Self {
        src.with(FnPtr::move_from)
    }
}

impl<T: FnSig> MoveCtorUnsafe for FnPtr<T> {
    unsafe fn move_ctor(src: *mut Self) -> Self {
        unsafe { (*src).move_from() }
    }
}

// The library types whose move leaves the source empty.
macro_rules! impl_move_ctor_by_take {
    ($(<$($param:ident),*> $ty:ty),* $(,)?) => {$(
        impl<$($param),*> MoveCtor for $ty
        where
            Self: ByteRepr,
        {
            fn move_ctor(src: Ptr<Self>) -> Self {
                src.with_mut(std::mem::take)
            }
        }

        impl<$($param),*> MoveCtorUnsafe for $ty {
            unsafe fn move_ctor(src: *mut Self) -> Self {
                std::mem::take(unsafe { &mut *src })
            }
        }
    )*};
}

impl_move_ctor_by_take!(
    <T> Vec<T>,
    <K, V> BTreeMap<K, V>,
);

// unique_ptr.
impl<T> MoveCtor for Option<Value<T>>
where
    Self: ByteRepr,
{
    fn move_ctor(src: Ptr<Self>) -> Self {
        src.with_mut(Option::take)
    }
}

impl<T: ?Sized> MoveCtorUnsafe for Option<Box<T>> {
    unsafe fn move_ctor(src: *mut Self) -> Self {
        unsafe { (*src).take() }
    }
}

impl<T: MoveCtorUnsafe, const N: usize> MoveCtorUnsafe for [T; N] {
    unsafe fn move_ctor(src: *mut Self) -> Self {
        let src = src as *mut T;
        std::array::from_fn(|i| unsafe { T::move_ctor(src.add(i)) })
    }
}

// The C++ destructor of a type, in the refcount model, for the types that
// have one. Records implement it with #[derive(Destructor)].
pub trait Destructor: Sized {
    fn destroy(p: Ptr<Self>);
}

// The C++ destructor of a type, in the unsafe model. Records implement it
// with #[derive(DestructorUnsafe)].
pub trait DestructorUnsafe: Sized {
    /// # Safety
    /// `p` must point to a live value.
    unsafe fn destroy(p: *mut Self);
}

impl<T: FnSig> Destructor for FnPtr<T> {
    fn destroy(p: Ptr<Self>) {
        p.with(|f| f.destroy())
    }
}

impl<T: FnSig> DestructorUnsafe for FnPtr<T> {
    unsafe fn destroy(p: *mut Self) {
        unsafe { (*p).destroy() }
    }
}

impl<T: DestructorUnsafe, const N: usize> DestructorUnsafe for [T; N] {
    unsafe fn destroy(p: *mut Self) {
        let p = p as *mut T;
        for i in (0..N).rev() {
            unsafe { T::destroy(p.add(i)) }
        }
    }
}

// A capture of lambda!, which holds a Value or a Ptr.
pub struct Capture<'a, T>(pub &'a T);

pub trait CopyLambda<T> {
    fn copy_capture(&self) -> T;
}

impl<S: FnSig> CopyLambda<Value<FnPtr<S>>> for &&Capture<'_, Value<FnPtr<S>>> {
    fn copy_capture(&self) -> Value<FnPtr<S>> {
        Rc::new(RefCell::new(self.0.borrow().copy_from()))
    }
}

pub trait CopyClone<T> {
    fn copy_capture(&self) -> T;
}

impl<T: Clone> CopyClone<Value<T>> for &Capture<'_, Value<T>> {
    fn copy_capture(&self) -> Value<T> {
        self.0.deep_clone()
    }
}

impl<T> CopyClone<Ptr<T>> for &Capture<'_, Ptr<T>> {
    fn copy_capture(&self) -> Ptr<T> {
        self.0.clone()
    }
}

pub trait CopyNone<T> {
    fn copy_capture(&self) -> T;
}

impl<T> CopyNone<T> for Capture<'_, T> {
    fn copy_capture(&self) -> T {
        unreachable!("the lambda has no copy constructor")
    }
}

pub trait MoveElems<T> {
    fn move_capture(&self) -> T;
}

impl<T: MoveCtor> MoveElems<Value<Box<[T]>>> for &&&Capture<'_, Value<Box<[T]>>> {
    fn move_capture(&self) -> Value<Box<[T]>> {
        let src: Ptr<T> = self.0.as_pointer();
        let len = self.0.borrow().len();
        Rc::new(RefCell::new(
            (0..len).map(|i| T::move_ctor(src.offset(i))).collect(),
        ))
    }
}

pub trait MoveWithCtor<T> {
    fn move_capture(&self) -> T;
}

impl<T: MoveCtor> MoveWithCtor<Value<T>> for &&Capture<'_, Value<T>> {
    fn move_capture(&self) -> Value<T> {
        Rc::new(RefCell::new(T::move_ctor(AsPointer::<T>::as_pointer(
            self.0,
        ))))
    }
}

pub trait MoveClone<T> {
    fn move_capture(&self) -> T;
}

impl<T: Clone> MoveClone<Value<T>> for &Capture<'_, Value<T>> {
    fn move_capture(&self) -> Value<T> {
        self.0.deep_clone()
    }
}

impl<T> MoveClone<Ptr<T>> for &Capture<'_, Ptr<T>> {
    fn move_capture(&self) -> Ptr<T> {
        self.0.clone()
    }
}

pub trait MoveNone<T> {
    fn move_capture(&self) -> T;
}

impl<T> MoveNone<T> for Capture<'_, T> {
    fn move_capture(&self) -> T {
        unreachable!("the lambda has no move constructor")
    }
}

pub trait DestroyElems {
    fn destroy_capture(&self);
}

impl<T: Destructor> DestroyElems for &&Capture<'_, Value<Box<[T]>>> {
    fn destroy_capture(&self) {
        let p: Ptr<T> = self.0.as_pointer();
        for i in (0..self.0.borrow().len()).rev() {
            T::destroy(p.offset(i));
        }
    }
}

pub trait DestroyWithDtor {
    fn destroy_capture(&self);
}

impl<T: Destructor> DestroyWithDtor for &Capture<'_, Value<T>> {
    fn destroy_capture(&self) {
        T::destroy(AsPointer::<T>::as_pointer(self.0))
    }
}

pub trait DestroyNone {
    fn destroy_capture(&self);
}

impl<T> DestroyNone for Capture<'_, T> {
    fn destroy_capture(&self) {}
}

// A capture of lambda_unsafe!. Its methods are unsafe because they read
// through its pointer, which must point to the capture.
pub struct CaptureUnsafe<T>(pub *mut T);

pub trait CopyLambdaUnsafe<T> {
    unsafe fn copy_capture(&self) -> T;
}

impl<S: FnSig> CopyLambdaUnsafe<FnPtr<S>> for &&CaptureUnsafe<FnPtr<S>> {
    unsafe fn copy_capture(&self) -> FnPtr<S> {
        unsafe { (*self.0).copy_from() }
    }
}

pub trait CopyCloneUnsafe<T> {
    unsafe fn copy_capture(&self) -> T;
}

impl<T: Clone> CopyCloneUnsafe<T> for &CaptureUnsafe<T> {
    unsafe fn copy_capture(&self) -> T {
        unsafe { (*self.0).clone() }
    }
}

pub trait CopyNoneUnsafe<T> {
    unsafe fn copy_capture(&self) -> T;
}

impl<T> CopyNoneUnsafe<T> for CaptureUnsafe<T> {
    unsafe fn copy_capture(&self) -> T {
        unreachable!("the lambda has no copy constructor")
    }
}

pub trait MoveWithCtorUnsafe<T> {
    unsafe fn move_capture(&self) -> T;
}

impl<T: MoveCtorUnsafe> MoveWithCtorUnsafe<T> for &&CaptureUnsafe<T> {
    unsafe fn move_capture(&self) -> T {
        unsafe { T::move_ctor(self.0) }
    }
}

pub trait MoveCloneUnsafe<T> {
    unsafe fn move_capture(&self) -> T;
}

impl<T: Clone> MoveCloneUnsafe<T> for &CaptureUnsafe<T> {
    unsafe fn move_capture(&self) -> T {
        unsafe { (*self.0).clone() }
    }
}

pub trait MoveNoneUnsafe<T> {
    unsafe fn move_capture(&self) -> T;
}

impl<T> MoveNoneUnsafe<T> for CaptureUnsafe<T> {
    unsafe fn move_capture(&self) -> T {
        unreachable!("the lambda has no move constructor")
    }
}

pub trait DestroyWithDtorUnsafe {
    unsafe fn destroy_capture(&self);
}

impl<T: DestructorUnsafe> DestroyWithDtorUnsafe for &CaptureUnsafe<T> {
    unsafe fn destroy_capture(&self) {
        unsafe { T::destroy(self.0) }
    }
}

pub trait DestroyNoneUnsafe {
    unsafe fn destroy_capture(&self);
}

impl<T> DestroyNoneUnsafe for CaptureUnsafe<T> {
    unsafe fn destroy_capture(&self) {}
}
