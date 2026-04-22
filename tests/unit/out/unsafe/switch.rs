extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn basic_0(mut x: i32) -> i32 {
    let mut r: i32 = 0;
    'switch: {
        let __match_cond = x;
        match __match_cond {
            v if v == 0 => {
                r = 10;
                break 'switch;
            }
            v if v == 1 => {
                r = 20;
                break 'switch;
            }
            v if v == 2 => {
                r = 30;
                break 'switch;
            }
            _ => {
                r = 40;
                break 'switch;
            }
        }
    };
    return r;
}
pub unsafe fn stacked_1(mut x: i32) -> i32 {
    let mut r: i32 = 0;
    'switch: {
        let __match_cond = x;
        match __match_cond {
            v if v == 1 || v == 2 || v == 3 => {
                r = 100;
                break 'switch;
            }
            v if v == 4 || v == 5 => {
                r = 200;
                break 'switch;
            }
            _ => {
                r = 300;
                break 'switch;
            }
        }
    };
    return r;
}
pub unsafe fn no_default_2(mut x: i32) -> i32 {
    let mut r: i32 = -1_i32;
    'switch: {
        let __match_cond = x;
        match __match_cond {
            v if v == 7 => {
                r = 1;
                break 'switch;
            }
            v if v == 8 => {
                r = 2;
                break 'switch;
            }
            _ => {}
        }
    };
    return r;
}
pub unsafe fn only_default_3(mut x: i32) -> i32 {
    let mut r: i32 = 0;
    'switch: {
        let __match_cond = x;
        match __match_cond {
            _ => {
                r = 42;
                break 'switch;
            }
        }
    };
    return r;
}
pub unsafe fn default_middle_4(mut x: i32) -> i32 {
    let mut r: i32 = 0;
    'switch: {
        let __match_cond = x;
        match __match_cond {
            v if v == 1 => {
                r = 1;
                break 'switch;
            }
            v if v == 2 => {
                r = 2;
                break 'switch;
            }
            _ => {
                r = 99;
                break 'switch;
            }
        }
    };
    return r;
}
pub unsafe fn default_first_5(mut x: i32) -> i32 {
    let mut r: i32 = 0;
    'switch: {
        let __match_cond = x;
        match __match_cond {
            v if v == 1 => {
                r = 1;
                break 'switch;
            }
            v if v == 2 => {
                r = 2;
                break 'switch;
            }
            _ => {
                r = 7;
                break 'switch;
            }
        }
    };
    return r;
}
pub unsafe fn empty_switch_6(mut x: i32) -> i32 {
    'switch: {
        let __match_cond = x;
        match __match_cond {
            _ => {}
        }
    };
    return x;
}
pub unsafe fn switch_char_7(mut c: u8) -> i32 {
    'switch: {
        let __match_cond = (c as i32);
        match __match_cond {
            v if v == (('a' as u8) as i32) => {
                return 1;
            }
            v if v == (('b' as u8) as i32) => {
                return 2;
            }
            v if v == (('\n' as u8) as i32) => {
                return 3;
            }
            v if v == (('\0' as u8) as i32) => {
                return 4;
            }
            _ => {
                return 0;
            }
        }
    };
    panic!("ub: non-void function does not return a value")
}
#[derive(Clone, Copy, PartialEq, Debug, Default)]
enum Color {
    #[default]
    kRed = 0,
    kGreen = 1,
    kBlue = 2,
}
pub unsafe fn switch_enum_8(mut c: Color) -> i32 {
    'switch: {
        let __match_cond = (c as i32);
        match __match_cond {
            v if v == (Color::kRed as i32) => {
                return 10;
            }
            v if v == (Color::kGreen as i32) => {
                return 20;
            }
            v if v == (Color::kBlue as i32) => {
                return 30;
            }
            _ => {}
        }
    };
    return -1_i32;
}
pub unsafe fn compound_case_body_9(mut x: i32) -> i32 {
    let mut r: i32 = 0;
    switch!(match x {
        v if v == 1 => {
            let mut y: i32 = 10;
            let mut z: i32 = 20;
            r = ((y) + (z));
            break;
        }
        v if v == 2 => {
            let mut y: i32 = 100;
            r = ((y) - (1));
            break;
        }
        _ => {
            r = -1_i32;
            break;
        }
    });
    return r;
}
pub unsafe fn nested_10(mut a: i32, mut b: i32) -> i32 {
    let mut r: i32 = 0;
    'switch: {
        let __match_cond = a;
        match __match_cond {
            v if v == 1 => {
                'switch: {
                    let __match_cond = b;
                    match __match_cond {
                        v if v == 10 => {
                            r = 11;
                            break 'switch;
                        }
                        v if v == 20 => {
                            r = 12;
                            break 'switch;
                        }
                        _ => {
                            r = 13;
                            break 'switch;
                        }
                    }
                };
                r += 1;
                break 'switch;
            }
            v if v == 2 => {
                r = 2;
                break 'switch;
            }
            _ => {
                r = -1_i32;
                break 'switch;
            }
        }
    };
    return r;
}
pub unsafe fn switch_in_loop_11(mut n: i32) -> i32 {
    let mut r: i32 = 0;
    let mut i: i32 = 0;
    'loop_: while ((i) < (n)) {
        'switch: {
            let __match_cond = ((i) % (3));
            match __match_cond {
                v if v == 0 => {
                    r += 1;
                    break 'switch;
                }
                v if v == 1 => {
                    r += 2;
                    break 'switch;
                }
                _ => {
                    r += 3;
                    break 'switch;
                }
            }
        };
        r += 10;
        i.prefix_inc();
    }
    return r;
}
pub unsafe fn stacked_block_12(mut x: i32) -> i32 {
    let mut r: i32 = 0;
    switch!(match x {
        v if v == 1 || v == 2 || v == 3 => {
            let mut y: i32 = ((x) * (2));
            r = ((y) + (1));
            break;
        }
        _ => {
            r = 0;
            break;
        }
    });
    return r;
}
pub unsafe fn double_it_13(mut v: i32) -> i32 {
    return ((v) * (2));
}
pub unsafe fn switch_on_call_14(mut x: i32) -> i32 {
    'switch: {
        let __match_cond = (unsafe {
            let _v: i32 = x;
            double_it_13(_v)
        });
        match __match_cond {
            v if v == 0 => {
                return 100;
            }
            v if v == 2 => {
                return 200;
            }
            v if v == 4 => {
                return 400;
            }
            _ => {
                return -1_i32;
            }
        }
    };
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn switch_on_assignment_15(mut x: i32) -> i32 {
    let mut y: i32 = 0;
    let mut r: i32 = 0;
    'switch: {
        let __match_cond = {
            y = ((x) + (1));
            y
        };
        match __match_cond {
            v if v == 1 => {
                r = 10;
                break 'switch;
            }
            v if v == 2 => {
                r = 20;
                break 'switch;
            }
            _ => {
                r = y;
                break 'switch;
            }
        }
    };
    return r;
}
pub unsafe fn mixed_literal_cases_16(mut x: i32) -> i32 {
    'switch: {
        let __match_cond = x;
        match __match_cond {
            v if v == -1_i32 => {
                return 1;
            }
            v if v == 16 => {
                return 2;
            }
            v if v == 65152 => {
                return 3;
            }
            v if v == -255_i32 => {
                return 4;
            }
            _ => {
                return 0;
            }
        }
    };
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn mixed_return_break_17(mut x: i32) -> i32 {
    let mut r: i32 = -1_i32;
    'switch: {
        let __match_cond = x;
        match __match_cond {
            v if v == 0 => {
                return 100;
            }
            v if v == 1 => {
                r = 10;
                break 'switch;
            }
            v if v == 2 => {
                return 200;
            }
            _ => {
                r = 99;
                break 'switch;
            }
        }
    };
    return r;
}
pub unsafe fn empty_case_with_break_18(mut x: i32) -> i32 {
    let mut r: i32 = 5;
    'switch: {
        let __match_cond = x;
        match __match_cond {
            v if v == 1 => {
                break 'switch;
            }
            v if v == 2 => {
                r = 2;
                break 'switch;
            }
            _ => {
                r = 9;
                break 'switch;
            }
        }
    };
    return r;
}
pub unsafe fn fallthrough_one_19(mut x: i32) -> i32 {
    let mut r: i32 = 0;
    switch!(match x {
        v if v == 1 => {
            r += 10;
        }
        v if v == 2 => {
            r += 20;
            break;
        }
        _ => {
            r = -1_i32;
            break;
        }
    });
    return r;
}
pub unsafe fn fallthrough_chain_20(mut x: i32) -> i32 {
    let mut r: i32 = 0;
    switch!(match x {
        v if v == 1 => {
            r += 1;
        }
        v if v == 2 => {
            r += 2;
        }
        v if v == 3 => {
            r += 4;
        }
        v if v == 4 => {
            r += 8;
            break;
        }
        _ => {
            r = -1_i32;
            break;
        }
    });
    return r;
}
pub unsafe fn fallthrough_default_21(mut x: i32, mut flag: i32) -> i32 {
    let mut r: i32 = 0;
    switch!(match x {
        v if v == 7 => {
            if (flag != 0) {
                r = 100;
                break;
            };
        }
        _ => {
            r = 42;
            break;
        }
    });
    return r;
}
pub unsafe fn fallthrough_into_block_22(mut x: i32) -> i32 {
    let mut r: i32 = 0;
    switch!(match x {
        v if v == 1 => {
            r += 1;
        }
        v if v == 2 => {
            let mut tmp: i32 = ((r) * (10));
            r = ((tmp) + (5));
            break;
        }
        _ => {
            r = -1_i32;
            break;
        }
    });
    return r;
}
pub unsafe fn switch_complex_cond_23(mut p: *mut i32, mut bias: i32) -> i32 {
    'switch: {
        let __match_cond = ((*p) + (bias));
        match __match_cond {
            v if v == 0 => {
                return 1;
            }
            v if v == 5 => {
                return 2;
            }
            v if v == 10 => {
                return 3;
            }
            _ => {
                return 0;
            }
        }
    };
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn switch_in_dowhile_24(mut n: i32) -> i32 {
    let mut r: i32 = 0;
    let mut i: i32 = 0;
    'loop_: loop {
        'switch: {
            let __match_cond = i;
            match __match_cond {
                v if v == 0 => {
                    r += 1;
                    break 'switch;
                }
                v if v == 1 => {
                    r += 10;
                    break 'switch;
                }
                _ => {
                    r += 100;
                    break 'switch;
                }
            }
        };
        i.prefix_inc();
        if !((i) < (n)) {
            break;
        }
    }
    return r;
}
pub unsafe fn continue_inside_switch_25(mut n: i32) -> i32 {
    let mut r: i32 = 0;
    let mut i: i32 = 0;
    'loop_: while ((i) < (n)) {
        switch!(match i {
            v if v == 0 || v == 2 || v == 4 => {
                i.prefix_inc();
                continue 'loop_;
            }
            _ => {
                r += i;
                break;
            }
        });
        r += 1000;
        i.prefix_inc();
    }
    return r;
}
pub unsafe fn case_then_default_26(mut x: i32) -> i32 {
    let mut r: i32 = 0;
    'switch: {
        let __match_cond = x;
        match __match_cond {
            v if v == 2 => {
                r = 20;
                break 'switch;
            }
            _ => {
                r = 10;
                break 'switch;
            }
        }
    };
    return r;
}
pub unsafe fn default_then_case_27(mut x: i32) -> i32 {
    let mut r: i32 = 0;
    'switch: {
        let __match_cond = x;
        match __match_cond {
            v if v == 1 => {
                r = 1;
                break 'switch;
            }
            v if v == 3 => {
                r = 3;
                break 'switch;
            }
            _ => {
                r = 77;
                break 'switch;
            }
        }
    };
    return r;
}
pub unsafe fn cases_and_default_stacked_28(mut x: i32) -> i32 {
    let mut r: i32 = 0;
    'switch: {
        let __match_cond = x;
        match __match_cond {
            v if v == 3 => {
                r = 3;
                break 'switch;
            }
            _ => {
                r = 42;
                break 'switch;
            }
        }
    };
    return r;
}
pub unsafe fn stacked_with_inner_fallthrough_29(mut x: i32, mut flag: i32) -> i32 {
    let mut r: i32 = 0;
    switch!(match x {
        v if v == 1 || v == 2 || v == 3 => {
            if !(flag != 0) {
                r = 50;
                break;
            };
        }
        _ => {
            r = 999;
            break;
        }
    });
    return r;
}
pub unsafe fn borrow_in_condition_and_in_body_30(mut x: i32) -> i32 {
    switch!(match x {
        v if v == 0 => {}
        _ => {
            return ((x) + (1));
        }
    });
    panic!("ub: non-void function does not return a value")
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(
        ((unsafe {
            let _x: i32 = 0;
            basic_0(_x)
        }) == (10))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 2;
            basic_0(_x)
        }) == (30))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 99;
            basic_0(_x)
        }) == (40))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 1;
            stacked_1(_x)
        }) == (100))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 3;
            stacked_1(_x)
        }) == (100))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 5;
            stacked_1(_x)
        }) == (200))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 9;
            stacked_1(_x)
        }) == (300))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 7;
            no_default_2(_x)
        }) == (1))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 42;
            no_default_2(_x)
        }) == (-1_i32))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 1;
            only_default_3(_x)
        }) == (42))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 1;
            default_middle_4(_x)
        }) == (1))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 2;
            default_middle_4(_x)
        }) == (2))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 99;
            default_middle_4(_x)
        }) == (99))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 1;
            default_first_5(_x)
        }) == (1))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 99;
            default_first_5(_x)
        }) == (7))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 5;
            empty_switch_6(_x)
        }) == (5))
    );
    assert!(
        ((unsafe {
            let _c: u8 = ('a' as u8);
            switch_char_7(_c)
        }) == (1))
    );
    assert!(
        ((unsafe {
            let _c: u8 = ('b' as u8);
            switch_char_7(_c)
        }) == (2))
    );
    assert!(
        ((unsafe {
            let _c: u8 = ('\n' as u8);
            switch_char_7(_c)
        }) == (3))
    );
    assert!(
        ((unsafe {
            let _c: u8 = ('\0' as u8);
            switch_char_7(_c)
        }) == (4))
    );
    assert!(
        ((unsafe {
            let _c: u8 = ('z' as u8);
            switch_char_7(_c)
        }) == (0))
    );
    assert!(
        ((unsafe {
            let _c: Color = Color::kRed;
            switch_enum_8(_c)
        }) == (10))
    );
    assert!(
        ((unsafe {
            let _c: Color = Color::kGreen;
            switch_enum_8(_c)
        }) == (20))
    );
    assert!(
        ((unsafe {
            let _c: Color = Color::kBlue;
            switch_enum_8(_c)
        }) == (30))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 1;
            compound_case_body_9(_x)
        }) == (30))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 2;
            compound_case_body_9(_x)
        }) == (99))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 9;
            compound_case_body_9(_x)
        }) == (-1_i32))
    );
    assert!(
        ((unsafe {
            let _a: i32 = 1;
            let _b: i32 = 10;
            nested_10(_a, _b)
        }) == (12))
    );
    assert!(
        ((unsafe {
            let _a: i32 = 1;
            let _b: i32 = 99;
            nested_10(_a, _b)
        }) == (14))
    );
    assert!(
        ((unsafe {
            let _a: i32 = 2;
            let _b: i32 = 0;
            nested_10(_a, _b)
        }) == (2))
    );
    assert!(
        ((unsafe {
            let _a: i32 = 3;
            let _b: i32 = 3;
            nested_10(_a, _b)
        }) == (-1_i32))
    );
    assert!(
        ((unsafe {
            let _n: i32 = 6;
            switch_in_loop_11(_n)
        }) == (72))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 2;
            stacked_block_12(_x)
        }) == (5))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 9;
            stacked_block_12(_x)
        }) == (0))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 0;
            switch_on_call_14(_x)
        }) == (100))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 1;
            switch_on_call_14(_x)
        }) == (200))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 2;
            switch_on_call_14(_x)
        }) == (400))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 99;
            switch_on_call_14(_x)
        }) == (-1_i32))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 0;
            switch_on_assignment_15(_x)
        }) == (10))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 1;
            switch_on_assignment_15(_x)
        }) == (20))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 9;
            switch_on_assignment_15(_x)
        }) == (10))
    );
    assert!(
        ((unsafe {
            let _x: i32 = -1_i32;
            mixed_literal_cases_16(_x)
        }) == (1))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 16;
            mixed_literal_cases_16(_x)
        }) == (2))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 65152;
            mixed_literal_cases_16(_x)
        }) == (3))
    );
    assert!(
        ((unsafe {
            let _x: i32 = -255_i32;
            mixed_literal_cases_16(_x)
        }) == (4))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 7;
            mixed_literal_cases_16(_x)
        }) == (0))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 0;
            mixed_return_break_17(_x)
        }) == (100))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 1;
            mixed_return_break_17(_x)
        }) == (10))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 2;
            mixed_return_break_17(_x)
        }) == (200))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 99;
            mixed_return_break_17(_x)
        }) == (99))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 1;
            empty_case_with_break_18(_x)
        }) == (5))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 2;
            empty_case_with_break_18(_x)
        }) == (2))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 9;
            empty_case_with_break_18(_x)
        }) == (9))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 1;
            fallthrough_one_19(_x)
        }) == (30))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 2;
            fallthrough_one_19(_x)
        }) == (20))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 99;
            fallthrough_one_19(_x)
        }) == (-1_i32))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 1;
            fallthrough_chain_20(_x)
        }) == (15))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 2;
            fallthrough_chain_20(_x)
        }) == (14))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 3;
            fallthrough_chain_20(_x)
        }) == (12))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 4;
            fallthrough_chain_20(_x)
        }) == (8))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 99;
            fallthrough_chain_20(_x)
        }) == (-1_i32))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 7;
            let _flag: i32 = 0;
            fallthrough_default_21(_x, _flag)
        }) == (42))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 7;
            let _flag: i32 = 1;
            fallthrough_default_21(_x, _flag)
        }) == (100))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 99;
            let _flag: i32 = 0;
            fallthrough_default_21(_x, _flag)
        }) == (42))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 1;
            fallthrough_into_block_22(_x)
        }) == (15))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 2;
            fallthrough_into_block_22(_x)
        }) == (5))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 99;
            fallthrough_into_block_22(_x)
        }) == (-1_i32))
    );
    let mut p_val: i32 = 5;
    assert!(
        ((unsafe {
            let _p: *mut i32 = (&mut p_val as *mut i32);
            let _bias: i32 = 0;
            switch_complex_cond_23(_p, _bias)
        }) == (2))
    );
    assert!(
        ((unsafe {
            let _p: *mut i32 = (&mut p_val as *mut i32);
            let _bias: i32 = 5;
            switch_complex_cond_23(_p, _bias)
        }) == (3))
    );
    assert!(
        ((unsafe {
            let _p: *mut i32 = (&mut p_val as *mut i32);
            let _bias: i32 = -5_i32;
            switch_complex_cond_23(_p, _bias)
        }) == (1))
    );
    assert!(
        ((unsafe {
            let _p: *mut i32 = (&mut p_val as *mut i32);
            let _bias: i32 = 99;
            switch_complex_cond_23(_p, _bias)
        }) == (0))
    );
    assert!(
        ((unsafe {
            let _n: i32 = 1;
            switch_in_dowhile_24(_n)
        }) == (1))
    );
    assert!(
        ((unsafe {
            let _n: i32 = 3;
            switch_in_dowhile_24(_n)
        }) == (((1) + (10)) + (100)))
    );
    assert!(
        ((unsafe {
            let _n: i32 = 6;
            continue_inside_switch_25(_n)
        }) == (((((1) + (3)) + (5)) as i32) + ((3) * (1000))))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 1;
            case_then_default_26(_x)
        }) == (10))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 2;
            case_then_default_26(_x)
        }) == (20))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 99;
            case_then_default_26(_x)
        }) == (10))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 1;
            default_then_case_27(_x)
        }) == (1))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 2;
            default_then_case_27(_x)
        }) == (77))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 3;
            default_then_case_27(_x)
        }) == (3))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 99;
            default_then_case_27(_x)
        }) == (77))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 1;
            cases_and_default_stacked_28(_x)
        }) == (42))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 2;
            cases_and_default_stacked_28(_x)
        }) == (42))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 3;
            cases_and_default_stacked_28(_x)
        }) == (3))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 99;
            cases_and_default_stacked_28(_x)
        }) == (42))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 1;
            let _flag: i32 = 0;
            stacked_with_inner_fallthrough_29(_x, _flag)
        }) == (50))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 2;
            let _flag: i32 = 1;
            stacked_with_inner_fallthrough_29(_x, _flag)
        }) == (999))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 99;
            let _flag: i32 = 0;
            stacked_with_inner_fallthrough_29(_x, _flag)
        }) == (999))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 0;
            borrow_in_condition_and_in_body_30(_x)
        }) == (1))
    );
    assert!(
        ((unsafe {
            let _x: i32 = 1;
            borrow_in_condition_and_in_body_30(_x)
        }) == (2))
    );
    return 0;
}
