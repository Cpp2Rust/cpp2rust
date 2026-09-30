// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

// Pointers to the fields of structs.
//
// Struct fields are stored inline in their struct, and hence a whole struct
// lives in a single Value. A pointer to a field records the allocation that
// holds the struct and the byte offset of the field in it (in the C layout),
// similarly to how a pointer to an array element records the array and the
// index of the element. The field is found from the offset when accessed.

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
    // itself, a field, a field of a field, and so on. For a field that is an
    // array of `ty`, the array.
    fn locate(&self, offset: usize, ty: TypeId) -> Option<&dyn Any>;
    fn locate_mut(&mut self, offset: usize, ty: TypeId) -> Option<&mut dyn Any>;
}

// The elements that a pointer of type T sees in an object: the object itself,
// or the elements of an array.
pub trait Elems<T> {}
impl<T> Elems<T> for T {}
impl<T> Elems<T> for Box<[T]> {}

#[inline]
pub(crate) fn elems<T: 'static>(obj: &dyn Any) -> &[T] {
    if let Some(v) = obj.downcast_ref::<T>() {
        std::slice::from_ref(v)
    } else if let Some(a) = obj.downcast_ref::<Box<[T]>>() {
        a
    } else {
        invalid_field()
    }
}

#[inline]
pub(crate) fn elems_mut<T: 'static>(obj: &mut dyn Any) -> &mut [T] {
    if obj.is::<T>() {
        std::slice::from_mut(obj.downcast_mut::<T>().unwrap())
    } else if let Some(a) = obj.downcast_mut::<Box<[T]>>() {
        a
    } else {
        invalid_field()
    }
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
    // A pointer to the field at `offset(S::OFFSETS)`. The field has type U,
    // as given by the (uncalled) `field`, and T is either U or, if U is an
    // array, the type of its elements.
    fn field_ptr<T: ByteRepr, U: Elems<T>>(
        &self,
        field: fn(&S) -> &U,
        offset: fn(&S::Offsets) -> usize,
    ) -> Ptr<T>;
}

impl<S: Record> FieldPtr<S> for Ptr<S> {
    #[inline]
    fn field_ptr<T: ByteRepr, U: Elems<T>>(
        &self,
        _field: fn(&S) -> &U,
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
            PtrKind::Field(root, field) => (root.clone(), *field as usize + start()),
            PtrKind::Reinterpreted(_) => {
                let (alloc, byte_offset) = self.original_alloc().unwrap();
                return Ptr::from_original_alloc(alloc, byte_offset.wrapping_add(offset));
            }
        };
        Ptr {
            offset: 0,
            kind: PtrKind::Field(root, (start + offset) as u32),
        }
    }
}

impl<S: Record> FieldPtr<S> for Value<S> {
    #[inline]
    fn field_ptr<T: ByteRepr, U: Elems<T>>(
        &self,
        field: fn(&S) -> &U,
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
// specialization: `(&&&&Locate(&field)).locate(...)` picks the first of the
// traits below that is implemented for the field's type.
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
pub trait LocateRecords<'a> {
    fn locate(self, offset: usize, ty: TypeId) -> Option<&'a dyn Any>;
}
#[doc(hidden)]
pub trait LocateArray<'a> {
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
pub trait LocateRecordsMut<'a> {
    fn locate(self, offset: usize, ty: TypeId) -> Option<&'a mut dyn Any>;
}
#[doc(hidden)]
pub trait LocateArrayMut<'a> {
    fn locate(self, offset: usize, ty: TypeId) -> Option<&'a mut dyn Any>;
}
#[doc(hidden)]
pub trait LocateLeafMut<'a> {
    fn locate(self, offset: usize, ty: TypeId) -> Option<&'a mut dyn Any>;
}

// A struct.
impl<'a, R: Record> LocateRecord<'a> for &&&&Locate<'a, R> {
    #[inline]
    fn locate(self, offset: usize, ty: TypeId) -> Option<&'a dyn Any> {
        self.0.locate(offset, ty)
    }
}

impl<'a, R: Record> LocateRecordMut<'a> for &&&&LocateMut<'a, R> {
    #[inline]
    fn locate(self, offset: usize, ty: TypeId) -> Option<&'a mut dyn Any> {
        self.take().locate_mut(offset, ty)
    }
}

// An array of structs.
impl<'a, R: Record> LocateRecords<'a> for &&&Locate<'a, Box<[R]>> {
    #[inline]
    fn locate(self, offset: usize, ty: TypeId) -> Option<&'a dyn Any> {
        // The Box itself is returned, as elems() looks for a Box<[R]>.
        let array = self.0;
        if offset == 0 && (ty == TypeId::of::<R>() || ty == TypeId::of::<Box<[R]>>()) {
            return Some(array as &dyn Any);
        }
        let (idx, offset) = split::<R>(offset);
        array.get(idx)?.locate(offset, ty)
    }
}

impl<'a, R: Record> LocateRecordsMut<'a> for &&&LocateMut<'a, Box<[R]>> {
    #[inline]
    fn locate(self, offset: usize, ty: TypeId) -> Option<&'a mut dyn Any> {
        let array: &'a mut Box<[R]> = self.take();
        if offset == 0 && (ty == TypeId::of::<R>() || ty == TypeId::of::<Box<[R]>>()) {
            return Some(array);
        }
        let (idx, offset) = split::<R>(offset);
        array.get_mut(idx)?.locate_mut(offset, ty)
    }
}

// An array of anything else, which is where it ends.
impl<'a, T: 'static> LocateArray<'a> for &&Locate<'a, Box<[T]>> {
    #[inline]
    fn locate(self, offset: usize, ty: TypeId) -> Option<&'a dyn Any> {
        (offset == 0 && (ty == TypeId::of::<T>() || ty == TypeId::of::<Box<[T]>>()))
            .then_some(self.0 as &dyn Any)
    }
}

impl<'a, T: 'static> LocateArrayMut<'a> for &&LocateMut<'a, Box<[T]>> {
    #[inline]
    fn locate(self, offset: usize, ty: TypeId) -> Option<&'a mut dyn Any> {
        let array: &'a mut Box<[T]> = self.take();
        (offset == 0 && (ty == TypeId::of::<T>() || ty == TypeId::of::<Box<[T]>>()))
            .then_some(array as &mut dyn Any)
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
        use $crate::__field::{LocateArray, LocateLeaf, LocateRecord, LocateRecords};
        (&&&&$crate::__field::Locate($field)).locate($offset, $ty)
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __locate_field_mut {
    ($field:expr, $offset:expr, $ty:expr) => {{
        #[allow(unused_imports)]
        use $crate::__field::{LocateArrayMut, LocateLeafMut, LocateRecordMut, LocateRecordsMut};
        (&&&&$crate::__field::LocateMut::new($field)).locate($offset, $ty)
    }};
}

impl<T: 'static> Ptr<T> {
    // Borrows the elements of the struct field pointed to by a Field pointer.
    #[inline]
    pub(crate) fn borrow_field<'a>(root: &'a dyn Root, field: u32) -> Ref<'a, [T]> {
        Ref::map(root.borrow_at(field as usize, TypeId::of::<T>()), |obj| {
            elems::<T>(obj)
        })
    }

    #[inline]
    pub(crate) fn borrow_field_mut<'a>(root: &'a dyn Root, field: u32) -> RefMut<'a, [T]> {
        RefMut::map(
            root.borrow_at_mut(field as usize, TypeId::of::<T>()),
            |obj| elems_mut::<T>(obj),
        )
    }
}

impl<T: 'static> Ptr<T> {
    pub(crate) fn field_len(root: &Weak<dyn Root>, field: u32) -> usize {
        let root = upgrade(root);
        Self::borrow_field(&*root, field).len()
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
        name: Box<[u8]>,
    }

    impl ByteRepr for Inner {
        fn byte_size() -> usize {
            8
        }
        fn to_bytes(&self, buf: &mut [u8]) {
            self.a.to_bytes(&mut buf[0..4]);
            buf[4..8].copy_from_slice(&self.name);
        }
        fn from_bytes(buf: &[u8]) -> Self {
            Self {
                a: i32::from_bytes(&buf[0..4]),
                name: Box::from(&buf[4..8]),
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
        items: Box<[Inner]>,
        #[offset(36)]
        v: Value<Vec<Inner>>,
    }

    impl ByteRepr for Outer {
        fn byte_size() -> usize {
            64
        }
    }

    fn inner(a: i32) -> Inner {
        Inner {
            a,
            name: Box::from(*b"abc\0"),
        }
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
        assert_eq!(std::mem::size_of::<Ptr<i32>>(), 32);
    }

    #[test]
    fn nested_field() {
        let s = outer();
        let p = field_ptr!(s, inner);
        let a = field_ptr!(p, a);
        a.write(3);
        assert_eq!(s.borrow().inner.a, 3);
        assert_eq!(p.upgrade().deref().a, 3);
        p.upgrade().deref_mut().a = 4;
        assert_eq!(a.read(), 4);
        p.with_mut(|p| p.a = 5);
        assert_eq!(a.read(), 5);
    }

    #[test]
    fn array_field() {
        let s: Value<Inner> = Rc::new(RefCell::new(inner(1)));
        let name: Ptr<u8> = field_ptr!(s, name);
        assert_eq!(name.len(), 4);
        assert_eq!(name.offset(1).read(), b'b');
        name.offset(2).write(b'x');
        assert_eq!(&*s.borrow().name, b"abx\0");
        assert_eq!(name.to_c_bytes(), b"abx");
        let whole: Ptr<Box<[u8]>> = field_ptr!(s, name);
        assert_eq!(whole.decay().offset(3), name.offset(3));
        assert_eq!(name.offset(3) - name.clone(), 3);
        assert!(name.offset(1) < name.offset(2));
        name.with_slice_mut(2, |b| b.copy_from_slice(b"zy"));
        assert_eq!(&*s.borrow().name, b"zyx\0");
    }

    #[test]
    fn field_of_array_element() {
        let s = outer();
        let items: Ptr<Inner> = field_ptr!(s, items);
        let a = field_ptr!(items.offset(1), a);
        assert_eq!(a.read(), 2);
        a.write(20);
        assert_eq!(s.borrow().items[1].a, 20);
        let name: Ptr<u8> = field_ptr!(items.offset(2), name);
        name.offset(1).write(b'q');
        assert_eq!(&*s.borrow().items[2].name, b"aqc\0");
        let first = field_ptr!(items, a);
        assert!(first < a);
        assert_ne!(first, a);
        assert_eq!(first, field_ptr!(items.offset(0), a));
        // A field of an element of an array.
        let arr: Value<Box<[Inner]>> =
            Rc::new(RefCell::new(vec![inner(1), inner(2)].into_boxed_slice()));
        let a = field_ptr!((arr.as_pointer() as Ptr<Inner>).offset(1), a);
        assert_eq!(a.read(), 2);
    }

    #[test]
    fn vector_field() {
        // Vectors are held in Values of their own, which are the roots of the
        // field pointers into their elements.
        let s = outer();
        *s.borrow().v.borrow_mut() = vec![inner(1), inner(2)];
        let v: Ptr<Inner> = s.borrow().v.as_pointer();
        let a = field_ptr!(v.offset(1), a);
        assert_eq!(a.read(), 2);
        s.borrow().v.borrow_mut().push(inner(3));
        a.write(20);
        assert_eq!(s.borrow().v.borrow()[1].a, 20);
        let name: Ptr<u8> = field_ptr!(v.offset(2), name);
        assert_eq!(name.offset(1).read(), b'b');
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
        let name: Ptr<u8> = field_ptr!(view, name);
        name.offset(1).write(b'q');
        assert_eq!(raw.offset(5).read(), b'q');
        view.with_mut(|v| v.a = 0x11223344);
        assert_eq!(raw.read(), 0x44);
        raw.delete();
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
