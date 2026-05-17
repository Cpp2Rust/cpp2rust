// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use crate::{PostfixDec, PostfixInc, PrefixDec, PrefixInc};
use std::any::{Any, TypeId};
use std::collections::HashMap;

use std::{
    cell::{Ref, RefCell, RefMut},
    fmt,
    ops::Sub,
    rc::{Rc, Weak},
};

use crate::reinterpret::{ByteRepr, OriginalAlloc, SingleOriginalAlloc, SliceOriginalAlloc};

pub type Value<T> = Rc<RefCell<T>>;

#[derive(Default)]
enum PtrKind<T> {
    #[default]
    Null,
    StackSingle(Weak<RefCell<T>>),
    StackArray(Weak<RefCell<Box<[T]>>>),
    HeapSingle(Weak<RefCell<T>>),
    HeapArray(Weak<RefCell<Box<[T]>>>),
    Vec(Weak<RefCell<Vec<T>>>),
    Reinterpreted(Rc<dyn OriginalAlloc>),
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
                let mut buf = vec![0u8; std::mem::size_of::<T>()];
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
                write!(f, "Reinterpreted(0x{:x})", data.address())
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
            PtrKind::Reinterpreted(data) => data.address(),
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
    offset: usize,
    kind: PtrKind<T>,
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
        assert_eq!(self.offset, 0, "ub: invalid delete");
        let weak = match self.kind {
            PtrKind::HeapSingle(ref weak) => weak,
            _ => panic!("ub: invalid delete"),
        };
        assert_eq!(Weak::strong_count(weak), 1, "ub: invalid delete");
        unsafe {
            let strong = weak.upgrade().expect("ub: dangling pointer");
            Rc::from_raw(Rc::as_ptr(&strong));
        }
        assert_eq!(Weak::strong_count(weak), 0, "ub: strong count is not zero");
    }

    #[inline]
    pub fn delete_array(&self) {
        assert_eq!(self.offset, 0, "ub: invalid delete");
        let weak = match self.kind {
            PtrKind::HeapArray(ref weak) => weak,
            _ => panic!("ub: invalid delete"),
        };
        assert_eq!(Weak::strong_count(weak), 1, "ub: invalid delete");
        unsafe {
            let strong = weak.upgrade().expect("ub: dangling pointer");
            Rc::from_raw(Rc::as_ptr(&strong));
        }
        assert_eq!(Weak::strong_count(weak), 0, "ub: strong count is not zero");
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
            _ => self.offset * std::mem::size_of::<T>(),
        }
    }

    // For Reinterpreted, Ptr::offset is in bytes. For all other variants,
    // Ptr::offset is in elements (step = 1). This helper converts between
    // user-facing element counts and the internal offset units.
    #[inline]
    fn elem_step(&self) -> usize {
        match &self.kind {
            PtrKind::Reinterpreted(_) => std::mem::size_of::<T>(),
            _ => 1,
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        match self.kind {
            PtrKind::Null => 0,
            PtrKind::StackSingle(_) | PtrKind::HeapSingle(_) => 1,
            PtrKind::Vec(ref weak) => weak.upgrade().expect("ub: dangling pointer").borrow().len(),
            PtrKind::StackArray(ref weak) | PtrKind::HeapArray(ref weak) => {
                weak.upgrade().expect("ub: dangling pointer").borrow().len()
            }
            PtrKind::Reinterpreted(ref data) => {
                let step = std::mem::size_of::<T>();
                (data.total_byte_len() - self.offset % step) / step
            }
        }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        match self.kind {
            PtrKind::Null => true,
            PtrKind::StackSingle(_) | PtrKind::HeapSingle(_) => false,
            PtrKind::Vec(ref weak) => weak
                .upgrade()
                .expect("ub: dangling pointer")
                .borrow()
                .is_empty(),
            PtrKind::StackArray(ref weak) | PtrKind::HeapArray(ref weak) => weak
                .upgrade()
                .expect("ub: dangling pointer")
                .borrow()
                .is_empty(),
            PtrKind::Reinterpreted(ref data) => self.offset >= data.total_byte_len(),
        }
    }

    #[inline]
    pub fn offset(&self, offset: isize) -> Self {
        let step = self.elem_step();
        Self {
            kind: self.kind.clone(),
            offset: self
                .offset
                .wrapping_add((offset.wrapping_mul(step as isize)) as usize),
        }
    }

    #[inline]
    pub fn get_offset(&self) -> usize {
        self.offset / self.elem_step()
    }

    #[inline]
    pub fn to_last(&self) -> Self {
        let step = self.elem_step();
        let base = self.offset % step;
        Self {
            kind: self.kind.clone(),
            offset: base + (self.len() - 1) * step,
        }
    }

    #[inline]
    pub fn to_end(&self) -> Self {
        let step = self.elem_step();
        let base = self.offset % step;
        Self {
            kind: self.kind.clone(),
            offset: base + self.len() * step,
        }
    }

    pub fn to_string_iterator(&self) -> StringIterator<T> {
        StringIterator { ptr: self.clone() }
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
                alloc: Rc::clone(data),
                byte_offset: self.offset,
                cell: RefCell::new(None),
            },
        }
    }

    pub fn write(&self, value: T)
    where
        T: ByteRepr,
    {
        match &self.kind {
            PtrKind::Null => panic!("ub: null pointer"),
            PtrKind::StackSingle(weak) | PtrKind::HeapSingle(weak) => {
                let rc = weak.upgrade().expect("ub: dangling pointer");
                *rc.borrow_mut() = value;
            }
            PtrKind::Vec(weak) => {
                let rc = weak.upgrade().expect("ub: dangling pointer");
                rc.borrow_mut()[self.offset] = value;
            }
            PtrKind::StackArray(weak) | PtrKind::HeapArray(weak) => {
                let rc = weak.upgrade().expect("ub: dangling pointer");
                rc.borrow_mut()[self.offset] = value;
            }
            PtrKind::Reinterpreted(data) => {
                let mut buf = vec![0u8; std::mem::size_of::<T>()];
                value.to_bytes(&mut buf);
                data.write_bytes(self.offset, &buf);
            }
        }
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

        if std::mem::size_of::<U>() == 0 {
            panic!("cannot reinterpret_cast to zero-sized type");
        }

        let (alloc, abs_byte_off): (Rc<dyn OriginalAlloc>, usize) = match &self.kind {
            PtrKind::Null => return Ptr::null(),
            PtrKind::StackSingle(weak) | PtrKind::HeapSingle(weak) => (
                Rc::new(SingleOriginalAlloc { weak: weak.clone() }),
                self.byte_offset(),
            ),
            PtrKind::Vec(weak) => (
                Rc::new(SliceOriginalAlloc { weak: weak.clone() }),
                self.byte_offset(),
            ),
            PtrKind::StackArray(weak) | PtrKind::HeapArray(weak) => (
                Rc::new(SliceOriginalAlloc { weak: weak.clone() }),
                self.byte_offset(),
            ),
            PtrKind::Reinterpreted(data) => (Rc::clone(data), self.offset),
        };

        Ptr {
            offset: abs_byte_off,
            kind: PtrKind::Reinterpreted(alloc),
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
                let mut buf = vec![0u8; std::mem::size_of::<T>()];
                data.read_bytes(self.offset, &mut buf);
                let mut val = T::from_bytes(&buf);
                let ret = f(&mut val);
                val.to_bytes(&mut buf);
                data.write_bytes(self.offset, &buf);
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
                let mut buf = vec![0u8; std::mem::size_of::<T>()];
                data.read_bytes(self.offset, &mut buf);
                let val = T::from_bytes(&buf);
                let ret = f(&val);
                val.to_bytes(&mut buf);
                data.write_bytes(self.offset, &buf);
                ret
            }
        }
    }
}

impl<T: Clone + ByteRepr> Ptr<T> {
    pub fn read(&self) -> T {
        match self.kind {
            PtrKind::Null => panic!("ub: null pointer"),
            PtrKind::StackSingle(ref weak) | PtrKind::HeapSingle(ref weak) => {
                assert_eq!(self.offset, 0, "ub: invalid offset");
                weak.upgrade()
                    .expect("ub: dangling pointer")
                    .borrow()
                    .clone()
            }
            PtrKind::Vec(ref weak) => {
                weak.upgrade().expect("ub: dangling pointer").borrow()[self.offset].clone()
            }
            PtrKind::StackArray(ref weak) | PtrKind::HeapArray(ref weak) => {
                weak.upgrade().expect("ub: dangling pointer").borrow()[self.offset].clone()
            }
            PtrKind::Reinterpreted(ref data) => {
                let mut buf = vec![0u8; std::mem::size_of::<T>()];
                data.read_bytes(self.offset, &mut buf);
                T::from_bytes(&buf)
            }
        }
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
            PtrKind::Reinterpreted(..) => {
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
        match self.kind {
            PtrKind::Null => panic!("ub: dereference of null pointer"),
            PtrKind::StackSingle(_) | PtrKind::HeapSingle(_) => {
                panic!("only vecs and arrays can be sorted")
            }
            PtrKind::Vec(ref weak) => {
                let strong = weak.upgrade().expect("ub: dangling pointer");
                (*strong.borrow_mut())[self.get_offset()..last].sort_by(|a, b| {
                    if cmp(
                        Rc::new(RefCell::new(a.clone())).as_pointer(),
                        Rc::new(RefCell::new(b.clone())).as_pointer(),
                    ) {
                        std::cmp::Ordering::Less
                    } else if cmp(
                        Rc::new(RefCell::new(b.clone())).as_pointer(),
                        Rc::new(RefCell::new(a.clone())).as_pointer(),
                    ) {
                        std::cmp::Ordering::Greater
                    } else {
                        std::cmp::Ordering::Equal
                    }
                });
            }
            PtrKind::StackArray(ref weak) | PtrKind::HeapArray(ref weak) => {
                let strong = weak.upgrade().expect("ub: dangling pointer");
                (*strong.borrow_mut())[self.get_offset()..last].sort_by(|a, b| {
                    if cmp(
                        Rc::new(RefCell::new(a.clone())).as_pointer(),
                        Rc::new(RefCell::new(b.clone())).as_pointer(),
                    ) {
                        std::cmp::Ordering::Less
                    } else if cmp(
                        Rc::new(RefCell::new(b.clone())).as_pointer(),
                        Rc::new(RefCell::new(a.clone())).as_pointer(),
                    ) {
                        std::cmp::Ordering::Greater
                    } else {
                        std::cmp::Ordering::Equal
                    }
                });
            }
            PtrKind::Reinterpreted(..) => {
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
        let step = self.elem_step();
        if self.offset / step < self.len() {
            let value = self.clone();
            self.offset += step;
            Some(value)
        } else {
            None
        }
    }
}

pub struct StringIterator<T> {
    ptr: Ptr<T>,
}

impl<T> Iterator for StringIterator<T> {
    type Item = Ptr<T>;
    fn next(&mut self) -> Option<Self::Item> {
        // skip the null terminator
        if self.ptr.get_offset() + 1 < self.ptr.len() {
            let value = self.ptr.clone();
            self.ptr += 1;
            Some(value)
        } else {
            None
        }
    }
}

pub struct CStringIterator {
    ptr: Ptr<u8>,
}

impl Iterator for CStringIterator {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        let ch = self.ptr.read();
        // read until the null terminator
        if ch != 0 {
            self.ptr += 1;
            Some(ch)
        } else {
            None
        }
    }
}

impl<T> Sub for Ptr<T> {
    type Output = usize;
    fn sub(self, other: Self) -> Self::Output {
        assert!(self.kind == other.kind, "ub: invalid subtraction");
        (self.offset / self.elem_step()).wrapping_sub(other.offset / other.elem_step())
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
impl_ptr_add_sub_assign!(i32, u32, u64, isize);

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
impl_ptr_add_sub!(i32, u32, u64, isize);

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
            PtrKind::Reinterpreted(..) => panic!("Can't own a reinterpreted pointer"),
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
            PtrKind::Reinterpreted(..) => panic!("Can't own a reinterpreted pointer"),
        }
    }
}

impl<T> fmt::Debug for Ptr<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let addr = match &self.kind {
            PtrKind::Null => 0,
            PtrKind::StackSingle(w) | PtrKind::HeapSingle(w) => (Weak::as_ptr(w) as usize)
                .wrapping_add(self.offset.wrapping_mul(std::mem::size_of::<T>())),
            PtrKind::StackArray(w) | PtrKind::HeapArray(w) => (Weak::as_ptr(w) as usize)
                .wrapping_add(self.offset.wrapping_mul(std::mem::size_of::<T>())),
            PtrKind::Vec(w) => (Weak::as_ptr(w) as usize)
                .wrapping_add(self.offset.wrapping_mul(std::mem::size_of::<T>())),
            PtrKind::Reinterpreted(data) => data.address().wrapping_add(self.offset),
        };
        write!(f, "0x{:x}", addr)
    }
}

impl fmt::Display for Ptr<u8> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            PtrKind::Null => write!(f, ""),
            _ => {
                for value in self {
                    let ch = value.read();
                    if ch == 0 {
                        break;
                    }
                    write!(f, "{}", char::from(ch))?;
                }
                Ok(())
            }
        }
    }
}

thread_local! {
    static STRING_LITERALS: RefCell<HashMap<&'static str, Rc<RefCell<Vec<u8>>>>> =
        RefCell::new(HashMap::new());
}

impl Ptr<u8> {
    #[allow(clippy::explicit_counter_loop)]
    pub fn memcpy(&self, src: &Self, len: usize) {
        let mut dst = self.clone();
        let mut i: usize = 0;
        for value in src {
            if i >= len {
                break;
            }
            dst.write(value.read());
            dst += 1;
            i += 1;
        }
        assert_eq!(i, len, "ub: memcpy");
    }

    #[allow(clippy::explicit_counter_loop)]
    pub fn memset(&self, value: u8, num: usize) {
        let mut dst = self.clone();
        for _ in 0..num {
            dst.write(value);
            dst += 1;
        }
    }

    #[allow(clippy::explicit_counter_loop)]
    pub fn memcmp(&self, other: &Self, len: usize) -> i32 {
        let mut a = self.clone();
        let mut b = other.clone();
        for _ in 0..len {
            let va = a.read();
            let vb = b.read();
            if va != vb {
                return va as i32 - vb as i32;
            }
            a += 1;
            b += 1;
        }
        0
    }

    pub fn slice_until(&self, end: &Self) -> Vec<u8> {
        assert!(self.kind == end.kind, "ub: invalid slice");
        let start: usize = self.offset;
        let end: usize = end.offset;
        assert!(start <= end);
        assert!(end <= self.len());
        match self.kind {
            PtrKind::Null => panic!("ub: dereference of null pointer"),
            PtrKind::StackSingle(_) | PtrKind::HeapSingle(_) => {
                if start < end {
                    vec![self.read()]
                } else {
                    Vec::new()
                }
            }
            PtrKind::Vec(ref weak) => {
                let strong = weak.upgrade().expect("ub: dangling pointer");
                let raw = strong.borrow();
                raw[start..end].to_vec()
            }
            PtrKind::StackArray(ref weak) | PtrKind::HeapArray(ref weak) => {
                let strong = weak.upgrade().expect("ub: dangling pointer");
                let raw = strong.borrow();
                raw[start..end].to_vec()
            }
            PtrKind::Reinterpreted(ref data) => {
                let mut buf = vec![0u8; end - start];
                data.read_bytes(start, &mut buf);
                buf
            }
        }
    }

    #[inline]
    pub fn from_string_literal(s: &'static str) -> Self {
        STRING_LITERALS.with(|literals| {
            let mut literals = literals.borrow_mut();
            let weak = Rc::downgrade(literals.entry(s).or_insert_with(|| {
                Rc::new(RefCell::new({
                    let mut v = s.as_bytes().to_vec();
                    v.push(0);
                    v
                }))
            }));
            Ptr {
                offset: 0,
                kind: PtrKind::Vec(weak),
            }
        })
    }

    pub fn to_c_string_iterator(&self) -> CStringIterator {
        CStringIterator { ptr: self.clone() }
    }

    pub fn to_rust_string(&self) -> String {
        let bytes: Vec<u8> = self.to_string_iterator().map(|p| p.read()).collect();
        String::from_utf8_lossy(&bytes).into_owned()
    }
}

pub(crate) trait ErasedPtr: std::any::Any {
    fn pointee_type_id(&self) -> std::any::TypeId;
    fn memcpy(&self, src: &dyn ErasedPtr, len: usize);
    fn as_any(&self) -> &dyn std::any::Any;
    fn equals(&self, other: &dyn ErasedPtr) -> Option<bool>;
    fn is_null(&self) -> bool;
}

impl<T> ErasedPtr for Ptr<T>
where
    T: ByteRepr + 'static,
    Ptr<T>: PartialEq,
{
    fn pointee_type_id(&self) -> std::any::TypeId {
        std::any::TypeId::of::<T>()
    }

    fn memcpy(&self, src: &dyn ErasedPtr, len: usize) {
        if self.pointee_type_id() != src.pointee_type_id() {
            panic!("memcpy: type mismatch");
        }
        let src_ptr = src
            .as_any()
            .downcast_ref::<Ptr<T>>()
            .expect("memcpy: downcast to Ptr<T> failed");
        let dst_bytes: Ptr<u8> = self.reinterpret_cast();
        let src_bytes: Ptr<u8> = src_ptr.reinterpret_cast();
        dst_bytes.memcpy(&src_bytes, len);
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn equals(&self, other: &dyn ErasedPtr) -> Option<bool> {
        if self.pointee_type_id() != other.pointee_type_id() {
            return None;
        }

        if let Some(other_ptr) = other.as_any().downcast_ref::<Ptr<T>>() {
            return Some(self == other_ptr);
        }

        None
    }

    fn is_null(&self) -> bool {
        Ptr::is_null(self)
    }
}

#[derive(Clone)]
pub struct AnyPtr {
    pub(crate) ptr: Rc<dyn ErasedPtr>,
}

impl<T: ByteRepr + 'static> Ptr<T> {
    pub fn to_any(&self) -> AnyPtr {
        AnyPtr {
            ptr: Rc::new(self.clone()),
        }
    }
}

impl Default for AnyPtr {
    fn default() -> Self {
        Ptr::<()>::null().to_any()
    }
}

impl AnyPtr {
    pub fn cast<T: 'static>(&self) -> Option<Ptr<T>> {
        if self.ptr.is_null() {
            return Some(Ptr::<T>::null());
        }
        self.ptr.as_any().downcast_ref::<Ptr<T>>().cloned()
    }

    pub fn reinterpret_cast<T: ByteRepr>(&self) -> Ptr<T> {
        macro_rules! try_src {
            ($ty:ty) => {{
                if let Some(p) = self.cast::<$ty>() {
                    return p.reinterpret_cast::<T>();
                }
                if let Some(pv) = self.cast::<Vec<$ty>>() {
                    return pv.reinterpret_cast::<T>();
                }
            }};
        }

        try_src!(u8);
        try_src!(i8);
        try_src!(u16);
        try_src!(i16);
        try_src!(u32);
        try_src!(i32);
        try_src!(u64);
        try_src!(i64);
        try_src!(usize);
        try_src!(isize);

        panic!("reinterpret_cast: unsupported AnyPtr source");
    }
}

impl PartialEq for AnyPtr {
    fn eq(&self, other: &Self) -> bool {
        let lhs: &dyn ErasedPtr = self.ptr.as_ref();
        let rhs: &dyn ErasedPtr = other.ptr.as_ref();

        lhs.equals(rhs).unwrap_or_default()
    }
}

impl AnyPtr {
    pub fn memcpy(&self, src: &AnyPtr, len: usize) {
        let dst_erased = &*self.ptr;
        let src_erased = &*src.ptr;

        if dst_erased.pointee_type_id() == src_erased.pointee_type_id() {
            dst_erased.memcpy(src_erased, len);
            return;
        }

        let dst_u8: Ptr<u8> = self.reinterpret_cast();
        let src_u8: Ptr<u8> = src.reinterpret_cast();
        dst_u8.memcpy(&src_u8, len);
    }

    pub fn memset(&self, value: u8, num: usize) {
        let dst_u8: Ptr<u8> = self.reinterpret_cast();
        dst_u8.memset(value, num);
    }

    pub fn memcmp(&self, other: &AnyPtr, len: usize) -> i32 {
        let a: Ptr<u8> = self.reinterpret_cast();
        let b: Ptr<u8> = other.reinterpret_cast();
        a.memcmp(&b, len)
    }
}

pub struct StrongPtrDyn<T: ?Sized> {
    rc: Rc<RefCell<T>>,
}

impl<T: ?Sized> StrongPtrDyn<T> {
    pub fn deref(&self) -> Ref<'_, T> {
        self.rc.borrow()
    }

    pub fn deref_mut(&self) -> RefMut<'_, T> {
        self.rc.borrow_mut()
    }
}

#[derive(Default, Debug)]
enum PtrKindDyn<T: ?Sized> {
    #[default]
    Null, // TODO: is this useful?
    StackSingle(Weak<RefCell<T>>),
}

impl<T: ?Sized> Clone for PtrKindDyn<T> {
    fn clone(&self) -> Self {
        match &self {
            PtrKindDyn::Null => PtrKindDyn::Null,
            PtrKindDyn::StackSingle(weak) => PtrKindDyn::StackSingle(weak.clone()),
        }
    }
}

#[derive(Debug, Default)]
pub struct PtrDyn<T: ?Sized> {
    offset: usize,
    kind: PtrKindDyn<T>,
}

impl<T: ?Sized> PtrDyn<T> {
    pub fn upgrade(&self) -> StrongPtrDyn<T> {
        match &self.kind {
            PtrKindDyn::Null => panic!("ub: dereference of null pointer"),
            PtrKindDyn::StackSingle(weak) => {
                assert_eq!(self.offset, 0, "ub: invalid offset");
                StrongPtrDyn {
                    rc: weak.upgrade().expect("ub: dangling pointer"),
                }
            }
        }
    }
}

impl<T: ?Sized> Clone for PtrDyn<T> {
    fn clone(&self) -> Self {
        Self {
            offset: self.offset,
            kind: self.kind.clone(),
        }
    }
}

pub trait AsPointerDyn<T: ?Sized> {
    fn as_pointer_dyn(&self) -> PtrDyn<T>;
}

impl<T: ?Sized> AsPointerDyn<T> for Rc<RefCell<T>> {
    #[inline]
    fn as_pointer_dyn(&self) -> PtrDyn<T> {
        PtrDyn {
            offset: 0,
            kind: PtrKindDyn::StackSingle(Rc::downgrade(self)),
        }
    }
}

impl<T: 'static> ByteRepr for Ptr<T> {}

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

    #[test]
    fn anyptr_null_cast() {
        // void* nullptr
        let any = Ptr::<()>::null().to_any();
        let p: Option<Ptr<u32>> = any.cast::<u32>();
        assert!(p.is_some());
        assert!(p.unwrap().is_null());

        let p2: Option<Ptr<u8>> = any.cast::<u8>();
        assert!(p2.is_some());
        assert!(p2.unwrap().is_null());

        // int* nullptr
        let any2 = Ptr::<i32>::null().to_any();
        let p3: Option<Ptr<f32>> = any2.cast::<f32>();
        assert!(p3.is_some());
        assert!(p3.unwrap().is_null());
    }

    #[test]
    fn to_any_without_clone() {
        let p: Ptr<std::fs::File> = Ptr::null(); // std::fs::File is not Clone
        let any = p.to_any();
        let recovered = any.cast::<std::fs::File>();
        assert!(recovered.is_some());
        assert!(recovered.unwrap().is_null());
    }
}
