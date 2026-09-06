// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

fn f1(a0: i32, a1: u64, va: &[VaArg]) -> i32 {
    panic!(
        "ioctl is not supported in the refcount model (fd={}, request={})",
        a0, a1
    );
    0
}
