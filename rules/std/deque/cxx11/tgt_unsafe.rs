// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

unsafe fn f1<T1>(a0: &mut Vec<T1>, init: T1) {
    let __init = init;
    a0.push(__init)
}

unsafe fn f2<T1>(a0: &mut Vec<Vec<T1>>, init: Vec<T1>) {
    let __init = init;
    a0.push(__init)
}
