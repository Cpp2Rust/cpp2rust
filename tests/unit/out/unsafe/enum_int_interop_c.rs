extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type Color = u32;
pub const Color_RED: Color = 0;
pub const Color_GREEN: Color = 1;
pub const Color_BLUE: Color = 2;
pub type Option = u32;
pub const Option_OPT_NONE: Option = 0;
pub const Option_OPT_A: Option = 10;
pub const Option_OPT_B: Option = 20;
pub const Option_OPT_C: Option = 30;
pub type Tag_enum = u32;
pub const Tag_enum_TAG_ZERO: Tag_enum = 0;
pub const Tag_enum_TAG_ONE: Tag_enum = 1;
pub const Tag_enum_TAG_TWO: Tag_enum = 2;
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Entry {
    pub name: *const libc::c_char,
    pub color: Color,
    pub opt: Option,
}
pub static mut global_color_0: Color = unsafe { Color_GREEN };
pub static mut global_opt_1: Option = unsafe { Option_OPT_B };
pub static mut global_tag_2: Tag_enum = unsafe { Tag_enum_TAG_TWO };
pub static mut entries_3: [Entry; 3] = unsafe {
    [
        Entry {
            name: (c"first".as_ptr().cast_mut()).cast_const(),
            color: Color_RED,
            opt: Option_OPT_NONE,
        },
        Entry {
            name: (c"second".as_ptr().cast_mut()).cast_const(),
            color: Color_GREEN,
            opt: Option_OPT_A,
        },
        Entry {
            name: (c"third".as_ptr().cast_mut()).cast_const(),
            color: Color_BLUE,
            opt: Option_OPT_C,
        },
    ]
};
pub unsafe fn as_int_4(mut c: Color) -> i32 {
    return (c as i32);
}
pub unsafe fn classify_option_5(mut option: i32) -> i32 {
    'switch: {
        let __match_cond = option;
        match __match_cond {
            __v if __v == (Option_OPT_NONE as i32) => {
                return -1_i32;
            }
            __v if __v == (Option_OPT_A as i32) => {
                return 1;
            }
            __v if __v == (Option_OPT_B as i32) => {
                return 2;
            }
            __v if __v == (Option_OPT_C as i32) => {
                return 3;
            }
            _ => {
                return 0;
            }
        }
    };
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn make_color_6(mut n: i32) -> Color {
    return ((n) as Color);
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut c: Color = Color_RED;
    assert!(((((c as u32) == ((Color_RED as i32) as u32)) as i32) != 0));
    assert!(((((c as u32) == (0_u32)) as i32) != 0));
    assert!(((((c as u32) != (1_u32)) as i32) != 0));
    if ((((c as u32) == ((Color_GREEN as i32) as u32)) as i32) != 0) {
        return 1;
    }
    'switch: {
        let __match_cond = (c as u32);
        match __match_cond {
            __v if __v == (0 as u32) => {
                break 'switch;
            }
            __v if __v == (1 as u32) => {
                return 1;
            }
            __v if __v == (2 as u32) => {
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
    c = ((2) as Color);
    assert!(((((c as u32) == ((Color_BLUE as i32) as u32)) as i32) != 0));
    assert!(((((c as u32) == (2_u32)) as i32) != 0));
    c = (unsafe { make_color_6(1) });
    assert!(((((c as u32) == ((Color_GREEN as i32) as u32)) as i32) != 0));
    let mut cmp: Color = (((c as u32).wrapping_add(1_u32)) as Color);
    assert!(((((cmp as u32) == ((Color_BLUE as i32) as u32)) as i32) != 0));
    let mut o: Option = Option_OPT_A;
    assert!(((((o as u32) == ((Option_OPT_A as i32) as u32)) as i32) != 0));
    assert!(((((o as u32) == (10_u32)) as i32) != 0));
    let mut oi: i32 = (o as i32);
    assert!(((((oi) == (10)) as i32) != 0));
    o = ((20) as Option);
    assert!(((((o as u32) == ((Option_OPT_B as i32) as u32)) as i32) != 0));
    let mut rc: i32 = (unsafe { classify_option_5((o as i32)) });
    assert!(((((rc) == (2)) as i32) != 0));
    rc = (unsafe { classify_option_5(20) });
    assert!(((((rc) == (2)) as i32) != 0));
    rc = (unsafe { classify_option_5((Option_OPT_C as i32)) });
    assert!(((((rc) == (3)) as i32) != 0));
    let mut t: Tag_enum = Tag_enum_TAG_ONE;
    assert!(((((t as u32) == (1_u32)) as i32) != 0));
    assert!(((((t as u32) == ((Tag_enum_TAG_ONE as i32) as u32)) as i32) != 0));
    let mut ti: i32 = (t as i32);
    assert!(((((ti) == (1)) as i32) != 0));
    t = ((2) as Tag_enum);
    assert!(((((t as u32) == ((Tag_enum_TAG_TWO as i32) as u32)) as i32) != 0));
    'switch: {
        let __match_cond = (t as u32);
        match __match_cond {
            __v if __v == ((Tag_enum_TAG_ZERO as i32) as u32) => {
                return 90;
            }
            __v if __v == (1 as u32) => {
                return 91;
            }
            __v if __v == (2 as u32) => {
                break 'switch;
            }
            _ => {}
        }
    };
    let mut extra: i32 = (((Color_RED as i32) + (Color_GREEN as i32)) + (Color_BLUE as i32));
    assert!(((((extra) == (((0) + (1)) + (2))) as i32) != 0));
    assert!(((((global_color_0 as u32) == ((Color_GREEN as i32) as u32)) as i32) != 0));
    assert!(((((global_opt_1 as u32) == ((Option_OPT_B as i32) as u32)) as i32) != 0));
    assert!(((((global_tag_2 as u32) == ((Tag_enum_TAG_TWO as i32) as u32)) as i32) != 0));
    assert!(
        ((((entries_3[(0) as usize].color as u32) == ((Color_RED as i32) as u32)) as i32) != 0)
    );
    assert!(
        ((((entries_3[(0) as usize].opt as u32) == ((Option_OPT_NONE as i32) as u32)) as i32) != 0)
    );
    assert!(
        ((((entries_3[(1) as usize].color as u32) == ((Color_GREEN as i32) as u32)) as i32) != 0)
    );
    assert!(
        ((((entries_3[(1) as usize].opt as u32) == ((Option_OPT_A as i32) as u32)) as i32) != 0)
    );
    assert!(
        ((((entries_3[(2) as usize].color as u32) == ((Color_BLUE as i32) as u32)) as i32) != 0)
    );
    assert!(
        ((((entries_3[(2) as usize].opt as u32) == ((Option_OPT_C as i32) as u32)) as i32) != 0)
    );
    let mut names: [*const libc::c_char; 3] = [
        (c"red".as_ptr().cast_mut()).cast_const(),
        (c"green".as_ptr().cast_mut()).cast_const(),
        (c"blue".as_ptr().cast_mut()).cast_const(),
    ];
    let mut idx: Color = Color_GREEN;
    assert!(
        (((((*names[(idx) as usize].offset((0) as isize)) as i32) == ('g' as i32)) as i32) != 0)
    );
    assert!(
        ((((entries_3[(idx) as usize].opt as u32) == ((Option_OPT_A as i32) as u32)) as i32) != 0)
    );
    assert!(
        (((((*names[(global_tag_2) as usize].offset((0) as isize)) as i32) == ('b' as i32))
            as i32)
            != 0)
    );
    let mut pp: *mut *const libc::c_char = (&mut names[(idx) as usize] as *mut *const libc::c_char);
    assert!((((((*(*pp).offset((0) as isize)) as i32) == ('g' as i32)) as i32) != 0));
    let mut pe: *mut Entry = (&mut entries_3[(idx) as usize] as *mut Entry);
    assert!((((((*pe).opt as u32) == ((Option_OPT_A as i32) as u32)) as i32) != 0));
    return 0;
}
