// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

fn t1() -> Vec<libc::c_char> {
    vec![0]
}

fn t2() -> Vec<libc::c_char> {
    vec![0]
}

unsafe fn f1(a0: *const libc::c_char) -> Vec<libc::c_char> {
    let mut __v = ::std::ffi::CStr::from_ptr(a0)
        .to_bytes()
        .iter()
        .map(|&__b| __b as libc::c_char)
        .collect::<Vec<libc::c_char>>();
    __v.push(0);
    __v
}

unsafe fn f2(a0: Vec<libc::c_char>) -> Vec<libc::c_char> {
    a0
}
