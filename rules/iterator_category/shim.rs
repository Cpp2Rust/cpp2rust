// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use std::cmp::Ordering;
use std::convert::Infallible;
use std::marker::PhantomData;

use crate::{PostfixDec, PostfixInc, PrefixDec, PrefixInc, Ptr};

pub trait CppIterator<T>: Clone {
    fn deref(&self) -> Ptr<T>;
    fn prefix_inc(&mut self) -> Self;
    fn postfix_inc(&mut self) -> Self;
}

pub trait CppInputIterator<T>: CppIterator<T> + PartialEq {}

pub trait CppBidirectionalIterator<T>: CppInputIterator<T> {
    fn prefix_dec(&mut self) -> Self;
    fn postfix_dec(&mut self) -> Self;
}

pub trait CppRandomAccessIterator<T>: CppBidirectionalIterator<T> + PartialOrd {
    fn plus(&self, n: isize) -> Self;
    fn plus_assign(&mut self, n: isize) -> Self;
    fn distance(&self, from: &Self) -> isize;
    fn at(&self, n: isize) -> Ptr<T>;
}

pub trait UnsafeCppIterator<T>: Clone {
    fn deref(&self) -> *mut T;
    fn prefix_inc(&mut self) -> Self;
    fn postfix_inc(&mut self) -> Self;
}

pub trait UnsafeCppInputIterator<T>: UnsafeCppIterator<T> + PartialEq {}

pub trait UnsafeCppBidirectionalIterator<T>: UnsafeCppInputIterator<T> {
    fn prefix_dec(&mut self) -> Self;
    fn postfix_dec(&mut self) -> Self;
}

pub trait UnsafeCppRandomAccessIterator<T>:
    UnsafeCppBidirectionalIterator<T> + PartialOrd
{
    fn plus(&self, n: isize) -> Self;
    fn plus_assign(&mut self, n: isize) -> Self;
    fn distance(&self, from: &Self) -> isize;
    fn at(&self, n: isize) -> *mut T;
}

macro_rules! uninhabited_iterator {
    ($name:ident, $what:literal) => {
        pub enum $name<T> {
            #[doc(hidden)]
            None(Infallible, PhantomData<T>),
        }

        impl<T> $name<T> {
            pub fn uninitialized() -> Self {
                panic!(concat!("no container provides ", $what, " yet"))
            }

            fn never(&self) -> ! {
                match self {
                    Self::None(never, _) => match *never {},
                }
            }
        }

        impl<T> Clone for $name<T> {
            fn clone(&self) -> Self {
                self.never()
            }
        }

        impl<T> PartialEq for $name<T> {
            fn eq(&self, _: &Self) -> bool {
                self.never()
            }
        }
    };
}

macro_rules! uninhabited_iterator_ops {
    ($name:ident, $iter:ident, $ptr:ty) => {
        impl<T> $iter<T> for $name<T> {
            fn deref(&self) -> $ptr {
                self.never()
            }
            fn prefix_inc(&mut self) -> Self {
                self.never()
            }
            fn postfix_inc(&mut self) -> Self {
                self.never()
            }
        }
    };
}

uninhabited_iterator!(InputIterator, "input iterators");
uninhabited_iterator!(OutputIterator, "output iterators");
uninhabited_iterator!(ForwardIterator, "forward iterators");
uninhabited_iterator!(BidirectionalIterator, "bidirectional iterators");
uninhabited_iterator!(UnsafeInputIterator, "input iterators");
uninhabited_iterator!(UnsafeOutputIterator, "output iterators");
uninhabited_iterator!(UnsafeForwardIterator, "forward iterators");
uninhabited_iterator!(UnsafeBidirectionalIterator, "bidirectional iterators");

uninhabited_iterator_ops!(InputIterator, CppIterator, Ptr<T>);
uninhabited_iterator_ops!(OutputIterator, CppIterator, Ptr<T>);
uninhabited_iterator_ops!(ForwardIterator, CppIterator, Ptr<T>);
uninhabited_iterator_ops!(BidirectionalIterator, CppIterator, Ptr<T>);
uninhabited_iterator_ops!(UnsafeInputIterator, UnsafeCppIterator, *mut T);
uninhabited_iterator_ops!(UnsafeOutputIterator, UnsafeCppIterator, *mut T);
uninhabited_iterator_ops!(UnsafeForwardIterator, UnsafeCppIterator, *mut T);
uninhabited_iterator_ops!(UnsafeBidirectionalIterator, UnsafeCppIterator, *mut T);

impl<T> CppInputIterator<T> for InputIterator<T> {}
impl<T> CppInputIterator<T> for ForwardIterator<T> {}
impl<T> CppInputIterator<T> for BidirectionalIterator<T> {}
impl<T> UnsafeCppInputIterator<T> for UnsafeInputIterator<T> {}
impl<T> UnsafeCppInputIterator<T> for UnsafeForwardIterator<T> {}
impl<T> UnsafeCppInputIterator<T> for UnsafeBidirectionalIterator<T> {}

impl<T> CppBidirectionalIterator<T> for BidirectionalIterator<T> {
    fn prefix_dec(&mut self) -> Self {
        self.never()
    }
    fn postfix_dec(&mut self) -> Self {
        self.never()
    }
}

impl<T> UnsafeCppBidirectionalIterator<T> for UnsafeBidirectionalIterator<T> {
    fn prefix_dec(&mut self) -> Self {
        self.never()
    }
    fn postfix_dec(&mut self) -> Self {
        self.never()
    }
}

pub enum RandomAccessIterator<T> {
    Vec(Ptr<T>),
}

impl<T> RandomAccessIterator<T> {
    pub fn vec_ptr(&self) -> Ptr<T> {
        match self {
            Self::Vec(p) => p.clone(),
        }
    }
}

impl<T> Default for RandomAccessIterator<T> {
    fn default() -> Self {
        Self::Vec(Ptr::null())
    }
}

impl<T> Clone for RandomAccessIterator<T> {
    fn clone(&self) -> Self {
        match self {
            Self::Vec(p) => Self::Vec(p.clone()),
        }
    }
}

impl<T> PartialEq for RandomAccessIterator<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Vec(a), Self::Vec(b)) => a == b,
        }
    }
}

impl<T> PartialOrd for RandomAccessIterator<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self::Vec(a), Self::Vec(b)) => a.partial_cmp(b),
        }
    }
}

impl<T> CppIterator<T> for RandomAccessIterator<T> {
    fn deref(&self) -> Ptr<T> {
        self.vec_ptr()
    }
    fn prefix_inc(&mut self) -> Self {
        match self {
            Self::Vec(p) => Self::Vec(PrefixInc::prefix_inc(p)),
        }
    }
    fn postfix_inc(&mut self) -> Self {
        match self {
            Self::Vec(p) => Self::Vec(PostfixInc::postfix_inc(p)),
        }
    }
}

impl<T> CppInputIterator<T> for RandomAccessIterator<T> {}

impl<T> CppBidirectionalIterator<T> for RandomAccessIterator<T> {
    fn prefix_dec(&mut self) -> Self {
        match self {
            Self::Vec(p) => Self::Vec(PrefixDec::prefix_dec(p)),
        }
    }
    fn postfix_dec(&mut self) -> Self {
        match self {
            Self::Vec(p) => Self::Vec(PostfixDec::postfix_dec(p)),
        }
    }
}

impl<T> CppRandomAccessIterator<T> for RandomAccessIterator<T> {
    fn plus(&self, n: isize) -> Self {
        match self {
            Self::Vec(p) => Self::Vec(p.offset(n)),
        }
    }
    fn plus_assign(&mut self, n: isize) -> Self {
        match self {
            Self::Vec(p) => {
                *p = p.offset(n);
                Self::Vec(p.clone())
            }
        }
    }
    fn distance(&self, from: &Self) -> isize {
        match (self, from) {
            (Self::Vec(a), Self::Vec(b)) => a.get_offset() as isize - b.get_offset() as isize,
        }
    }
    fn at(&self, n: isize) -> Ptr<T> {
        match self {
            Self::Vec(p) => p.offset(n),
        }
    }
}

pub enum UnsafeRandomAccessIterator<T> {
    Vec(*mut T),
}

impl<T> UnsafeRandomAccessIterator<T> {
    pub fn vec_ptr(&self) -> *mut T {
        match self {
            Self::Vec(p) => *p,
        }
    }
}

impl<T> Default for UnsafeRandomAccessIterator<T> {
    fn default() -> Self {
        Self::Vec(std::ptr::null_mut())
    }
}

impl<T> Clone for UnsafeRandomAccessIterator<T> {
    fn clone(&self) -> Self {
        Self::Vec(self.vec_ptr())
    }
}

impl<T> PartialEq for UnsafeRandomAccessIterator<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Vec(a), Self::Vec(b)) => a == b,
        }
    }
}

impl<T> PartialOrd for UnsafeRandomAccessIterator<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self::Vec(a), Self::Vec(b)) => a.partial_cmp(b),
        }
    }
}

impl<T> UnsafeCppIterator<T> for UnsafeRandomAccessIterator<T> {
    fn deref(&self) -> *mut T {
        self.vec_ptr()
    }
    fn prefix_inc(&mut self) -> Self {
        match self {
            Self::Vec(p) => {
                *p = p.wrapping_add(1);
                Self::Vec(*p)
            }
        }
    }
    fn postfix_inc(&mut self) -> Self {
        match self {
            Self::Vec(p) => {
                let old = *p;
                *p = p.wrapping_add(1);
                Self::Vec(old)
            }
        }
    }
}

impl<T> UnsafeCppInputIterator<T> for UnsafeRandomAccessIterator<T> {}

impl<T> UnsafeCppBidirectionalIterator<T> for UnsafeRandomAccessIterator<T> {
    fn prefix_dec(&mut self) -> Self {
        match self {
            Self::Vec(p) => {
                *p = p.wrapping_sub(1);
                Self::Vec(*p)
            }
        }
    }
    fn postfix_dec(&mut self) -> Self {
        match self {
            Self::Vec(p) => {
                let old = *p;
                *p = p.wrapping_sub(1);
                Self::Vec(old)
            }
        }
    }
}

impl<T> UnsafeCppRandomAccessIterator<T> for UnsafeRandomAccessIterator<T> {
    fn plus(&self, n: isize) -> Self {
        Self::Vec(self.vec_ptr().wrapping_offset(n))
    }
    fn plus_assign(&mut self, n: isize) -> Self {
        match self {
            Self::Vec(p) => {
                *p = p.wrapping_offset(n);
                Self::Vec(*p)
            }
        }
    }
    fn distance(&self, from: &Self) -> isize {
        match (self, from) {
            (Self::Vec(a), Self::Vec(b)) => unsafe { a.offset_from(*b) },
        }
    }
    fn at(&self, n: isize) -> *mut T {
        self.vec_ptr().wrapping_offset(n)
    }
}
