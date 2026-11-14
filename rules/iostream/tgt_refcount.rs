// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

// TODO: t2 and t3 should be translated to Ptr<dyn Traits>
fn t2() -> Ptr<std::fs::File> {
    Ptr::null()
}

fn t3() -> Ptr<std::fs::File> {
    Ptr::null()
}

fn t5() -> Ptr<std::fs::File> {
    Ptr::null()
}

fn t6() -> Ptr<std::fs::File> {
    Ptr::null()
}

fn f1() -> Ptr<::std::fs::File> {
    libcc2rs::cout()
}

fn f2() -> Ptr<::std::fs::File> {
    libcc2rs::cerr()
}

fn f3() -> Ptr<::std::fs::File> {
    libcc2rs::cin()
}

fn f4(a0: Ptr<::std::fs::File>) -> i32 {
    a0.with_mut(libcc2rs::istream_get)
}
