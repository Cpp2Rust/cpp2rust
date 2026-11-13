// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;
use std::cell::RefCell;
use std::rc::Rc;

fn f1(a0: Ptr<i8>) -> libcc2rs::CppStream {
    ::std::fs::File::create(a0.to_string())
        .map(libcc2rs::CppStream::from)
        .expect("Failed to open file")
}

fn f5(a0: Ptr<i8>) -> libcc2rs::CppStream {
    ::std::fs::File::open(a0.to_string())
        .map(libcc2rs::CppStream::from)
        .expect("Failed to open file")
}

fn t3<T1>() -> Ptr<libcc2rs::CppStream> {
    Ptr::null()
}

fn t5<T1>() -> Ptr<libcc2rs::CppStream> {
    Ptr::null()
}

fn f2<T1>(a0: Ptr<libcc2rs::CppStream>) -> Ptr<libcc2rs::CppStream> {
    a0
}

fn f3<T1>(a0: Ptr<libcc2rs::CppStream>) -> Ptr<libcc2rs::CppStream> {
    a0
}

fn f4(a0: Ptr<libcc2rs::CppStream>) -> Ptr<libcc2rs::CppStream> {
    a0
}

fn f6<T1>(a0: Ptr<libcc2rs::CppStream>) -> Ptr<libcc2rs::CppStream> {
    a0
}

fn f7<T1>(a0: Ptr<libcc2rs::CppStream>) -> Ptr<libcc2rs::CppStream> {
    a0
}

fn f8(a0: Ptr<libcc2rs::CppStream>) -> Ptr<libcc2rs::CppStream> {
    a0
}
