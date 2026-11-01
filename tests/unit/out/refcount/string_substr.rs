extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Text {
    #[offset(0)]
    #[byte_size(8)]
    pub value: Ptr<Vec<i8>>,
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
        match (({ receiver_0(s.as_pointer(), calls.as_pointer()) }), {
            ((*calls.borrow()) as usize)
        }) {
            (__tmp_0, mut __tmp_1) => __tmp_0.with(|__s: &Vec<i8>| {
                let mut __tmp1 = __s
                    [__tmp_1..::std::cmp::min(__tmp_1 + 2_usize, __s.len().saturating_sub(1))]
                    .to_vec();
                __tmp1.push(0);
                __tmp1
            }),
        },
    ));
    println!("receiver: {} {}", out.decay(), (*calls.borrow()));
    (*calls.borrow_mut()) = 0;
    out.as_pointer().write(
        Rc::new(RefCell::new(({ temporary_1(calls.as_pointer()) })))
            .as_pointer()
            .with(|__s: &Vec<i8>| {
                let mut __tmp1 = __s
                    [1_usize..::std::cmp::min(1_usize + 2_usize, __s.len().saturating_sub(1))]
                    .to_vec();
                __tmp1.push(0);
                __tmp1
            }),
    );
    println!("temporary: {} {}", out.decay(), (*calls.borrow()));
    out.as_pointer().write(
        match ({
            ({
                (*s.borrow_mut())[0_usize] = ('j' as i8);
                0
            } as usize)
        },)
        {
            (mut __tmp_2,) => s.as_pointer().with(|__s: &Vec<i8>| {
                let mut __tmp1 = __s
                    [__tmp_2..::std::cmp::min(__tmp_2 + 2_usize, __s.len().saturating_sub(1))]
                    .to_vec();
                __tmp1.push(0);
                __tmp1
            }),
        },
    );
    println!("position mutation: {} {}", out.decay(), s.decay());
    out.as_pointer().write(
        match ({
            ({
                (*s.borrow_mut())[1_usize] = ('a' as i8);
                2
            } as usize)
        },)
        {
            (mut __tmp_3,) => s.as_pointer().with(|__s: &Vec<i8>| {
                let mut __tmp1 = __s
                    [0_usize..::std::cmp::min(0_usize + __tmp_3, __s.len().saturating_sub(1))]
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
    out.as_pointer().write(p.with(|__s: &Vec<i8>| {
        let mut __tmp1 =
            __s[1_usize..::std::cmp::min(1_usize + 2_usize, __s.len().saturating_sub(1))].to_vec();
        __tmp1.push(0);
        __tmp1
    }));
    println!("pointer: {}", out.decay());
    out.as_pointer().write(
        Rc::new(RefCell::new(
            Rc::new(RefCell::new({
                let mut __bytes = Ptr::<i8>::from_string_literal(b"hello").to_c_bytes();
                __bytes.push(0);
                __bytes
            }))
            .as_pointer()
            .with(|__s: &Vec<i8>| {
                let mut __tmp1 = __s
                    [1_usize..::std::cmp::min(1_usize + 3_usize, __s.len().saturating_sub(1))]
                    .to_vec();
                __tmp1.push(0);
                __tmp1
            }),
        ))
        .as_pointer()
        .with(|__s: &Vec<i8>| {
            let mut __tmp1 = __s
                [1_usize..::std::cmp::min(1_usize + 1_usize, __s.len().saturating_sub(1))]
                .to_vec();
            __tmp1.push(0);
            __tmp1
        }),
    );
    println!("nested: {}", out.decay());
    out.as_pointer()
        .write(match ({ ((*s.borrow()).len() - 1) },) {
            (mut __tmp_4,) => s.as_pointer().with(|__s: &Vec<i8>| {
                let mut __tmp1 = __s
                    [__tmp_4..::std::cmp::min(__tmp_4 + 0_usize, __s.len().saturating_sub(1))]
                    .to_vec();
                __tmp1.push(0);
                __tmp1
            }),
        });
    println!("empty: {}", ((*out.borrow()).len() - 1));
    let field: Value<Vec<i8>> = Rc::new(RefCell::new({
        let mut __bytes = Ptr::<i8>::from_string_literal(b"hello").to_c_bytes();
        __bytes.push(0);
        __bytes
    }));
    let text: Value<Text> = Rc::new(RefCell::new(Text {
        value: field.as_pointer(),
    }));
    out.as_pointer()
        .write({ (*text.borrow()).value.clone() }.with(|__s: &Vec<i8>| {
            let mut __tmp1 = __s
                [1_usize..::std::cmp::min(1_usize + 2_usize, __s.len().saturating_sub(1))]
                .to_vec();
            __tmp1.push(0);
            __tmp1
        }));
    println!(
        "field: {} {}",
        out.decay(),
        { (*text.borrow()).value.clone() }.decay()
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
        strings
            .decay()
            .offset(index.postfix_inc())
            .with(|__s: &Vec<i8>| {
                let mut __tmp1 = __s
                    [1_usize..::std::cmp::min(1_usize + 2_usize, __s.len().saturating_sub(1))]
                    .to_vec();
                __tmp1.push(0);
                __tmp1
            }),
    );
    println!("element: {} {}", out.decay(), index);
    return 0;
}
pub fn __cpp2rust_init_globals() {}
