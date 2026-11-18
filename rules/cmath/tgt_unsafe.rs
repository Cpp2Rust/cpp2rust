// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

unsafe fn f1(a0: i32) -> i32 {
    a0.abs()
}
unsafe fn f2(a0: i64) -> i64 {
    a0.abs()
}
unsafe fn f3(a0: i64) -> i64 {
    a0.abs()
}
unsafe fn f4(a0: f64) -> f64 {
    a0.log2()
}
unsafe fn f5(a0: i32) -> f64 {
    (a0 as f64).log2()
}
