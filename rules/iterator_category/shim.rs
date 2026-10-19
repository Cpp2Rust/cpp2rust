// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use std::cmp::Ordering;
use std::convert::Infallible;
use std::marker::PhantomData;

use std::rc::Rc;

use crate::{MapNode, PostfixDec, PostfixInc, PrefixDec, PrefixInc, Ptr, UnsafeMapNode, Value};

pub trait CppIterator<T>: Clone {
    fn iter_deref(&self) -> Ptr<T>;
    fn iter_inc(&mut self) -> Self;
    fn iter_post_inc(&mut self) -> Self;
}

pub trait CppInputIterator<T>: CppIterator<T> + PartialEq {}

pub trait CppBidirectionalIterator<T>: CppInputIterator<T> {
    fn iter_dec(&mut self) -> Self;
    fn iter_post_dec(&mut self) -> Self;
}

pub trait CppRandomAccessIterator<T>: CppBidirectionalIterator<T> + PartialOrd {
    fn iter_add(&self, n: isize) -> Self;
    fn iter_add_assign(&mut self, n: isize) -> Self;
    fn iter_distance(&self, from: &Self) -> isize;
    fn iter_at(&self, n: isize) -> Ptr<T>;
}

pub trait UnsafeCppIterator<T>: Clone {
    fn iter_deref(&self) -> *mut T;
    fn iter_inc(&mut self) -> Self;
    fn iter_post_inc(&mut self) -> Self;
}

pub trait UnsafeCppInputIterator<T>: UnsafeCppIterator<T> + PartialEq {}

pub trait UnsafeCppBidirectionalIterator<T>: UnsafeCppInputIterator<T> {
    fn iter_dec(&mut self) -> Self;
    fn iter_post_dec(&mut self) -> Self;
}

pub trait UnsafeCppRandomAccessIterator<T>:
    UnsafeCppBidirectionalIterator<T> + PartialOrd
{
    fn iter_add(&self, n: isize) -> Self;
    fn iter_add_assign(&mut self, n: isize) -> Self;
    fn iter_distance(&self, from: &Self) -> isize;
    fn iter_at(&self, n: isize) -> *mut T;
}

impl<T> CppIterator<T> for Ptr<T> {
    fn iter_deref(&self) -> Ptr<T> {
        self.clone()
    }
    fn iter_inc(&mut self) -> Self {
        PrefixInc::prefix_inc(self)
    }
    fn iter_post_inc(&mut self) -> Self {
        PostfixInc::postfix_inc(self)
    }
}

impl<T> CppInputIterator<T> for Ptr<T> {}

impl<T> CppBidirectionalIterator<T> for Ptr<T> {
    fn iter_dec(&mut self) -> Self {
        PrefixDec::prefix_dec(self)
    }
    fn iter_post_dec(&mut self) -> Self {
        PostfixDec::postfix_dec(self)
    }
}

impl<T> CppRandomAccessIterator<T> for Ptr<T> {
    fn iter_add(&self, n: isize) -> Self {
        self.offset(n)
    }
    fn iter_add_assign(&mut self, n: isize) -> Self {
        *self = self.offset(n);
        self.clone()
    }
    fn iter_distance(&self, from: &Self) -> isize {
        self.get_offset() as isize - from.get_offset() as isize
    }
    fn iter_at(&self, n: isize) -> Ptr<T> {
        self.offset(n)
    }
}

macro_rules! raw_pointer_iterator {
    ($ptr:ty, $deref:expr) => {
        impl<T> UnsafeCppIterator<T> for $ptr {
            fn iter_deref(&self) -> *mut T {
                $deref(*self)
            }
            fn iter_inc(&mut self) -> Self {
                *self = self.wrapping_add(1);
                *self
            }
            fn iter_post_inc(&mut self) -> Self {
                let old = *self;
                *self = self.wrapping_add(1);
                old
            }
        }

        impl<T> UnsafeCppInputIterator<T> for $ptr {}

        impl<T> UnsafeCppBidirectionalIterator<T> for $ptr {
            fn iter_dec(&mut self) -> Self {
                *self = self.wrapping_sub(1);
                *self
            }
            fn iter_post_dec(&mut self) -> Self {
                let old = *self;
                *self = self.wrapping_sub(1);
                old
            }
        }

        impl<T> UnsafeCppRandomAccessIterator<T> for $ptr {
            fn iter_add(&self, n: isize) -> Self {
                self.wrapping_offset(n)
            }
            fn iter_add_assign(&mut self, n: isize) -> Self {
                *self = self.wrapping_offset(n);
                *self
            }
            fn iter_distance(&self, from: &Self) -> isize {
                unsafe { self.offset_from(*from) }
            }
            fn iter_at(&self, n: isize) -> *mut T {
                $deref(self.wrapping_offset(n))
            }
        }
    };
}

raw_pointer_iterator!(*mut T, |p: *mut T| p);
raw_pointer_iterator!(*const T, |p: *const T| p.cast_mut());

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
            fn iter_deref(&self) -> $ptr {
                self.never()
            }
            fn iter_inc(&mut self) -> Self {
                self.never()
            }
            fn iter_post_inc(&mut self) -> Self {
                self.never()
            }
        }
    };
}

uninhabited_iterator!(InputIterator, "input iterators");
uninhabited_iterator!(OutputIterator, "output iterators");
uninhabited_iterator!(ForwardIterator, "forward iterators");
uninhabited_iterator!(UnsafeInputIterator, "input iterators");
uninhabited_iterator!(UnsafeOutputIterator, "output iterators");
uninhabited_iterator!(UnsafeForwardIterator, "forward iterators");

uninhabited_iterator_ops!(InputIterator, CppIterator, Ptr<T>);
uninhabited_iterator_ops!(OutputIterator, CppIterator, Ptr<T>);
uninhabited_iterator_ops!(ForwardIterator, CppIterator, Ptr<T>);
uninhabited_iterator_ops!(UnsafeInputIterator, UnsafeCppIterator, *mut T);
uninhabited_iterator_ops!(UnsafeOutputIterator, UnsafeCppIterator, *mut T);
uninhabited_iterator_ops!(UnsafeForwardIterator, UnsafeCppIterator, *mut T);

impl<T> CppInputIterator<T> for InputIterator<T> {}
impl<T> CppInputIterator<T> for ForwardIterator<T> {}
impl<T> UnsafeCppInputIterator<T> for UnsafeInputIterator<T> {}
impl<T> UnsafeCppInputIterator<T> for UnsafeForwardIterator<T> {}

pub enum BidirectionalIterator<T> {
    Map(Value<MapNode<T>>),
}

impl<T> Default for BidirectionalIterator<T> {
    fn default() -> Self {
        Self::Map(MapNode::header())
    }
}

impl<T> Clone for BidirectionalIterator<T> {
    fn clone(&self) -> Self {
        match self {
            Self::Map(node) => Self::Map(node.clone()),
        }
    }
}

impl<T> PartialEq for BidirectionalIterator<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Map(a), Self::Map(b)) => Rc::ptr_eq(a, b),
        }
    }
}

impl<T> CppIterator<T> for BidirectionalIterator<T> {
    fn iter_deref(&self) -> Ptr<T> {
        match self {
            Self::Map(node) => MapNode::value(node),
        }
    }
    fn iter_inc(&mut self) -> Self {
        match self {
            Self::Map(node) => *node = MapNode::next(node),
        }
        self.clone()
    }
    fn iter_post_inc(&mut self) -> Self {
        let old = self.clone();
        self.iter_inc();
        old
    }
}

impl<T> CppInputIterator<T> for BidirectionalIterator<T> {}

impl<T> CppBidirectionalIterator<T> for BidirectionalIterator<T> {
    fn iter_dec(&mut self) -> Self {
        match self {
            Self::Map(node) => *node = MapNode::prev(node),
        }
        self.clone()
    }
    fn iter_post_dec(&mut self) -> Self {
        let old = self.clone();
        self.iter_dec();
        old
    }
}

pub enum UnsafeBidirectionalIterator<T> {
    Map(*mut UnsafeMapNode<T>),
}

impl<T> Default for UnsafeBidirectionalIterator<T> {
    fn default() -> Self {
        Self::Map(std::ptr::null_mut())
    }
}

impl<T> Clone for UnsafeBidirectionalIterator<T> {
    fn clone(&self) -> Self {
        match self {
            Self::Map(node) => Self::Map(*node),
        }
    }
}

impl<T> PartialEq for UnsafeBidirectionalIterator<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Map(a), Self::Map(b)) => a == b,
        }
    }
}

impl<T> UnsafeCppIterator<T> for UnsafeBidirectionalIterator<T> {
    fn iter_deref(&self) -> *mut T {
        match self {
            Self::Map(node) => UnsafeMapNode::value(*node),
        }
    }
    fn iter_inc(&mut self) -> Self {
        match self {
            Self::Map(node) => *node = UnsafeMapNode::next(*node),
        }
        self.clone()
    }
    fn iter_post_inc(&mut self) -> Self {
        let old = self.clone();
        self.iter_inc();
        old
    }
}

impl<T> UnsafeCppInputIterator<T> for UnsafeBidirectionalIterator<T> {}

impl<T> UnsafeCppBidirectionalIterator<T> for UnsafeBidirectionalIterator<T> {
    fn iter_dec(&mut self) -> Self {
        match self {
            Self::Map(node) => *node = UnsafeMapNode::prev(*node),
        }
        self.clone()
    }
    fn iter_post_dec(&mut self) -> Self {
        let old = self.clone();
        self.iter_dec();
        old
    }
}

macro_rules! random_access_iterator {
    ($name:ident, $ptr:ty, $iter:ident, $input:ident, $bidi:ident, $random:ident, $null:expr) => {
        pub enum $name<T> {
            Vec($ptr),
        }

        impl<T> $name<T> {
            pub fn vec_ptr(&self) -> $ptr {
                match self {
                    Self::Vec(p) => p.clone(),
                }
            }
        }

        impl<T> Default for $name<T> {
            fn default() -> Self {
                Self::Vec($null)
            }
        }

        impl<T> Clone for $name<T> {
            fn clone(&self) -> Self {
                Self::Vec(self.vec_ptr())
            }
        }

        impl<T> PartialEq for $name<T> {
            fn eq(&self, other: &Self) -> bool {
                match (self, other) {
                    (Self::Vec(a), Self::Vec(b)) => a == b,
                }
            }
        }

        impl<T> PartialOrd for $name<T> {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                match (self, other) {
                    (Self::Vec(a), Self::Vec(b)) => a.partial_cmp(b),
                }
            }
        }

        impl<T> $iter<T> for $name<T> {
            fn iter_deref(&self) -> $ptr {
                match self {
                    Self::Vec(p) => $iter::<T>::iter_deref(p),
                }
            }
            fn iter_inc(&mut self) -> Self {
                match self {
                    Self::Vec(p) => Self::Vec($iter::<T>::iter_inc(p)),
                }
            }
            fn iter_post_inc(&mut self) -> Self {
                match self {
                    Self::Vec(p) => Self::Vec($iter::<T>::iter_post_inc(p)),
                }
            }
        }

        impl<T> $input<T> for $name<T> {}

        impl<T> $bidi<T> for $name<T> {
            fn iter_dec(&mut self) -> Self {
                match self {
                    Self::Vec(p) => Self::Vec($bidi::<T>::iter_dec(p)),
                }
            }
            fn iter_post_dec(&mut self) -> Self {
                match self {
                    Self::Vec(p) => Self::Vec($bidi::<T>::iter_post_dec(p)),
                }
            }
        }

        impl<T> $random<T> for $name<T> {
            fn iter_add(&self, n: isize) -> Self {
                match self {
                    Self::Vec(p) => Self::Vec($random::<T>::iter_add(p, n)),
                }
            }
            fn iter_add_assign(&mut self, n: isize) -> Self {
                match self {
                    Self::Vec(p) => Self::Vec($random::<T>::iter_add_assign(p, n)),
                }
            }
            fn iter_distance(&self, from: &Self) -> isize {
                match (self, from) {
                    (Self::Vec(a), Self::Vec(b)) => $random::<T>::iter_distance(a, b),
                }
            }
            fn iter_at(&self, n: isize) -> $ptr {
                match self {
                    Self::Vec(p) => $random::<T>::iter_at(p, n),
                }
            }
        }
    };
}

random_access_iterator!(
    RandomAccessIterator,
    Ptr<T>,
    CppIterator,
    CppInputIterator,
    CppBidirectionalIterator,
    CppRandomAccessIterator,
    Ptr::null()
);

random_access_iterator!(
    UnsafeRandomAccessIterator,
    *mut T,
    UnsafeCppIterator,
    UnsafeCppInputIterator,
    UnsafeCppBidirectionalIterator,
    UnsafeCppRandomAccessIterator,
    std::ptr::null_mut()
);

pub struct IterRange<I, T> {
    cur: I,
    end: I,
    elem: PhantomData<T>,
}

impl<I, T> IterRange<I, T> {
    pub fn new(cur: I, end: I) -> Self {
        Self {
            cur,
            end,
            elem: PhantomData,
        }
    }
}

impl<I: CppInputIterator<T>, T> Iterator for IterRange<I, T> {
    type Item = I;

    fn next(&mut self) -> Option<I> {
        if self.cur == self.end {
            return None;
        }
        let it = self.cur.clone();
        self.cur.iter_inc();
        Some(it)
    }
}

pub struct UnsafeIterRange<I, T> {
    cur: I,
    end: I,
    elem: PhantomData<T>,
}

impl<I, T> UnsafeIterRange<I, T> {
    pub fn new(cur: I, end: I) -> Self {
        Self {
            cur,
            end,
            elem: PhantomData,
        }
    }
}

impl<I: UnsafeCppInputIterator<T>, T> Iterator for UnsafeIterRange<I, T> {
    type Item = I;

    fn next(&mut self) -> Option<I> {
        if self.cur == self.end {
            return None;
        }
        let it = self.cur.clone();
        self.cur.iter_inc();
        Some(it)
    }
}
