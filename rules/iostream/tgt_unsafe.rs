// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

fn t1() -> std::fs::File {
    std::fs::File::open("").unwrap()
}

// TODO: t2 and t3 should be translated to *mut dyn Traits
unsafe fn t2() -> *mut std::fs::File {
    libcc2rs::cout_unsafe()
}

unsafe fn t3() -> *mut std::fs::File {
    libcc2rs::cout_unsafe()
}

fn t4() -> std::fs::File {
    std::fs::File::open("").unwrap()
}

unsafe fn t5() -> *mut std::fs::File {
    libcc2rs::cin_unsafe()
}

unsafe fn t6() -> *mut std::fs::File {
    libcc2rs::cin_unsafe()
}

unsafe fn f1() -> *mut ::std::fs::File {
    libcc2rs::cout_unsafe()
}

unsafe fn f2() -> *mut ::std::fs::File {
    libcc2rs::cerr_unsafe()
}

unsafe fn f3() -> *mut ::std::fs::File {
    libcc2rs::cin_unsafe()
}

unsafe fn f4(a0: *mut ::std::fs::File) -> i32 {
    libcc2rs::istream_get(&mut *a0)
}
