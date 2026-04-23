// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs_macros::{goto, goto_block, switch, switch_break};

fn switch_bare_break() {
    let mut v = 0;
    switch!(match 0 {
        0 => { v = 1; break; } // should be switch_break!()
        _ => { v = 2; }
    });
    let _ = v;
}

fn switch_bare_continue() {
    let mut v = 0;
    switch!(match 0 {
        0 => { v = 1; continue; } // no continue, switch! does not act as a loop
        _ => { v = 2; }
    });
    let _ = v;
}

fn switch_labeled_break() {
    let mut v = 0;
    'outer: loop {
        switch!(match 0 {
            0 => { v = 1; break 'outer; } // no break with named label
            _ => { v = 2; }
        });
    }
    let _ = v;
}

fn switch_break_inside_nested_loop() {
    let mut v = 0;
    switch!(match 0 {
        0 => {
            for _ in 0..3 {
                switch_break!(); // break inside loop cannot exit the swich
            }
            v = 1;
        }
        _ => { v = 2; }
    });
    let _ = v;
}

fn goto_block_bare_break() {
    let mut v = 0;
    goto_block! {
        'a => { v = 1; break; },
        'b => { v = 2; },
    };
    let _ = v;
}

fn goto_block_labeled_continue() {
    let mut v = 0;
    'outer: loop {
        goto_block! {
            'a => { v = 1; continue 'outer; },
            'b => { v = 2; },
        };
        break;
    }
    let _ = v;
}

fn switch_break_outside_switch() {
    // no switch_break!() outside switch!
    switch_break!();
}

fn goto_outside_goto_block() {
    // no goto!() outside goto_block!
    goto!('nowhere);
}

fn main() {}
