// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use crate::{CChar, Ptr};

pub struct CStringIterator<T: CChar = u8> {
    pub(crate) ptr: Ptr<T>,
}

impl<T: CChar> Iterator for CStringIterator<T> {
    type Item = T;

    fn count(self) -> usize {
        self.ptr.c_str_len()
    }

    fn next(&mut self) -> Option<Self::Item> {
        // read until the null terminator
        let ch = self.ptr.read();
        if ch == T::default() {
            return None;
        }
        self.ptr += 1;
        Some(ch)
    }
}

impl<T> Ptr<T> {
    pub fn to_string_iterator(&self) -> StringIterator<T> {
        StringIterator { ptr: self.clone() }
    }
}

pub struct StringIterator<T> {
    ptr: Ptr<T>,
}

impl<T> Iterator for StringIterator<T> {
    type Item = Ptr<T>;
    fn next(&mut self) -> Option<Self::Item> {
        // stop before the null terminator at the last position
        if self.ptr.get_offset().wrapping_add(1) < self.ptr.len() {
            let value = self.ptr.clone();
            self.ptr += 1;
            Some(value)
        } else {
            None
        }
    }
}
