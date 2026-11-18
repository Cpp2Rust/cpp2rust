// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use std::{
    any::{Any, TypeId},
    cell::RefCell,
    marker::PhantomData,
    rc::{Rc, Weak},
};

use crate::escape;
use crate::field::{Root, upgrade as upgrade_root};
use crate::rc::{ElemStorage, Ptr, SliceParent, SliceRoot, reclaim};

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
    // The alignment of the type in C, where it is larger than that of malloc,
    // as that of over-aligned structs.
    fn byte_align() -> usize
    where
        Self: Sized,
    {
        1
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
    // Whether the values of the type have different sizes, as arrays (boxed
    // slices) do, which have no byte_size.
    const VARIABLE_SIZE: bool = false;
    fn value_byte_size(&self) -> usize
    where
        Self: Sized,
    {
        Self::byte_size()
    }
    // Writes the bytes of a value, as from_bytes reads them, to this one,
    // keeping the lengths of its arrays.
    fn assign_bytes(&mut self, buf: &[u8])
    where
        Self: Sized,
    {
        *self = Self::from_bytes(buf);
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
    fn byte_align() -> usize {
        T::byte_align()
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
    fn byte_align() -> usize {
        T::byte_align()
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
    fn byte_align() -> usize {
        T::byte_align()
    }

    fn to_bytes(&self, buf: &mut [u8]) {
        let mut offset = 0;
        for elem in self.iter() {
            let size = elem.value_byte_size();
            elem.to_bytes(&mut buf[offset..offset + size]);
            offset += size;
        }
    }

    const VARIABLE_SIZE: bool = true;

    fn value_byte_size(&self) -> usize {
        self.iter().map(T::value_byte_size).sum()
    }

    fn assign_bytes(&mut self, buf: &[u8]) {
        let mut offset = 0;
        for elem in self.iter_mut() {
            let size = elem.value_byte_size();
            elem.assign_bytes(&buf[offset..offset + size]);
            offset += size;
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
impl ByteRepr for std::rc::Rc<dyn std::any::Any> {}

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
    fn byte_align(&self) -> usize;
    // The type of the contents of the Value of the storage, if it is one.
    fn value_type(&self) -> Option<TypeId> {
        None
    }
    // The bytes that the elements of a vector can take before it is
    // reallocated, if it is not borrowed mutably. None for other storage,
    // which is never reallocated.
    fn byte_capacity(&self, _cell: &dyn Any) -> Option<usize> {
        None
    }
    // Where the elements of nested storage are, the address of their buffer
    // and its byte capacity if it is their own, and the Value that holds the
    // container. None for other storage.
    fn elem_storage(&self, _storage: &dyn Any) -> Option<ElemStorage> {
        None
    }
    fn own_buffer(&self, _storage: &dyn Any) -> Option<(usize, usize)> {
        None
    }
    fn root(&self, _storage: &dyn Any) -> Option<Weak<dyn Any>> {
        None
    }
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
    Nested(Rc<dyn Any>, usize),
}

// The storage passed to the AllocOps of a field.
struct FieldStorage {
    root: Rc<dyn Root>,
    field: usize,
}

impl OriginalAlloc {
    pub(crate) fn single<T: ByteRepr>(weak: &Weak<RefCell<T>>) -> Self {
        Self::single_in::<T>(weak.clone())
    }

    pub(crate) fn slice<T: AsSlice>(weak: &Weak<RefCell<T>>) -> Self {
        Self::slice_in::<T>(weak.clone())
    }

    // The Value `weak`, which holds a RefCell<T>.
    pub(crate) fn single_in<T: ByteRepr>(weak: Weak<dyn Any>) -> Self {
        Self {
            storage: Storage::Alloc(weak),
            ops: &SingleOps::<T>(PhantomData),
        }
    }

    pub(crate) fn slice_in<T: AsSlice>(weak: Weak<dyn Any>) -> Self {
        Self {
            storage: Storage::Alloc(weak),
            ops: &SliceOps::<T>(PhantomData),
        }
    }

    // The Value of an allocation that is a whole Value.
    pub(crate) fn value(&self) -> Option<&Weak<dyn Any>> {
        match &self.storage {
            Storage::Alloc(weak) => Some(weak),
            _ => None,
        }
    }

    // The type of the contents of the Value of the storage, if it is one.
    pub(crate) fn value_type(&self) -> Option<TypeId> {
        self.ops.value_type()
    }

    // The container of nested storage, as the parent of pointers to its
    // elements.
    pub(crate) fn nested_parent<T: 'static>(&self) -> Option<Rc<dyn SliceParent<T>>> {
        match &self.storage {
            Storage::Nested(parent, _) => parent.downcast_ref::<Rc<dyn SliceParent<T>>>().cloned(),
            _ => None,
        }
    }

    // What identifies the allocation of the integers of pointers into it: the
    // address of its Value, or of the buffer of a vector that is not a Value,
    // e.g., a vector field.
    pub(crate) fn key(&self) -> usize {
        match &self.storage {
            Storage::Alloc(weak) => escape::key(weak),
            _ => self.own_buffer().unwrap().0,
        }
    }

    pub(crate) fn is_freed(&self) -> bool {
        self.value().unwrap().strong_count() == 0
    }

    // The address of the buffer of the elements of nested storage that are
    // in one of their own, and its byte capacity.
    pub(crate) fn own_buffer(&self) -> Option<(usize, usize)> {
        match &self.storage {
            Storage::Nested(parent, _) => self.ops.own_buffer(&**parent),
            _ => None,
        }
    }

    // The Value that holds nested storage.
    pub(crate) fn root(&self) -> Option<Weak<dyn Any>> {
        match &self.storage {
            Storage::Nested(parent, _) => self.ops.root(&**parent),
            _ => None,
        }
    }

    pub(crate) fn byte_align(&self) -> usize {
        self.ops.byte_align()
    }

    pub(crate) fn byte_capacity(&self) -> Option<usize> {
        self.with_storage(|storage| self.ops.byte_capacity(storage))
    }

    // The field of type T at byte offset `field` of `root`.
    pub(crate) fn field<T: ByteRepr>(root: &Weak<dyn Root>, field: usize) -> Self {
        Self {
            storage: Storage::Field(root.clone(), field),
            ops: &FieldOps::<T>(PhantomData),
        }
    }

    pub(crate) fn nested<T: ByteRepr>(parent: &Rc<dyn SliceParent<T>>) -> Self {
        Self {
            storage: Storage::Nested(Rc::new(Rc::clone(parent)), parent.address()),
            ops: &NestedOps::<T>(PhantomData),
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
            Storage::Nested(parent, _) => f(&**parent),
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
            Storage::Nested(_, address) => *address,
        }
    }

    // The Value that holds byte `offset` of the storage, and the offset of
    // the byte in it, for the integers that pointers are cast to.
    pub(crate) fn in_value(self, offset: usize) -> (Self, usize) {
        match &self.storage {
            Storage::Alloc(_) => (self, offset),
            Storage::Field(root, field) => (
                upgrade_root(root).original_alloc(root.clone()),
                field.wrapping_add(offset),
            ),
            // The elements of a vector are in an allocation of their own.
            Storage::Nested(parent, _) => match self.ops.elem_storage(&**parent) {
                Some(ElemStorage::In(alloc, start)) => alloc.in_value(start.wrapping_add(offset)),
                _ => (self, offset),
            },
        }
    }

    // The address of the Value that holds byte `offset` of the storage, and
    // the offset of the byte in it, which order pointers. Unlike in_value,
    // it doesn't need the Value to be alive.
    pub(crate) fn value_place(&self, offset: usize) -> (usize, usize) {
        match &self.storage {
            Storage::Alloc(_) => (self.value_address(), offset),
            Storage::Field(_, field) => (self.value_address(), field.wrapping_add(offset)),
            Storage::Nested(parent, address) => match self.ops.elem_storage(&**parent) {
                Some(ElemStorage::In(alloc, start)) => {
                    alloc.value_place(start.wrapping_add(offset))
                }
                _ => (*address, offset),
            },
        }
    }

    // The address of the Value of the storage, which orders pointers into
    // different Values.
    pub(crate) fn value_address(&self) -> usize {
        match &self.storage {
            Storage::Alloc(weak) => weak.as_ptr() as *const () as usize,
            Storage::Field(root, _) => root.as_ptr() as *const () as usize,
            Storage::Nested(_, address) => *address,
        }
    }

    pub(crate) fn delete(&self) {
        let Storage::Alloc(weak) = &self.storage else {
            panic!("ub: invalid delete");
        };
        drop(reclaim(weak, "ub: invalid delete"));
    }
}

// Read bytes starting at `byte_offset` from a slice of S elements into `buf`.
// Only serializes the overlapping elements, not the whole slice.
fn slice_read_bytes<S: ByteRepr>(slice: &[S], byte_offset: usize, buf: &mut [u8]) {
    if S::VARIABLE_SIZE {
        return rows_read_bytes(slice, byte_offset, buf);
    }
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

// The bytes of rows, whose sizes differ, e.g., of a 2D array.
fn rows_read_bytes<S: ByteRepr>(rows: &[S], byte_offset: usize, buf: &mut [u8]) {
    let end = byte_offset + buf.len();
    let mut start = 0;
    for row in rows {
        let size = row.value_byte_size();
        if start < end && byte_offset < start + size {
            let mut bytes = vec![0; size];
            row.to_bytes(&mut bytes);
            let (from, to) = (byte_offset.max(start), end.min(start + size));
            buf[from - byte_offset..to - byte_offset]
                .copy_from_slice(&bytes[from - start..to - start]);
        }
        start += size;
    }
}

fn rows_write_bytes<S: ByteRepr>(rows: &mut [S], byte_offset: usize, data: &[u8]) {
    let end = byte_offset + data.len();
    let mut start = 0;
    for row in rows {
        let size = row.value_byte_size();
        if start < end && byte_offset < start + size {
            let mut bytes = vec![0; size];
            row.to_bytes(&mut bytes);
            let (from, to) = (byte_offset.max(start), end.min(start + size));
            bytes[from - start..to - start]
                .copy_from_slice(&data[from - byte_offset..to - byte_offset]);
            row.assign_bytes(&bytes);
        }
        start += size;
    }
}

// Write `data` at `byte_offset` into a slice of S elements.
// Only deserializes/reserializes the overlapping elements.
fn slice_write_bytes<S: ByteRepr>(slice: &mut [S], byte_offset: usize, data: &[u8]) {
    if S::VARIABLE_SIZE {
        return rows_write_bytes(slice, byte_offset, data);
    }
    let elem_size = S::byte_size();
    let first_elem = byte_offset / elem_size;
    let last_elem = (byte_offset + data.len()).div_ceil(elem_size);
    if byte_offset.is_multiple_of(elem_size) && data.len() == elem_size {
        slice[first_elem].assign_bytes(data);
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
            elem.assign_bytes(elem_buf);
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

    fn total_byte_len(&self, cell: &dyn Any) -> usize {
        if !T::VARIABLE_SIZE {
            return T::byte_size();
        }
        let cell = cell.downcast_ref::<RefCell<T>>().unwrap();
        cell.borrow().value_byte_size()
    }

    fn byte_align(&self) -> usize {
        T::byte_align()
    }

    fn value_type(&self) -> Option<TypeId> {
        Some(TypeId::of::<T>())
    }
}

pub(crate) trait AsSlice: 'static {
    type Elem: ByteRepr;
    // Whether the storage of the elements is reallocated as they are added.
    const RESIZABLE: bool;
    fn capacity(&self) -> usize;
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
    const RESIZABLE: bool = true;
    fn capacity(&self) -> usize {
        self.capacity()
    }
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
    const RESIZABLE: bool = false;
    fn capacity(&self) -> usize {
        self.len()
    }
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
        elems_byte_len(cell.borrow().as_slice())
    }

    fn byte_align(&self) -> usize {
        <T::Elem as ByteRepr>::byte_align()
    }

    fn value_type(&self) -> Option<TypeId> {
        Some(TypeId::of::<T>())
    }

    fn byte_capacity(&self, cell: &dyn Any) -> Option<usize> {
        if !T::RESIZABLE {
            return None;
        }
        let cell = cell.downcast_ref::<RefCell<T>>().unwrap();
        let value = cell.try_borrow().ok()?;
        Some(capacity_bytes(value.as_slice(), value.capacity()))
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

    fn byte_align(&self) -> usize {
        T::byte_align()
    }
}

struct NestedOps<T>(PhantomData<fn() -> T>);

impl<T: ByteRepr> AllocOps for NestedOps<T> {
    fn read_bytes(&self, parent: &dyn Any, byte_offset: usize, buf: &mut [u8]) {
        let parent = parent.downcast_ref::<Rc<dyn SliceParent<T>>>().unwrap();
        let root = parent.root();
        slice_read_bytes(&parent.borrow_slice(&root), byte_offset, buf);
    }

    fn write_bytes(&self, parent: &dyn Any, byte_offset: usize, data: &[u8]) {
        let parent = parent.downcast_ref::<Rc<dyn SliceParent<T>>>().unwrap();
        let root = parent.root();
        slice_write_bytes(&mut parent.borrow_slice_mut(&root), byte_offset, data);
    }

    fn total_byte_len(&self, parent: &dyn Any) -> usize {
        let parent = parent.downcast_ref::<Rc<dyn SliceParent<T>>>().unwrap();
        let root = parent.root();
        elems_byte_len(&parent.borrow_slice(&root))
    }

    fn byte_align(&self) -> usize {
        T::byte_align()
    }

    fn byte_capacity(&self, parent: &dyn Any) -> Option<usize> {
        Some(self.own_buffer(parent)?.1)
    }

    fn elem_storage(&self, parent: &dyn Any) -> Option<ElemStorage> {
        let parent = parent.downcast_ref::<Rc<dyn SliceParent<T>>>().unwrap();
        Some(parent.elem_storage())
    }

    fn own_buffer(&self, parent: &dyn Any) -> Option<(usize, usize)> {
        let parent = parent.downcast_ref::<Rc<dyn SliceParent<T>>>().unwrap();
        let ElemStorage::Own(address, capacity) = parent.elem_storage() else {
            return None;
        };
        let root = parent.root();
        let elems = parent.borrow_slice(&root);
        Some((address, capacity_bytes(&elems, capacity)))
    }

    fn root(&self, parent: &dyn Any) -> Option<Weak<dyn Any>> {
        let parent = parent.downcast_ref::<Rc<dyn SliceParent<T>>>().unwrap();
        Some(match parent.root() {
            SliceRoot::Alloc(root) => Rc::downgrade(&root),
            SliceRoot::Field(root) => Rc::downgrade(&(root as Rc<dyn Any>)),
        })
    }
}

// The bytes of the elements, whose sizes differ if they are rows.
fn elems_byte_len<S: ByteRepr>(elems: &[S]) -> usize {
    if S::VARIABLE_SIZE {
        elems.iter().map(S::value_byte_size).sum()
    } else {
        elems.len() * S::byte_size()
    }
}

// The bytes that `capacity` elements take, which are rows as long as the
// first one if their sizes differ, as the arrays in a vector are.
fn capacity_bytes<S: ByteRepr>(elems: &[S], capacity: usize) -> usize {
    if S::VARIABLE_SIZE {
        capacity * elems.first().map_or(0, S::value_byte_size)
    } else {
        capacity * S::byte_size()
    }
}
