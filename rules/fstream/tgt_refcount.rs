// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;
use std::cell::RefCell;
use std::rc::Rc;

fn f1(a0: Ptr<u8>) -> ::std::fs::File {
    ::std::fs::File::create(a0.to_string()).expect("Failed to open file")
}

fn f5(a0: Ptr<u8>) -> ::std::fs::File {
    ::std::fs::File::open(a0.to_string()).expect("Failed to open file")
}
