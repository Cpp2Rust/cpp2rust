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
libcc2rs::impl_enum_inc_dec!(Color);
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
libcc2rs::impl_enum_inc_dec!(Option);
#[derive(Clone, Copy, PartialEq, Debug, Default)]
enum Tag_enum {
    #[default]
    TAG_ZERO = 0,
    TAG_ONE = 1,
    TAG_TWO = 2,
}
impl From<i32> for Tag_enum {
    fn from(n: i32) -> Tag_enum {
        match n {
            0 => Tag_enum::TAG_ZERO,
            1 => Tag_enum::TAG_ONE,
            2 => Tag_enum::TAG_TWO,
            _ => panic!("invalid Tag_enum value: {}", n),
        }
    }
}
libcc2rs::impl_enum_inc_dec!(Tag_enum);
#[derive(Default)]
pub struct Entry {
    pub name: Value<Ptr<u8>>,
    pub color: Value<Color>,
    pub opt: Value<Option>,
}
impl ByteRepr for Entry {}
thread_local!(
    pub static global_color_0: Value<Color> = Rc::new(RefCell::new(Color::GREEN));
);
thread_local!(
    pub static global_opt_1: Value<Option> = Rc::new(RefCell::new(Option::OPT_B));
);
thread_local!(
    pub static global_tag_2: Value<Tag_enum> = Rc::new(RefCell::new(Tag_enum::TAG_TWO));
);
thread_local!(
    pub static entries_3: Value<Box<[Entry]>> = Rc::new(RefCell::new(Box::new([
        Entry {
            name: Rc::new(RefCell::new(Ptr::from_string_literal("first"))),
            color: Rc::new(RefCell::new(Color::RED)),
            opt: Rc::new(RefCell::new(Option::OPT_NONE)),
        },
        Entry {
            name: Rc::new(RefCell::new(Ptr::from_string_literal("second"))),
            color: Rc::new(RefCell::new(Color::GREEN)),
            opt: Rc::new(RefCell::new(Option::OPT_A)),
        },
        Entry {
            name: Rc::new(RefCell::new(Ptr::from_string_literal("third"))),
            color: Rc::new(RefCell::new(Color::BLUE)),
            opt: Rc::new(RefCell::new(Option::OPT_C)),
        },
    ])));
);
pub fn as_int_4(c: Color) -> i32 {
    let c: Value<Color> = Rc::new(RefCell::new(c));
    return ((*c.borrow()) as i32).clone();
}
pub fn classify_option_5(option: i32) -> i32 {
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
pub fn make_color_6(n: i32) -> Color {
    let n: Value<i32> = Rc::new(RefCell::new(n));
    return Color::from((*n.borrow()));
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let c: Value<Color> = Rc::new(RefCell::new(Color::RED));
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
        make_color_6(_n)
    });
    assert!((((((*c.borrow()) as u32) == ((Color::GREEN as i32) as u32)) as i32) != 0));
    let cmp: Value<Color> = Rc::new(RefCell::new(Color::from(
        (((*c.borrow()) as u32).wrapping_add(1_u32)) as i32,
    )));
    assert!((((((*cmp.borrow()) as u32) == ((Color::BLUE as i32) as u32)) as i32) != 0));
    let o: Value<Option> = Rc::new(RefCell::new(Option::OPT_A));
    assert!((((((*o.borrow()) as u32) == ((Option::OPT_A as i32) as u32)) as i32) != 0));
    assert!((((((*o.borrow()) as u32) == 10_u32) as i32) != 0));
    let oi: Value<i32> = Rc::new(RefCell::new(((*o.borrow()) as i32).clone()));
    assert!(((((*oi.borrow()) == 10) as i32) != 0));
    (*o.borrow_mut()) = Option::from(20);
    assert!((((((*o.borrow()) as u32) == ((Option::OPT_B as i32) as u32)) as i32) != 0));
    let rc: Value<i32> = Rc::new(RefCell::new(
        ({
            let _option: i32 = ((*o.borrow()) as i32).clone();
            classify_option_5(_option)
        }),
    ));
    assert!(((((*rc.borrow()) == 2) as i32) != 0));
    (*rc.borrow_mut()) = ({
        let _option: i32 = 20;
        classify_option_5(_option)
    });
    assert!(((((*rc.borrow()) == 2) as i32) != 0));
    (*rc.borrow_mut()) = ({
        let _option: i32 = (Option::OPT_C as i32);
        classify_option_5(_option)
    });
    assert!(((((*rc.borrow()) == 3) as i32) != 0));
    let t: Value<Tag_enum> = Rc::new(RefCell::new(Tag_enum::TAG_ONE));
    assert!((((((*t.borrow()) as u32) == 1_u32) as i32) != 0));
    assert!((((((*t.borrow()) as u32) == ((Tag_enum::TAG_ONE as i32) as u32)) as i32) != 0));
    let ti: Value<i32> = Rc::new(RefCell::new(((*t.borrow()) as i32).clone()));
    assert!(((((*ti.borrow()) == 1) as i32) != 0));
    (*t.borrow_mut()) = Tag_enum::from(2);
    assert!((((((*t.borrow()) as u32) == ((Tag_enum::TAG_TWO as i32) as u32)) as i32) != 0));
    'switch: {
        let __match_cond = ((*t.borrow()) as u32);
        match __match_cond {
            v if v == ((Tag_enum::TAG_ZERO as i32) as u32) => {
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
    assert!(
        (((((*global_color_0.with(Value::clone).borrow()) as u32) == ((Color::GREEN as i32) as u32))
            as i32)
            != 0)
    );
    assert!(
        (((((*global_opt_1.with(Value::clone).borrow()) as u32) == ((Option::OPT_B as i32) as u32))
            as i32)
            != 0)
    );
    assert!(
        (((((*global_tag_2.with(Value::clone).borrow()) as u32)
            == ((Tag_enum::TAG_TWO as i32) as u32)) as i32)
            != 0)
    );
    assert!(
        (((((*(*entries_3.with(Value::clone).borrow())[(0) as usize]
            .color
            .borrow()) as u32)
            == ((Color::RED as i32) as u32)) as i32)
            != 0)
    );
    assert!(
        (((((*(*entries_3.with(Value::clone).borrow())[(0) as usize]
            .opt
            .borrow()) as u32)
            == ((Option::OPT_NONE as i32) as u32)) as i32)
            != 0)
    );
    assert!(
        (((((*(*entries_3.with(Value::clone).borrow())[(1) as usize]
            .color
            .borrow()) as u32)
            == ((Color::GREEN as i32) as u32)) as i32)
            != 0)
    );
    assert!(
        (((((*(*entries_3.with(Value::clone).borrow())[(1) as usize]
            .opt
            .borrow()) as u32)
            == ((Option::OPT_A as i32) as u32)) as i32)
            != 0)
    );
    assert!(
        (((((*(*entries_3.with(Value::clone).borrow())[(2) as usize]
            .color
            .borrow()) as u32)
            == ((Color::BLUE as i32) as u32)) as i32)
            != 0)
    );
    assert!(
        (((((*(*entries_3.with(Value::clone).borrow())[(2) as usize]
            .opt
            .borrow()) as u32)
            == ((Option::OPT_C as i32) as u32)) as i32)
            != 0)
    );
    return 0;
}
