// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

unsafe fn f1() -> *mut i32 {
    libcc2rs::cpp2rust_errno_unsafe()
}
