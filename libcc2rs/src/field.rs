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
// Arrays and vectors are not stored inline, but in Values of their own, so
// that pointers to their elements point into those Values. A field pointer
// hence always points to a single object.

use std::any::{Any, TypeId};
use std::cell::{Cell, Ref, RefCell, RefMut};
use std::rc::{Rc, Weak};

use crate::rc::{AsPointer, Ptr, PtrKind, Value, null_deref};
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
}

// Creates pointers to the fields of the struct S pointed to by self. Use it
// through the field_ptr! macro.
pub trait FieldPtr<S: Record> {
    // A pointer to the field at `offset(S::OFFSETS)`, whose type is given by
    // the (uncalled) `field`.
    fn field_ptr<T: ByteRepr>(
        &self,
        field: fn(&S) -> &T,
        offset: fn(&S::Offsets) -> usize,
    ) -> Ptr<T>;
}

impl<S: Record> FieldPtr<S> for Ptr<S> {
    #[inline]
    fn field_ptr<T: ByteRepr>(
        &self,
        _field: fn(&S) -> &T,
        offset: fn(&S::Offsets) -> usize,
    ) -> Ptr<T> {
        let offset = offset(&S::OFFSETS);
        // The byte offset of the struct in its allocation.
        let start = || self.offset.wrapping_mul(S::byte_size());
        let (root, start): (Weak<dyn Root>, usize) = match &self.kind {
            PtrKind::Null => null_deref(),
            PtrKind::StackSingle(w) | PtrKind::HeapSingle(w) => (w.clone(), start()),
            PtrKind::StackArray(w) | PtrKind::HeapArray(w) => (w.clone(), start()),
            PtrKind::StackVec(w) | PtrKind::HeapVec(w) => (w.clone(), start()),
            PtrKind::Field(root) => (root.clone(), self.offset),
            PtrKind::Reinterpreted(_) => {
                let (alloc, byte_offset) = self.original_alloc().unwrap();
                return Ptr::from_original_alloc(alloc, byte_offset.wrapping_add(offset));
            }
        };
        Ptr {
            offset: start.wrapping_add(offset),
            kind: PtrKind::Field(root),
        }
    }
}

impl<S: Record> FieldPtr<S> for Value<S> {
    #[inline]
    fn field_ptr<T: ByteRepr>(
        &self,
        field: fn(&S) -> &T,
        offset: fn(&S::Offsets) -> usize,
    ) -> Ptr<T> {
        self.as_pointer().field_ptr(field, offset)
    }
}

// A pointer to field `field` of the struct that a Ptr or a Value holds,
// e.g., `field_ptr!(p, x)`.
#[macro_export]
macro_rules! field_ptr {
    ($base:expr, $field:ident) => {{
        use $crate::FieldPtr as _;
        ($base).field_ptr(|__s| &__s.$field, |__o| __o.$field)
    }};
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

    #[derive(Default, Record)]
    struct Inner {
        #[offset(0)]
        a: i32,
        #[offset(4)]
        b: i32,
    }

    impl ByteRepr for Inner {
        fn byte_size() -> usize {
            8
        }
        fn to_bytes(&self, buf: &mut [u8]) {
            self.a.to_bytes(&mut buf[0..4]);
            self.b.to_bytes(&mut buf[4..8]);
        }
        fn from_bytes(buf: &[u8]) -> Self {
            Self {
                a: i32::from_bytes(&buf[0..4]),
                b: i32::from_bytes(&buf[4..8]),
            }
        }
    }

    #[derive(Default, Record)]
    struct Outer {
        #[offset(0)]
        x: i32,
        #[offset(4)]
        inner: Inner,
        #[offset(12)]
        items: Value<Box<[Inner]>>,
        #[offset(40)]
        v: Value<Vec<Inner>>,
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
            items: Rc::new(RefCell::new(
                vec![inner(1), inner(2), inner(3)].into_boxed_slice(),
            )),
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
        // Arrays are held in Values of their own, which are the roots of the
        // field pointers into their elements.
        let s = outer();
        let items: Ptr<Inner> = s.borrow().items.as_pointer();
        let a = field_ptr!(items.offset(1), a);
        assert_eq!(a.read(), 2);
        a.write(20);
        assert_eq!(s.borrow().items.borrow()[1].a, 20);
        let b = field_ptr!(items.offset(2), b);
        assert_eq!(b.read(), -3);
        let first = field_ptr!(items, a);
        assert!(first < a);
        assert_eq!(first, field_ptr!(items.offset(0), a));
    }

    #[test]
    fn vector_field() {
        let s = outer();
        *s.borrow().v.borrow_mut() = vec![inner(1), inner(2)];
        let v: Ptr<Inner> = s.borrow().v.as_pointer();
        let a = field_ptr!(v.offset(1), a);
        assert_eq!(a.read(), 2);
        s.borrow().v.borrow_mut().push(inner(3));
        a.write(20);
        assert_eq!(s.borrow().v.borrow()[1].a, 20);
        assert_eq!(field_ptr!(v.offset(2), b).read(), -3);
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
