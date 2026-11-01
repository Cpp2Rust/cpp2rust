// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;
use std::cell::RefCell;
use std::rc::Rc;

fn f1(a0: Ptr<i8>) -> libcc2rs::CppStream {
    ::std::fs::File::create(a0.to_string())
        .map(libcc2rs::CppStream::from)
        .expect("Failed to open file")
}

fn f5(a0: Ptr<i8>) -> libcc2rs::CppStream {
    ::std::fs::File::open(a0.to_string())
        .map(libcc2rs::CppStream::from)
        .expect("Failed to open file")
}
