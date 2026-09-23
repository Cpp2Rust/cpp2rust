// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use std::collections::BTreeSet;

#[derive(Clone, Default)]
pub struct CFdSet {
    fds: BTreeSet<i32>,
}

impl crate::ByteRepr for CFdSet {}

impl CFdSet {
    pub fn zero(&mut self) {
        self.fds.clear();
    }
    pub fn set(&mut self, fd: i32) {
        self.fds.insert(fd);
    }
    pub fn clr(&mut self, fd: i32) {
        self.fds.remove(&fd);
    }
    pub fn isset(&self, fd: i32) -> bool {
        self.fds.contains(&fd)
    }
}
