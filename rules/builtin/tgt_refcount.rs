// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

fn f9(a0: i64, a1: i64, a2: Ptr<i64>) -> bool {
    let (val, ovf) = a0.overflowing_mul(a1);
    a2.write(val);
    ovf
}
fn f10(a0: i64, a1: i64, a2: Ptr<i64>) -> bool {
    let (val, ovf) = a0.overflowing_mul(a1);
    a2.write(val);
    ovf
}
fn f12(a0: i64, a1: i64, a2: Ptr<i64>) -> bool {
    let (val, ovf) = a0.overflowing_mul(a1);
    a2.write(val);
    ovf
}
fn f13(a0: i64, a1: i64, a2: Ptr<i64>) -> bool {
    let (val, ovf) = a0.overflowing_mul(a1);
    a2.write(val);
    ovf
}
