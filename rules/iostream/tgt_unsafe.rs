// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

fn t1() -> libcc2rs::CppStream {
    ::std::fs::File::open("")
        .map(libcc2rs::CppStream::from)
        .unwrap()
}

// TODO: t2 and t3 should be translated to *mut dyn Traits
unsafe fn t2() -> *mut libcc2rs::CppStream {
    libcc2rs::cout_unsafe()
}

unsafe fn t3() -> *mut libcc2rs::CppStream {
    libcc2rs::cout_unsafe()
}

unsafe fn f1() -> *mut libcc2rs::CppStream {
    libcc2rs::cout_unsafe()
}

unsafe fn f2() -> *mut libcc2rs::CppStream {
    libcc2rs::cerr_unsafe()
}
