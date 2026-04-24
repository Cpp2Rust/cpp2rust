// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs_macros::{goto_block, switch};

#[test]
fn switch_dispatch_each_case() {
    for (input, expected) in [(0, "zero"), (1, "one"), (2, "two"), (9, "default")] {
        let mut out = "";
        switch!(match input {
            0 => {
                out = "zero";
                break;
            }
            1 => {
                out = "one";
                break;
            }
            2 => {
                out = "two";
                break;
            }
            _ => {
                out = "default";
                break;
            }
        });
        assert_eq!(out, expected, "input = {}", input);
    }
}

#[test]
fn switch_fallthrough() {
    let mut v = 0;
    switch!(match 0 {
        0 => {
            v += 1;
        } // fall-through
        1 => {
            v += 10;
            break;
        }
        _ => {
            v = -1;
            break;
        }
    });
    assert_eq!(v, 11);
}

#[test]
fn switch_fallthrough_chain() {
    let mut v = 0;
    switch!(match 0 {
        0 => {
            v += 1;
        }
        1 => {
            v += 10;
        }
        2 => {
            v += 100;
            break;
        }
        _ => {
            v = -1;
            break;
        }
    });
    assert_eq!(v, 111);
}

#[test]
fn switch_break_exits() {
    let mut v = 0;
    switch!(match 1 {
        0 => {
            v = 999;
            break;
        }
        1 => {
            v = 1;
            break;
        }
        _ => {
            v = 2;
            break;
        }
    });
    assert_eq!(v, 1);
}

#[test]
fn switch_default_case() {
    let mut v = 0;
    switch!(match 42 {
        0 => {
            v = 1;
            break;
        }
        _ => {
            v = 2;
            break;
        }
    });
    assert_eq!(v, 2);
}

#[test]
fn switch_or_pattern() {
    for x in [1, 2, 5] {
        let mut v = "other";
        switch!(match x {
            1 | 2 | 3 => {
                v = "low";
                break;
            }
            _ => {
                v = "other";
                break;
            }
        });
        let expected = if x <= 3 { "low" } else { "other" };
        assert_eq!(v, expected);
    }
}

#[test]
fn switch_guard_stacked_cases() {
    for (x, expected) in [(1, 100), (2, 100), (3, 100), (4, 200), (5, 200), (9, 300)] {
        let mut r = 0;
        switch!(match x {
            v if v == 1 || v == 2 || v == 3 => {
                r = 100;
                break;
            }
            v if v == 4 || v == 5 => {
                r = 200;
                break;
            }
            _ => {
                r = 300;
                break;
            }
        });
        assert_eq!(r, expected, "x = {}", x);
    }
}

#[test]
fn switch_guard_fallthrough() {
    let mut v = 0;
    switch!(match 1 {
        w if w == 0 || w == 1 => {
            v += 1;
        }
        w if w == 2 => {
            v += 10;
            break;
        }
        _ => {
            v = -1;
            break;
        }
    });
    assert_eq!(v, 11);
}

#[test]
fn switch_guard_no_match_hits_default() {
    let mut r = 0;
    switch!(match 42 {
        v if v == 1 => {
            r = 1;
            break;
        }
        v if v == 2 || v == 3 => {
            r = 2;
            break;
        }
        _ => {
            r = 99;
            break;
        }
    });
    assert_eq!(r, 99);
}

#[test]
fn switch_range_pattern() {
    let mut v = 0;
    switch!(match 50i32 {
        0..=9 => {
            v = 1;
            break;
        }
        10..=99 => {
            v = 2;
            break;
        }
        _ => {
            v = 3;
            break;
        }
    });
    assert_eq!(v, 2);
}

#[test]
fn switch_nested_loop_break_targets_inner() {
    let mut sum = 0;
    switch!(match 0 {
        0 => {
            for i in 0..10 {
                if i == 3 {
                    break; // targets the for loop (LoopControlForbidden skips descent)
                }
                sum += i;
            }
            sum += 100;
            break;
        }
        _ => {
            break;
        }
    });
    assert_eq!(sum, 103);
}

#[test]
fn switch_return_from_case() {
    #[allow(unreachable_code)]
    fn classify(x: i32) -> &'static str {
        switch!(match x {
            0 => {
                return "zero";
            }
            1 => {
                return "one";
            }
            _ => {
                return "other";
            }
        });
        "unreachable"
    }
    assert_eq!(classify(0), "zero");
    assert_eq!(classify(1), "one");
    assert_eq!(classify(5), "other");
}

#[test]
fn switch_in_loop() {
    let mut count = 0;
    for x in 0..5 {
        switch!(match x {
            0 | 2 | 4 => {
                count += 1;
                break;
            }
            _ => {
                break;
            }
        });
    }
    assert_eq!(count, 3);
}

#[test]
fn goto_block_linear_fallthrough() {
    let mut v = 0;
    goto_block! {
        'a => { v += 1; },
        'b => { v += 10; },
        'c => { v += 100; },
    };
    assert_eq!(v, 111);
}

#[test]
fn goto_block_return_from_arm() {
    #[allow(unreachable_code)]
    fn run(start: u32) -> &'static str {
        goto_block! {
            'a => { if start == 0 { return "a"; } },
            'b => { if start == 1 { return "b"; } },
            'c => { return "c"; },
        };
        "fallthrough"
    }
    assert_eq!(run(0), "a");
    assert_eq!(run(1), "b");
    assert_eq!(run(2), "c");
}

#[test]
fn goto_block_nested_loop_break_targets_inner() {
    let mut sum = 0;
    goto_block! {
        'entry => {
            for i in 0..10 {
                if i == 3 { break; } // targets the for loop
                sum += i;
            }
            sum += 100;
        },
    };
    assert_eq!(sum, 103);
}
