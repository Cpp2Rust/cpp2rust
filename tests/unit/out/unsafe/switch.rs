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
    'switch: {
        let __match_cond = x;
        match __match_cond {
            v if v == 1 => {
                let mut y: i32 = 10;
                let mut z: i32 = 20;
                r = ((y) + (z));
                break 'switch;
            }
            v if v == 2 => {
                let mut y: i32 = 100;
                r = ((y) - (1));
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
    'switch: {
        let __match_cond = x;
        match __match_cond {
            v if v == 1 || v == 2 || v == 3 => {
                let mut y: i32 = ((x) * (2));
                r = ((y) + (1));
                break 'switch;
            }
            _ => {
                r = 0;
                break 'switch;
            }
        }
    };
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
pub unsafe fn switch_complex_cond_19(mut p: *mut i32, mut bias: i32) -> i32 {
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
pub unsafe fn switch_in_dowhile_20(mut n: i32) -> i32 {
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
pub unsafe fn continue_inside_switch_21(mut n: i32) -> i32 {
    let mut r: i32 = 0;
    let mut i: i32 = 0;
    'loop_: while ((i) < (n)) {
        'switch: {
            let __match_cond = i;
            match __match_cond {
                v if v == 0 || v == 2 || v == 4 => {
                    i.prefix_inc();
                    continue 'loop_;
                }
                _ => {
                    r += i;
                    break 'switch;
                }
            }
        };
        r += 1000;
        i.prefix_inc();
    }
    return r;
}
pub unsafe fn for_in_switch_break_22(mut n: i32) -> i32 {
    let mut r: i32 = 0;
    'switch: {
        let __match_cond = n;
        match __match_cond {
            v if v == 0 => {
                let mut i: i32 = 0;
                'loop_: while ((i) < (10)) {
                    if ((i) == (3)) {
                        break;
                    }
                    r += i;
                    i.prefix_inc();
                }
                r += 100;
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
pub unsafe fn for_in_switch_continue_23(mut n: i32) -> i32 {
    let mut r: i32 = 0;
    'switch: {
        let __match_cond = n;
        match __match_cond {
            v if v == 0 => {
                let mut i: i32 = 0;
                'loop_: while ((i) < (5)) {
                    if (((i) % (2)) == (0)) {
                        i.prefix_inc();
                        continue 'loop_;
                    }
                    r += i;
                    i.prefix_inc();
                }
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
pub unsafe fn while_in_switch_break_24(mut n: i32) -> i32 {
    let mut r: i32 = 0;
    'switch: {
        let __match_cond = n;
        match __match_cond {
            v if v == 0 => {
                let mut i: i32 = 0;
                'loop_: while ((i) < (10)) {
                    if ((i) == (4)) {
                        break;
                    }
                    r += i;
                    i.prefix_inc();
                }
                r += 1000;
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
pub unsafe fn for_switch_for_break_25(mut n: i32) -> i32 {
    let mut r: i32 = 0;
    let mut i: i32 = 0;
    'loop_: while ((i) < (n)) {
        'switch: {
            let __match_cond = i;
            match __match_cond {
                v if v == 1 => {
                    let mut j: i32 = 0;
                    'loop_: while ((j) < (10)) {
                        if ((j) == (2)) {
                            break;
                        }
                        r += 1;
                        j.prefix_inc();
                    }
                    r += 100;
                    break 'switch;
                }
                _ => {
                    r += 10;
                    break 'switch;
                }
            }
        };
        i.prefix_inc();
    }
    return r;
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
    let mut p_val: i32 = 5;
    assert!(
        ((unsafe {
            let _p: *mut i32 = (&mut p_val as *mut i32);
            let _bias: i32 = 0;
            switch_complex_cond_19(_p, _bias)
        }) == (2))
    );
    assert!(
        ((unsafe {
            let _p: *mut i32 = (&mut p_val as *mut i32);
            let _bias: i32 = 5;
            switch_complex_cond_19(_p, _bias)
        }) == (3))
    );
    assert!(
        ((unsafe {
            let _p: *mut i32 = (&mut p_val as *mut i32);
            let _bias: i32 = -5_i32;
            switch_complex_cond_19(_p, _bias)
        }) == (1))
    );
    assert!(
        ((unsafe {
            let _p: *mut i32 = (&mut p_val as *mut i32);
            let _bias: i32 = 99;
            switch_complex_cond_19(_p, _bias)
        }) == (0))
    );
    assert!(
        ((unsafe {
            let _n: i32 = 1;
            switch_in_dowhile_20(_n)
        }) == (1))
    );
    assert!(
        ((unsafe {
            let _n: i32 = 3;
            switch_in_dowhile_20(_n)
        }) == (((1) + (10)) + (100)))
    );
    assert!(
        ((unsafe {
            let _n: i32 = 6;
            continue_inside_switch_21(_n)
        }) == (((((1) + (3)) + (5)) as i32) + ((3) * (1000))))
    );
    assert!(
        ((unsafe {
            let _n: i32 = 0;
            for_in_switch_break_22(_n)
        }) == (103))
    );
    assert!(
        ((unsafe {
            let _n: i32 = 99;
            for_in_switch_break_22(_n)
        }) == (-1_i32))
    );
    assert!(
        ((unsafe {
            let _n: i32 = 0;
            for_in_switch_continue_23(_n)
        }) == (4))
    );
    assert!(
        ((unsafe {
            let _n: i32 = 99;
            for_in_switch_continue_23(_n)
        }) == (-1_i32))
    );
    assert!(
        ((unsafe {
            let _n: i32 = 0;
            while_in_switch_break_24(_n)
        }) == (1006))
    );
    assert!(
        ((unsafe {
            let _n: i32 = 99;
            while_in_switch_break_24(_n)
        }) == (-1_i32))
    );
    assert!(
        ((unsafe {
            let _n: i32 = 3;
            for_switch_for_break_25(_n)
        }) == (122))
    );
    return 0;
}
