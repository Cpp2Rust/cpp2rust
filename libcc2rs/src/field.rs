// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

// Pointers to the fields of structs.
//
// Struct fields are stored inline in their struct, and hence a whole struct
// lives in a single Value. A pointer to a field records the allocation that
// holds the struct and the byte offset of the field in it (in the C layout),
// similarly to how a pointer to an array element records the array and the
// index of the element. The field is found from the offset when accessed.
//
// Arrays and vectors are stored inline too. A field pointer points to a
// single object, which may be a whole array or vector. A pointer to an element
// of one is a Field pointer too, which records the allocation of the struct,
// the offset of the field and the index of the element. Structs
// that are elements of those have no offset in the allocation, so pointers to
// their fields are Nested pointers, which reach the field through the pointer
// to the struct (see FieldOf).

use std::any::{Any, TypeId};
use std::cell::{Cell, Ref, RefCell, RefMut};
use std::marker::PhantomData;
use std::rc::{Rc, Weak};

use crate::rc::{
    AsPointer, FieldShape, Ptr, PtrKind, SliceParent, SliceRoot, StrongPtr, Value, dangling,
    null_deref,
};
use crate::reinterpret::ByteRepr;

// A struct whose fields can be pointed to. Implemented by #[derive(Record)],
// from the C offsets of the fields given by their #[offset(...)] attributes.
pub trait Record: ByteRepr {
    // A struct with a usize member per field, holding the offset of the
    // field. Used by field_ptr! to name fields.
    type Offsets;
    const OFFSETS: Self::Offsets;

    // The object of type `ty` at byte `offset` of the struct: the struct
    // itself, a field, a field of a field, and so on.
    fn locate(&self, offset: usize, ty: TypeId) -> Option<&dyn Any>;
    fn locate_mut(&mut self, offset: usize, ty: TypeId) -> Option<&mut dyn Any>;

    // The length of the array or vector at byte `offset` of the struct, a
    // field or a field of a field.
    fn len_at(&self, offset: usize) -> Option<usize>;
}

#[cold]
#[inline(never)]
#[track_caller]
fn invalid_field() -> ! {
    panic!("ub: invalid field pointer")
}

// The allocation of a Value<R>, Value<Box<[R]>> or Value<Vec<R>>, where R is
// a struct, that field pointers point into.
pub trait Root: Any {
    fn borrow_at(&self, offset: usize, ty: TypeId) -> Ref<'_, dyn Any>;
    fn borrow_at_mut(&self, offset: usize, ty: TypeId) -> RefMut<'_, dyn Any>;
    // The length of the array or vector at byte `offset`, which is found
    // without knowing its type.
    fn len_at(&self, offset: usize) -> usize;
}

impl<R: Record> Root for RefCell<R> {
    #[inline]
    fn borrow_at(&self, offset: usize, ty: TypeId) -> Ref<'_, dyn Any> {
        Ref::map(self.borrow(), |r| {
            r.locate(offset, ty).unwrap_or_else(|| invalid_field())
        })
    }

    #[inline]
    fn borrow_at_mut(&self, offset: usize, ty: TypeId) -> RefMut<'_, dyn Any> {
        RefMut::map(self.borrow_mut(), |r| {
            r.locate_mut(offset, ty).unwrap_or_else(|| invalid_field())
        })
    }

    fn len_at(&self, offset: usize) -> usize {
        self.borrow()
            .len_at(offset)
            .unwrap_or_else(|| invalid_field())
    }
}

// The element of an array of structs, and the offset in it, that contains
// byte `offset` of the array.
#[inline]
fn split<R: Record>(offset: usize) -> (usize, usize) {
    let size = R::byte_size();
    (offset / size, offset % size)
}

impl<R: Record> Root for RefCell<Box<[R]>> {
    #[inline]
    fn borrow_at(&self, offset: usize, ty: TypeId) -> Ref<'_, dyn Any> {
        let (idx, offset) = split::<R>(offset);
        Ref::map(self.borrow(), |a| {
            a[idx].locate(offset, ty).unwrap_or_else(|| invalid_field())
        })
    }

    #[inline]
    fn borrow_at_mut(&self, offset: usize, ty: TypeId) -> RefMut<'_, dyn Any> {
        let (idx, offset) = split::<R>(offset);
        RefMut::map(self.borrow_mut(), |a| {
            a[idx]
                .locate_mut(offset, ty)
                .unwrap_or_else(|| invalid_field())
        })
    }

    fn len_at(&self, offset: usize) -> usize {
        let (idx, offset) = split::<R>(offset);
        self.borrow()[idx]
            .len_at(offset)
            .unwrap_or_else(|| invalid_field())
    }
}

impl<R: Record> Root for RefCell<Vec<R>> {
    #[inline]
    fn borrow_at(&self, offset: usize, ty: TypeId) -> Ref<'_, dyn Any> {
        let (idx, offset) = split::<R>(offset);
        Ref::map(self.borrow(), |v| {
            v[idx].locate(offset, ty).unwrap_or_else(|| invalid_field())
        })
    }

    #[inline]
    fn borrow_at_mut(&self, offset: usize, ty: TypeId) -> RefMut<'_, dyn Any> {
        let (idx, offset) = split::<R>(offset);
        RefMut::map(self.borrow_mut(), |v| {
            v[idx]
                .locate_mut(offset, ty)
                .unwrap_or_else(|| invalid_field())
        })
    }

    fn len_at(&self, offset: usize) -> usize {
        let (idx, offset) = split::<R>(offset);
        self.borrow()[idx]
            .len_at(offset)
            .unwrap_or_else(|| invalid_field())
    }
}

// Creates pointers to the fields of the struct S pointed to by self. Use it
// through the field_ptr!, array_field_ptr! and field_ref! macros, which pass
// the field as functions that give a reference to it, and as the function
// that gives its offset in S::OFFSETS.
pub trait FieldPtr<S: Record> {
    // The pointer that the struct is reached through.
    fn record_ptr(&self) -> Ptr<S>;

    // A pointer to the field.
    #[inline]
    fn field_ptr<T, G, M>(&self, get: G, get_mut: M, offset: fn(&S::Offsets) -> usize) -> Ptr<T>
    where
        T: ByteRepr,
        G: Fn(&S) -> &T + Copy + 'static,
        M: Fn(&mut S) -> &mut T + Copy + 'static,
    {
        self.record_ptr().field_ptr(get, get_mut, offset)
    }

    // A pointer to the first element of the array field. If the struct is
    // reinterpreted memory, the pointer points into that memory, and can go
    // past the end of the array.
    #[inline]
    fn array_field_ptr<T, G, M>(
        &self,
        get: G,
        get_mut: M,
        offset: fn(&S::Offsets) -> usize,
    ) -> Ptr<T>
    where
        T: ByteRepr,
        G: Fn(&S) -> &Box<[T]> + Copy + 'static,
        M: Fn(&mut S) -> &mut Box<[T]> + Copy + 'static,
    {
        self.record_ptr().array_field_ptr(get, get_mut, offset)
    }

    // The field, which is made a pointer to when needed, to the whole field
    // or to its first element, depending on the type asked for.
    #[inline]
    fn field_ref<C, G, M>(
        &self,
        get: G,
        get_mut: M,
        offset: fn(&S::Offsets) -> usize,
    ) -> FieldRef<S, C, G, M>
    where
        G: Fn(&S) -> &C,
        M: Fn(&mut S) -> &mut C,
    {
        FieldRef {
            base: self.record_ptr(),
            get,
            get_mut,
            offset,
            field: PhantomData,
        }
    }
}

impl<S: Record> FieldPtr<S> for Ptr<S> {
    #[inline]
    fn record_ptr(&self) -> Ptr<S> {
        self.clone()
    }

    #[inline]
    fn field_ptr<T, G, M>(&self, get: G, get_mut: M, offset: fn(&S::Offsets) -> usize) -> Ptr<T>
    where
        T: ByteRepr,
        G: Fn(&S) -> &T + Copy + 'static,
        M: Fn(&mut S) -> &mut T + Copy + 'static,
    {
        let offset = offset(&S::OFFSETS);
        // The byte offset of the struct in its allocation.
        let start = || self.offset.wrapping_mul(S::byte_size());
        let (root, start): (Weak<dyn Root>, usize) = match &self.kind {
            PtrKind::Null => null_deref(),
            PtrKind::StackSingle(w) | PtrKind::HeapSingle(w) => (w.clone(), start()),
            PtrKind::StackArray(w) | PtrKind::HeapArray(w) => (w.clone(), start()),
            PtrKind::StackVec(w) | PtrKind::HeapVec(w) => (w.clone(), start()),
            PtrKind::Field(root, _, FieldShape::Single) => (root.clone(), self.offset),
            // A struct in an array or vector field, or in a nested container,
            // has no offset in its allocation; its field is reached through
            // it.
            PtrKind::Field(..) | PtrKind::Nested(_) => {
                return Ptr {
                    offset: 0,
                    kind: PtrKind::Nested(Rc::new(FieldOf::new(
                        self.clone(),
                        move |s| std::slice::from_ref(get(s)),
                        move |s| std::slice::from_mut(get_mut(s)),
                        offset,
                    ))),
                };
            }
            PtrKind::Reinterpreted(_) => {
                let (alloc, byte_offset) = self.original_alloc().unwrap();
                return Ptr::from_original_alloc(alloc, byte_offset.wrapping_add(offset));
            }
        };
        Ptr {
            offset: start.wrapping_add(offset),
            kind: PtrKind::Field(root, 0, FieldShape::Single),
        }
    }

    #[inline]
    fn array_field_ptr<T, G, M>(
        &self,
        get: G,
        get_mut: M,
        offset: fn(&S::Offsets) -> usize,
    ) -> Ptr<T>
    where
        T: ByteRepr,
        G: Fn(&S) -> &Box<[T]> + Copy + 'static,
        M: Fn(&mut S) -> &mut Box<[T]> + Copy + 'static,
    {
        match &self.kind {
            PtrKind::Reinterpreted(_) => {
                let (alloc, byte_offset) = self.original_alloc().unwrap();
                Ptr::from_original_alloc(alloc, byte_offset.wrapping_add(offset(&S::OFFSETS)))
            }
            PtrKind::Field(_, _, FieldShape::Single) => {
                self.field_ptr(get, get_mut, offset).decay()
            }
            _ => self.elems_of(get, get_mut, offset(&S::OFFSETS), true),
        }
    }
}

impl<S: Record> FieldPtr<S> for Value<S> {
    #[inline]
    fn record_ptr(&self) -> Ptr<S> {
        self.as_pointer()
    }
}

impl<S: Record> Ptr<S> {
    // A pointer to the first element of the array or vector that `get` gives
    // in the struct that self points to, which is at byte `offset` of the
    // struct. The pointer reaches the array or vector through the Value that
    // holds the struct, or else through self. `inline` tells if the elements
    // are in the memory of the struct, as those of an array are.
    // The projection of an array of structs takes the array as a Box, the
    // contents of its Value.
    #[allow(clippy::borrowed_box)]
    #[inline]
    fn elems_of<C, T>(
        &self,
        get: impl Fn(&S) -> &C + 'static,
        get_mut: impl Fn(&mut S) -> &mut C + 'static,
        offset: usize,
        inline: bool,
    ) -> Ptr<T>
    where
        C: AsRef<[T]> + AsMut<[T]> + 'static,
        T: 'static,
    {
        let idx = self.offset;
        // The address of the field, for comparisons.
        let address = |root: *const ()| {
            (root as usize)
                .wrapping_add(idx.wrapping_mul(S::byte_size()))
                .wrapping_add(offset)
        };
        let parent: Rc<dyn SliceParent<T>> = match &self.kind {
            PtrKind::StackSingle(w) | PtrKind::HeapSingle(w) => Rc::new(InValue::new(
                w,
                move |s: &S| get(s).as_ref(),
                move |s: &mut S| get_mut(s).as_mut(),
                address(w.as_ptr() as *const ()),
                inline,
            )),
            PtrKind::StackArray(w) | PtrKind::HeapArray(w) => Rc::new(InValue::new(
                w,
                move |a: &Box<[S]>| get(&a[idx]).as_ref(),
                move |a: &mut Box<[S]>| get_mut(&mut a[idx]).as_mut(),
                address(w.as_ptr() as *const ()),
                inline,
            )),
            PtrKind::StackVec(w) | PtrKind::HeapVec(w) => Rc::new(InValue::new(
                w,
                move |v: &Vec<S>| get(&v[idx]).as_ref(),
                move |v: &mut Vec<S>| get_mut(&mut v[idx]).as_mut(),
                address(w.as_ptr() as *const ()),
                inline,
            )),
            _ => Rc::new(FieldOf::new(
                self.clone(),
                move |s| get(s).as_ref(),
                move |s| get_mut(s).as_mut(),
                offset,
            )),
        };
        Ptr {
            offset: 0,
            kind: PtrKind::Nested(parent),
        }
    }
}

// The sequence that `get` gives in the contents of the Value `root`, e.g.,
// an array field of a struct in it, or of an element of an array in it. Its
// type is known, so the sequence is reached without looking for the field by
// its offset as a Field pointer does.
struct InValue<R, G, M> {
    root: Weak<RefCell<R>>,
    get: G,
    get_mut: M,
    // The address of the field, for comparisons.
    address: usize,
    // Whether the elements are in the memory of the struct.
    inline: bool,
}

impl<R, G, M> InValue<R, G, M> {
    #[inline]
    fn new<T>(root: &Weak<RefCell<R>>, get: G, get_mut: M, address: usize, inline: bool) -> Self
    where
        G: Fn(&R) -> &[T],
        M: Fn(&mut R) -> &mut [T],
    {
        Self {
            root: root.clone(),
            get,
            get_mut,
            address,
            inline,
        }
    }
}

impl<R, T, G, M> SliceParent<T> for InValue<R, G, M>
where
    R: 'static,
    G: Fn(&R) -> &[T],
    M: Fn(&mut R) -> &mut [T],
{
    fn root(&self) -> SliceRoot {
        SliceRoot::Alloc(self.root.upgrade().unwrap_or_else(|| dangling()))
    }

    fn borrow_slice<'a>(&self, root: &'a SliceRoot) -> Ref<'a, [T]> {
        Ref::map(root.alloc::<R>().borrow(), |r| (self.get)(r))
    }

    fn borrow_slice_mut<'a>(&self, root: &'a SliceRoot) -> RefMut<'a, [T]> {
        RefMut::map(root.alloc::<R>().borrow_mut(), |r| (self.get_mut)(r))
    }

    fn with_elem(&self, idx: usize, f: &mut dyn FnMut(&T)) {
        let root = self.root.upgrade().unwrap_or_else(|| dangling());
        f(&(self.get)(&root.borrow())[idx])
    }

    fn with_elem_mut(&self, idx: usize, f: &mut dyn FnMut(&mut T)) {
        let root = self.root.upgrade().unwrap_or_else(|| dangling());
        f(&mut (self.get_mut)(&mut root.borrow_mut())[idx])
    }

    fn address(&self) -> usize {
        self.address
    }

    fn struct_root(&self) -> Option<usize> {
        self.inline
            .then_some(self.root.as_ptr() as *const () as usize)
    }
}

// A field of a struct, as returned by field_ref!, which is made a pointer to
// the field, or to the first element of the field if it is a vector.
pub struct FieldRef<S: Record, C, G, M> {
    base: Ptr<S>,
    get: G,
    get_mut: M,
    offset: fn(&S::Offsets) -> usize,
    field: PhantomData<fn() -> C>,
}

impl<S, T, G, M> AsPointer<Vec<T>> for FieldRef<S, Vec<T>, G, M>
where
    S: Record,
    T: ByteRepr,
    G: Fn(&S) -> &Vec<T> + Copy + 'static,
    M: Fn(&mut S) -> &mut Vec<T> + Copy + 'static,
{
    #[inline]
    fn as_pointer(&self) -> Ptr<Vec<T>> {
        self.base.field_ptr(self.get, self.get_mut, self.offset)
    }
}

impl<S, T, G, M> AsPointer<T> for FieldRef<S, Vec<T>, G, M>
where
    S: Record,
    T: ByteRepr,
    G: Fn(&S) -> &Vec<T> + Copy + 'static,
    M: Fn(&mut S) -> &mut Vec<T> + Copy + 'static,
{
    #[inline]
    fn as_pointer(&self) -> Ptr<T> {
        match &self.base.kind {
            PtrKind::Field(_, _, FieldShape::Single) => self
                .base
                .field_ptr(self.get, self.get_mut, self.offset)
                .decay(),
            _ => self
                .base
                .elems_of(self.get, self.get_mut, (self.offset)(&S::OFFSETS), false),
        }
    }
}

// The sequence that `get` gives in the struct that `base` points to, which is
// reached through `base`: a field, as a sequence of one element, or the
// elements of an array or vector field. It is the parent of the pointers into
// the fields of structs that are elements of arrays and vectors in fields, or
// of nested containers, which have no offset in the allocation of the
// outermost struct.
struct FieldOf<S, G, M> {
    base: Ptr<S>,
    get: G,
    get_mut: M,
    // The byte offset of the field in the struct, which identifies the field
    // for comparisons.
    offset: usize,
}

impl<S, G, M> FieldOf<S, G, M> {
    #[inline]
    fn new<T>(base: Ptr<S>, get: G, get_mut: M, offset: usize) -> Self
    where
        G: Fn(&S) -> &[T],
        M: Fn(&mut S) -> &mut [T],
    {
        Self {
            base,
            get,
            get_mut,
            offset,
        }
    }
}

// The struct is reached through a Field pointer to an element, or a Nested
// pointer, which are the pointers that have no offset in their allocation.
impl<S, T, G, M> SliceParent<T> for FieldOf<S, G, M>
where
    S: Record,
    G: Fn(&S) -> &[T],
    M: Fn(&mut S) -> &mut [T],
{
    fn root(&self) -> SliceRoot {
        match &self.base.kind {
            PtrKind::Field(root, ..) => SliceRoot::Field(upgrade(root)),
            PtrKind::Nested(parent) => parent.root(),
            _ => unreachable!(),
        }
    }

    fn borrow_slice<'a>(&self, root: &'a SliceRoot) -> Ref<'a, [T]> {
        let idx = self.base.offset;
        let s = match &self.base.kind {
            PtrKind::Field(_, field, shape) => {
                Ptr::borrow_field_elem(root.field(), *field, *shape, idx)
            }
            PtrKind::Nested(parent) => Ref::map(parent.borrow_slice(root), |s| &s[idx]),
            _ => unreachable!(),
        };
        Ref::map(s, |s| (self.get)(s))
    }

    fn borrow_slice_mut<'a>(&self, root: &'a SliceRoot) -> RefMut<'a, [T]> {
        let idx = self.base.offset;
        let s = match &self.base.kind {
            PtrKind::Field(_, field, shape) => {
                Ptr::borrow_field_elem_mut(root.field(), *field, *shape, idx)
            }
            PtrKind::Nested(parent) => RefMut::map(parent.borrow_slice_mut(root), |s| &mut s[idx]),
            _ => unreachable!(),
        };
        RefMut::map(s, |s| (self.get_mut)(s))
    }

    fn with_elem(&self, idx: usize, f: &mut dyn FnMut(&T)) {
        match &self.base.kind {
            PtrKind::Field(root, field, shape) => {
                let root = upgrade(root);
                let s = Ptr::borrow_field_elem(&*root, *field, *shape, self.base.offset);
                f(&(self.get)(&s)[idx])
            }
            PtrKind::Nested(parent) => {
                parent.with_elem(self.base.offset, &mut |s| f(&(self.get)(s)[idx]))
            }
            _ => unreachable!(),
        }
    }

    fn with_elem_mut(&self, idx: usize, f: &mut dyn FnMut(&mut T)) {
        match &self.base.kind {
            PtrKind::Field(root, field, shape) => {
                let root = upgrade(root);
                let mut s = Ptr::borrow_field_elem_mut(&*root, *field, *shape, self.base.offset);
                f(&mut (self.get_mut)(&mut s)[idx])
            }
            PtrKind::Nested(parent) => {
                parent.with_elem_mut(self.base.offset, &mut |s| f(&mut (self.get_mut)(s)[idx]))
            }
            _ => unreachable!(),
        }
    }

    fn address(&self) -> usize {
        self.base.address().wrapping_add(self.offset)
    }
}

// A pointer to field `field` of the struct that a Ptr or a Value holds,
// e.g., `field_ptr!(p, x)`.
#[macro_export]
macro_rules! field_ptr {
    ($base:expr, $field:ident) => {{
        use $crate::FieldPtr as _;
        ($base).field_ptr(|__s| &__s.$field, |__s| &mut __s.$field, |__o| __o.$field)
    }};
}

// A pointer to the first element of array field `field` of the struct that a
// Ptr or a Value holds, e.g., `array_field_ptr!(p, arr)`.
#[macro_export]
macro_rules! array_field_ptr {
    ($base:expr, $field:ident) => {{
        use $crate::FieldPtr as _;
        ($base).array_field_ptr(|__s| &__s.$field, |__s| &mut __s.$field, |__o| __o.$field)
    }};
}

// Field `field` of the struct that a Ptr or a Value holds, to make a pointer
// to with as_pointer(), e.g., `field_ref!(p, v).as_pointer()` for a vector
// field `v`, which gives a pointer to the vector or to its first element.
#[macro_export]
macro_rules! field_ref {
    ($base:expr, $field:ident) => {{
        use $crate::FieldPtr as _;
        ($base).field_ref(|__s| &__s.$field, |__s| &mut __s.$field, |__o| __o.$field)
    }};
}

// The byte size of field `field` of struct `ty`, like offset_of! gives its
// offset, e.g., `size_of_field!(::libc::dirent, d_name)`.
#[macro_export]
macro_rules! size_of_field {
    ($ty:ty, $field:ident) => {
        $crate::__field::size_of_pointee(|__s: &$ty| &__s.$field)
    };
}

// The size of the type that `f` gives a reference to.
pub const fn size_of_pointee<S, T>(_f: fn(&S) -> &T) -> usize {
    std::mem::size_of::<T>()
}

// Something that is read and written in place, like the object a pointer
// points to.
pub trait Place: Clone {
    type Target;
    fn with<R>(&self, f: impl FnOnce(&Self::Target) -> R) -> R;
    fn with_mut<R>(&self, f: impl FnOnce(&mut Self::Target) -> R) -> R;
}

impl<T: ByteRepr> Place for Ptr<T> {
    type Target = T;
    #[inline]
    fn with<R>(&self, f: impl FnOnce(&T) -> R) -> R {
        Ptr::with(self, f)
    }
    #[inline]
    fn with_mut<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        Ptr::with_mut(self, f)
    }
}

// A field of the struct in place `base`, which is accessed through `base` and
// projected to the field, without looking the field up as a pointer to it
// does. Written by `field!(p, x)`.
pub struct FieldPlace<'a, B: Place, T> {
    pub base: &'a B,
    pub get: fn(&B::Target) -> &T,
    pub get_mut: fn(&mut B::Target) -> &mut T,
}

impl<B: Place, T> Clone for FieldPlace<'_, B, T> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<B: Place, T> Copy for FieldPlace<'_, B, T> {}

impl<B: Place, T> FieldPlace<'_, B, T> {
    #[inline]
    pub fn with<R>(&self, f: impl FnOnce(&T) -> R) -> R {
        self.base.with(|s| f((self.get)(s)))
    }

    #[inline]
    pub fn with_mut<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        self.base.with_mut(|s| f((self.get_mut)(s)))
    }

    #[inline]
    pub fn read(&self) -> T
    where
        T: Clone,
    {
        self.with(T::clone)
    }

    #[inline]
    pub fn write(&self, value: T) {
        self.with_mut(|v| *v = value)
    }
}

impl<B: Place, T> Place for FieldPlace<'_, B, T> {
    type Target = T;
    #[inline]
    fn with<R>(&self, f: impl FnOnce(&T) -> R) -> R {
        FieldPlace::with(self, f)
    }
    #[inline]
    fn with_mut<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        FieldPlace::with_mut(self, f)
    }
}

// Field `field` of the struct in the place `base` (a Ptr, or another field),
// e.g., `field!(p, x).write(1)`. It is a struct expression, so that a
// temporary base lives as long as the field when bound to a variable.
#[macro_export]
macro_rules! field {
    ($base:expr, $field:ident) => {
        $crate::FieldPlace {
            base: &$base,
            get: |__s| &__s.$field,
            get_mut: |__s| &mut __s.$field,
        }
    };
}

// Element `idx` of the sequence that pointer `base` points into, which is
// accessed through `base` without creating a pointer to it as
// `base.offset(idx)` does. Written by `elem!(p, i)`.
pub struct ElemPlace<'a, T> {
    pub base: &'a Ptr<T>,
    pub idx: isize,
}

impl<T> Clone for ElemPlace<'_, T> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for ElemPlace<'_, T> {}

impl<T: ByteRepr> ElemPlace<'_, T> {
    #[inline]
    pub fn with<R>(&self, f: impl FnOnce(&T) -> R) -> R {
        self.base.with_at(self.idx, f)
    }

    #[inline]
    pub fn with_mut<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        self.base.with_mut_at(self.idx, f)
    }

    #[inline]
    pub fn read(&self) -> T
    where
        T: Clone,
    {
        self.base.read_at(self.idx)
    }

    #[inline]
    pub fn write(&self, value: T) {
        self.base.write_at(self.idx, value)
    }

    #[inline]
    pub fn upgrade(&self) -> StrongPtr<T> {
        self.base.offset(self.idx).upgrade()
    }
}

impl<T: ByteRepr> Place for ElemPlace<'_, T> {
    type Target = T;
    #[inline]
    fn with<R>(&self, f: impl FnOnce(&T) -> R) -> R {
        ElemPlace::with(self, f)
    }
    #[inline]
    fn with_mut<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        ElemPlace::with_mut(self, f)
    }
}

// Element `idx` of the sequence that pointer `base` points into, e.g.,
// `elem!(p, i).write(1)` for `p[i] = 1`. It is a struct expression, like
// field!, so that a temporary base lives as long as the element when bound to
// a variable.
#[macro_export]
macro_rules! elem {
    ($base:expr, $idx:expr) => {
        $crate::ElemPlace {
            base: &$base,
            idx: ($idx) as isize,
        }
    };
}

// An array or vector, whose elements are indexed by IndexPlace.
pub trait Seq {
    type Elem;
    fn elems(&self) -> &[Self::Elem];
    fn elems_mut(&mut self) -> &mut [Self::Elem];
}

impl<T> Seq for Box<[T]> {
    type Elem = T;
    #[inline]
    fn elems(&self) -> &[T] {
        self
    }
    #[inline]
    fn elems_mut(&mut self) -> &mut [T] {
        self
    }
}

impl<T> Seq for Vec<T> {
    type Elem = T;
    #[inline]
    fn elems(&self) -> &[T] {
        self
    }
    #[inline]
    fn elems_mut(&mut self) -> &mut [T] {
        self
    }
}

// Element `idx` of the array or vector in place `base`, e.g., a row of an
// array field of a struct reached through a pointer. Written by
// `index!(place, i)`. It is a struct expression, like elem!.
pub struct IndexPlace<'a, B> {
    pub base: &'a B,
    pub idx: usize,
}

impl<B> Clone for IndexPlace<'_, B> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<B> Copy for IndexPlace<'_, B> {}

impl<B: Place> IndexPlace<'_, B>
where
    B::Target: Seq,
{
    #[inline]
    pub fn with<R>(&self, f: impl FnOnce(&<B::Target as Seq>::Elem) -> R) -> R {
        self.base.with(|c| f(&c.elems()[self.idx]))
    }

    #[inline]
    pub fn with_mut<R>(&self, f: impl FnOnce(&mut <B::Target as Seq>::Elem) -> R) -> R {
        self.base.with_mut(|c| f(&mut c.elems_mut()[self.idx]))
    }

    #[inline]
    pub fn read(&self) -> <B::Target as Seq>::Elem
    where
        <B::Target as Seq>::Elem: Clone,
    {
        self.with(Clone::clone)
    }

    #[inline]
    pub fn write(&self, value: <B::Target as Seq>::Elem) {
        self.with_mut(|v| *v = value)
    }
}

impl<T: ByteRepr> IndexPlace<'_, Ptr<Vec<T>>> {
    #[inline]
    pub fn upgrade(&self) -> StrongPtr<T> {
        self.base.decay().offset(self.idx).upgrade()
    }
}

impl<B: Place> Place for IndexPlace<'_, B>
where
    B::Target: Seq,
{
    type Target = <B::Target as Seq>::Elem;
    #[inline]
    fn with<R>(&self, f: impl FnOnce(&Self::Target) -> R) -> R {
        IndexPlace::with(self, f)
    }
    #[inline]
    fn with_mut<R>(&self, f: impl FnOnce(&mut Self::Target) -> R) -> R {
        IndexPlace::with_mut(self, f)
    }
}

// Element `idx` of the array or vector in place `base`, e.g.,
// `index!(array_elem!(p, a, i), j).write(1)` for `p->a[i][j] = 1`.
#[macro_export]
macro_rules! index {
    ($base:expr, $idx:expr) => {
        $crate::IndexPlace {
            base: &$base,
            idx: ($idx) as usize,
        }
    };
}

// Element `idx` of the array field that `get` gives in the struct that `base`
// points to, which is accessed in place, without creating a pointer to it as
// `elem!(array_field_ptr!(p, a), i)` does, unless the struct is reinterpreted
// memory. Written by `array_elem!(p, a, i)`. It is a struct expression, like
// elem!.
pub struct ArrayElem<'a, S: Record, T> {
    pub base: &'a Ptr<S>,
    pub get: fn(&S) -> &Box<[T]>,
    pub get_mut: fn(&mut S) -> &mut Box<[T]>,
    pub offset: fn(&S::Offsets) -> usize,
    pub idx: isize,
}

impl<S: Record, T> Clone for ArrayElem<'_, S, T> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<S: Record, T> Copy for ArrayElem<'_, S, T> {}

impl<S: Record, T: ByteRepr> ArrayElem<'_, S, T> {
    // A pointer to the element.
    #[inline]
    fn ptr(&self) -> Ptr<T> {
        self.base
            .array_field_ptr(self.get, self.get_mut, self.offset)
            .offset(self.idx)
    }

    #[inline]
    pub fn with<R>(&self, f: impl FnOnce(&T) -> R) -> R {
        if let PtrKind::Reinterpreted(_) = self.base.kind {
            return self.ptr().with(f);
        }
        self.base.with(|s| f(&(self.get)(s)[self.idx as usize]))
    }

    #[inline]
    pub fn with_mut<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        if let PtrKind::Reinterpreted(_) = self.base.kind {
            return self.ptr().with_mut(f);
        }
        self.base
            .with_mut(|s| f(&mut (self.get_mut)(s)[self.idx as usize]))
    }

    #[inline]
    pub fn read(&self) -> T
    where
        T: Clone,
    {
        self.with(T::clone)
    }

    #[inline]
    pub fn write(&self, value: T) {
        self.with_mut(|v| *v = value)
    }

    #[inline]
    pub fn upgrade(&self) -> StrongPtr<T> {
        self.ptr().upgrade()
    }
}

impl<S: Record, T: ByteRepr> Place for ArrayElem<'_, S, T> {
    type Target = T;
    #[inline]
    fn with<R>(&self, f: impl FnOnce(&T) -> R) -> R {
        ArrayElem::with(self, f)
    }
    #[inline]
    fn with_mut<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        ArrayElem::with_mut(self, f)
    }
}

// Element `idx` of array field `field` of the struct that pointer `base`
// points to, e.g., `array_elem!(p, a, i).write(1)` for `p->a[i] = 1`.
#[macro_export]
macro_rules! array_elem {
    ($base:expr, $field:ident, $idx:expr) => {
        $crate::ArrayElem {
            base: &$base,
            get: |__s| &__s.$field,
            get_mut: |__s| &mut __s.$field,
            offset: |__o| __o.$field,
            idx: ($idx) as isize,
        }
    };
}

// Element `idx` of the vector field that `get` gives in the struct that
// `base` points to, which is accessed in place, like ArrayElem. Written by
// `vec_elem!(p, v, i)`.
pub struct VecElem<'a, S: Record, T> {
    pub base: &'a Ptr<S>,
    pub get: fn(&S) -> &Vec<T>,
    pub get_mut: fn(&mut S) -> &mut Vec<T>,
    pub offset: fn(&S::Offsets) -> usize,
    pub idx: usize,
}

impl<S: Record, T> Clone for VecElem<'_, S, T> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<S: Record, T> Copy for VecElem<'_, S, T> {}

impl<S: Record, T: ByteRepr> VecElem<'_, S, T> {
    #[inline]
    pub fn with<R>(&self, f: impl FnOnce(&T) -> R) -> R {
        self.base.with(|s| f(&(self.get)(s)[self.idx]))
    }

    #[inline]
    pub fn with_mut<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        self.base.with_mut(|s| f(&mut (self.get_mut)(s)[self.idx]))
    }

    #[inline]
    pub fn read(&self) -> T
    where
        T: Clone,
    {
        self.with(T::clone)
    }

    #[inline]
    pub fn write(&self, value: T) {
        self.with_mut(|v| *v = value)
    }

    #[inline]
    pub fn upgrade(&self) -> StrongPtr<T> {
        let elems: Ptr<T> = self
            .base
            .field_ref(self.get, self.get_mut, self.offset)
            .as_pointer();
        elems.offset(self.idx).upgrade()
    }
}

impl<S: Record, T: ByteRepr> Place for VecElem<'_, S, T> {
    type Target = T;
    #[inline]
    fn with<R>(&self, f: impl FnOnce(&T) -> R) -> R {
        VecElem::with(self, f)
    }
    #[inline]
    fn with_mut<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        VecElem::with_mut(self, f)
    }
}

// Element `idx` of vector field `field` of the struct that pointer `base`
// points to, e.g., `vec_elem!(p, v, i).write(1)` for `p->v[i] = 1`.
#[macro_export]
macro_rules! vec_elem {
    ($base:expr, $field:ident, $idx:expr) => {
        $crate::VecElem {
            base: &$base,
            get: |__s| &__s.$field,
            get_mut: |__s| &mut __s.$field,
            offset: |__o| __o.$field,
            idx: ($idx) as usize,
        }
    };
}

// Finds the object at a byte offset of a field for Record::locate. The
// implementation is chosen based on the type of the field, by autoref-based
// specialization: `(&&Locate(&field)).locate(...)` picks LocateRecord if the
// field is a struct, and LocateLeaf otherwise.
#[doc(hidden)]
pub struct Locate<'a, T>(pub &'a T);

#[doc(hidden)]
pub struct LocateMut<'a, T>(pub Cell<Option<&'a mut T>>);

impl<'a, T> LocateMut<'a, T> {
    #[inline]
    pub fn new(field: &'a mut T) -> Self {
        Self(Cell::new(Some(field)))
    }
    #[inline]
    fn take(&self) -> &'a mut T {
        self.0.take().unwrap()
    }
}

#[doc(hidden)]
pub trait LocateRecord<'a> {
    fn locate(self, offset: usize, ty: TypeId) -> Option<&'a dyn Any>;
}
#[doc(hidden)]
pub trait LocateLeaf<'a> {
    fn locate(self, offset: usize, ty: TypeId) -> Option<&'a dyn Any>;
}
#[doc(hidden)]
pub trait LocateRecordMut<'a> {
    fn locate(self, offset: usize, ty: TypeId) -> Option<&'a mut dyn Any>;
}
#[doc(hidden)]
pub trait LocateLeafMut<'a> {
    fn locate(self, offset: usize, ty: TypeId) -> Option<&'a mut dyn Any>;
}

// A struct.
impl<'a, R: Record> LocateRecord<'a> for &&Locate<'a, R> {
    #[inline]
    fn locate(self, offset: usize, ty: TypeId) -> Option<&'a dyn Any> {
        self.0.locate(offset, ty)
    }
}

impl<'a, R: Record> LocateRecordMut<'a> for &&LocateMut<'a, R> {
    #[inline]
    fn locate(self, offset: usize, ty: TypeId) -> Option<&'a mut dyn Any> {
        self.take().locate_mut(offset, ty)
    }
}

// Anything else.
impl<'a, T: 'static> LocateLeaf<'a> for &Locate<'a, T> {
    #[inline]
    fn locate(self, offset: usize, ty: TypeId) -> Option<&'a dyn Any> {
        (offset == 0 && ty == TypeId::of::<T>()).then_some(self.0 as &dyn Any)
    }
}

impl<'a, T: 'static> LocateLeafMut<'a> for &LocateMut<'a, T> {
    #[inline]
    fn locate(self, offset: usize, ty: TypeId) -> Option<&'a mut dyn Any> {
        (offset == 0 && ty == TypeId::of::<T>()).then_some(self.take() as &mut dyn Any)
    }
}

// Used by #[derive(Record)].
#[doc(hidden)]
#[macro_export]
macro_rules! __locate_field {
    ($field:expr, $offset:expr, $ty:expr) => {{
        #[allow(unused_imports)]
        use $crate::__field::{LocateLeaf, LocateRecord};
        (&&$crate::__field::Locate($field)).locate($offset, $ty)
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __locate_field_mut {
    ($field:expr, $offset:expr, $ty:expr) => {{
        #[allow(unused_imports)]
        use $crate::__field::{LocateLeafMut, LocateRecordMut};
        (&&$crate::__field::LocateMut::new($field)).locate($offset, $ty)
    }};
}

// Finds the length of the array or vector at a byte offset of a field for
// Record::len_at, by autoref-based specialization like Locate:
// `(&&&LenOf(&field)).len_at(...)` picks LenSlice if the field is an array or
// a vector, LenRecord if it is a struct, and LenLeaf otherwise.
#[doc(hidden)]
pub struct LenOf<'a, T>(pub &'a T);

#[doc(hidden)]
pub trait LenSlice {
    fn len_at(self, offset: usize) -> Option<usize>;
}
#[doc(hidden)]
pub trait LenRecord {
    fn len_at(self, offset: usize) -> Option<usize>;
}
#[doc(hidden)]
pub trait LenLeaf {
    fn len_at(self, offset: usize) -> Option<usize>;
}

impl<T> LenSlice for &&&LenOf<'_, Box<[T]>> {
    #[inline]
    fn len_at(self, offset: usize) -> Option<usize> {
        (offset == 0).then_some(self.0.len())
    }
}

impl<T> LenSlice for &&&LenOf<'_, Vec<T>> {
    #[inline]
    fn len_at(self, offset: usize) -> Option<usize> {
        (offset == 0).then_some(self.0.len())
    }
}

impl<R: Record> LenRecord for &&LenOf<'_, R> {
    #[inline]
    fn len_at(self, offset: usize) -> Option<usize> {
        self.0.len_at(offset)
    }
}

impl<T> LenLeaf for &LenOf<'_, T> {
    #[inline]
    fn len_at(self, _offset: usize) -> Option<usize> {
        None
    }
}

// Used by #[derive(Record)].
#[doc(hidden)]
#[macro_export]
macro_rules! __len_field {
    ($field:expr, $offset:expr) => {{
        #[allow(unused_imports)]
        use $crate::__field::{LenLeaf, LenRecord, LenSlice};
        (&&&$crate::__field::LenOf($field)).len_at($offset)
    }};
}

impl<T: 'static> Ptr<T> {
    // Borrows the field at byte `offset` of `root`.
    #[inline]
    pub(crate) fn borrow_field(root: &dyn Root, offset: usize) -> Ref<'_, T> {
        Ref::map(root.borrow_at(offset, TypeId::of::<T>()), |obj| {
            obj.downcast_ref::<T>().unwrap_or_else(|| invalid_field())
        })
    }

    #[inline]
    pub(crate) fn borrow_field_mut(root: &dyn Root, offset: usize) -> RefMut<'_, T> {
        RefMut::map(root.borrow_at_mut(offset, TypeId::of::<T>()), |obj| {
            obj.downcast_mut::<T>().unwrap_or_else(|| invalid_field())
        })
    }
}

#[inline]
pub(crate) fn upgrade(root: &Weak<dyn Root>) -> Rc<dyn Root> {
    root.upgrade().unwrap_or_else(|| crate::rc::dangling())
}

#[cfg(test)]
mod tests {
    use crate::{AsPointer, ByteRepr, Ptr, Record, Value};
    use std::cell::RefCell;
    use std::rc::Rc;

    #[derive(Clone, Default, Record, ByteRepr)]
    #[byte_size(8)]
    struct Inner {
        #[offset(0)]
        a: i32,
        #[offset(4)]
        b: i32,
    }

    #[derive(Clone, Default, Record)]
    struct Outer {
        #[offset(0)]
        x: i32,
        #[offset(4)]
        inner: Inner,
        #[offset(12)]
        items: Box<[Inner]>,
        #[offset(40)]
        v: Vec<Inner>,
    }

    impl ByteRepr for Outer {
        fn byte_size() -> usize {
            64
        }
    }

    fn inner(a: i32) -> Inner {
        Inner { a, b: -a }
    }

    fn outer() -> Value<Outer> {
        Rc::new(RefCell::new(Outer {
            items: vec![inner(1), inner(2), inner(3)].into_boxed_slice(),
            ..Default::default()
        }))
    }

    #[test]
    fn read_write_field() {
        let s = outer();
        let x = field_ptr!(s, x);
        x.write(5);
        assert_eq!(s.borrow().x, 5);
        s.borrow_mut().x = 7;
        assert_eq!(x.read(), 7);
        x.with_mut(|v| *v += 1);
        assert_eq!(s.borrow().x, 8);
        assert_eq!(x.len(), 1);
        assert_eq!(x.offset(1) - x.clone(), 1);
    }

    #[test]
    fn nested_field() {
        let s = outer();
        let p = field_ptr!(s, inner);
        let a = field_ptr!(p, a);
        a.write(3);
        assert_eq!(s.borrow().inner.a, 3);
        assert_eq!(p.upgrade().deref().a, 3);
        p.with_mut(|p| p.a = 5);
        assert_eq!(a.read(), 5);
        let b = field_ptr!(p, b);
        assert!(a < b);
        assert_ne!(a, b);
        assert_eq!(b, field_ptr!(field_ptr!(s, inner), b));
    }

    #[test]
    fn field_of_array_element() {
        // Arrays are stored inline, and reached through the pointers to them.
        let s = outer();
        let items: Ptr<Inner> = field_ptr!(s, items).decay();
        let a = field_ptr!(items.offset(1), a);
        assert_eq!(a.read(), 2);
        a.write(20);
        assert_eq!(s.borrow().items[1].a, 20);
        let b = field_ptr!(items.offset(2), b);
        assert_eq!(b.read(), -3);
        let first = field_ptr!(items, a);
        assert!(first < a);
        assert_eq!(first, field_ptr!(items.offset(0), a));
        assert_eq!(items.offset(1), field_ptr!(s, items).decay().offset(1));
        assert_eq!(items.offset(3) - items.clone(), 3);
        assert_eq!(items.len(), 3);
    }

    #[test]
    fn vector_field() {
        let s = outer();
        s.borrow_mut().v = vec![inner(1), inner(2)];
        let v: Ptr<Inner> = field_ptr!(s, v).decay();
        let a = field_ptr!(v.offset(1), a);
        assert_eq!(a.read(), 2);
        s.borrow_mut().v.push(inner(3));
        a.write(20);
        assert_eq!(s.borrow().v[1].a, 20);
        assert_eq!(field_ptr!(v.offset(2), b).read(), -3);
        field!(v.offset(2), a).write(30);
        assert_eq!(s.borrow().v[2].a, 30);
    }

    #[derive(Default, Record)]
    struct Nest {
        #[offset(0)]
        rows: Vec<Outer>,
        #[offset(24)]
        grid: Box<[Box<[i32]>]>,
    }

    impl ByteRepr for Nest {
        fn byte_size() -> usize {
            48
        }
    }

    #[test]
    fn nested_fields() {
        let s: Value<Nest> = Rc::new(RefCell::new(Nest {
            rows: vec![Outer::default(), outer().borrow().clone()],
            grid: vec![vec![1, 2].into_boxed_slice(), vec![3, 4].into_boxed_slice()]
                .into_boxed_slice(),
        }));
        // A field of an element of a vector field, and an element of an array
        // field of it.
        let rows: Ptr<Outer> = field_ptr!(s, rows).decay();
        let x = field_ptr!(rows.offset(1), x);
        x.write(4);
        assert_eq!(s.borrow().rows[1].x, 4);
        let b = field_ptr!(field_ptr!(rows.offset(1), inner), b);
        b.write(6);
        assert_eq!(s.borrow().rows[1].inner.b, 6);
        let items: Ptr<Inner> = field_ptr!(rows.offset(1), items).decay();
        field_ptr!(items.offset(2), a).write(8);
        assert_eq!(s.borrow().rows[1].items[2].a, 8);
        assert_eq!(items.len(), 3);
        // A row of a 2D array field.
        let grid: Ptr<Box<[i32]>> = field_ptr!(s, grid).decay();
        let row: Ptr<i32> = grid.offset(1).decay();
        assert_eq!(row.offset(1).read(), 4);
        row.write(5);
        assert_eq!(s.borrow().grid[1][0], 5);
    }

    #[test]
    fn reinterpret_field() {
        let s: Value<Inner> = Rc::new(RefCell::new(inner(0x01020304)));
        let a = field_ptr!(s, a);
        let bytes = a.reinterpret_cast::<u8>();
        assert_eq!(bytes.read(), 0x04);
        bytes.write(0xff);
        assert_eq!(s.borrow().a, 0x010203ff);
        // The field is viewed as a separate allocation.
        assert_eq!(bytes.len(), 4);
        // A field of a reinterpreted struct is written through.
        let raw: Ptr<u8> = Ptr::alloc_array(vec![0u8; 8].into_boxed_slice());
        let view: Ptr<Inner> = raw.reinterpret_cast();
        let b: Ptr<i32> = field_ptr!(view, b);
        b.write(0x0a0b0c0d);
        assert_eq!(raw.offset(4).read(), 0x0d);
        view.with_mut(|v| v.a = 0x11223344);
        assert_eq!(raw.read(), 0x44);
        raw.delete();
    }

    #[derive(Record, ByteRepr)]
    #[byte_size(8)]
    struct Masked {
        #[offset(0)]
        n: i32,
        #[offset(4)]
        #[byte_size(4)]
        mask: Box<[u8]>,
    }

    #[test]
    fn size_of_field() {
        assert_eq!(size_of_field!(Masked, n), 4);
        assert_eq!(size_of_field!(Outer, inner), size_of::<Inner>());
    }

    #[test]
    fn array_field() {
        let s: Value<Masked> = Rc::new(RefCell::new(Masked::from_bytes(&[0; 8])));
        let m = array_field_ptr!(s, mask);
        m.offset(2).write(5);
        assert_eq!(s.borrow().mask[2], 5);
        assert_eq!(array_field_ptr!(s.as_pointer(), mask), m);
        // The array field of a reinterpreted struct is in its memory, which
        // the pointer can go past the end of the struct in.
        let raw: Ptr<u8> = Ptr::alloc_array(vec![0u8; 12].into_boxed_slice());
        let view: Ptr<Masked> = raw.reinterpret_cast();
        let m = array_field_ptr!(view, mask);
        m.offset(1).write(7);
        assert_eq!(raw.offset(5).read(), 7);
        m.offset(6).write(9);
        assert_eq!(raw.offset(10).read(), 9);
        field_ptr!(view, n).write(3);
        assert_eq!(m.offset(1).read(), 7);
        raw.delete();
    }

    #[test]
    fn field_place() {
        let s = outer();
        let p = s.as_pointer();
        field!(p, x).write(3);
        assert_eq!(s.borrow().x, 3);
        field!(field!(p, inner), b).with_mut(|b| *b += 2);
        assert_eq!(field!(field!(p, inner), b).read(), 2);
        let items: Ptr<Inner> = array_field_ptr!(s, items);
        field!(items.offset(1), a).write(7);
        assert_eq!(s.borrow().items[1].a, 7);
    }

    #[test]
    fn elements_of_fields() {
        let s = outer();
        s.borrow_mut().v = vec![inner(1), inner(2), inner(3)];
        let items: Ptr<Inner> = array_field_ptr!(s, items);
        let v: Ptr<Inner> = field_ptr!(s, v).decay();
        // Pointer arithmetic and comparisons.
        assert_eq!(items.len(), 3);
        assert_eq!(v.len(), 3);
        assert_eq!(items.to_end() - items.clone(), 3);
        assert_eq!(items.to_last(), items.offset(2));
        assert!(items.offset(1) < items.offset(2));
        assert_ne!(items.offset(1), v.offset(1));
        assert_eq!(v.offset(2).get_offset(), 2);
        assert_eq!((&v + 2_usize).offset(-1), v.offset(1));
        // Reads and writes through the elements.
        assert_eq!(v.offset(2).read().a, 3);
        v.offset(1).with_mut(|e| e.b = 7);
        assert_eq!(s.borrow().v[1].b, 7);
        assert_eq!(v.offset(1).upgrade().deref().b, 7);
        items.offset(2).write(inner(9));
        assert_eq!(s.borrow().items[2].a, 9);
        // The vector is reached anew on every access.
        s.borrow_mut().v.push(inner(4));
        assert_eq!(v.len(), 4);
        assert_eq!(v.offset(3).read().a, 4);
        // Bytes of the elements.
        let bytes: Ptr<u8> = items.offset(2).reinterpret_cast();
        assert_eq!(bytes.read(), 9);
        bytes.write(10);
        assert_eq!(s.borrow().items[2].a, 10);
    }

    #[derive(Default, Record, ByteRepr)]
    #[byte_size(16)]
    struct Chars {
        #[offset(0)]
        n: i32,
        #[offset(4)]
        #[byte_size(8)]
        name: Box<[u8]>,
        #[offset(12)]
        nums: Vec<i32>,
    }

    #[test]
    fn slices_of_fields() {
        let s: Value<Chars> = Rc::new(RefCell::new(Chars {
            n: 0,
            name: b"abc\0def\0".to_vec().into_boxed_slice(),
            nums: vec![3, 1, 2],
        }));
        let name: Ptr<u8> = array_field_ptr!(s, name);
        assert_eq!(name.c_str_len(), 3);
        assert_eq!(name.offset(4).c_str_len(), 3);
        name.offset(1)
            .with_slice_mut(2, |b| b.copy_from_slice(b"xy"));
        assert_eq!(&s.borrow().name[..4], b"axy\0");
        assert_eq!(name.slice_until(&name.offset(3)), b"axy".to_vec());
        let nums: Ptr<i32> = field_ptr!(s, nums).decay();
        nums.sort(3);
        assert_eq!(s.borrow().nums, vec![1, 2, 3]);
        assert_eq!(nums.collect::<Vec<_>>().len(), 3);
    }

    #[test]
    fn element_places() {
        let s = outer();
        s.borrow_mut().v = vec![inner(1), inner(2)];
        let p = s.as_pointer();
        array_elem!(p, items, 1).write(inner(5));
        assert_eq!(s.borrow().items[1].a, 5);
        field!(array_elem!(p, items, 2), b).write(6);
        assert_eq!(array_elem!(s.as_pointer(), items, 2).read().b, 6);
        assert_eq!(array_elem!(p, items, 2).upgrade().deref().b, 6);
        vec_elem!(p, v, 1).with_mut(|e| e.a = 7);
        assert_eq!(vec_elem!(p, v, 1).read().a, 7);
        field!(vec_elem!(s.as_pointer(), v, 0), b).write(8);
        assert_eq!(s.borrow().v[0].b, 8);
        assert_eq!(vec_elem!(p, v, 0).upgrade().deref().b, 8);
        // The array field of a reinterpreted struct is in its memory.
        let raw: Ptr<u8> = Ptr::alloc_array(vec![0u8; 12].into_boxed_slice());
        let view: Ptr<Masked> = raw.reinterpret_cast();
        array_elem!(view, mask, 6).write(9);
        assert_eq!(raw.offset(10).read(), 9);
        assert_eq!(array_elem!(view, mask, 6).read(), 9);
        raw.delete();
    }

    // A struct whose array field is beyond 4 GiB into it, which the
    // attributes lay out without allocating it.
    #[derive(Clone, Default, Record, ByteRepr)]
    #[byte_size(0x1_0000_0010)]
    struct Far {
        #[offset(0)]
        n: i32,
        #[offset(0x1_0000_0000)]
        #[byte_size(16)]
        arr: Box<[i32]>,
        #[offset(0x1_0000_0000)]
        #[byte_size(16)]
        v: Vec<i32>,
    }

    #[derive(Default, Record, ByteRepr)]
    #[byte_size(0x1_0000_0010)]
    struct HoldsFar {
        #[offset(0)]
        #[byte_size(0x1_0000_0010)]
        far: Far,
    }

    fn holds_far() -> Value<HoldsFar> {
        Rc::new(RefCell::new(HoldsFar {
            far: Far {
                n: 0,
                arr: vec![1, 2, 3, 4].into_boxed_slice(),
                v: vec![5, 6],
            },
        }))
    }

    #[test]
    #[should_panic(expected = "too large for a pointer to its elements")]
    fn far_array_field_ptr() {
        // The struct is reached through a field pointer, whose root has lost
        // its type, so the array is found from its offset, which doesn't fit.
        let far: Ptr<Far> = field_ptr!(holds_far(), far);
        let _: Ptr<i32> = array_field_ptr!(far, arr);
    }

    #[test]
    #[should_panic(expected = "too large for a pointer to its elements")]
    fn far_vector_field_ptr() {
        let far: Ptr<Far> = field_ptr!(holds_far(), far);
        let _: Ptr<i32> = field_ref!(far, v).as_pointer();
    }

    #[test]
    fn far_field_of_value() {
        // From the Value that holds the struct, the array is reached without
        // its offset.
        let s: Value<Far> = Rc::new(RefCell::new(holds_far().borrow().far.clone()));
        let arr: Ptr<i32> = array_field_ptr!(s, arr);
        assert_eq!(arr.offset(2).read(), 3);
        let v: Ptr<i32> = field_ref!(s, v).as_pointer();
        assert_eq!(v.offset(1).read(), 6);
    }

    #[derive(Clone, Default, Record, ByteRepr)]
    #[byte_size(44)]
    struct Adjacent {
        #[offset(0)]
        #[byte_size(8)]
        a: Box<[i32]>,
        #[offset(8)]
        #[byte_size(8)]
        b: Box<[i32]>,
        #[offset(16)]
        #[byte_size(24)]
        v: Vec<i32>,
        #[offset(40)]
        c: i32,
    }

    #[derive(Default, Record, ByteRepr)]
    #[byte_size(48)]
    struct HoldsAdjacent {
        #[offset(0)]
        n: i32,
        #[offset(4)]
        #[byte_size(44)]
        adj: Adjacent,
    }

    fn adjacent() -> Adjacent {
        Adjacent {
            a: vec![1, 2].into_boxed_slice(),
            b: vec![3, 4].into_boxed_slice(),
            v: vec![5, 6, 7, 8, 9, 10],
            c: 11,
        }
    }

    fn check_adjacent(s: Ptr<Adjacent>) {
        // One past the end of an array field is the next field.
        let a: Ptr<i32> = array_field_ptr!(s, a);
        let b: Ptr<i32> = array_field_ptr!(s, b);
        assert_eq!(a.offset(2), b);
        assert!(a.offset(2) <= b && a.offset(2) >= b);
        assert!(a.offset(1) < b && b < a.offset(3));
        assert_ne!(a.offset(1), b);
        assert_eq!(b.offset(-2), a);
        // The elements of a vector are not in the struct.
        let v: Ptr<i32> = field_ref!(s, v).as_pointer();
        assert_ne!(v.offset(6), field_ptr!(s, c));
    }

    #[test]
    fn adjacent_array_fields() {
        let s: Value<Adjacent> = Rc::new(RefCell::new(adjacent()));
        check_adjacent(s.as_pointer());
        let held: Value<HoldsAdjacent> = Rc::new(RefCell::new(HoldsAdjacent {
            n: 0,
            adj: adjacent(),
        }));
        check_adjacent(field_ptr!(held, adj));
        // Fields of different structs are not equal.
        let other: Value<Adjacent> = Rc::new(RefCell::new(adjacent()));
        let a: Ptr<i32> = array_field_ptr!(s, a);
        assert_ne!(a.offset(2), array_field_ptr!(other, b));
    }

    #[test]
    fn pointer_size() {
        // Pointers to elements of fields are no larger than other pointers.
        assert_eq!(
            std::mem::size_of::<Ptr<u8>>(),
            4 * std::mem::size_of::<usize>()
        );
    }

    #[test]
    #[should_panic(expected = "ub: invalid field pointer")]
    fn out_of_bounds_field() {
        let s = outer();
        field_ptr!(s, x).offset(-1).read();
    }

    #[test]
    #[should_panic(expected = "ub: dangling pointer")]
    fn dangling_field() {
        let p = {
            let s = outer();
            field_ptr!(s, x)
        };
        p.read();
    }
}
