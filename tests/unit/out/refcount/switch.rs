extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn basic_0(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    let r: Value<i32> = Rc::new(RefCell::new(0));
    'switch: {
        let __match_cond = (*x.borrow());
        match __match_cond {
            v if v == 0 => {
                (*r.borrow_mut()) = 10;
                break 'switch;
            }
            v if v == 1 => {
                (*r.borrow_mut()) = 20;
                break 'switch;
            }
            v if v == 2 => {
                (*r.borrow_mut()) = 30;
                break 'switch;
            }
            _ => {
                (*r.borrow_mut()) = 40;
                break 'switch;
            }
        }
    };
    return (*r.borrow());
}
pub fn stacked_1(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    let r: Value<i32> = Rc::new(RefCell::new(0));
    'switch: {
        let __match_cond = (*x.borrow());
        match __match_cond {
            v if v == 1 || v == 2 || v == 3 => {
                (*r.borrow_mut()) = 100;
                break 'switch;
            }
            v if v == 4 || v == 5 => {
                (*r.borrow_mut()) = 200;
                break 'switch;
            }
            _ => {
                (*r.borrow_mut()) = 300;
                break 'switch;
            }
        }
    };
    return (*r.borrow());
}
pub fn no_default_2(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    let r: Value<i32> = Rc::new(RefCell::new(-1_i32));
    'switch: {
        let __match_cond = (*x.borrow());
        match __match_cond {
            v if v == 7 => {
                (*r.borrow_mut()) = 1;
                break 'switch;
            }
            v if v == 8 => {
                (*r.borrow_mut()) = 2;
                break 'switch;
            }
            _ => {}
        }
    };
    return (*r.borrow());
}
pub fn only_default_3(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    let r: Value<i32> = Rc::new(RefCell::new(0));
    'switch: {
        let __match_cond = (*x.borrow());
        match __match_cond {
            _ => {
                (*r.borrow_mut()) = 42;
                break 'switch;
            }
        }
    };
    return (*r.borrow());
}
pub fn default_middle_4(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    let r: Value<i32> = Rc::new(RefCell::new(0));
    'switch: {
        let __match_cond = (*x.borrow());
        match __match_cond {
            v if v == 1 => {
                (*r.borrow_mut()) = 1;
                break 'switch;
            }
            v if v == 2 => {
                (*r.borrow_mut()) = 2;
                break 'switch;
            }
            _ => {
                (*r.borrow_mut()) = 99;
                break 'switch;
            }
        }
    };
    return (*r.borrow());
}
pub fn default_first_5(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    let r: Value<i32> = Rc::new(RefCell::new(0));
    'switch: {
        let __match_cond = (*x.borrow());
        match __match_cond {
            v if v == 1 => {
                (*r.borrow_mut()) = 1;
                break 'switch;
            }
            v if v == 2 => {
                (*r.borrow_mut()) = 2;
                break 'switch;
            }
            _ => {
                (*r.borrow_mut()) = 7;
                break 'switch;
            }
        }
    };
    return (*r.borrow());
}
pub fn empty_switch_6(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    'switch: {
        let __match_cond = (*x.borrow());
        match __match_cond {
            _ => {}
        }
    };
    return (*x.borrow());
}
pub fn switch_char_7(c: u8) -> i32 {
    let c: Value<u8> = Rc::new(RefCell::new(c));
    'switch: {
        let __match_cond = ((*c.borrow()) as i32);
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
pub fn switch_enum_8(c: Color) -> i32 {
    let c: Value<Color> = Rc::new(RefCell::new(c));
    'switch: {
        let __match_cond = ((*c.borrow()) as i32);
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
pub fn compound_case_body_9(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    let r: Value<i32> = Rc::new(RefCell::new(0));
    switch!(match (*x.borrow()) {
        v if v == 1 => {
            let y: Value<i32> = Rc::new(RefCell::new(10));
            let z: Value<i32> = Rc::new(RefCell::new(20));
            (*r.borrow_mut()) = ((*y.borrow()) + (*z.borrow()));
            break;
        }
        v if v == 2 => {
            let y: Value<i32> = Rc::new(RefCell::new(100));
            (*r.borrow_mut()) = ((*y.borrow()) - 1);
            break;
        }
        _ => {
            (*r.borrow_mut()) = -1_i32;
            break;
        }
    });
    return (*r.borrow());
}
pub fn nested_10(a: i32, b: i32) -> i32 {
    let a: Value<i32> = Rc::new(RefCell::new(a));
    let b: Value<i32> = Rc::new(RefCell::new(b));
    let r: Value<i32> = Rc::new(RefCell::new(0));
    'switch: {
        let __match_cond = (*a.borrow());
        match __match_cond {
            v if v == 1 => {
                'switch: {
                    let __match_cond = (*b.borrow());
                    match __match_cond {
                        v if v == 10 => {
                            (*r.borrow_mut()) = 11;
                            break 'switch;
                        }
                        v if v == 20 => {
                            (*r.borrow_mut()) = 12;
                            break 'switch;
                        }
                        _ => {
                            (*r.borrow_mut()) = 13;
                            break 'switch;
                        }
                    }
                };
                (*r.borrow_mut()) += 1;
                break 'switch;
            }
            v if v == 2 => {
                (*r.borrow_mut()) = 2;
                break 'switch;
            }
            _ => {
                (*r.borrow_mut()) = -1_i32;
                break 'switch;
            }
        }
    };
    return (*r.borrow());
}
pub fn switch_in_loop_11(n: i32) -> i32 {
    let n: Value<i32> = Rc::new(RefCell::new(n));
    let r: Value<i32> = Rc::new(RefCell::new(0));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < (*n.borrow())) {
        'switch: {
            let __match_cond = ((*i.borrow()) % 3);
            match __match_cond {
                v if v == 0 => {
                    (*r.borrow_mut()) += 1;
                    break 'switch;
                }
                v if v == 1 => {
                    (*r.borrow_mut()) += 2;
                    break 'switch;
                }
                _ => {
                    (*r.borrow_mut()) += 3;
                    break 'switch;
                }
            }
        };
        (*r.borrow_mut()) += 10;
        (*i.borrow_mut()).prefix_inc();
    }
    return (*r.borrow());
}
pub fn stacked_block_12(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    let r: Value<i32> = Rc::new(RefCell::new(0));
    switch!(match (*x.borrow()) {
        v if v == 1 || v == 2 || v == 3 => {
            let y: Value<i32> = Rc::new(RefCell::new(((*x.borrow()) * 2)));
            (*r.borrow_mut()) = ((*y.borrow()) + 1);
            break;
        }
        _ => {
            (*r.borrow_mut()) = 0;
            break;
        }
    });
    return (*r.borrow());
}
pub fn double_it_13(v: i32) -> i32 {
    let v: Value<i32> = Rc::new(RefCell::new(v));
    return ((*v.borrow()) * 2);
}
pub fn switch_on_call_14(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    'switch: {
        let __match_cond = ({
            let _v: i32 = (*x.borrow());
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
pub fn switch_on_assignment_15(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    let y: Value<i32> = Rc::new(RefCell::new(0));
    let r: Value<i32> = Rc::new(RefCell::new(0));
    'switch: {
        let __match_cond = {
            (*y.borrow_mut()) = ((*x.borrow()) + 1);
            (*y.borrow())
        };
        match __match_cond {
            v if v == 1 => {
                (*r.borrow_mut()) = 10;
                break 'switch;
            }
            v if v == 2 => {
                (*r.borrow_mut()) = 20;
                break 'switch;
            }
            _ => {
                (*r.borrow_mut()) = (*y.borrow());
                break 'switch;
            }
        }
    };
    return (*r.borrow());
}
pub fn mixed_literal_cases_16(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    'switch: {
        let __match_cond = (*x.borrow());
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
pub fn mixed_return_break_17(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    let r: Value<i32> = Rc::new(RefCell::new(-1_i32));
    'switch: {
        let __match_cond = (*x.borrow());
        match __match_cond {
            v if v == 0 => {
                return 100;
            }
            v if v == 1 => {
                (*r.borrow_mut()) = 10;
                break 'switch;
            }
            v if v == 2 => {
                return 200;
            }
            _ => {
                (*r.borrow_mut()) = 99;
                break 'switch;
            }
        }
    };
    return (*r.borrow());
}
pub fn empty_case_with_break_18(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    let r: Value<i32> = Rc::new(RefCell::new(5));
    'switch: {
        let __match_cond = (*x.borrow());
        match __match_cond {
            v if v == 1 => {
                break 'switch;
            }
            v if v == 2 => {
                (*r.borrow_mut()) = 2;
                break 'switch;
            }
            _ => {
                (*r.borrow_mut()) = 9;
                break 'switch;
            }
        }
    };
    return (*r.borrow());
}
pub fn fallthrough_one_19(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    let r: Value<i32> = Rc::new(RefCell::new(0));
    switch!(match (*x.borrow()) {
        v if v == 1 => {
            (*r.borrow_mut()) += 10;
        }
        v if v == 2 => {
            (*r.borrow_mut()) += 20;
            break;
        }
        _ => {
            (*r.borrow_mut()) = -1_i32;
            break;
        }
    });
    return (*r.borrow());
}
pub fn fallthrough_chain_20(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    let r: Value<i32> = Rc::new(RefCell::new(0));
    switch!(match (*x.borrow()) {
        v if v == 1 => {
            (*r.borrow_mut()) += 1;
        }
        v if v == 2 => {
            (*r.borrow_mut()) += 2;
        }
        v if v == 3 => {
            (*r.borrow_mut()) += 4;
        }
        v if v == 4 => {
            (*r.borrow_mut()) += 8;
            break;
        }
        _ => {
            (*r.borrow_mut()) = -1_i32;
            break;
        }
    });
    return (*r.borrow());
}
pub fn fallthrough_default_21(x: i32, flag: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    let flag: Value<i32> = Rc::new(RefCell::new(flag));
    let r: Value<i32> = Rc::new(RefCell::new(0));
    switch!(match (*x.borrow()) {
        v if v == 7 => {
            if ((*flag.borrow()) != 0) {
                (*r.borrow_mut()) = 100;
                break;
            };
        }
        _ => {
            (*r.borrow_mut()) = 42;
            break;
        }
    });
    return (*r.borrow());
}
pub fn fallthrough_into_block_22(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    let r: Value<i32> = Rc::new(RefCell::new(0));
    switch!(match (*x.borrow()) {
        v if v == 1 => {
            (*r.borrow_mut()) += 1;
        }
        v if v == 2 => {
            let tmp: Value<i32> = Rc::new(RefCell::new(((*r.borrow()) * 10)));
            (*r.borrow_mut()) = ((*tmp.borrow()) + 5);
            break;
        }
        _ => {
            (*r.borrow_mut()) = -1_i32;
            break;
        }
    });
    return (*r.borrow());
}
pub fn switch_complex_cond_23(p: Ptr<i32>, bias: i32) -> i32 {
    let p: Value<Ptr<i32>> = Rc::new(RefCell::new(p));
    let bias: Value<i32> = Rc::new(RefCell::new(bias));
    'switch: {
        let __match_cond = {
            let _lhs = ((*p.borrow()).read());
            _lhs + (*bias.borrow())
        };
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
pub fn switch_in_dowhile_24(n: i32) -> i32 {
    let n: Value<i32> = Rc::new(RefCell::new(n));
    let r: Value<i32> = Rc::new(RefCell::new(0));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: loop {
        'switch: {
            let __match_cond = (*i.borrow());
            match __match_cond {
                v if v == 0 => {
                    (*r.borrow_mut()) += 1;
                    break 'switch;
                }
                v if v == 1 => {
                    (*r.borrow_mut()) += 10;
                    break 'switch;
                }
                _ => {
                    (*r.borrow_mut()) += 100;
                    break 'switch;
                }
            }
        };
        (*i.borrow_mut()).prefix_inc();
        if !((*i.borrow()) < (*n.borrow())) {
            break;
        }
    }
    return (*r.borrow());
}
pub fn continue_inside_switch_25(n: i32) -> i32 {
    let n: Value<i32> = Rc::new(RefCell::new(n));
    let r: Value<i32> = Rc::new(RefCell::new(0));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < (*n.borrow())) {
        switch!(match (*i.borrow()) {
            v if v == 0 || v == 2 || v == 4 => {
                (*i.borrow_mut()).prefix_inc();
                continue 'loop_;
            }
            _ => {
                (*r.borrow_mut()) += (*i.borrow());
                break;
            }
        });
        (*r.borrow_mut()) += 1000;
        (*i.borrow_mut()).prefix_inc();
    }
    return (*r.borrow());
}
pub fn case_then_default_26(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    let r: Value<i32> = Rc::new(RefCell::new(0));
    'switch: {
        let __match_cond = (*x.borrow());
        match __match_cond {
            v if v == 2 => {
                (*r.borrow_mut()) = 20;
                break 'switch;
            }
            _ => {
                (*r.borrow_mut()) = 10;
                break 'switch;
            }
        }
    };
    return (*r.borrow());
}
pub fn default_then_case_27(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    let r: Value<i32> = Rc::new(RefCell::new(0));
    'switch: {
        let __match_cond = (*x.borrow());
        match __match_cond {
            v if v == 1 => {
                (*r.borrow_mut()) = 1;
                break 'switch;
            }
            v if v == 3 => {
                (*r.borrow_mut()) = 3;
                break 'switch;
            }
            _ => {
                (*r.borrow_mut()) = 77;
                break 'switch;
            }
        }
    };
    return (*r.borrow());
}
pub fn cases_and_default_stacked_28(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    let r: Value<i32> = Rc::new(RefCell::new(0));
    'switch: {
        let __match_cond = (*x.borrow());
        match __match_cond {
            v if v == 3 => {
                (*r.borrow_mut()) = 3;
                break 'switch;
            }
            _ => {
                (*r.borrow_mut()) = 42;
                break 'switch;
            }
        }
    };
    return (*r.borrow());
}
pub fn stacked_with_inner_fallthrough_29(x: i32, flag: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    let flag: Value<i32> = Rc::new(RefCell::new(flag));
    let r: Value<i32> = Rc::new(RefCell::new(0));
    switch!(match (*x.borrow()) {
        v if v == 1 || v == 2 || v == 3 => {
            if !((*flag.borrow()) != 0) {
                (*r.borrow_mut()) = 50;
                break;
            };
        }
        _ => {
            (*r.borrow_mut()) = 999;
            break;
        }
    });
    return (*r.borrow());
}
pub fn borrow_in_condition_and_in_body_30(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    switch!(match (*x.borrow()) {
        v if v == 0 => {}
        _ => {
            return ((*x.borrow()) + 1);
        }
    });
    panic!("ub: non-void function does not return a value")
}
pub fn for_in_switch_break_31(n: i32) -> i32 {
    let n: Value<i32> = Rc::new(RefCell::new(n));
    let r: Value<i32> = Rc::new(RefCell::new(0));
    'switch: {
        let __match_cond = (*n.borrow());
        match __match_cond {
            v if v == 0 => {
                let i: Value<i32> = Rc::new(RefCell::new(0));
                'loop_: while ((*i.borrow()) < 10) {
                    if ((*i.borrow()) == 3) {
                        break;
                    }
                    (*r.borrow_mut()) += (*i.borrow());
                    (*i.borrow_mut()).prefix_inc();
                }
                (*r.borrow_mut()) += 100;
                break 'switch;
            }
            _ => {
                (*r.borrow_mut()) = -1_i32;
                break 'switch;
            }
        }
    };
    return (*r.borrow());
}
pub fn for_in_switch_continue_32(n: i32) -> i32 {
    let n: Value<i32> = Rc::new(RefCell::new(n));
    let r: Value<i32> = Rc::new(RefCell::new(0));
    'switch: {
        let __match_cond = (*n.borrow());
        match __match_cond {
            v if v == 0 => {
                let i: Value<i32> = Rc::new(RefCell::new(0));
                'loop_: while ((*i.borrow()) < 5) {
                    if (((*i.borrow()) % 2) == 0) {
                        (*i.borrow_mut()).prefix_inc();
                        continue 'loop_;
                    }
                    (*r.borrow_mut()) += (*i.borrow());
                    (*i.borrow_mut()).prefix_inc();
                }
                break 'switch;
            }
            _ => {
                (*r.borrow_mut()) = -1_i32;
                break 'switch;
            }
        }
    };
    return (*r.borrow());
}
pub fn while_in_switch_break_33(n: i32) -> i32 {
    let n: Value<i32> = Rc::new(RefCell::new(n));
    let r: Value<i32> = Rc::new(RefCell::new(0));
    switch!(match (*n.borrow()) {
        v if v == 0 => {
            let i: Value<i32> = Rc::new(RefCell::new(0));
            'loop_: while ((*i.borrow()) < 10) {
                if ((*i.borrow()) == 4) {
                    break;
                }
                (*r.borrow_mut()) += (*i.borrow());
                (*i.borrow_mut()).prefix_inc();
            }
            (*r.borrow_mut()) += 1000;
            break;
        }
        _ => {
            (*r.borrow_mut()) = -1_i32;
            break;
        }
    });
    return (*r.borrow());
}
pub fn for_switch_for_break_34(n: i32) -> i32 {
    let n: Value<i32> = Rc::new(RefCell::new(n));
    let r: Value<i32> = Rc::new(RefCell::new(0));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < (*n.borrow())) {
        'switch: {
            let __match_cond = (*i.borrow());
            match __match_cond {
                v if v == 1 => {
                    let j: Value<i32> = Rc::new(RefCell::new(0));
                    'loop_: while ((*j.borrow()) < 10) {
                        if ((*j.borrow()) == 2) {
                            break;
                        }
                        (*r.borrow_mut()) += 1;
                        (*j.borrow_mut()).prefix_inc();
                    }
                    (*r.borrow_mut()) += 100;
                    break 'switch;
                }
                _ => {
                    (*r.borrow_mut()) += 10;
                    break 'switch;
                }
            }
        };
        (*i.borrow_mut()).prefix_inc();
    }
    return (*r.borrow());
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!(
        (({
            let _x: i32 = 0;
            basic_0(_x)
        }) == 10)
    );
    assert!(
        (({
            let _x: i32 = 2;
            basic_0(_x)
        }) == 30)
    );
    assert!(
        (({
            let _x: i32 = 99;
            basic_0(_x)
        }) == 40)
    );
    assert!(
        (({
            let _x: i32 = 1;
            stacked_1(_x)
        }) == 100)
    );
    assert!(
        (({
            let _x: i32 = 3;
            stacked_1(_x)
        }) == 100)
    );
    assert!(
        (({
            let _x: i32 = 5;
            stacked_1(_x)
        }) == 200)
    );
    assert!(
        (({
            let _x: i32 = 9;
            stacked_1(_x)
        }) == 300)
    );
    assert!(
        (({
            let _x: i32 = 7;
            no_default_2(_x)
        }) == 1)
    );
    assert!(
        (({
            let _x: i32 = 42;
            no_default_2(_x)
        }) == -1_i32)
    );
    assert!(
        (({
            let _x: i32 = 1;
            only_default_3(_x)
        }) == 42)
    );
    assert!(
        (({
            let _x: i32 = 1;
            default_middle_4(_x)
        }) == 1)
    );
    assert!(
        (({
            let _x: i32 = 2;
            default_middle_4(_x)
        }) == 2)
    );
    assert!(
        (({
            let _x: i32 = 99;
            default_middle_4(_x)
        }) == 99)
    );
    assert!(
        (({
            let _x: i32 = 1;
            default_first_5(_x)
        }) == 1)
    );
    assert!(
        (({
            let _x: i32 = 99;
            default_first_5(_x)
        }) == 7)
    );
    assert!(
        (({
            let _x: i32 = 5;
            empty_switch_6(_x)
        }) == 5)
    );
    assert!(
        (({
            let _c: u8 = ('a' as u8);
            switch_char_7(_c)
        }) == 1)
    );
    assert!(
        (({
            let _c: u8 = ('b' as u8);
            switch_char_7(_c)
        }) == 2)
    );
    assert!(
        (({
            let _c: u8 = ('\n' as u8);
            switch_char_7(_c)
        }) == 3)
    );
    assert!(
        (({
            let _c: u8 = ('\0' as u8);
            switch_char_7(_c)
        }) == 4)
    );
    assert!(
        (({
            let _c: u8 = ('z' as u8);
            switch_char_7(_c)
        }) == 0)
    );
    assert!(
        (({
            let _c: Color = Color::kRed;
            switch_enum_8(_c)
        }) == 10)
    );
    assert!(
        (({
            let _c: Color = Color::kGreen;
            switch_enum_8(_c)
        }) == 20)
    );
    assert!(
        (({
            let _c: Color = Color::kBlue;
            switch_enum_8(_c)
        }) == 30)
    );
    assert!(
        (({
            let _x: i32 = 1;
            compound_case_body_9(_x)
        }) == 30)
    );
    assert!(
        (({
            let _x: i32 = 2;
            compound_case_body_9(_x)
        }) == 99)
    );
    assert!(
        (({
            let _x: i32 = 9;
            compound_case_body_9(_x)
        }) == -1_i32)
    );
    assert!(
        (({
            let _a: i32 = 1;
            let _b: i32 = 10;
            nested_10(_a, _b)
        }) == 12)
    );
    assert!(
        (({
            let _a: i32 = 1;
            let _b: i32 = 99;
            nested_10(_a, _b)
        }) == 14)
    );
    assert!(
        (({
            let _a: i32 = 2;
            let _b: i32 = 0;
            nested_10(_a, _b)
        }) == 2)
    );
    assert!(
        (({
            let _a: i32 = 3;
            let _b: i32 = 3;
            nested_10(_a, _b)
        }) == -1_i32)
    );
    assert!(
        (({
            let _n: i32 = 6;
            switch_in_loop_11(_n)
        }) == 72)
    );
    assert!(
        (({
            let _x: i32 = 2;
            stacked_block_12(_x)
        }) == 5)
    );
    assert!(
        (({
            let _x: i32 = 9;
            stacked_block_12(_x)
        }) == 0)
    );
    assert!(
        (({
            let _x: i32 = 0;
            switch_on_call_14(_x)
        }) == 100)
    );
    assert!(
        (({
            let _x: i32 = 1;
            switch_on_call_14(_x)
        }) == 200)
    );
    assert!(
        (({
            let _x: i32 = 2;
            switch_on_call_14(_x)
        }) == 400)
    );
    assert!(
        (({
            let _x: i32 = 99;
            switch_on_call_14(_x)
        }) == -1_i32)
    );
    assert!(
        (({
            let _x: i32 = 0;
            switch_on_assignment_15(_x)
        }) == 10)
    );
    assert!(
        (({
            let _x: i32 = 1;
            switch_on_assignment_15(_x)
        }) == 20)
    );
    assert!(
        (({
            let _x: i32 = 9;
            switch_on_assignment_15(_x)
        }) == 10)
    );
    assert!(
        (({
            let _x: i32 = -1_i32;
            mixed_literal_cases_16(_x)
        }) == 1)
    );
    assert!(
        (({
            let _x: i32 = 16;
            mixed_literal_cases_16(_x)
        }) == 2)
    );
    assert!(
        (({
            let _x: i32 = 65152;
            mixed_literal_cases_16(_x)
        }) == 3)
    );
    assert!(
        (({
            let _x: i32 = -255_i32;
            mixed_literal_cases_16(_x)
        }) == 4)
    );
    assert!(
        (({
            let _x: i32 = 7;
            mixed_literal_cases_16(_x)
        }) == 0)
    );
    assert!(
        (({
            let _x: i32 = 0;
            mixed_return_break_17(_x)
        }) == 100)
    );
    assert!(
        (({
            let _x: i32 = 1;
            mixed_return_break_17(_x)
        }) == 10)
    );
    assert!(
        (({
            let _x: i32 = 2;
            mixed_return_break_17(_x)
        }) == 200)
    );
    assert!(
        (({
            let _x: i32 = 99;
            mixed_return_break_17(_x)
        }) == 99)
    );
    assert!(
        (({
            let _x: i32 = 1;
            empty_case_with_break_18(_x)
        }) == 5)
    );
    assert!(
        (({
            let _x: i32 = 2;
            empty_case_with_break_18(_x)
        }) == 2)
    );
    assert!(
        (({
            let _x: i32 = 9;
            empty_case_with_break_18(_x)
        }) == 9)
    );
    assert!(
        (({
            let _x: i32 = 1;
            fallthrough_one_19(_x)
        }) == 30)
    );
    assert!(
        (({
            let _x: i32 = 2;
            fallthrough_one_19(_x)
        }) == 20)
    );
    assert!(
        (({
            let _x: i32 = 99;
            fallthrough_one_19(_x)
        }) == -1_i32)
    );
    assert!(
        (({
            let _x: i32 = 1;
            fallthrough_chain_20(_x)
        }) == 15)
    );
    assert!(
        (({
            let _x: i32 = 2;
            fallthrough_chain_20(_x)
        }) == 14)
    );
    assert!(
        (({
            let _x: i32 = 3;
            fallthrough_chain_20(_x)
        }) == 12)
    );
    assert!(
        (({
            let _x: i32 = 4;
            fallthrough_chain_20(_x)
        }) == 8)
    );
    assert!(
        (({
            let _x: i32 = 99;
            fallthrough_chain_20(_x)
        }) == -1_i32)
    );
    assert!(
        (({
            let _x: i32 = 7;
            let _flag: i32 = 0;
            fallthrough_default_21(_x, _flag)
        }) == 42)
    );
    assert!(
        (({
            let _x: i32 = 7;
            let _flag: i32 = 1;
            fallthrough_default_21(_x, _flag)
        }) == 100)
    );
    assert!(
        (({
            let _x: i32 = 99;
            let _flag: i32 = 0;
            fallthrough_default_21(_x, _flag)
        }) == 42)
    );
    assert!(
        (({
            let _x: i32 = 1;
            fallthrough_into_block_22(_x)
        }) == 15)
    );
    assert!(
        (({
            let _x: i32 = 2;
            fallthrough_into_block_22(_x)
        }) == 5)
    );
    assert!(
        (({
            let _x: i32 = 99;
            fallthrough_into_block_22(_x)
        }) == -1_i32)
    );
    let p_val: Value<i32> = Rc::new(RefCell::new(5));
    assert!(
        (({
            let _p: Ptr<i32> = (p_val.as_pointer());
            let _bias: i32 = 0;
            switch_complex_cond_23(_p, _bias)
        }) == 2)
    );
    assert!(
        (({
            let _p: Ptr<i32> = (p_val.as_pointer());
            let _bias: i32 = 5;
            switch_complex_cond_23(_p, _bias)
        }) == 3)
    );
    assert!(
        (({
            let _p: Ptr<i32> = (p_val.as_pointer());
            let _bias: i32 = -5_i32;
            switch_complex_cond_23(_p, _bias)
        }) == 1)
    );
    assert!(
        (({
            let _p: Ptr<i32> = (p_val.as_pointer());
            let _bias: i32 = 99;
            switch_complex_cond_23(_p, _bias)
        }) == 0)
    );
    assert!(
        (({
            let _n: i32 = 1;
            switch_in_dowhile_24(_n)
        }) == 1)
    );
    assert!(
        (({
            let _n: i32 = 3;
            switch_in_dowhile_24(_n)
        }) == ((1 + 10) + 100))
    );
    assert!(
        (({
            let _n: i32 = 6;
            continue_inside_switch_25(_n)
        }) == ((((1 + 3) + 5) as i32) + (3 * 1000)))
    );
    assert!(
        (({
            let _x: i32 = 1;
            case_then_default_26(_x)
        }) == 10)
    );
    assert!(
        (({
            let _x: i32 = 2;
            case_then_default_26(_x)
        }) == 20)
    );
    assert!(
        (({
            let _x: i32 = 99;
            case_then_default_26(_x)
        }) == 10)
    );
    assert!(
        (({
            let _x: i32 = 1;
            default_then_case_27(_x)
        }) == 1)
    );
    assert!(
        (({
            let _x: i32 = 2;
            default_then_case_27(_x)
        }) == 77)
    );
    assert!(
        (({
            let _x: i32 = 3;
            default_then_case_27(_x)
        }) == 3)
    );
    assert!(
        (({
            let _x: i32 = 99;
            default_then_case_27(_x)
        }) == 77)
    );
    assert!(
        (({
            let _x: i32 = 1;
            cases_and_default_stacked_28(_x)
        }) == 42)
    );
    assert!(
        (({
            let _x: i32 = 2;
            cases_and_default_stacked_28(_x)
        }) == 42)
    );
    assert!(
        (({
            let _x: i32 = 3;
            cases_and_default_stacked_28(_x)
        }) == 3)
    );
    assert!(
        (({
            let _x: i32 = 99;
            cases_and_default_stacked_28(_x)
        }) == 42)
    );
    assert!(
        (({
            let _x: i32 = 1;
            let _flag: i32 = 0;
            stacked_with_inner_fallthrough_29(_x, _flag)
        }) == 50)
    );
    assert!(
        (({
            let _x: i32 = 2;
            let _flag: i32 = 1;
            stacked_with_inner_fallthrough_29(_x, _flag)
        }) == 999)
    );
    assert!(
        (({
            let _x: i32 = 99;
            let _flag: i32 = 0;
            stacked_with_inner_fallthrough_29(_x, _flag)
        }) == 999)
    );
    assert!(
        (({
            let _x: i32 = 0;
            borrow_in_condition_and_in_body_30(_x)
        }) == 1)
    );
    assert!(
        (({
            let _x: i32 = 1;
            borrow_in_condition_and_in_body_30(_x)
        }) == 2)
    );
    assert!(
        (({
            let _n: i32 = 0;
            for_in_switch_break_31(_n)
        }) == 103)
    );
    assert!(
        (({
            let _n: i32 = 99;
            for_in_switch_break_31(_n)
        }) == -1_i32)
    );
    assert!(
        (({
            let _n: i32 = 0;
            for_in_switch_continue_32(_n)
        }) == 4)
    );
    assert!(
        (({
            let _n: i32 = 99;
            for_in_switch_continue_32(_n)
        }) == -1_i32)
    );
    assert!(
        (({
            let _n: i32 = 0;
            while_in_switch_break_33(_n)
        }) == 1006)
    );
    assert!(
        (({
            let _n: i32 = 99;
            while_in_switch_break_33(_n)
        }) == -1_i32)
    );
    assert!(
        (({
            let _n: i32 = 3;
            for_switch_for_break_34(_n)
        }) == 122)
    );
    return 0;
}
