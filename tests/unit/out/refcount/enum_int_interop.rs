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
#[derive(Default)]
pub struct Entry {
    pub name: Value<Ptr<u8>>,
    pub color: Value<Color>,
    pub opt: Value<Option>,
}
impl Clone for Entry {
    fn clone(&self) -> Self {
        let mut this = Self {
            name: Rc::new(RefCell::new((*self.name.borrow()).clone())),
            color: Rc::new(RefCell::new((*self.color.borrow()).clone())),
            opt: Rc::new(RefCell::new((*self.opt.borrow()).clone())),
        };
        this
    }
}
impl ByteRepr for Entry {}
thread_local!(
    pub static global_color: Value<Color> = Rc::new(RefCell::new(Color::GREEN));
);
thread_local!(
    pub static global_opt: Value<Option> = Rc::new(RefCell::new(Option::OPT_B));
);
thread_local!(
    pub static global_tag: Value<Tag> = Rc::new(RefCell::new(Tag::TAG_TWO));
);
thread_local!(
    pub static entries: Value<Box<[Entry]>> = Rc::new(RefCell::new(Box::new([
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
    let c: Value<Color> = Rc::new(RefCell::new(Color::RED));
    assert!((((*c.borrow()) as i32) == (Color::RED as i32)));
    assert!((((*c.borrow()) as i32) == 0));
    assert!((((*c.borrow()) as i32) != 1));
    if (((*c.borrow()) as i32) == (Color::GREEN as i32)) {
        return 1;
    }
    'switch: {
        let __match_cond = ((*c.borrow()) as i32);
        match __match_cond {
            v if v == 0 => {
                break 'switch;
            }
            v if v == 1 => {
                return 1;
            }
            v if v == 2 => {
                return 2;
            }
            _ => {
                return 99;
            }
        }
    };
    let x: Value<i32> = Rc::new(RefCell::new(((*c.borrow()) as i32).clone()));
    assert!(((*x.borrow()) == 0));
    let y: Value<i32> = Rc::new(RefCell::new((((*c.borrow()) as i32) + 1)));
    assert!(((*y.borrow()) == 1));
    (*c.borrow_mut()) = Color::from(2);
    assert!((((*c.borrow()) as i32) == (Color::BLUE as i32)));
    assert!((((*c.borrow()) as i32) == 2));
    (*c.borrow_mut()) = ({
        let _n: i32 = 1;
        make_color_2(_n)
    });
    assert!((((*c.borrow()) as i32) == (Color::GREEN as i32)));
    let cmp: Value<Color> = Rc::new(RefCell::new(Color::from((((*c.borrow()) as i32) + 1))));
    assert!((((*cmp.borrow()) as i32) == (Color::BLUE as i32)));
    let o: Value<Option> = Rc::new(RefCell::new(Option::OPT_A));
    assert!((((*o.borrow()) as i32) == (Option::OPT_A as i32)));
    assert!((((*o.borrow()) as i32) == 10));
    let oi: Value<i32> = Rc::new(RefCell::new(((*o.borrow()) as i32).clone()));
    assert!(((*oi.borrow()) == 10));
    (*o.borrow_mut()) = Option::from(20);
    assert!((((*o.borrow()) as i32) == (Option::OPT_B as i32)));
    let rc: Value<i32> = Rc::new(RefCell::new(
        ({
            let _option: i32 = ((*o.borrow()) as i32).clone();
            classify_option_1(_option)
        }),
    ));
    assert!(((*rc.borrow()) == 2));
    (*rc.borrow_mut()) = ({
        let _option: i32 = 20;
        classify_option_1(_option)
    });
    assert!(((*rc.borrow()) == 2));
    (*rc.borrow_mut()) = ({
        let _option: i32 = (Option::OPT_C as i32);
        classify_option_1(_option)
    });
    assert!(((*rc.borrow()) == 3));
    let t: Value<Tag> = Rc::new(RefCell::new(Tag::TAG_ONE));
    assert!((((*t.borrow()) as i32) == 1));
    assert!((((*t.borrow()) as i32) == (Tag::TAG_ONE as i32)));
    let ti: Value<i32> = Rc::new(RefCell::new(((*t.borrow()) as i32).clone()));
    assert!(((*ti.borrow()) == 1));
    (*t.borrow_mut()) = Tag::from(2);
    assert!((((*t.borrow()) as i32) == (Tag::TAG_TWO as i32)));
    'switch: {
        let __match_cond = ((*t.borrow()) as i32);
        match __match_cond {
            v if v == (Tag::TAG_ZERO as i32) => {
                return 90;
            }
            v if v == 1 => {
                return 91;
            }
            v if v == 2 => {
                break 'switch;
            }
            _ => {}
        }
    };
    let extra: Value<i32> = Rc::new(RefCell::new(
        (((Color::RED as i32) + (Color::GREEN as i32)) + (Color::BLUE as i32)),
    ));
    assert!(((*extra.borrow()) == ((0 + 1) + 2)));
    assert!((((*global_color.with(Value::clone).borrow()) as i32) == (Color::GREEN as i32)));
    assert!((((*global_opt.with(Value::clone).borrow()) as i32) == (Option::OPT_B as i32)));
    assert!((((*global_tag.with(Value::clone).borrow()) as i32) == (Tag::TAG_TWO as i32)));
    assert!(
        (((*(*entries.with(Value::clone).borrow())[(0) as usize]
            .color
            .borrow()) as i32)
            == (Color::RED as i32))
    );
    assert!(
        (((*(*entries.with(Value::clone).borrow())[(0) as usize]
            .opt
            .borrow()) as i32)
            == (Option::OPT_NONE as i32))
    );
    assert!(
        (((*(*entries.with(Value::clone).borrow())[(1) as usize]
            .color
            .borrow()) as i32)
            == (Color::GREEN as i32))
    );
    assert!(
        (((*(*entries.with(Value::clone).borrow())[(1) as usize]
            .opt
            .borrow()) as i32)
            == (Option::OPT_A as i32))
    );
    assert!(
        (((*(*entries.with(Value::clone).borrow())[(2) as usize]
            .color
            .borrow()) as i32)
            == (Color::BLUE as i32))
    );
    assert!(
        (((*(*entries.with(Value::clone).borrow())[(2) as usize]
            .opt
            .borrow()) as i32)
            == (Option::OPT_C as i32))
    );
    return 0;
}
