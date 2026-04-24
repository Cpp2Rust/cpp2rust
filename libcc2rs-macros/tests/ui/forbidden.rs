// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs_macros::{goto, switch};

fn continue_with_no_outer_loop() {
    let mut v = 0;
    // No outer loop to continue
    switch!(match 0 {
        0 => { v = 1; continue; }
        _ => { v = 2; }
    });
    let _ = v;
}

fn goto_outside_goto_block() {
    // goto!() used outside goto_block!
    goto!('nowhere);
}

fn main() {}
