// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

fn t1<T1>() -> Box<[Value<Box<[T1]>>]> {
    Box::new([Default::default()])
}

fn t2<T1>() -> Box<[Value<Box<[Value<Box<[T1]>>]>>]> {
    Box::new([Default::default()])
}
