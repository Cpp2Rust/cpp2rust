// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

// Rust does not have support for localization.
//
// TODO: we need to track ourselves the locale settings and change the behavior of the relevant
// functions based on the set locale.
fn f1(a0: i32, a1: Ptr<u8>) -> Ptr<u8> {
    Ptr::<u8>::from_string_literal(b"C")
}
