// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

unsafe fn f1(a0: i32) -> i32 {
    if (0..=255).contains(&a0) {
        (a0 as u8).to_ascii_uppercase() as i32
    } else {
        a0
    }
}
