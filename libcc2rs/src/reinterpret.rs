// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use std::{cell::RefCell, rc::Weak};

pub trait ByteRepr: 'static {
    fn to_bytes(&self, _buf: &mut [u8]) {
        panic!("ByteRepr not supported for this type");
    }
    fn from_bytes(_buf: &[u8]) -> Self
    where
        Self: Sized,
    {
        panic!("ByteRepr not supported for this type");
    }
}

macro_rules! impl_byte_repr {
    ($ty:ty) => {
        impl ByteRepr for $ty {
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
impl_byte_repr!(usize);
impl_byte_repr!(isize);
impl_byte_repr!(f32);
impl_byte_repr!(f64);

impl ByteRepr for bool {
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
impl<T: ByteRepr> ByteRepr for std::rc::Rc<T> {}
impl<T: ByteRepr> ByteRepr for std::cell::RefCell<T> {}
impl<T: ByteRepr> ByteRepr for Box<[T]> {}
impl<T: ByteRepr> ByteRepr for Box<T> {}
impl<T: 'static> ByteRepr for *const T {}
impl<T: 'static> ByteRepr for *mut T {}
impl<A: ByteRepr, B: ByteRepr> ByteRepr for (A, B) {}
impl<K: 'static, V: 'static> ByteRepr for std::collections::BTreeMap<K, V> {}

// Type-erased byte-level access to the original allocation.
// Concrete impls know the source type S; callers only see bytes.
pub trait OriginalAlloc {
    fn read_bytes(&self, byte_offset: usize, buf: &mut [u8]);
    fn write_bytes(&self, byte_offset: usize, data: &[u8]);
    fn total_byte_len(&self) -> usize;
    // Stable address used for pointer equality across PtrKind variants.
    fn address(&self) -> usize;
}

// Read bytes starting at `byte_offset` from a slice of S elements into `buf`.
// Only serializes the overlapping elements, not the whole slice.
fn slice_read_bytes<S: ByteRepr>(slice: &[S], byte_offset: usize, buf: &mut [u8]) {
    let len = buf.len();
    let elem_size = std::mem::size_of::<S>();
    let first_elem = byte_offset / elem_size;
    let last_elem = (byte_offset + len).div_ceil(elem_size);
    let tmp_len = (last_elem - first_elem) * elem_size;
    let mut tmp = vec![0u8; tmp_len];
    for (i, elem_idx) in (first_elem..last_elem).enumerate() {
        slice[elem_idx].to_bytes(&mut tmp[i * elem_size..(i + 1) * elem_size]);
    }
    let start_in_tmp = byte_offset - first_elem * elem_size;
    buf.copy_from_slice(&tmp[start_in_tmp..start_in_tmp + len]);
}

// Write `data` at `byte_offset` into a slice of S elements.
// Only deserializes/reserializes the overlapping elements.
fn slice_write_bytes<S: ByteRepr>(slice: &mut [S], byte_offset: usize, data: &[u8]) {
    let elem_size = std::mem::size_of::<S>();
    let first_elem = byte_offset / elem_size;
    let last_elem = (byte_offset + data.len()).div_ceil(elem_size);
    for elem_idx in first_elem..last_elem {
        let elem_byte_start = elem_idx * elem_size;
        let mut elem_buf = vec![0u8; elem_size];
        slice[elem_idx].to_bytes(&mut elem_buf);
        let overlap_start = byte_offset.max(elem_byte_start) - elem_byte_start;
        let overlap_end =
            (byte_offset + data.len()).min(elem_byte_start + elem_size) - elem_byte_start;
        let data_start = byte_offset.max(elem_byte_start) - byte_offset;
        elem_buf[overlap_start..overlap_end]
            .copy_from_slice(&data[data_start..data_start + (overlap_end - overlap_start)]);
        slice[elem_idx] = S::from_bytes(&elem_buf);
    }
}

pub(crate) struct SingleOriginalAlloc<T: ByteRepr> {
    pub weak: Weak<RefCell<T>>,
}

impl<T: ByteRepr> OriginalAlloc for SingleOriginalAlloc<T> {
    fn read_bytes(&self, byte_offset: usize, buf: &mut [u8]) {
        let rc = self.weak.upgrade().expect("ub: dangling pointer");
        let val = rc.borrow();
        slice_read_bytes(std::slice::from_ref(&*val), byte_offset, buf);
    }

    fn write_bytes(&self, byte_offset: usize, data: &[u8]) {
        let rc = self.weak.upgrade().expect("ub: dangling pointer");
        let mut val = rc.borrow_mut();
        slice_write_bytes(std::slice::from_mut(&mut *val), byte_offset, data);
    }

    fn total_byte_len(&self) -> usize {
        std::mem::size_of::<T>()
    }

    fn address(&self) -> usize {
        self.weak.as_ptr() as usize
    }
}

pub(crate) trait AsSlice {
    type Elem: ByteRepr;
    fn as_slice(&self) -> &[Self::Elem];
    fn as_slice_mut(&mut self) -> &mut [Self::Elem];
}

impl<S: ByteRepr> AsSlice for Vec<S> {
    type Elem = S;
    fn as_slice(&self) -> &[S] {
        self
    }
    fn as_slice_mut(&mut self) -> &mut [S] {
        self
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
}

pub(crate) struct SliceOriginalAlloc<T: AsSlice> {
    pub weak: Weak<RefCell<T>>,
}

impl<T: AsSlice + 'static> OriginalAlloc for SliceOriginalAlloc<T> {
    fn read_bytes(&self, byte_offset: usize, buf: &mut [u8]) {
        let rc = self.weak.upgrade().expect("ub: dangling pointer");
        let val = rc.borrow();
        slice_read_bytes(val.as_slice(), byte_offset, buf);
    }

    fn write_bytes(&self, byte_offset: usize, data: &[u8]) {
        let rc = self.weak.upgrade().expect("ub: dangling pointer");
        let mut val = rc.borrow_mut();
        slice_write_bytes(val.as_slice_mut(), byte_offset, data);
    }

    fn total_byte_len(&self) -> usize {
        let rc = self.weak.upgrade().expect("ub: dangling pointer");
        let val = rc.borrow();
        std::mem::size_of_val(val.as_slice())
    }

    fn address(&self) -> usize {
        self.weak.as_ptr() as usize
    }
}
