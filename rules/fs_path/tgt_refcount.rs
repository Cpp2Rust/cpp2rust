// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

fn t1() -> Vec<i8> {
    vec![0]
}

fn f1(a0: Ptr<i8>) -> Vec<i8> {
    let mut __v = Vec::new();
    a0.with_c_str(|__s| __v.extend_from_slice(__s));
    __v.push(0);
    __v
}

fn f2(a0: Vec<i8>) -> Vec<i8> {
    a0.clone()
}
