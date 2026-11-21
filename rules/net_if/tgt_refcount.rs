// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

fn f1(a0: Ptr<i8>) -> u32 {
    match a0.with_c_bytes(nix::net::if_::if_nametoindex) {
        Ok(__i) => __i,
        Err(__e) => {
            libcc2rs::cpp2rust_errno().write(__e as i32);
            0
        }
    }
}
