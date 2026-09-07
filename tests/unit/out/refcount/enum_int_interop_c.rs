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
pub type Tag_enum = u32;
pub const Tag_enum_TAG_ZERO: Tag_enum = 0;
pub const Tag_enum_TAG_ONE: Tag_enum = 1;
pub const Tag_enum_TAG_TWO: Tag_enum = 2;
#[derive(Default)]
pub struct Entry {
    pub name: Value<Ptr<u8>>,
    pub color: Value<Color>,
    pub opt: Value<Option>,
}
impl Clone for Entry {
    fn clone(&self) -> Self {
        Self {
            name: Rc::new(RefCell::new((*self.name.borrow()).clone())),
            color: Rc::new(RefCell::new((*self.color.borrow()).clone())),
            opt: Rc::new(RefCell::new((*self.opt.borrow()).clone())),
        }
    }
}
impl ByteRepr for Entry {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.name.borrow()).to_bytes(&mut buf[0..8]);
        (*self.color.borrow()).to_bytes(&mut buf[8..12]);
        (*self.opt.borrow()).to_bytes(&mut buf[12..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            name: Rc::new(RefCell::new(<Ptr<u8>>::from_bytes(&buf[0..8]))),
            color: Rc::new(RefCell::new(<Color>::from_bytes(&buf[8..12]))),
            opt: Rc::new(RefCell::new(<Option>::from_bytes(&buf[12..16]))),
        }
    }
}
thread_local!(
    pub static global_color_0: Value<Color> = Rc::new(RefCell::new(Color_GREEN));
);
thread_local!(
    pub static global_opt_1: Value<Option> = Rc::new(RefCell::new(Option_OPT_B));
);
thread_local!(
    pub static global_tag_2: Value<Tag_enum> = Rc::new(RefCell::new(Tag_enum_TAG_TWO));
);
thread_local!(
    pub static entries_3: Value<Box<[Entry]>> = Rc::new(RefCell::new(Box::new([
        Entry {
            name: Rc::new(RefCell::new(Ptr::from_string_literal(b"first"))),
            color: Rc::new(RefCell::new(Color_RED)),
            opt: Rc::new(RefCell::new(Option_OPT_NONE)),
        },
        Entry {
            name: Rc::new(RefCell::new(Ptr::from_string_literal(b"second"))),
            color: Rc::new(RefCell::new(Color_GREEN)),
            opt: Rc::new(RefCell::new(Option_OPT_A)),
        },
        Entry {
            name: Rc::new(RefCell::new(Ptr::from_string_literal(b"third"))),
            color: Rc::new(RefCell::new(Color_BLUE)),
            opt: Rc::new(RefCell::new(Option_OPT_C)),
        },
    ])));
);
pub fn as_int_4(c: Color) -> i32 {
    let c: Value<Color> = Rc::new(RefCell::new(c));
    return ((*c.borrow()) as i32);
}
pub fn classify_option_5(option: i32) -> i32 {
    let option: Value<i32> = Rc::new(RefCell::new(option));
    'switch: {
        let __match_cond = (*option.borrow());
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
pub fn make_color_6(n: i32) -> Color {
    let n: Value<i32> = Rc::new(RefCell::new(n));
    return ((*n.borrow()) as Color);
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let c: Value<Color> = Rc::new(RefCell::new(Color_RED));
    assert!((((((*c.borrow()) as u32) == ((Color_RED as i32) as u32)) as i32) != 0));
    assert!((((((*c.borrow()) as u32) == 0_u32) as i32) != 0));
    assert!((((((*c.borrow()) as u32) != 1_u32) as i32) != 0));
    if (((((*c.borrow()) as u32) == ((Color_GREEN as i32) as u32)) as i32) != 0) {
        return 1;
    }
    'switch: {
        let __match_cond = ((*c.borrow()) as u32);
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
    let x: Value<i32> = Rc::new(RefCell::new(((*c.borrow()) as i32)));
    assert!(((((*x.borrow()) == 0) as i32) != 0));
    let y: Value<i32> = Rc::new(RefCell::new(
        ((((*c.borrow()) as u32).wrapping_add(1_u32)) as i32),
    ));
    assert!(((((*y.borrow()) == 1) as i32) != 0));
    (*c.borrow_mut()) = ((2) as Color);
    assert!((((((*c.borrow()) as u32) == ((Color_BLUE as i32) as u32)) as i32) != 0));
    assert!((((((*c.borrow()) as u32) == 2_u32) as i32) != 0));
    (*c.borrow_mut()) = ({ make_color_6(1) });
    assert!((((((*c.borrow()) as u32) == ((Color_GREEN as i32) as u32)) as i32) != 0));
    let cmp: Value<Color> = Rc::new(RefCell::new(
        ((((*c.borrow()) as u32).wrapping_add(1_u32)) as Color),
    ));
    assert!((((((*cmp.borrow()) as u32) == ((Color_BLUE as i32) as u32)) as i32) != 0));
    let o: Value<Option> = Rc::new(RefCell::new(Option_OPT_A));
    assert!((((((*o.borrow()) as u32) == ((Option_OPT_A as i32) as u32)) as i32) != 0));
    assert!((((((*o.borrow()) as u32) == 10_u32) as i32) != 0));
    let oi: Value<i32> = Rc::new(RefCell::new(((*o.borrow()) as i32)));
    assert!(((((*oi.borrow()) == 10) as i32) != 0));
    (*o.borrow_mut()) = ((20) as Option);
    assert!((((((*o.borrow()) as u32) == ((Option_OPT_B as i32) as u32)) as i32) != 0));
    let rc: Value<i32> = Rc::new(RefCell::new(
        ({ classify_option_5(((*o.borrow()) as i32)) }),
    ));
    assert!(((((*rc.borrow()) == 2) as i32) != 0));
    (*rc.borrow_mut()) = ({ classify_option_5(20) });
    assert!(((((*rc.borrow()) == 2) as i32) != 0));
    (*rc.borrow_mut()) = ({ classify_option_5((Option_OPT_C as i32)) });
    assert!(((((*rc.borrow()) == 3) as i32) != 0));
    let t: Value<Tag_enum> = Rc::new(RefCell::new(Tag_enum_TAG_ONE));
    assert!((((((*t.borrow()) as u32) == 1_u32) as i32) != 0));
    assert!((((((*t.borrow()) as u32) == ((Tag_enum_TAG_ONE as i32) as u32)) as i32) != 0));
    let ti: Value<i32> = Rc::new(RefCell::new(((*t.borrow()) as i32)));
    assert!(((((*ti.borrow()) == 1) as i32) != 0));
    (*t.borrow_mut()) = ((2) as Tag_enum);
    assert!((((((*t.borrow()) as u32) == ((Tag_enum_TAG_TWO as i32) as u32)) as i32) != 0));
    'switch: {
        let __match_cond = ((*t.borrow()) as u32);
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
    let extra: Value<i32> = Rc::new(RefCell::new(
        (((Color_RED as i32) + (Color_GREEN as i32)) + (Color_BLUE as i32)),
    ));
    assert!(((((*extra.borrow()) == ((0 + 1) + 2)) as i32) != 0));
    assert!(
        (((((*global_color_0.with(Value::clone).borrow()) as u32) == ((Color_GREEN as i32) as u32))
            as i32)
            != 0)
    );
    assert!(
        (((((*global_opt_1.with(Value::clone).borrow()) as u32) == ((Option_OPT_B as i32) as u32))
            as i32)
            != 0)
    );
    assert!(
        (((((*global_tag_2.with(Value::clone).borrow()) as u32)
            == ((Tag_enum_TAG_TWO as i32) as u32)) as i32)
            != 0)
    );
    assert!(
        (((((*(*entries_3.with(Value::clone).borrow())[(0) as usize]
            .color
            .borrow()) as u32)
            == ((Color_RED as i32) as u32)) as i32)
            != 0)
    );
    assert!(
        (((((*(*entries_3.with(Value::clone).borrow())[(0) as usize]
            .opt
            .borrow()) as u32)
            == ((Option_OPT_NONE as i32) as u32)) as i32)
            != 0)
    );
    assert!(
        (((((*(*entries_3.with(Value::clone).borrow())[(1) as usize]
            .color
            .borrow()) as u32)
            == ((Color_GREEN as i32) as u32)) as i32)
            != 0)
    );
    assert!(
        (((((*(*entries_3.with(Value::clone).borrow())[(1) as usize]
            .opt
            .borrow()) as u32)
            == ((Option_OPT_A as i32) as u32)) as i32)
            != 0)
    );
    assert!(
        (((((*(*entries_3.with(Value::clone).borrow())[(2) as usize]
            .color
            .borrow()) as u32)
            == ((Color_BLUE as i32) as u32)) as i32)
            != 0)
    );
    assert!(
        (((((*(*entries_3.with(Value::clone).borrow())[(2) as usize]
            .opt
            .borrow()) as u32)
            == ((Option_OPT_C as i32) as u32)) as i32)
            != 0)
    );
    let names: Value<Box<[Ptr<u8>]>> = Rc::new(RefCell::new(Box::new([
        Ptr::from_string_literal(b"red"),
        Ptr::from_string_literal(b"green"),
        Ptr::from_string_literal(b"blue"),
    ])));
    let idx: Value<Color> = Rc::new(RefCell::new(Color_GREEN));
    assert!(
        ((((((*names.borrow())[(*idx.borrow()) as usize]
            .offset((0) as isize)
            .read()) as i32)
            == ('g' as i32)) as i32)
            != 0)
    );
    assert!(
        (((((*(*entries_3.with(Value::clone).borrow())[(*idx.borrow()) as usize]
            .opt
            .borrow()) as u32)
            == ((Option_OPT_A as i32) as u32)) as i32)
            != 0)
    );
    assert!(
        ((((((*names.borrow())[(*global_tag_2.with(Value::clone).borrow()) as usize]
            .offset((0) as isize)
            .read()) as i32)
            == ('b' as i32)) as i32)
            != 0)
    );
    let pp: Value<Ptr<Ptr<u8>>> = Rc::new(RefCell::new(
        ((names.as_pointer() as Ptr<Ptr<u8>>).offset((*idx.borrow()) as isize)),
    ));
    assert!(
        (((((((*pp.borrow()).read()).offset((0) as isize).read()) as i32) == ('g' as i32)) as i32)
            != 0)
    );
    let pe: Value<Ptr<Entry>> = Rc::new(RefCell::new(
        ((entries_3.with(Value::clone).as_pointer() as Ptr<Entry>)
            .offset((*idx.borrow()) as isize)),
    ));
    assert!(
        (((((*(*(*pe.borrow()).upgrade().deref()).opt.borrow()) as u32)
            == ((Option_OPT_A as i32) as u32)) as i32)
            != 0)
    );
    return 0;
}
