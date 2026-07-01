// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

fn t1() -> *mut ::libc::DIR {
    std::ptr::null_mut()
}

unsafe fn f1(a0: *const libc::c_char) -> *mut ::libc::DIR {
    libc::opendir(a0)
}

unsafe fn f2(a0: *mut ::libc::DIR) -> *mut ::libc::dirent {
    libc::readdir(a0)
}

unsafe fn f3(a0: *mut ::libc::DIR) -> i32 {
    libc::closedir(a0)
}
