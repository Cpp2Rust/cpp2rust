// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use std::rc::Rc;

use crate::rc::Ptr;
use crate::reinterpret::ByteRepr;

pub(crate) trait ErasedPtr: std::any::Any {
    fn as_bytes(&self) -> Ptr<u8>;
    fn as_any(&self) -> &dyn std::any::Any;
    fn equals(&self, other: &dyn ErasedPtr) -> bool;
    fn is_null(&self) -> bool;
}

impl PartialEq for dyn ErasedPtr {
    fn eq(&self, other: &Self) -> bool {
        self.equals(other)
    }
}

impl<T> ErasedPtr for Ptr<T>
where
    T: ByteRepr + 'static,
    Ptr<T>: PartialEq,
{
    fn as_bytes(&self) -> Ptr<u8> {
        self.reinterpret_cast::<u8>()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn equals(&self, other: &dyn ErasedPtr) -> bool {
        other.as_any().downcast_ref::<Ptr<T>>() == Some(self)
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
    pub fn reinterpret_cast<T: ByteRepr>(&self) -> Ptr<T> {
        if self.ptr.is_null() {
            return Ptr::<T>::null();
        }
        if let Some(p) = self.ptr.as_any().downcast_ref::<Ptr<T>>() {
            return p.clone();
        }
        self.ptr.as_bytes().reinterpret_cast::<T>()
    }

    pub fn is_null(&self) -> bool {
        self.ptr.is_null()
    }
}

impl PartialEq for AnyPtr {
    fn eq(&self, other: &Self) -> bool {
        *self.ptr == *other.ptr
    }
}

impl AnyPtr {
    pub fn memcpy(&self, src: &AnyPtr, len: usize) {
        let dst_u8 = self.ptr.as_bytes();
        let src_u8 = src.ptr.as_bytes();
        dst_u8.memcpy(&src_u8, len);
    }

    pub fn memset(&self, value: u8, num: usize) {
        self.ptr.as_bytes().memset(value, num);
    }

    pub fn memcmp(&self, other: &AnyPtr, len: usize) -> i32 {
        let a = self.ptr.as_bytes();
        let b = other.ptr.as_bytes();
        a.memcmp(&b, len)
    }
}

impl ByteRepr for AnyPtr {}

impl AnyPtr {
    pub fn to_int(&self) -> usize {
        self.reinterpret_cast::<u8>().to_int()
    }

    pub fn from_int(value: usize) -> Self {
        Ptr::<u8>::from_int(value).to_any()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn anyptr_null_cast() {
        // void* nullptr
        let any = Ptr::<()>::null().to_any();
        let p = any.reinterpret_cast::<u32>();
        assert!(p.is_null());

        let p2 = any.reinterpret_cast::<u8>();
        assert!(p2.is_null());

        // int* nullptr
        let any2 = Ptr::<i32>::null().to_any();
        let p3 = any2.reinterpret_cast::<f32>();
        assert!(p3.is_null());
    }

    #[test]
    fn to_any_without_clone() {
        let p: Ptr<std::fs::File> = Ptr::null(); // std::fs::File is not Clone
        let any = p.to_any();
        let recovered = any.reinterpret_cast::<std::fs::File>();
        assert!(recovered.is_null());
    }
}
