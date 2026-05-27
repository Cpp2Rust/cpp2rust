// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

unsafe extern "C" {
    fn f1(a0: i32, a1: i32, ...) -> i32;
    fn f2(a0: *const i8, a1: i32, ...) -> i32;
}
