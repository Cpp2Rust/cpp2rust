// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use crate::{PostfixDec, PostfixInc, PrefixDec, PrefixInc};
use std::any::{Any, TypeId};

use std::{
    cell::{Ref, RefCell},
    fmt,
    ops::Sub,
    rc::{Rc, Weak},
};

use crate::reinterpret::{ByteRepr, OriginalAlloc, SingleOriginalAlloc, SliceOriginalAlloc};

pub type Value<T> = Rc<RefCell<T>>;

pub(crate) struct ReinterpretedView {
    // Pointer to the source of reinterpret
    pub(crate) alloc: Rc<dyn OriginalAlloc>,
    // C++ size of the reinterpreted view
    elem_byte_size: usize,
}

#[derive(Default)]
pub(crate) enum PtrKind<T> {
    #[default]
    Null,
    StackSingle(Weak<RefCell<T>>),
    StackArray(Weak<RefCell<Box<[T]>>>),
    HeapSingle(Weak<RefCell<T>>),
    HeapArray(Weak<RefCell<Box<[T]>>>),
    Vec(Weak<RefCell<Vec<T>>>),
    Reinterpreted(Rc<ReinterpretedView>),
}

pub enum StrongPtr<T> {
    StackSingle(Rc<RefCell<T>>),
    Vec {
        rc: Rc<RefCell<Vec<T>>>,
        offset: usize,
    },
    StackArray {
        rc: Rc<RefCell<Box<[T]>>>,
        offset: usize,
    },
    Reinterpreted {
        alloc: Rc<dyn OriginalAlloc>,
        byte_offset: usize,
        // Local buffer for deref(). None until first access.
        // Read-through: refreshed from alloc on every deref() call.
        cell: RefCell<Option<T>>,
    },
}

impl<T: ByteRepr> StrongPtr<T> {
    pub fn deref(&self) -> Ref<'_, T> {
        match self {
            StrongPtr::StackSingle(rc) => rc.borrow(),
            StrongPtr::Vec { rc, offset } => Ref::map(rc.borrow(), |v| &v[*offset]),
            StrongPtr::StackArray { rc, offset } => Ref::map(rc.borrow(), |a| &a[*offset]),
            StrongPtr::Reinterpreted {
                alloc,
                byte_offset,
                cell,
            } => {
                // Read-through: always re-read from the original allocation.
                let mut buf = vec![0u8; T::byte_size()];
                alloc.read_bytes(*byte_offset, &mut buf);
                *cell.borrow_mut() = Some(T::from_bytes(&buf));
                Ref::map(cell.borrow(), |opt| opt.as_ref().unwrap())
            }
        }
    }
}

impl<T> fmt::Debug for PtrKind<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PtrKind::Null => write!(f, "Null"),
            PtrKind::Vec(w) => write!(f, "Vec({:?})", w.as_ptr()),
            PtrKind::StackSingle(w) => write!(f, "StackSingle({:?})", w.as_ptr()),
            PtrKind::HeapSingle(w) => write!(f, "HeapSingle({:?})", w.as_ptr()),
            PtrKind::StackArray(w) => write!(f, "StackArray({:?})", w.as_ptr()),
            PtrKind::HeapArray(w) => write!(f, "HeapArray({:?})", w.as_ptr()),
            PtrKind::Reinterpreted(data) => {
                write!(f, "Reinterpreted(0x{:x})", data.alloc.address())
            }
        }
    }
}

impl<T> Clone for PtrKind<T> {
    fn clone(&self) -> Self {
        match self {
            PtrKind::Null => PtrKind::Null,
            PtrKind::Vec(weak) => PtrKind::Vec(weak.clone()),
            PtrKind::StackSingle(weak) => PtrKind::StackSingle(weak.clone()),
            PtrKind::HeapSingle(weak) => PtrKind::HeapSingle(weak.clone()),
            PtrKind::StackArray(weak) => PtrKind::StackArray(weak.clone()),
            PtrKind::HeapArray(weak) => PtrKind::HeapArray(weak.clone()),
            PtrKind::Reinterpreted(data) => PtrKind::Reinterpreted(Rc::clone(data)),
        }
    }
}

impl<T> PtrKind<T> {
    fn address(&self) -> usize {
        match self {
            PtrKind::Null => 0,
            PtrKind::StackSingle(w) | PtrKind::HeapSingle(w) => w.as_ptr() as usize,
            PtrKind::Vec(w) => w.as_ptr() as usize,
            PtrKind::StackArray(w) | PtrKind::HeapArray(w) => w.as_ptr() as usize,
            PtrKind::Reinterpreted(data) => data.alloc.address(),
        }
    }
}

impl<T> Eq for PtrKind<T> {}

impl<T> PartialEq for PtrKind<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (PtrKind::Null, PtrKind::Null) => true,
            _ => self.address() == other.address(),
        }
    }
}

impl<T> PartialOrd for PtrKind<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (PtrKind::Null, PtrKind::Null) => Some(std::cmp::Ordering::Equal),
            _ => self.address().partial_cmp(&other.address()),
        }
    }
}

pub struct Ptr<T> {
    pub(crate) offset: usize,
    pub(crate) kind: PtrKind<T>,
}

impl<T> Default for Ptr<T> {
    fn default() -> Self {
        Self {
            offset: 0,
            kind: Default::default(),
        }
    }
}

impl<T> Clone for Ptr<T> {
    fn clone(&self) -> Self {
        Self {
            offset: self.offset,
            kind: self.kind.clone(),
        }
    }
}

impl<T> PartialEq for Ptr<T> {
    fn eq(&self, other: &Self) -> bool {
        self.byte_offset() == other.byte_offset() && self.kind == other.kind
    }
}

impl<T> Eq for Ptr<T> {}

impl<T> PartialOrd for Ptr<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.kind.partial_cmp(&other.kind) {
            Some(std::cmp::Ordering::Equal) => self.byte_offset().partial_cmp(&other.byte_offset()),
            ord => ord,
        }
    }
}

impl<T> Ptr<T> {
    #[inline]
    pub fn null() -> Self {
        Self {
            offset: 0,
            kind: PtrKind::Null,
        }
    }

    #[inline]
    pub fn alloc(value: T) -> Self {
        let owner = Rc::new(RefCell::new(value));
        let weak = Rc::downgrade(&owner);
        let _ = Rc::into_raw(owner);
        Self {
            offset: 0,
            kind: PtrKind::HeapSingle(weak),
        }
    }

    #[inline]
    pub fn alloc_array(array: Box<[T]>) -> Self {
        let owner = Rc::new(RefCell::new(array));
        let weak = Rc::downgrade(&owner);
        let _ = Rc::into_raw(owner);
        Self {
            offset: 0,
            kind: PtrKind::HeapArray(weak),
        }
    }

    #[inline]
    pub fn delete(&self) {
        match &self.kind {
            PtrKind::HeapSingle(weak) => {
                assert_eq!(self.offset, 0, "ub: invalid delete");
                assert_eq!(Weak::strong_count(weak), 1, "ub: invalid delete");
                unsafe {
                    let strong = weak.upgrade().expect("ub: dangling pointer");
                    Rc::from_raw(Rc::as_ptr(&strong));
                }
                assert_eq!(Weak::strong_count(weak), 0, "ub: double free");
            }
            PtrKind::Reinterpreted(data) => data.alloc.delete(),
            PtrKind::Null => {}
            _ => panic!("ub: invalid delete"),
        }
    }

    #[inline]
    pub fn delete_array(&self) {
        match &self.kind {
            PtrKind::HeapArray(weak) => {
                assert_eq!(self.offset, 0, "ub: invalid delete");
                assert_eq!(Weak::strong_count(weak), 1, "ub: invalid delete");
                unsafe {
                    let strong = weak.upgrade().expect("ub: dangling pointer");
                    Rc::from_raw(Rc::as_ptr(&strong));
                }
                assert_eq!(Weak::strong_count(weak), 0, "ub: double free");
            }
            PtrKind::Reinterpreted(data) => data.alloc.delete(),
            PtrKind::Null => {}
            _ => panic!("ub: invalid delete"),
        }
    }

    #[inline]
    pub fn is_null(&self) -> bool {
        matches!(self.kind, PtrKind::Null)
    }

    // Normalize offset to bytes for cross-variant comparison.
    #[inline]
    fn byte_offset(&self) -> usize {
        match &self.kind {
            PtrKind::Reinterpreted(_) => self.offset,
            _ => self.offset.wrapping_mul(std::mem::size_of::<T>()),
        }
    }

    // For Reinterpreted, Ptr::offset is in bytes. For all other variants,
    // Ptr::offset is in elements (step = 1). This helper converts between
    // user-facing element counts and the internal offset units.
    #[inline]
    fn elem_step(&self) -> usize {
        match &self.kind {
            PtrKind::Reinterpreted(data) => data.elem_byte_size,
            _ => 1,
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        match &self.kind {
            PtrKind::Null => 0,
            PtrKind::StackSingle(_) | PtrKind::HeapSingle(_) => 1,
            PtrKind::Vec(weak) => weak.upgrade().expect("ub: dangling pointer").borrow().len(),
            PtrKind::StackArray(weak) | PtrKind::HeapArray(weak) => {
                weak.upgrade().expect("ub: dangling pointer").borrow().len()
            }
            PtrKind::Reinterpreted(data) => data.alloc.total_byte_len() / data.elem_byte_size,
        }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        match &self.kind {
            PtrKind::Null => true,
            PtrKind::StackSingle(_) | PtrKind::HeapSingle(_) => false,
            PtrKind::Vec(weak) => weak
                .upgrade()
                .expect("ub: dangling pointer")
                .borrow()
                .is_empty(),
            PtrKind::StackArray(weak) | PtrKind::HeapArray(weak) => weak
                .upgrade()
                .expect("ub: dangling pointer")
                .borrow()
                .is_empty(),
            PtrKind::Reinterpreted(data) => self.offset >= data.alloc.total_byte_len(),
        }
    }

    #[inline]
    pub fn offset(&self, offset: impl TryInto<isize>) -> Self {
        let offset = offset
            .try_into()
            .ok()
            .expect("the offset must fit in a isize");
        let step = self.elem_step();
        Self {
            kind: self.kind.clone(),
            offset: self
                .offset
                .wrapping_add(offset.wrapping_mul(step as isize) as usize),
        }
    }

    #[inline]
    pub fn get_offset(&self) -> usize {
        self.offset / self.elem_step()
    }

    #[inline]
    pub fn to_last(&self) -> Self {
        Self {
            kind: self.kind.clone(),
            offset: self.len().wrapping_sub(1).wrapping_mul(self.elem_step()),
        }
    }

    #[inline]
    pub fn to_end(&self) -> Self {
        Self {
            kind: self.kind.clone(),
            offset: self.len().wrapping_mul(self.elem_step()),
        }
    }

    pub fn upgrade(&self) -> StrongPtr<T> {
        match &self.kind {
            PtrKind::Null => panic!("ub: null pointer"),
            PtrKind::StackSingle(weak) | PtrKind::HeapSingle(weak) => {
                assert_eq!(self.offset, 0, "ub: invalid offset");
                StrongPtr::StackSingle(weak.upgrade().expect("ub: dangling pointer"))
            }
            PtrKind::Vec(weak) => StrongPtr::Vec {
                rc: weak.upgrade().expect("ub: dangling pointer"),
                offset: self.offset,
            },
            PtrKind::StackArray(weak) | PtrKind::HeapArray(weak) => StrongPtr::StackArray {
                rc: weak.upgrade().expect("ub: dangling pointer"),
                offset: self.offset,
            },
            PtrKind::Reinterpreted(data) => StrongPtr::Reinterpreted {
                alloc: Rc::clone(&data.alloc),
                byte_offset: self.offset,
                cell: RefCell::new(None),
            },
        }
    }

    pub fn write(&self, value: T)
    where
        T: ByteRepr,
    {
        self.with_mut(|v| *v = value);
    }

    pub fn to_strong(&self) -> Value<T> {
        match &self.kind {
            PtrKind::StackSingle(weak) | PtrKind::HeapSingle(weak) => {
                weak.upgrade().expect("ub: dangling pointer")
            }
            _ => panic!("Only StackSingle and HeapSingle implement to_strong"),
        }
    }

    pub fn reinterpret_cast<U: ByteRepr>(&self) -> Ptr<U>
    where
        T: ByteRepr,
    {
        if TypeId::of::<T>() == TypeId::of::<U>() {
            let self_any: &dyn Any = self;
            return self_any.downcast_ref::<Ptr<U>>().unwrap().clone();
        }

        if U::byte_size() == 0 {
            panic!("cannot reinterpret_cast to zero-sized type");
        }

        let src_byte_off = self.offset.wrapping_mul(T::byte_size());
        let (alloc, abs_byte_off): (Rc<dyn OriginalAlloc>, usize) = match &self.kind {
            PtrKind::Null => return Ptr::null(),
            PtrKind::StackSingle(weak) | PtrKind::HeapSingle(weak) => (
                Rc::new(SingleOriginalAlloc { weak: weak.clone() }),
                src_byte_off,
            ),
            PtrKind::Vec(weak) => (
                Rc::new(SliceOriginalAlloc { weak: weak.clone() }),
                src_byte_off,
            ),
            PtrKind::StackArray(weak) | PtrKind::HeapArray(weak) => (
                Rc::new(SliceOriginalAlloc { weak: weak.clone() }),
                src_byte_off,
            ),
            PtrKind::Reinterpreted(data) => (Rc::clone(&data.alloc), self.offset),
        };

        Ptr {
            offset: abs_byte_off,
            kind: PtrKind::Reinterpreted(Rc::new(ReinterpretedView {
                alloc,
                elem_byte_size: U::byte_size(),
            })),
        }
    }
}

impl<T> Ptr<T> {
    pub fn with_mut<R>(&self, f: impl FnOnce(&mut T) -> R) -> R
    where
        T: ByteRepr,
    {
        match &self.kind {
            PtrKind::Null => panic!("ub: null pointer"),
            PtrKind::StackSingle(weak) | PtrKind::HeapSingle(weak) => {
                assert_eq!(self.offset, 0, "ub: invalid offset");
                let rc = weak.upgrade().expect("ub: dangling pointer");
                let mut borrow = rc.borrow_mut();
                f(&mut *borrow)
            }
            PtrKind::Vec(weak) => {
                let rc = weak.upgrade().expect("ub: dangling pointer");
                let mut borrow = rc.borrow_mut();
                f(&mut borrow[self.offset])
            }
            PtrKind::StackArray(weak) | PtrKind::HeapArray(weak) => {
                let rc = weak.upgrade().expect("ub: dangling pointer");
                let mut borrow = rc.borrow_mut();
                f(&mut borrow[self.offset])
            }
            PtrKind::Reinterpreted(data) => {
                let mut buf = vec![0u8; T::byte_size()];
                data.alloc.read_bytes(self.offset, &mut buf);
                let mut val = T::from_bytes(&buf);
                let ret = f(&mut val);
                val.to_bytes(&mut buf);
                data.alloc.write_bytes(self.offset, &buf);
                ret
            }
        }
    }

    pub fn with<R>(&self, f: impl FnOnce(&T) -> R) -> R
    where
        T: ByteRepr,
    {
        match &self.kind {
            PtrKind::Null => panic!("ub: null pointer"),
            PtrKind::StackSingle(weak) | PtrKind::HeapSingle(weak) => {
                assert_eq!(self.offset, 0, "ub: invalid offset");
                let rc = weak.upgrade().expect("ub: dangling pointer");
                let borrow = rc.borrow();
                f(&*borrow)
            }
            PtrKind::Vec(weak) => {
                let rc = weak.upgrade().expect("ub: dangling pointer");
                let borrow = rc.borrow();
                f(&borrow[self.offset])
            }
            PtrKind::StackArray(weak) | PtrKind::HeapArray(weak) => {
                let rc = weak.upgrade().expect("ub: dangling pointer");
                let borrow = rc.borrow();
                f(&borrow[self.offset])
            }
            PtrKind::Reinterpreted(data) => {
                let mut buf = vec![0u8; T::byte_size()];
                data.alloc.read_bytes(self.offset, &mut buf);
                let val = T::from_bytes(&buf);
                f(&val)
            }
        }
    }
}

impl Ptr<u8> {
    pub fn with_slice_mut<R>(&self, len: usize, f: impl FnOnce(&mut [u8]) -> R) -> R {
        let off = self.offset;
        match &self.kind {
            PtrKind::Null => panic!("ub: null pointer"),
            PtrKind::StackSingle(weak) | PtrKind::HeapSingle(weak) => {
                assert!(off == 0 && len <= 1, "ub: with_slice_mut out of bounds");
                let rc = weak.upgrade().expect("ub: dangling pointer");
                let mut b = rc.borrow_mut();
                f(&mut std::slice::from_mut(&mut *b)[..len])
            }
            PtrKind::StackArray(weak) | PtrKind::HeapArray(weak) => {
                let rc = weak.upgrade().expect("ub: dangling pointer");
                let mut b = rc.borrow_mut();
                f(&mut b[off..off + len])
            }
            PtrKind::Vec(weak) => {
                let rc = weak.upgrade().expect("ub: dangling pointer");
                let mut b = rc.borrow_mut();
                f(&mut b[off..off + len])
            }
            PtrKind::Reinterpreted(data) => {
                let mut buf = vec![0u8; len];
                data.alloc.read_bytes(off, &mut buf);
                let r = f(&mut buf);
                data.alloc.write_bytes(off, &buf);
                r
            }
        }
    }

    pub fn with_slice<R>(&self, len: usize, f: impl FnOnce(&[u8]) -> R) -> R {
        let off = self.offset;
        match &self.kind {
            PtrKind::Null => panic!("ub: null pointer"),
            PtrKind::StackSingle(weak) | PtrKind::HeapSingle(weak) => {
                assert!(off == 0 && len <= 1, "ub: with_slice out of bounds");
                let rc = weak.upgrade().expect("ub: dangling pointer");
                let b = rc.borrow();
                f(&std::slice::from_ref(&*b)[..len])
            }
            PtrKind::StackArray(weak) | PtrKind::HeapArray(weak) => {
                let rc = weak.upgrade().expect("ub: dangling pointer");
                let b = rc.borrow();
                f(&b[off..off + len])
            }
            PtrKind::Vec(weak) => {
                let rc = weak.upgrade().expect("ub: dangling pointer");
                let b = rc.borrow();
                f(&b[off..off + len])
            }
            PtrKind::Reinterpreted(data) => {
                let mut buf = vec![0u8; len];
                data.alloc.read_bytes(off, &mut buf);
                f(&buf)
            }
        }
    }
}

impl<T: Clone + ByteRepr> Ptr<T> {
    pub fn read(&self) -> T {
        self.with(|v| v.clone())
    }
}

impl<T: std::io::Write + ByteRepr> Ptr<T> {
    pub fn write_fmt(&self, args: std::fmt::Arguments<'_>) -> std::io::Result<()> {
        self.with_mut(|inner| inner.write_fmt(args))
    }

    pub fn write_all(&self, buf: &[u8]) -> std::io::Result<()> {
        self.with_mut(|inner| inner.write_all(buf))
    }
}

impl<T: std::cmp::Ord> Ptr<T> {
    pub fn sort(&self, last: usize) {
        match self.kind {
            PtrKind::Null => panic!("ub: dereference of null pointer"),
            PtrKind::StackSingle(_) | PtrKind::HeapSingle(_) => {
                panic!("only vecs and arrays can be sorted")
            }
            PtrKind::Vec(ref weak) => {
                let strong = weak.upgrade().expect("ub: dangling pointer");
                (*strong.borrow_mut())[self.get_offset()..last].sort();
            }
            PtrKind::StackArray(ref weak) | PtrKind::HeapArray(ref weak) => {
                let strong = weak.upgrade().expect("ub: dangling pointer");
                (*strong.borrow_mut())[self.get_offset()..last].sort();
            }
            PtrKind::Reinterpreted(_) => {
                panic!("sorting not supported for reinterpreted pointers")
            }
        }
    }
}

impl<T: Clone> Ptr<T> {
    pub fn sort_with_cmp<F>(&self, last: usize, mut cmp: F)
    where
        F: FnMut(Ptr<T>, Ptr<T>) -> bool,
    {
        fn sort<T: Clone, F: FnMut(Ptr<T>, Ptr<T>) -> bool>(
            slice: &mut [T],
            offset: usize,
            last: usize,
            cmp: &mut F,
        ) {
            slice[offset..last].sort_by(|a, b| {
                let val_a = Rc::new(RefCell::new(a.clone()));
                let val_b = Rc::new(RefCell::new(b.clone()));
                if cmp(val_a.as_pointer(), val_b.as_pointer()) {
                    std::cmp::Ordering::Less
                } else if cmp(val_b.as_pointer(), val_a.as_pointer()) {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Equal
                }
            });
        }
        match self.kind {
            PtrKind::Null => panic!("ub: dereference of null pointer"),
            PtrKind::StackSingle(_) | PtrKind::HeapSingle(_) => {
                panic!("only vecs and arrays can be sorted")
            }
            PtrKind::Vec(ref weak) => {
                let strong = weak.upgrade().expect("ub: dangling pointer");
                let mut borrow = strong.borrow_mut();
                sort(&mut borrow, self.get_offset(), last, &mut cmp);
            }
            PtrKind::StackArray(ref weak) | PtrKind::HeapArray(ref weak) => {
                let strong = weak.upgrade().expect("ub: dangling pointer");
                let mut borrow = strong.borrow_mut();
                sort(&mut borrow, self.get_offset(), last, &mut cmp);
            }
            PtrKind::Reinterpreted(_) => {
                panic!("sorting not supported for reinterpreted pointers")
            }
        }
    }
}

impl<T> IntoIterator for &Ptr<T>
where
    T: Clone,
{
    type Item = Ptr<T>;
    type IntoIter = Ptr<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.clone()
    }
}

impl<T> Iterator for Ptr<T> {
    type Item = Ptr<T>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.get_offset() < self.len() {
            let value = self.clone();
            self.offset += self.elem_step();
            Some(value)
        } else {
            None
        }
    }
}

// Ptr iterator that yields values instead of pointers.
// It's more efficient and it's useful to implement idiomatic iterator patterns
pub struct PtrValueIter<T> {
    ptr: Ptr<T>,
    n: usize,
}

impl<T> PtrValueIter<T> {
    pub fn new(ptr: &Ptr<T>, n: usize) -> Self {
        Self {
            ptr: ptr.clone(),
            n,
        }
    }
}

impl<T: Clone + ByteRepr> Iterator for PtrValueIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.n > 0 {
            let value = self.ptr.read();
            self.ptr += 1;
            self.n -= 1;
            Some(value)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.n, Some(self.n))
    }
}

impl<T: Clone + ByteRepr> ExactSizeIterator for PtrValueIter<T> {}

impl<T> Sub for Ptr<T> {
    type Output = isize;
    fn sub(self, other: Self) -> Self::Output {
        assert!(self.kind == other.kind, "ub: invalid subtraction");
        (self.get_offset() as isize).wrapping_sub(other.get_offset() as isize)
    }
}

macro_rules! impl_ptr_add_sub_assign {
    ($($rhs:ty),+) => { $(
        impl<T> std::ops::AddAssign<$rhs> for Ptr<T> {
            #[inline]
            fn add_assign(&mut self, other: $rhs) {
                let step = self.elem_step();
                self.offset = self.offset.wrapping_add(
                    ((other as isize).wrapping_mul(step as isize)) as usize,
                );
            }
        }
        impl<T> std::ops::SubAssign<$rhs> for Ptr<T> {
            #[inline]
            fn sub_assign(&mut self, other: $rhs) {
                let step = self.elem_step();
                self.offset = self.offset.wrapping_sub(
                    ((other as isize).wrapping_mul(step as isize)) as usize,
                );
            }
        }
    )+ }
}
impl_ptr_add_sub_assign!(i32, u32, i64, u64, isize, usize);

macro_rules! impl_ptr_add_sub {
    ($($rhs:ty),+) => { $(
        impl<T> std::ops::Add<$rhs> for &Ptr<T> {
            type Output = Ptr<T>;
            #[inline]
            fn add(self, other: $rhs) -> Ptr<T> { let mut r = self.clone(); r += other; r }
        }
        impl<T> std::ops::Sub<$rhs> for &Ptr<T> {
            type Output = Ptr<T>;
            #[inline]
            fn sub(self, other: $rhs) -> Ptr<T> { let mut r = self.clone(); r += -(other as isize); r }
        }
        impl<T> std::ops::Add<$rhs> for Ptr<T> {
            type Output = Self;
            #[inline]
            fn add(mut self, other: $rhs) -> Self { self += other; self }
        }
        impl<T> std::ops::Sub<$rhs> for Ptr<T> {
            type Output = Self;
            #[inline]
            fn sub(mut self, other: $rhs) -> Self { self += -(other as isize); self }
        }
    )+ }
}
impl_ptr_add_sub!(i32, u32, u64, isize, usize);

impl<T> PostfixInc for Ptr<T> {
    #[inline]
    fn postfix_inc(&mut self) -> Self {
        let ret = self.clone();
        self.offset = self.offset.wrapping_add(self.elem_step());
        ret
    }
}

impl<T> PostfixDec for Ptr<T> {
    #[inline]
    fn postfix_dec(&mut self) -> Self {
        let ret = self.clone();
        self.offset = self.offset.wrapping_sub(self.elem_step());
        ret
    }
}

impl<T> PrefixInc for Ptr<T> {
    #[inline]
    fn prefix_inc(&mut self) -> Self {
        self.offset = self.offset.wrapping_add(self.elem_step());
        self.clone()
    }
}

impl<T> PrefixDec for Ptr<T> {
    #[inline]
    fn prefix_dec(&mut self) -> Self {
        self.offset = self.offset.wrapping_sub(self.elem_step());
        self.clone()
    }
}

pub trait AsPointer<T> {
    fn as_pointer(&self) -> Ptr<T>;
}

impl<T> AsPointer<T> for Rc<RefCell<T>> {
    #[inline]
    fn as_pointer(&self) -> Ptr<T> {
        Ptr {
            offset: 0,
            kind: PtrKind::StackSingle(Rc::downgrade(self)),
        }
    }
}

impl<T> AsPointer<T> for Option<Rc<RefCell<T>>> {
    #[inline]
    fn as_pointer(&self) -> Ptr<T> {
        match self {
            None => Ptr::null(),
            Some(p) => p.as_pointer(),
        }
    }
}

impl<T> AsPointer<T> for Rc<RefCell<Box<[T]>>> {
    #[inline]
    fn as_pointer(&self) -> Ptr<T> {
        Ptr {
            offset: 0,
            kind: PtrKind::StackArray(Rc::downgrade(self)),
        }
    }
}

impl<T> AsPointer<T> for Option<Rc<RefCell<Box<[T]>>>> {
    #[inline]
    fn as_pointer(&self) -> Ptr<T> {
        match self {
            None => Ptr::null(),
            Some(p) => p.as_pointer(),
        }
    }
}

impl<T> AsPointer<T> for Rc<RefCell<Vec<T>>> {
    #[inline]
    fn as_pointer(&self) -> Ptr<T> {
        Ptr {
            offset: 0,
            kind: PtrKind::Vec(Rc::downgrade(self)),
        }
    }
}

pub trait ToOwnedOption<T, O> {
    fn to_owned_opt(&self) -> Option<Rc<RefCell<O>>>;
}

impl<T> ToOwnedOption<T, T> for Ptr<T> {
    #[inline]
    fn to_owned_opt(&self) -> Option<Rc<RefCell<T>>> {
        match self.kind {
            PtrKind::Null => None,
            PtrKind::HeapSingle(ref weak) => {
                assert_eq!(self.offset, 0, "ub: invalid offset");
                assert_eq!(Weak::strong_count(weak), 1, "ub: invalid pointer");
                let strong = weak.upgrade().expect("ub: dangling pointer");
                // Delete the leaked reference
                unsafe {
                    Rc::from_raw(Rc::as_ptr(&strong));
                }
                assert_eq!(Rc::strong_count(&strong), 1, "wrong refs");
                Some(strong)
            }
            PtrKind::StackSingle(_) | PtrKind::StackArray(_) => {
                panic!("Can't own a stack variable")
            }
            PtrKind::Vec(_) => panic!("Can't own a vector"),
            PtrKind::HeapArray(_) => panic!("Can't own an array variable as single"),
            PtrKind::Reinterpreted(_) => panic!("Can't own a reinterpreted pointer"),
        }
    }
}

impl<T> ToOwnedOption<T, Box<[T]>> for Ptr<T> {
    #[inline]
    fn to_owned_opt(&self) -> Option<Rc<RefCell<Box<[T]>>>> {
        match self.kind {
            PtrKind::Null => None,
            PtrKind::HeapArray(ref weak) => {
                assert_eq!(self.offset, 0, "ub: invalid offset");
                assert_eq!(Weak::strong_count(weak), 1, "ub: invalid pointer");
                let strong = weak.upgrade().expect("ub: dangling pointer");
                // Delete the leaked reference
                unsafe {
                    Rc::from_raw(Rc::as_ptr(&strong));
                }
                assert_eq!(Rc::strong_count(&strong), 1, "wrong refs");
                Some(strong)
            }
            PtrKind::StackSingle(_) | PtrKind::StackArray(_) => {
                panic!("Can't own a stack variable")
            }
            PtrKind::Vec(_) => panic!("Can't own a vector"),
            PtrKind::HeapSingle(_) => panic!("Can't own a single variable as an array"),
            PtrKind::Reinterpreted(_) => panic!("Can't own a reinterpreted pointer"),
        }
    }
}

impl<T> fmt::Debug for Ptr<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let addr = match &self.kind {
            PtrKind::Null => 0,
            PtrKind::StackSingle(w) | PtrKind::HeapSingle(w) => {
                (Weak::as_ptr(w) as usize).wrapping_add(self.byte_offset())
            }
            PtrKind::StackArray(w) | PtrKind::HeapArray(w) => {
                (Weak::as_ptr(w) as usize).wrapping_add(self.byte_offset())
            }
            PtrKind::Vec(w) => (Weak::as_ptr(w) as usize).wrapping_add(self.byte_offset()),
            PtrKind::Reinterpreted(data) => data.alloc.address().wrapping_add(self.byte_offset()),
        };
        write!(f, "0x{:x}", addr)
    }
}

impl<T: 'static> ByteRepr for Ptr<T> {}

impl<T: 'static> Ptr<T> {
    pub fn to_int(&self) -> usize {
        let mut buf = vec![0u8; Self::byte_size()];
        self.to_bytes(&mut buf);
        usize::from_bytes(&buf[..std::mem::size_of::<usize>()])
    }

    pub fn from_int(value: usize) -> Self {
        let mut buf = vec![0u8; Self::byte_size()];
        value.to_bytes(&mut buf[..std::mem::size_of::<usize>()]);
        Self::from_bytes(&buf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reinterpreted_cast() {
        let p: Ptr<u64> = Ptr::alloc(0x0807060504030201u64);

        // Reinterpreted Ptr views using with/get.
        let bytes: Ptr<u8> = p.reinterpret_cast::<u8>();

        assert_eq!(bytes.read(), 0x01);
        assert_eq!(bytes.offset(3).read(), 0x04);
        assert_eq!(bytes.offset(7).read(), 0x08);

        // Write through original, Ptr reads must see the new data.
        p.write(0xAABBCCDDEEFF1122);
        assert_eq!(bytes.read(), 0x22);
        assert_eq!(bytes.offset(3).read(), 0xEE);
        assert_eq!(bytes.offset(7).read(), 0xAA);

        // Create a second reinterpreted view (u16).
        let words: Ptr<u16> = p.reinterpret_cast::<u16>();

        assert_eq!(words.read(), 0x1122);
        assert_eq!(words.offset(1).read(), 0xEEFF);
        assert_eq!(words.offset(3).read(), 0xAABB);

        // Write through original again. Both views must update.
        p.write(0x0000000000000000);
        assert_eq!(bytes.read(), 0x00);
        assert_eq!(bytes.offset(7).read(), 0x00);
        assert_eq!(words.read(), 0x0000);
        assert_eq!(words.offset(3).read(), 0x0000);

        // Write through byte Ptr, read through word Ptr.
        bytes.write(0xCE);
        bytes.offset(1).write(0xFA);
        assert_eq!(words.read(), 0xFACE);
        assert_eq!(bytes.read(), 0xCE);
        assert_eq!(bytes.offset(3).read(), 0x00);

        // Write through word Ptr, read through byte Ptr.
        words.offset(1).write(0xDEAD);
        assert_eq!(bytes.offset(3).read(), 0xDE);
        assert_eq!(words.offset(1).read(), 0xDEAD);

        // Final state: 0x00000000DEADFACE
        assert_eq!(p.read(), 0x00000000DEADFACE);

        p.delete();
    }
}
