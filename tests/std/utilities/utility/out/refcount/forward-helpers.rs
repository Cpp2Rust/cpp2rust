extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_0() {
    let value: Value<i32> = Rc::new(RefCell::new(1));
    let mut forwarded: i32 = (*value.borrow());
    assert!((forwarded == 1));
    let lvalue: Ptr<i32> = value.as_pointer();
    lvalue.write(2);
    assert!(({ ((*value.borrow()) == 2) }));
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, CppFrom, Default)]
#[cpp_from(i32, new, value)]
#[byte_size(4)]
pub struct p6_forward_like_Movable {
    #[offset(0)]
    pub value: i32,
}
impl p6_forward_like_Movable {
    pub fn new(mut value: i32) -> Self {
        Self { value: value }
    }
    pub fn copy_from(other: Ptr<p6_forward_like_Movable>) -> Self {
        Self {
            value: other.with(|__s| __s.value),
        }
    }
    pub fn move_from(other: Ptr<p6_forward_like_Movable>) -> Self {
        let __this: p6_forward_like_Movable = Self {
            value: other.with(|__s| __s.value),
        };
        field!(other, value).write(0);
        __this
    }
}
impl Clone for p6_forward_like_Movable {
    fn clone(&self) -> Self {
        let __src: Value<p6_forward_like_Movable> =
            Rc::new(RefCell::new(p6_forward_like_Movable {
                value: self.value.clone(),
            }));
        p6_forward_like_Movable::copy_from(__src.as_pointer())
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor)]
#[byte_size(4)]
pub struct p6_forward_like_Holder {
    #[offset(0)]
    #[byte_size(4)]
    pub member: p6_forward_like_Movable,
}
impl p6_forward_like_Holder {
    pub fn move_from(_a0: Ptr<p6_forward_like_Holder>) -> Self {
        Self {
            member: p6_forward_like_Movable::move_from({ field_ptr!(_a0, member) }),
        }
    }
}
impl Default for p6_forward_like_Holder {
    fn default() -> Self {
        {
            p6_forward_like_Holder {
                member: p6_forward_like_Movable::new({ 1 }),
            }
        }
    }
}
pub fn take_1(self_: Ptr<p6_forward_like_Holder>) -> p6_forward_like_Movable {
    return p6_forward_like_Movable::copy_from({ field_ptr!(self_, member) });
}
pub fn take_2(self_: Ptr<p6_forward_like_Holder>) -> p6_forward_like_Movable {
    return p6_forward_like_Movable::move_from({ field_ptr!(self_, member) });
}
pub fn test_3() {
    let holder: Value<p6_forward_like_Holder> =
        Rc::new(RefCell::new(<p6_forward_like_Holder>::default()));
    let mut copied: p6_forward_like_Movable = ({ take_1(holder.as_pointer()) });
    assert!(({ (copied.value == 1) && ({ (*holder.borrow()).member.value } == 1) }));
    let mut moved: p6_forward_like_Movable = ({ take_2(holder.as_pointer()) });
    assert!(({ (moved.value == 1) && ({ (*holder.borrow()).member.value } == 0) }));
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, CppFrom, Default)]
#[cpp_from(i32, new, value)]
#[byte_size(4)]
pub struct p9_move_Movable {
    #[offset(0)]
    pub value: i32,
}
impl p9_move_Movable {
    pub fn new(mut value: i32) -> Self {
        Self { value: value }
    }
    pub fn move_from(other: Ptr<p9_move_Movable>) -> Self {
        let __this: p9_move_Movable = Self {
            value: other.with(|__s| __s.value),
        };
        field!(other, value).write(0);
        __this
    }
}
pub fn test_4() {
    let mut number: i32 = 1;
    let mut moved: i32 = number;
    assert!((moved == 1) && (number == 1));
    let target: Value<i32> = Rc::new(RefCell::new(2));
    let mut pointer: Ptr<i32> = (target.as_pointer());
    let mut moved_pointer: Ptr<i32> = (pointer).clone();
    assert!(
        ({ (moved_pointer).clone() } == { (target.as_pointer()) })
            && ({ (pointer).clone() } == { (target.as_pointer()) })
    );
    let source: Value<p9_move_Movable> = Rc::new(RefCell::new(p9_move_Movable::new({ 3 })));
    let mut destination: p9_move_Movable = p9_move_Movable::move_from({ source.as_pointer() });
    assert!(({ (destination.value == 3) && ({ (*source.borrow()).value } == 0) }));
    let strings: Value<Vec<Vec<i8>>> = Rc::new(RefCell::new(Vec::new()));
    let mut text: Vec<i8> = {
        let mut __bytes = Ptr::<i8>::from_string_literal(b"text").to_c_bytes();
        __bytes.push(0);
        __bytes
    };
    {
        let __a1 = std::mem::replace(&mut text, vec![0]);
        (*strings.borrow_mut()).push(__a1)
    };
    assert!(
        ({
            match (strings.decay().offset(0_usize),) {
                (__tmp_0,) => Ptr::<i8>::from_string_literal(b"text").with_c_str(|__s| {
                    (*__tmp_0.upgrade().deref())
                        [..(*__tmp_0.upgrade().deref()).len().saturating_sub(1)]
                        == *__s
                }),
            }
        })
    );
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, CppFrom)]
#[cpp_from(i32, new, value)]
#[byte_size(12)]
pub struct p10_forward_in_template_A {
    #[offset(0)]
    pub value: i32,
    #[offset(4)]
    pub copies: i32,
    #[offset(8)]
    pub moves: i32,
}
impl p10_forward_in_template_A {
    pub fn new(mut value: i32) -> Self {
        Self {
            value: value,
            copies: 0,
            moves: 0,
        }
    }
    pub fn copy_from(other: Ptr<p10_forward_in_template_A>) -> Self {
        Self {
            value: other.with(|__s| __s.value),
            copies: (other.with(|__s| __s.copies) + 1),
            moves: 0,
        }
    }
    pub fn move_from(other: Ptr<p10_forward_in_template_A>) -> Self {
        let __this: p10_forward_in_template_A = Self {
            value: other.with(|__s| __s.value),
            copies: 0,
            moves: (other.with(|__s| __s.moves) + 1),
        };
        field!(other, value).write(0);
        __this
    }
}
impl Clone for p10_forward_in_template_A {
    fn clone(&self) -> Self {
        let __src: Value<p10_forward_in_template_A> =
            Rc::new(RefCell::new(p10_forward_in_template_A {
                value: self.value.clone(),
                copies: self.copies.clone(),
                moves: self.moves.clone(),
            }));
        p10_forward_in_template_A::copy_from(__src.as_pointer())
    }
}
impl Default for p10_forward_in_template_A {
    fn default() -> Self {
        {
            p10_forward_in_template_A {
                value: 0_i32,
                copies: 0,
                moves: 0,
            }
        }
    }
}
pub fn make_5(source: Ptr<p10_forward_in_template_A>) -> p10_forward_in_template_A {
    return p10_forward_in_template_A::copy_from({ (source).clone() });
}
pub fn make_6(source: Ptr<p10_forward_in_template_A>) -> p10_forward_in_template_A {
    return p10_forward_in_template_A::move_from({ (source).clone() });
}
pub fn test_7() {
    let a: Value<p10_forward_in_template_A> =
        Rc::new(RefCell::new(p10_forward_in_template_A::new({ 1 })));
    let mut copied: p10_forward_in_template_A = ({ make_5(a.as_pointer()) });
    assert!(((copied.value == 1) && (copied.copies == 1)) && (copied.moves == 0));
    assert!(({ ({ (*a.borrow()).value } == 1) }));
    let mut moved: p10_forward_in_template_A = ({ make_6(a.as_pointer()) });
    assert!(((moved.value == 1) && (moved.copies == 0)) && (moved.moves == 1));
    assert!(({ ({ (*a.borrow()).value } == 0) }));
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, CppFrom, Default)]
#[cpp_from(i32, new, value)]
#[byte_size(4)]
pub struct p11_move_if_noexcept_NoexceptMove {
    #[offset(0)]
    pub value: i32,
}
impl p11_move_if_noexcept_NoexceptMove {
    pub fn new(mut value: i32) -> Self {
        Self { value: value }
    }
    pub fn copy_from(other: Ptr<p11_move_if_noexcept_NoexceptMove>) -> Self {
        Self {
            value: other.with(|__s| __s.value),
        }
    }
    pub fn move_from(other: Ptr<p11_move_if_noexcept_NoexceptMove>) -> Self {
        let __this: p11_move_if_noexcept_NoexceptMove = Self {
            value: other.with(|__s| __s.value),
        };
        field!(other, value).write(0);
        __this
    }
}
impl Clone for p11_move_if_noexcept_NoexceptMove {
    fn clone(&self) -> Self {
        let __src: Value<p11_move_if_noexcept_NoexceptMove> =
            Rc::new(RefCell::new(p11_move_if_noexcept_NoexceptMove {
                value: self.value.clone(),
            }));
        p11_move_if_noexcept_NoexceptMove::copy_from(__src.as_pointer())
    }
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, CppFrom, Default)]
#[cpp_from(i32, new, value)]
#[byte_size(4)]
pub struct p11_move_if_noexcept_ThrowingMove {
    #[offset(0)]
    pub value: i32,
}
impl p11_move_if_noexcept_ThrowingMove {
    pub fn new(mut value: i32) -> Self {
        Self { value: value }
    }
    pub fn copy_from(other: Ptr<p11_move_if_noexcept_ThrowingMove>) -> Self {
        Self {
            value: other.with(|__s| __s.value),
        }
    }
    pub fn move_from(other: Ptr<p11_move_if_noexcept_ThrowingMove>) -> Self {
        let __this: p11_move_if_noexcept_ThrowingMove = Self {
            value: other.with(|__s| __s.value),
        };
        field!(other, value).write(0);
        __this
    }
}
impl Clone for p11_move_if_noexcept_ThrowingMove {
    fn clone(&self) -> Self {
        let __src: Value<p11_move_if_noexcept_ThrowingMove> =
            Rc::new(RefCell::new(p11_move_if_noexcept_ThrowingMove {
                value: self.value.clone(),
            }));
        p11_move_if_noexcept_ThrowingMove::copy_from(__src.as_pointer())
    }
}
pub fn test_8() {
    let moved_from: Value<p11_move_if_noexcept_NoexceptMove> =
        Rc::new(RefCell::new(p11_move_if_noexcept_NoexceptMove::new({ 1 })));
    let mut moved: p11_move_if_noexcept_NoexceptMove =
        p11_move_if_noexcept_NoexceptMove::move_from({ moved_from.as_pointer() });
    assert!(({ (moved.value == 1) && ({ (*moved_from.borrow()).value } == 0) }));
    let copied_from: Value<p11_move_if_noexcept_ThrowingMove> =
        Rc::new(RefCell::new(p11_move_if_noexcept_ThrowingMove::new({ 2 })));
    let mut copied: p11_move_if_noexcept_ThrowingMove =
        p11_move_if_noexcept_ThrowingMove::copy_from({ copied_from.as_pointer() });
    assert!(({ (copied.value == 2) && ({ (*copied_from.borrow()).value } == 2) }));
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    ({ test_0() });
    ({ test_3() });
    ({ test_4() });
    ({ test_7() });
    ({ test_8() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
