// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use std::{
    any::Any,
    cell::RefCell,
    marker::PhantomData,
    rc::{Rc, Weak},
};

use crate::field::{Root, upgrade as upgrade_root};
use crate::rc::Ptr;

pub trait ByteRepr: 'static {
    fn byte_size() -> usize
    where
        Self: Sized,
    {
        panic!(
            "byte_size is not implemented for {}",
            std::any::type_name::<Self>()
        )
    }
    fn to_bytes(&self, _buf: &mut [u8]) {
        panic!(
            "to_bytes is not implemented for {}",
            std::any::type_name::<Self>()
        )
    }
    fn from_bytes(_buf: &[u8]) -> Self
    where
        Self: Sized,
    {
        panic!(
            "from_bytes is not implemented for {}",
            std::any::type_name::<Self>()
        )
    }
}

macro_rules! impl_byte_repr {
    ($ty:ty) => {
        impl ByteRepr for $ty {
            #[inline]
            fn byte_size() -> usize {
                std::mem::size_of::<$ty>()
            }
            #[inline]
            fn to_bytes(&self, buf: &mut [u8]) {
                buf.copy_from_slice(&self.to_ne_bytes());
            }
            #[inline]
            fn from_bytes(buf: &[u8]) -> Self {
                let mut a = [0u8; std::mem::size_of::<$ty>()];
                a.copy_from_slice(buf);
                <$ty>::from_ne_bytes(a)
            }
        }
    };
}

impl_byte_repr!(u8);
impl_byte_repr!(i8);
impl_byte_repr!(u16);
impl_byte_repr!(i16);
impl_byte_repr!(u32);
impl_byte_repr!(i32);
impl_byte_repr!(u64);
impl_byte_repr!(i64);
impl_byte_repr!(u128);
impl_byte_repr!(i128);
impl_byte_repr!(usize);
impl_byte_repr!(isize);
impl_byte_repr!(f32);
impl_byte_repr!(f64);

impl ByteRepr for bool {
    #[inline]
    fn byte_size() -> usize {
        1
    }
    #[inline]
    fn to_bytes(&self, buf: &mut [u8]) {
        buf[0] = *self as u8;
    }
    #[inline]
    fn from_bytes(buf: &[u8]) -> Self {
        buf[0] != 0
    }
}

impl ByteRepr for () {}
impl ByteRepr for std::fs::File {}
impl<T: ByteRepr> ByteRepr for Vec<T> {}
impl<T: ByteRepr> ByteRepr for Option<T> {}
// The bytes of a Value, like a field stored in one, are those of its value.
impl<T: ByteRepr> ByteRepr for std::rc::Rc<T> {
    #[inline]
    fn byte_size() -> usize {
        T::byte_size()
    }
    #[inline]
    fn to_bytes(&self, buf: &mut [u8]) {
        (**self).to_bytes(buf)
    }
    #[inline]
    fn from_bytes(buf: &[u8]) -> Self {
        Rc::new(T::from_bytes(buf))
    }
}
impl<T: ByteRepr> ByteRepr for std::cell::RefCell<T> {
    #[inline]
    fn byte_size() -> usize {
        T::byte_size()
    }
    #[inline]
    fn to_bytes(&self, buf: &mut [u8]) {
        self.borrow().to_bytes(buf)
    }
    #[inline]
    fn from_bytes(buf: &[u8]) -> Self {
        RefCell::new(T::from_bytes(buf))
    }
}
impl<T: ByteRepr> ByteRepr for Box<[T]> {
    fn to_bytes(&self, buf: &mut [u8]) {
        let elem_size = T::byte_size();
        for (i, elem) in self.iter().enumerate() {
            elem.to_bytes(&mut buf[i * elem_size..(i + 1) * elem_size]);
        }
    }

    fn from_bytes(buf: &[u8]) -> Self {
        let elem_size = T::byte_size();
        assert_eq!(buf.len() % elem_size, 0, "ub: partial array element");
        buf.chunks(elem_size).map(T::from_bytes).collect()
    }
}
impl<T: ByteRepr> ByteRepr for Box<T> {}
impl<T: 'static> ByteRepr for *const T {}
impl<T: 'static> ByteRepr for *mut T {}
impl<A: ByteRepr, B: ByteRepr> ByteRepr for (A, B) {}
impl<K: 'static, V: 'static> ByteRepr for std::collections::BTreeMap<K, V> {}

// Runs `f` with a zeroed scratch buffer of `len` elements. Small buffers live
// on the stack so that accessing memory through a reinterpreted pointer does
// not hit the allocator.
#[inline]
pub(crate) fn with_scratch<T: Copy + Default, R>(len: usize, f: impl FnOnce(&mut [T]) -> R) -> R {
    const INLINE_LEN: usize = 64;
    if len <= INLINE_LEN {
        let mut buf = [T::default(); INLINE_LEN];
        f(&mut buf[..len])
    } else {
        f(&mut vec![T::default(); len])
    }
}

// Byte-level access to the storage of an original allocation. The
// implementations are stateless: they know the source type S, while the
// storage itself (a `RefCell<S>`, `RefCell<Vec<S>>` or `RefCell<Box<[S]>>`) is
// passed in type-erased.
pub(crate) trait AllocOps {
    fn read_bytes(&self, cell: &dyn Any, byte_offset: usize, buf: &mut [u8]);
    fn write_bytes(&self, cell: &dyn Any, byte_offset: usize, data: &[u8]);
    fn total_byte_len(&self, cell: &dyn Any) -> usize;
}

// Type-erased handle to the original allocation of a reinterpreted pointer.
// It is a plain weak reference plus a reference to the (stateless) operations
// for the concrete storage type. Hence, it doesn't require any heap
// allocation on its own.
// The storage of a field of a struct is the field alone, as if it was a
// separate allocation.
#[derive(Clone)]
pub struct OriginalAlloc {
    storage: Storage,
    ops: &'static dyn AllocOps,
}

#[derive(Clone)]
enum Storage {
    Alloc(Weak<dyn Any>),
    Field(Weak<dyn Root>, usize),
}

// The storage passed to the AllocOps of a field.
struct FieldStorage {
    root: Rc<dyn Root>,
    field: usize,
}

impl OriginalAlloc {
    pub(crate) fn single<T: ByteRepr>(weak: &Weak<RefCell<T>>) -> Self {
        Self {
            storage: Storage::Alloc(weak.clone()),
            ops: &SingleOps::<T>(PhantomData),
        }
    }

    pub(crate) fn slice<T: AsSlice>(weak: &Weak<RefCell<T>>) -> Self {
        Self {
            storage: Storage::Alloc(weak.clone()),
            ops: &SliceOps::<T>(PhantomData),
        }
    }

    // The field of type T at byte offset `field` of `root`.
    pub(crate) fn field<T: ByteRepr>(root: &Weak<dyn Root>, field: usize) -> Self {
        Self {
            storage: Storage::Field(root.clone(), field),
            ops: &FieldOps::<T>(PhantomData),
        }
    }

    #[inline]
    fn with_storage<R>(&self, f: impl FnOnce(&dyn Any) -> R) -> R {
        match &self.storage {
            Storage::Alloc(weak) => f(&*weak.upgrade().expect("ub: dangling pointer")),
            Storage::Field(root, field) => f(&FieldStorage {
                root: upgrade_root(root),
                field: *field,
            }),
        }
    }

    #[inline]
    pub(crate) fn read_bytes(&self, byte_offset: usize, buf: &mut [u8]) {
        self.with_storage(|storage| self.ops.read_bytes(storage, byte_offset, buf));
    }

    #[inline]
    pub(crate) fn write_bytes(&self, byte_offset: usize, data: &[u8]) {
        self.with_storage(|storage| self.ops.write_bytes(storage, byte_offset, data));
    }

    pub(crate) fn total_byte_len(&self) -> usize {
        self.with_storage(|storage| self.ops.total_byte_len(storage))
    }

    // Stable address used for pointer equality across PtrKind variants.
    pub(crate) fn address(&self) -> usize {
        match &self.storage {
            Storage::Alloc(weak) => weak.as_ptr() as *const () as usize,
            Storage::Field(root, field) => {
                (root.as_ptr() as *const () as usize).wrapping_add(*field)
            }
        }
    }

    pub(crate) fn delete(&self) {
        let Storage::Alloc(weak) = &self.storage else {
            panic!("ub: invalid delete");
        };
        assert_eq!(Weak::strong_count(weak), 1, "ub: invalid delete");
        unsafe {
            let strong = weak.upgrade().expect("ub: dangling pointer");
            Rc::from_raw(Rc::as_ptr(&strong));
        }
        assert_eq!(Weak::strong_count(weak), 0, "ub: double free");
    }
}

// Read bytes starting at `byte_offset` from a slice of S elements into `buf`.
// Only serializes the overlapping elements, not the whole slice.
fn slice_read_bytes<S: ByteRepr>(slice: &[S], byte_offset: usize, buf: &mut [u8]) {
    let len = buf.len();
    let elem_size = S::byte_size();
    let first_elem = byte_offset / elem_size;
    if byte_offset.is_multiple_of(elem_size) && len == elem_size {
        slice[first_elem].to_bytes(buf);
        return;
    }
    let last_elem = (byte_offset + len).div_ceil(elem_size);
    with_scratch(elem_size, |elem_buf| {
        for (idx, elem) in slice[first_elem..last_elem].iter().enumerate() {
            let elem_start = (first_elem + idx) * elem_size;
            let start = byte_offset.max(elem_start);
            let end = (byte_offset + len).min(elem_start + elem_size);
            elem.to_bytes(elem_buf);
            buf[start - byte_offset..end - byte_offset]
                .copy_from_slice(&elem_buf[start - elem_start..end - elem_start]);
        }
    });
}

// Write `data` at `byte_offset` into a slice of S elements.
// Only deserializes/reserializes the overlapping elements.
fn slice_write_bytes<S: ByteRepr>(slice: &mut [S], byte_offset: usize, data: &[u8]) {
    let elem_size = S::byte_size();
    let first_elem = byte_offset / elem_size;
    let last_elem = (byte_offset + data.len()).div_ceil(elem_size);
    if byte_offset.is_multiple_of(elem_size) && data.len() == elem_size {
        slice[first_elem] = S::from_bytes(data);
        return;
    }
    with_scratch(elem_size, |elem_buf| {
        for (idx, elem) in slice[first_elem..last_elem].iter_mut().enumerate() {
            let elem_byte_start = (first_elem + idx) * elem_size;
            elem.to_bytes(elem_buf);
            let overlap_start = byte_offset.max(elem_byte_start) - elem_byte_start;
            let overlap_end =
                (byte_offset + data.len()).min(elem_byte_start + elem_size) - elem_byte_start;
            let data_start = byte_offset.max(elem_byte_start) - byte_offset;
            elem_buf[overlap_start..overlap_end]
                .copy_from_slice(&data[data_start..data_start + (overlap_end - overlap_start)]);
            *elem = S::from_bytes(elem_buf);
        }
    });
}

struct SingleOps<T>(PhantomData<fn() -> T>);

impl<T: ByteRepr> AllocOps for SingleOps<T> {
    fn read_bytes(&self, cell: &dyn Any, byte_offset: usize, buf: &mut [u8]) {
        let cell = cell.downcast_ref::<RefCell<T>>().unwrap();
        slice_read_bytes(std::slice::from_ref(&*cell.borrow()), byte_offset, buf);
    }

    fn write_bytes(&self, cell: &dyn Any, byte_offset: usize, data: &[u8]) {
        let cell = cell.downcast_ref::<RefCell<T>>().unwrap();
        slice_write_bytes(
            std::slice::from_mut(&mut *cell.borrow_mut()),
            byte_offset,
            data,
        );
    }

    fn total_byte_len(&self, _cell: &dyn Any) -> usize {
        T::byte_size()
    }
}

pub(crate) trait AsSlice: 'static {
    type Elem: ByteRepr;
    fn as_slice(&self) -> &[Self::Elem];
    fn as_slice_mut(&mut self) -> &mut [Self::Elem];
    // Direct access to the bytes of an allocation of `u8`. Avoids going through
    // the generic (de)serialization of the elements.
    fn as_u8_slice(&self) -> Option<&[u8]>;
    fn as_u8_slice_mut(&mut self) -> Option<&mut [u8]>;
    // Likewise for `i8` (char), e.g., a std::string accessed as uint8_t*.
    fn as_i8_slice(&self) -> Option<&[i8]>;
    fn as_i8_slice_mut(&mut self) -> Option<&mut [i8]>;
}

impl<S: ByteRepr> AsSlice for Vec<S> {
    type Elem = S;
    fn as_slice(&self) -> &[S] {
        self
    }
    fn as_slice_mut(&mut self) -> &mut [S] {
        self
    }
    fn as_u8_slice(&self) -> Option<&[u8]> {
        (self as &dyn Any).downcast_ref::<Vec<u8>>().map(|v| &v[..])
    }
    fn as_u8_slice_mut(&mut self) -> Option<&mut [u8]> {
        (self as &mut dyn Any)
            .downcast_mut::<Vec<u8>>()
            .map(|v| &mut v[..])
    }
    fn as_i8_slice(&self) -> Option<&[i8]> {
        (self as &dyn Any).downcast_ref::<Vec<i8>>().map(|v| &v[..])
    }
    fn as_i8_slice_mut(&mut self) -> Option<&mut [i8]> {
        (self as &mut dyn Any)
            .downcast_mut::<Vec<i8>>()
            .map(|v| &mut v[..])
    }
}

impl<S: ByteRepr> AsSlice for Box<[S]> {
    type Elem = S;
    fn as_slice(&self) -> &[S] {
        self
    }
    fn as_slice_mut(&mut self) -> &mut [S] {
        self
    }
    fn as_u8_slice(&self) -> Option<&[u8]> {
        (self as &dyn Any)
            .downcast_ref::<Box<[u8]>>()
            .map(|v| &v[..])
    }
    fn as_u8_slice_mut(&mut self) -> Option<&mut [u8]> {
        (self as &mut dyn Any)
            .downcast_mut::<Box<[u8]>>()
            .map(|v| &mut v[..])
    }
    fn as_i8_slice(&self) -> Option<&[i8]> {
        (self as &dyn Any)
            .downcast_ref::<Box<[i8]>>()
            .map(|v| &v[..])
    }
    fn as_i8_slice_mut(&mut self) -> Option<&mut [i8]> {
        (self as &mut dyn Any)
            .downcast_mut::<Box<[i8]>>()
            .map(|v| &mut v[..])
    }
}

struct SliceOps<T>(PhantomData<fn() -> T>);

impl<T: AsSlice> AllocOps for SliceOps<T> {
    fn read_bytes(&self, cell: &dyn Any, byte_offset: usize, buf: &mut [u8]) {
        let cell = cell.downcast_ref::<RefCell<T>>().unwrap();
        let val = cell.borrow();
        if let Some(bytes) = val.as_u8_slice() {
            buf.copy_from_slice(&bytes[byte_offset..byte_offset + buf.len()]);
        } else if let Some(chars) = val.as_i8_slice() {
            let chars = &chars[byte_offset..byte_offset + buf.len()];
            for (b, &c) in buf.iter_mut().zip(chars) {
                *b = c as u8;
            }
        } else {
            slice_read_bytes(val.as_slice(), byte_offset, buf);
        }
    }

    fn write_bytes(&self, cell: &dyn Any, byte_offset: usize, data: &[u8]) {
        let cell = cell.downcast_ref::<RefCell<T>>().unwrap();
        let mut val = cell.borrow_mut();
        if let Some(bytes) = val.as_u8_slice_mut() {
            bytes[byte_offset..byte_offset + data.len()].copy_from_slice(data);
            return;
        }
        if let Some(chars) = val.as_i8_slice_mut() {
            let chars = &mut chars[byte_offset..byte_offset + data.len()];
            for (c, &b) in chars.iter_mut().zip(data) {
                *c = b as i8;
            }
            return;
        }
        slice_write_bytes(val.as_slice_mut(), byte_offset, data);
    }

    fn total_byte_len(&self, cell: &dyn Any) -> usize {
        let cell = cell.downcast_ref::<RefCell<T>>().unwrap();
        cell.borrow().as_slice().len() * <T::Elem as ByteRepr>::byte_size()
    }
}

struct FieldOps<T>(PhantomData<fn() -> T>);

impl<T: ByteRepr> AllocOps for FieldOps<T> {
    fn read_bytes(&self, storage: &dyn Any, byte_offset: usize, buf: &mut [u8]) {
        let storage = storage.downcast_ref::<FieldStorage>().unwrap();
        let field = Ptr::<T>::borrow_field(&*storage.root, storage.field);
        slice_read_bytes(std::slice::from_ref(&*field), byte_offset, buf);
    }

    fn write_bytes(&self, storage: &dyn Any, byte_offset: usize, data: &[u8]) {
        let storage = storage.downcast_ref::<FieldStorage>().unwrap();
        let mut field = Ptr::<T>::borrow_field_mut(&*storage.root, storage.field);
        slice_write_bytes(std::slice::from_mut(&mut *field), byte_offset, data);
    }

    fn total_byte_len(&self, _storage: &dyn Any) -> usize {
        T::byte_size()
    }
}
