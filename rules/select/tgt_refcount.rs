// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

fn t1() -> CFdSet {
    Default::default()
}

fn f2(a0: i32, a1: Ptr<CFdSet>) {
    a1.with_mut(|__s| __s.set(a0));
}

fn f3(a0: i32, a1: Ptr<CFdSet>) {
    a1.with_mut(|__s| __s.clr(a0));
}

fn f4(a0: i32, a1: Ptr<CFdSet>) -> i32 {
    if a1.with(|__s| __s.isset(a0)) { 1 } else { 0 }
}

fn f5(a0: Ptr<CFdSet>) {
    a0.with_mut(|__s| __s.zero());
}
