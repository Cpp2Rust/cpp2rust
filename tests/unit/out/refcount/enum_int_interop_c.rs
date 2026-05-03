extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Copy, PartialEq, Debug, Default)]
enum Color {
    #[default]
    RED = 0,
    GREEN = 1,
    BLUE = 2,
}
impl From<i32> for Color {
    fn from(n: i32) -> Color {
        match n {
            0 => Color::RED,
            1 => Color::GREEN,
            2 => Color::BLUE,
            _ => panic!("invalid Color value: {}", n),
        }
    }
}
#[derive(Clone, Copy, PartialEq, Debug, Default)]
enum Option {
    #[default]
    OPT_NONE = 0,
    OPT_A = 10,
    OPT_B = 20,
    OPT_C = 30,
}
impl From<i32> for Option {
    fn from(n: i32) -> Option {
        match n {
            0 => Option::OPT_NONE,
            10 => Option::OPT_A,
            20 => Option::OPT_B,
            30 => Option::OPT_C,
            _ => panic!("invalid Option value: {}", n),
        }
    }
}
#[derive(Clone, Copy, PartialEq, Debug, Default)]
enum Tag {
    #[default]
    TAG_ZERO = 0,
    TAG_ONE = 1,
    TAG_TWO = 2,
}
impl From<i32> for Tag {
    fn from(n: i32) -> Tag {
        match n {
            0 => Tag::TAG_ZERO,
            1 => Tag::TAG_ONE,
            2 => Tag::TAG_TWO,
            _ => panic!("invalid Tag value: {}", n),
        }
    }
}
pub fn as_int_0(c: Color) -> i32 {
    let c: Value<Color> = Rc::new(RefCell::new(c));
    return ((*c.borrow()) as i32).clone();
}
pub fn classify_option_1(option: i32) -> i32 {
    let option: Value<i32> = Rc::new(RefCell::new(option));
    'switch: {
        let __match_cond = (*option.borrow());
        match __match_cond {
            v if v == (Option::OPT_NONE as i32) => {
                return -1_i32;
            }
            v if v == (Option::OPT_A as i32) => {
                return 1;
            }
            v if v == (Option::OPT_B as i32) => {
                return 2;
            }
            v if v == (Option::OPT_C as i32) => {
                return 3;
            }
            _ => {
                return 0;
            }
        }
    };
    panic!("ub: non-void function does not return a value")
}
pub fn make_color_2(n: i32) -> Color {
    let n: Value<i32> = Rc::new(RefCell::new(n));
    return Color::from((*n.borrow()));
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let c: Value<Color> = Rc::new(RefCell::new(Color::from((Color::RED as i32))));
    assert!((((((*c.borrow()) as u32) == ((Color::RED as i32) as u32)) as i32) != 0));
    assert!((((((*c.borrow()) as u32) == 0_u32) as i32) != 0));
    assert!((((((*c.borrow()) as u32) != 1_u32) as i32) != 0));
    if (((((*c.borrow()) as u32) == ((Color::GREEN as i32) as u32)) as i32) != 0) {
        return 1;
    }
    'switch: {
        let __match_cond = ((*c.borrow()) as u32);
        match __match_cond {
            v if v == (0 as u32) => {
                break 'switch;
            }
            v if v == (1 as u32) => {
                return 1;
            }
            v if v == (2 as u32) => {
                return 2;
            }
            _ => {
                return 99;
            }
        }
    };
    let x: Value<i32> = Rc::new(RefCell::new(((*c.borrow()) as i32).clone()));
    assert!(((((*x.borrow()) == 0) as i32) != 0));
    let y: Value<i32> = Rc::new(RefCell::new(
        ((((*c.borrow()) as u32).wrapping_add(1_u32)) as i32),
    ));
    assert!(((((*y.borrow()) == 1) as i32) != 0));
    (*c.borrow_mut()) = Color::from(2);
    assert!((((((*c.borrow()) as u32) == ((Color::BLUE as i32) as u32)) as i32) != 0));
    assert!((((((*c.borrow()) as u32) == 2_u32) as i32) != 0));
    (*c.borrow_mut()) = ({
        let _n: i32 = 1;
        make_color_2(_n)
    });
    assert!((((((*c.borrow()) as u32) == ((Color::GREEN as i32) as u32)) as i32) != 0));
    let cmp: Value<Color> = Rc::new(RefCell::new(Color::from(
        (((*c.borrow()) as u32).wrapping_add(1_u32)) as i32,
    )));
    assert!((((((*cmp.borrow()) as u32) == ((Color::BLUE as i32) as u32)) as i32) != 0));
    let o: Value<Option> = Rc::new(RefCell::new(Option::from((Option::OPT_A as i32))));
    assert!((((((*o.borrow()) as u32) == ((Option::OPT_A as i32) as u32)) as i32) != 0));
    assert!((((((*o.borrow()) as u32) == 10_u32) as i32) != 0));
    let oi: Value<i32> = Rc::new(RefCell::new(((*o.borrow()) as i32).clone()));
    assert!(((((*oi.borrow()) == 10) as i32) != 0));
    (*o.borrow_mut()) = Option::from(20);
    assert!((((((*o.borrow()) as u32) == ((Option::OPT_B as i32) as u32)) as i32) != 0));
    let rc: Value<i32> = Rc::new(RefCell::new(
        ({
            let _option: i32 = ((*o.borrow()) as i32).clone();
            classify_option_1(_option)
        }),
    ));
    assert!(((((*rc.borrow()) == 2) as i32) != 0));
    (*rc.borrow_mut()) = ({
        let _option: i32 = 20;
        classify_option_1(_option)
    });
    assert!(((((*rc.borrow()) == 2) as i32) != 0));
    (*rc.borrow_mut()) = ({
        let _option: i32 = (Option::OPT_C as i32);
        classify_option_1(_option)
    });
    assert!(((((*rc.borrow()) == 3) as i32) != 0));
    let t: Value<Tag> = Rc::new(RefCell::new(Tag::from((Tag::TAG_ONE as i32))));
    assert!((((((*t.borrow()) as u32) == 1_u32) as i32) != 0));
    assert!((((((*t.borrow()) as u32) == ((Tag::TAG_ONE as i32) as u32)) as i32) != 0));
    let ti: Value<i32> = Rc::new(RefCell::new(((*t.borrow()) as i32).clone()));
    assert!(((((*ti.borrow()) == 1) as i32) != 0));
    (*t.borrow_mut()) = Tag::from(2);
    assert!((((((*t.borrow()) as u32) == ((Tag::TAG_TWO as i32) as u32)) as i32) != 0));
    'switch: {
        let __match_cond = ((*t.borrow()) as u32);
        match __match_cond {
            v if v == ((Tag::TAG_ZERO as i32) as u32) => {
                return 90;
            }
            v if v == (1 as u32) => {
                return 91;
            }
            v if v == (2 as u32) => {
                break 'switch;
            }
            _ => {}
        }
    };
    let extra: Value<i32> = Rc::new(RefCell::new(
        (((Color::RED as i32) + (Color::GREEN as i32)) + (Color::BLUE as i32)),
    ));
    assert!(((((*extra.borrow()) == ((0 + 1) + 2)) as i32) != 0));
    return 0;
}
