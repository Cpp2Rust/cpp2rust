extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type Color = u32;
pub const Color_RED: Color = 0;
pub const Color_GREEN: Color = 1;
pub const Color_BLUE: Color = 2;
pub type Option = u32;
pub const Option_OPT_NONE: Option = 0;
pub const Option_OPT_A: Option = 10;
pub const Option_OPT_B: Option = 20;
pub const Option_OPT_C: Option = 30;
pub type Tag = u32;
pub const Tag_TAG_ZERO: Tag = 0;
pub const Tag_TAG_ONE: Tag = 1;
pub const Tag_TAG_TWO: Tag = 2;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct Entry {
    #[offset(0)]
    #[byte_size(8)]
    pub name: Ptr<i8>,
    #[offset(8)]
    pub color: Color,
    #[offset(12)]
    pub opt: Option,
}
thread_local!(
    pub static global_color_0: Value<Color> = Rc::new(RefCell::new(Color_GREEN));
);
thread_local!(
    pub static global_opt_1: Value<Option> = Rc::new(RefCell::new(Option_OPT_B));
);
thread_local!(
    pub static global_tag_2: Value<Tag> = Rc::new(RefCell::new(Tag_TAG_TWO));
);
thread_local!(
    pub static entries_3: Value<Box<[Entry]>> = Rc::new(RefCell::new(Box::new([
        Entry {
            name: Ptr::<i8>::from_string_literal(b"first"),
            color: Color_RED,
            opt: Option_OPT_NONE,
        },
        Entry {
            name: Ptr::<i8>::from_string_literal(b"second"),
            color: Color_GREEN,
            opt: Option_OPT_A,
        },
        Entry {
            name: Ptr::<i8>::from_string_literal(b"third"),
            color: Color_BLUE,
            opt: Option_OPT_C,
        },
    ])));
);
pub fn as_int_4(mut c: Color) -> i32 {
    return (c as i32);
}
pub fn classify_option_5(mut option: i32) -> i32 {
    'switch: {
        match { option } {
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
pub fn make_color_6(mut n: i32) -> Color {
    return ((n) as Color);
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut c: Color = Color_RED;
    assert!(((c as i32) == (Color_RED as i32)));
    assert!(((c as i32) == 0));
    assert!(((c as i32) != 1));
    if ((c as i32) == (Color_GREEN as i32)) {
        return 1;
    }
    'switch: {
        match { (c as i32) } {
            __v if __v == 0 => {
                break 'switch;
            }
            __v if __v == 1 => {
                return 1;
            }
            __v if __v == 2 => {
                return 2;
            }
            _ => {
                return 99;
            }
        }
    };
    let mut x: i32 = (c as i32);
    assert!((x == 0));
    let mut y: i32 = ((c as i32) + 1);
    assert!((y == 1));
    c = ((2) as Color);
    assert!(((c as i32) == (Color_BLUE as i32)));
    assert!(((c as i32) == 2));
    c = ({ make_color_6(1) });
    assert!(((c as i32) == (Color_GREEN as i32)));
    let mut cmp: Color = (((c as i32) + 1) as Color);
    assert!(((cmp as i32) == (Color_BLUE as i32)));
    let mut o: Option = Option_OPT_A;
    assert!(((o as i32) == (Option_OPT_A as i32)));
    assert!(((o as i32) == 10));
    let mut oi: i32 = (o as i32);
    assert!((oi == 10));
    o = ((20) as Option);
    assert!(((o as i32) == (Option_OPT_B as i32)));
    let mut rc: i32 = ({ classify_option_5((o as i32)) });
    assert!((rc == 2));
    rc = ({ classify_option_5(20) });
    assert!((rc == 2));
    rc = ({ classify_option_5((Option_OPT_C as i32)) });
    assert!((rc == 3));
    let mut t: Tag = Tag_TAG_ONE;
    assert!(((t as i32) == 1));
    assert!(((t as i32) == (Tag_TAG_ONE as i32)));
    let mut ti: i32 = (t as i32);
    assert!((ti == 1));
    t = ((2) as Tag);
    assert!(((t as i32) == (Tag_TAG_TWO as i32)));
    'switch: {
        match { (t as i32) } {
            __v if __v == (Tag_TAG_ZERO as i32) => {
                return 90;
            }
            __v if __v == 1 => {
                return 91;
            }
            __v if __v == 2 => {
                break 'switch;
            }
            _ => {}
        }
    };
    let mut extra: i32 = (((Color_RED as i32) + (Color_GREEN as i32)) + (Color_BLUE as i32));
    assert!((extra == ((0 + 1) + 2)));
    assert!(((global_color_0.with(|rc| *rc.borrow()) as i32) == (Color_GREEN as i32)));
    assert!(((global_opt_1.with(|rc| *rc.borrow()) as i32) == (Option_OPT_B as i32)));
    assert!(((global_tag_2.with(|rc| *rc.borrow()) as i32) == (Tag_TAG_TWO as i32)));
    assert!(
        (({ (*entries_3.with(Value::clone).borrow())[(0) as usize].color } as i32)
            == (Color_RED as i32))
    );
    assert!(
        (({ (*entries_3.with(Value::clone).borrow())[(0) as usize].opt } as i32)
            == (Option_OPT_NONE as i32))
    );
    assert!(
        (({ (*entries_3.with(Value::clone).borrow())[(1) as usize].color } as i32)
            == (Color_GREEN as i32))
    );
    assert!(
        (({ (*entries_3.with(Value::clone).borrow())[(1) as usize].opt } as i32)
            == (Option_OPT_A as i32))
    );
    assert!(
        (({ (*entries_3.with(Value::clone).borrow())[(2) as usize].color } as i32)
            == (Color_BLUE as i32))
    );
    assert!(
        (({ (*entries_3.with(Value::clone).borrow())[(2) as usize].opt } as i32)
            == (Option_OPT_C as i32))
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = global_color_0.with(|_| ());
    let _ = global_opt_1.with(|_| ());
    let _ = global_tag_2.with(|_| ());
    let _ = entries_3.with(|_| ());
}
