// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

unsafe fn f1<T1: Into<i32>>(a0: T1) -> i32 {
    let __c: i32 = a0.into();
    if (0..=255).contains(&__c) {
        (__c as u8).to_ascii_uppercase() as i32
    } else {
        __c
    }
}
