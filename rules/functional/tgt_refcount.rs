// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

fn t1<T1>() -> Ptr<T1> {
    Ptr::null()
}

fn f1<T1>(a0: Ptr<T1>) -> Ptr<T1> {
    a0
}

fn f2<T1>(a0: Ptr<T1>) -> Ptr<T1> {
    a0.clone()
}

fn f3<T1>(a0: Ptr<T1>) -> Ptr<T1> {
    a0.clone()
}
