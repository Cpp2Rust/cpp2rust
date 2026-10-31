extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign)]
#[byte_size(32)]
pub struct Text {
    #[offset(0)]
    #[byte_size(32)]
    pub value: Vec<i8>,
}
impl Text {
    pub fn move_from(_a0: Ptr<Text>) -> Self {
        Self {
            value: field!(_a0, value).with_mut(std::mem::take),
        }
    }
}
impl Default for Text {
    fn default() -> Self {
        { Text { value: vec![0] } }
    }
}
pub fn receiver_0(s: Ptr<Vec<i8>>, calls: Ptr<i32>) -> Ptr<Vec<i8>> {
    calls.with_mut(|__v| __v.prefix_inc());
    return (s).clone();
}
pub fn temporary_1(calls: Ptr<i32>) -> Vec<i8> {
    calls.with_mut(|__v| __v.prefix_inc());
    return {
        let mut __bytes = Ptr::<i8>::from_string_literal(b"hello").to_c_bytes();
        __bytes.push(0);
        __bytes
    };
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let s: Value<Vec<i8>> = Rc::new(RefCell::new({
        let mut __bytes = Ptr::<i8>::from_string_literal(b"hello").to_c_bytes();
        __bytes.push(0);
        __bytes
    }));
    let calls: Value<i32> = Rc::new(RefCell::new(0));
    let out: Value<Vec<i8>> = Rc::new(RefCell::new(
        match (
            ({ receiver_0(s.as_pointer(), calls.as_pointer()) }),
            ((*calls.borrow()) as usize),
            2_usize,
        ) {
            (__source, __pos, __count) => __source.with(|__s: &Vec<i8>| {
                let mut __tmp1 = __s
                    [__pos..::std::cmp::min(__pos + __count, __s.len().saturating_sub(1))]
                    .to_vec();
                __tmp1.push(0);
                __tmp1
            }),
        },
    ));
    println!("receiver: {} {}", out.decay(), (*calls.borrow()));
    (*calls.borrow_mut()) = 0;
    out.as_pointer().write(
        match (
            Rc::new(RefCell::new(({ temporary_1(calls.as_pointer()) }))).as_pointer(),
            1_usize,
            2_usize,
        ) {
            (__source, __pos, __count) => __source.with(|__s: &Vec<i8>| {
                let mut __tmp1 = __s
                    [__pos..::std::cmp::min(__pos + __count, __s.len().saturating_sub(1))]
                    .to_vec();
                __tmp1.push(0);
                __tmp1
            }),
        },
    );
    println!("temporary: {} {}", out.decay(), (*calls.borrow()));
    out.as_pointer().write(
        match (
            s.as_pointer(),
            ({
                (*s.borrow_mut())[0_usize] = ('j' as i8);
                0
            } as usize),
            2_usize,
        ) {
            (__source, __pos, __count) => __source.with(|__s: &Vec<i8>| {
                let mut __tmp1 = __s
                    [__pos..::std::cmp::min(__pos + __count, __s.len().saturating_sub(1))]
                    .to_vec();
                __tmp1.push(0);
                __tmp1
            }),
        },
    );
    println!("position mutation: {} {}", out.decay(), s.decay());
    out.as_pointer().write(
        match (
            s.as_pointer(),
            0_usize,
            ({
                (*s.borrow_mut())[1_usize] = ('a' as i8);
                2
            } as usize),
        ) {
            (__source, __pos, __count) => __source.with(|__s: &Vec<i8>| {
                let mut __tmp1 = __s
                    [__pos..::std::cmp::min(__pos + __count, __s.len().saturating_sub(1))]
                    .to_vec();
                __tmp1.push(0);
                __tmp1
            }),
        },
    );
    println!("count mutation: {} {}", out.decay(), s.decay());
    (*out.borrow_mut())[0_usize] = ('X' as i8);
    (*s.borrow_mut())[1_usize] = ('b' as i8);
    println!("independent: {} {}", out.decay(), s.decay());
    let mut p: Ptr<Vec<i8>> = (s.as_pointer());
    out.as_pointer()
        .write(match ((p).clone(), 1_usize, 2_usize) {
            (__source, __pos, __count) => __source.with(|__s: &Vec<i8>| {
                let mut __tmp1 = __s
                    [__pos..::std::cmp::min(__pos + __count, __s.len().saturating_sub(1))]
                    .to_vec();
                __tmp1.push(0);
                __tmp1
            }),
        });
    println!("pointer: {}", out.decay());
    out.as_pointer().write(
        match (
            Rc::new(RefCell::new(
                match (
                    Rc::new(RefCell::new({
                        let mut __bytes = Ptr::<i8>::from_string_literal(b"hello").to_c_bytes();
                        __bytes.push(0);
                        __bytes
                    }))
                    .as_pointer(),
                    1_usize,
                    3_usize,
                ) {
                    (__source, __pos, __count) => __source.with(|__s: &Vec<i8>| {
                        let mut __tmp1 = __s
                            [__pos..::std::cmp::min(__pos + __count, __s.len().saturating_sub(1))]
                            .to_vec();
                        __tmp1.push(0);
                        __tmp1
                    }),
                },
            ))
            .as_pointer(),
            1_usize,
            1_usize,
        ) {
            (__source, __pos, __count) => __source.with(|__s: &Vec<i8>| {
                let mut __tmp1 = __s
                    [__pos..::std::cmp::min(__pos + __count, __s.len().saturating_sub(1))]
                    .to_vec();
                __tmp1.push(0);
                __tmp1
            }),
        },
    );
    println!("nested: {}", out.decay());
    out.as_pointer()
        .write(match (s.as_pointer(), ((*s.borrow()).len() - 1), 0_usize) {
            (__source, __pos, __count) => __source.with(|__s: &Vec<i8>| {
                let mut __tmp1 = __s
                    [__pos..::std::cmp::min(__pos + __count, __s.len().saturating_sub(1))]
                    .to_vec();
                __tmp1.push(0);
                __tmp1
            }),
        });
    println!("empty: {}", ((*out.borrow()).len() - 1));
    let text: Value<Text> = Rc::new(RefCell::new(Text {
        value: {
            let mut __bytes = Ptr::<i8>::from_string_literal(b"hello").to_c_bytes();
            __bytes.push(0);
            __bytes
        },
    }));
    out.as_pointer().write(
        match (
            field_ref!(text.as_pointer(), value).as_pointer(),
            1_usize,
            2_usize,
        ) {
            (__source, __pos, __count) => __source.with(|__s: &Vec<i8>| {
                let mut __tmp1 = __s
                    [__pos..::std::cmp::min(__pos + __count, __s.len().saturating_sub(1))]
                    .to_vec();
                __tmp1.push(0);
                __tmp1
            }),
        },
    );
    println!(
        "field: {} {}",
        out.decay(),
        field_ref!(text.as_pointer(), value).decay()
    );
    let strings: Value<Vec<Vec<i8>>> = Rc::new(RefCell::new(Vec::from([
        {
            let mut __bytes = Ptr::<i8>::from_string_literal(b"hello").to_c_bytes();
            __bytes.push(0);
            __bytes
        },
        {
            let mut __bytes = Ptr::<i8>::from_string_literal(b"world").to_c_bytes();
            __bytes.push(0);
            __bytes
        },
    ])));
    let mut index: usize = 0_usize;
    out.as_pointer().write(
        match (
            strings.decay().offset(index.postfix_inc()),
            1_usize,
            2_usize,
        ) {
            (__source, __pos, __count) => __source.with(|__s: &Vec<i8>| {
                let mut __tmp1 = __s
                    [__pos..::std::cmp::min(__pos + __count, __s.len().saturating_sub(1))]
                    .to_vec();
                __tmp1.push(0);
                __tmp1
            }),
        },
    );
    println!("element: {} {}", out.decay(), index);
    return 0;
}
pub trait TextImpl {
    fn move_assign(&self, _a0: Ptr<Text>) -> Ptr<Text>;
}
impl TextImpl for Ptr<Text> {
    fn move_assign(&self, _a0: Ptr<Text>) -> Ptr<Text> {
        field!((*self), value).write({ { (*_a0.upgrade().deref()).value.clone() } });
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
