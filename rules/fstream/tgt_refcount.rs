// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;
use std::cell::RefCell;
use std::rc::Rc;

fn f1(a0: Ptr<i8>) -> ::std::fs::File {
    a0.with_c_os_str(|__path| ::std::fs::File::create(__path))
        .expect("Failed to open file")
}

fn f5(a0: Ptr<i8>) -> ::std::fs::File {
    a0.with_c_os_str(|__path| ::std::fs::File::open(__path))
        .expect("Failed to open file")
}
