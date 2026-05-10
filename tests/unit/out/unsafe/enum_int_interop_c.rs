extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
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
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Entry {
    pub name: *const u8,
    pub color: Color,
    pub opt: Option,
}
// SAFETY: preserves unsafe C semantics; thread-safety is not enforced
unsafe impl Sync for Entry {}
pub static mut global_color: Color = Color::GREEN;
pub static mut global_opt: Option = Option::OPT_B;
pub static mut global_tag: Tag = Tag::TAG_TWO;
pub static mut entries: [Entry; 3] = [
    Entry {
        name: b"first\0".as_ptr().cast_mut().cast_const(),
        color: Color::RED,
        opt: Option::OPT_NONE,
    },
    Entry {
        name: b"second\0".as_ptr().cast_mut().cast_const(),
        color: Color::GREEN,
        opt: Option::OPT_A,
    },
    Entry {
        name: b"third\0".as_ptr().cast_mut().cast_const(),
        color: Color::BLUE,
        opt: Option::OPT_C,
    },
];
pub unsafe fn as_int_0(mut c: Color) -> i32 {
    return (c as i32);
}
pub unsafe fn classify_option_1(mut option: i32) -> i32 {
    'switch: {
        let __match_cond = option;
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
pub unsafe fn make_color_2(mut n: i32) -> Color {
    return Color::from(n);
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut c: Color = Color::RED;
    assert!(((((c as u32) == ((Color::RED as i32) as u32)) as i32) != 0));
    assert!(((((c as u32) == (0_u32)) as i32) != 0));
    assert!(((((c as u32) != (1_u32)) as i32) != 0));
    if ((((c as u32) == ((Color::GREEN as i32) as u32)) as i32) != 0) {
        return 1;
    }
    'switch: {
        let __match_cond = (c as u32);
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
    let mut x: i32 = (c as i32);
    assert!(((((x) == (0)) as i32) != 0));
    let mut y: i32 = (((c as u32).wrapping_add(1_u32)) as i32);
    assert!(((((y) == (1)) as i32) != 0));
    c = Color::from(2);
    assert!(((((c as u32) == ((Color::BLUE as i32) as u32)) as i32) != 0));
    assert!(((((c as u32) == (2_u32)) as i32) != 0));
    c = (unsafe {
        let _n: i32 = 1;
        make_color_2(_n)
    });
    assert!(((((c as u32) == ((Color::GREEN as i32) as u32)) as i32) != 0));
    let mut cmp: Color = Color::from(((c as u32).wrapping_add(1_u32)) as i32);
    assert!(((((cmp as u32) == ((Color::BLUE as i32) as u32)) as i32) != 0));
    let mut o: Option = Option::OPT_A;
    assert!(((((o as u32) == ((Option::OPT_A as i32) as u32)) as i32) != 0));
    assert!(((((o as u32) == (10_u32)) as i32) != 0));
    let mut oi: i32 = (o as i32);
    assert!(((((oi) == (10)) as i32) != 0));
    o = Option::from(20);
    assert!(((((o as u32) == ((Option::OPT_B as i32) as u32)) as i32) != 0));
    let mut rc: i32 = (unsafe {
        let _option: i32 = (o as i32);
        classify_option_1(_option)
    });
    assert!(((((rc) == (2)) as i32) != 0));
    rc = (unsafe {
        let _option: i32 = 20;
        classify_option_1(_option)
    });
    assert!(((((rc) == (2)) as i32) != 0));
    rc = (unsafe {
        let _option: i32 = (Option::OPT_C as i32);
        classify_option_1(_option)
    });
    assert!(((((rc) == (3)) as i32) != 0));
    let mut t: Tag = Tag::TAG_ONE;
    assert!(((((t as u32) == (1_u32)) as i32) != 0));
    assert!(((((t as u32) == ((Tag::TAG_ONE as i32) as u32)) as i32) != 0));
    let mut ti: i32 = (t as i32);
    assert!(((((ti) == (1)) as i32) != 0));
    t = Tag::from(2);
    assert!(((((t as u32) == ((Tag::TAG_TWO as i32) as u32)) as i32) != 0));
    'switch: {
        let __match_cond = (t as u32);
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
    let mut extra: i32 = (((Color::RED as i32) + (Color::GREEN as i32)) + (Color::BLUE as i32));
    assert!(((((extra) == (((0) + (1)) + (2))) as i32) != 0));
    assert!(((((global_color as u32) == ((Color::GREEN as i32) as u32)) as i32) != 0));
    assert!(((((global_opt as u32) == ((Option::OPT_B as i32) as u32)) as i32) != 0));
    assert!(((((global_tag as u32) == ((Tag::TAG_TWO as i32) as u32)) as i32) != 0));
    assert!(((((entries[(0) as usize].color as u32) == ((Color::RED as i32) as u32)) as i32) != 0));
    assert!(
        ((((entries[(0) as usize].opt as u32) == ((Option::OPT_NONE as i32) as u32)) as i32) != 0)
    );
    assert!(
        ((((entries[(1) as usize].color as u32) == ((Color::GREEN as i32) as u32)) as i32) != 0)
    );
    assert!(
        ((((entries[(1) as usize].opt as u32) == ((Option::OPT_A as i32) as u32)) as i32) != 0)
    );
    assert!(
        ((((entries[(2) as usize].color as u32) == ((Color::BLUE as i32) as u32)) as i32) != 0)
    );
    assert!(
        ((((entries[(2) as usize].opt as u32) == ((Option::OPT_C as i32) as u32)) as i32) != 0)
    );
    return 0;
}
