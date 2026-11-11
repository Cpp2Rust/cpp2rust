extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn print_moved_from_0(mut label: Ptr<i8>, from: Ptr<Vec<i8>>, to: Ptr<Vec<i8>>) {
    let sub: Value<Vec<i8>> = Rc::new(RefCell::new(from.with(|__s: &Vec<i8>| {
        let mut __tmp1 =
            __s[0_usize..::std::cmp::min(0_usize + 0_usize, __s.len().saturating_sub(1))].to_vec();
        __tmp1.push(0);
        __tmp1
    })));
    println!(
        "{}: {} {} {} {} {} [{}]",
        label,
        ((*from.upgrade().deref()).len() - 1),
        ((*sub.borrow()).len() - 1),
        ((elem!(from.decay(), 0).read()) as i32),
        ((*to.upgrade().deref()).len() - 1),
        ((elem!(sub.decay(), 0).read()) as i32),
        to.decay()
    );
}
pub fn return_local_1() -> Vec<i8> {
    let mut word: Vec<i8> = {
        let mut __bytes = Ptr::<i8>::from_string_literal(b"returned local").to_c_bytes();
        __bytes.push(0);
        __bytes
    };
    return std::mem::take(&mut (word));
}
pub fn return_parameter_2(mut word: Vec<i8>) -> Vec<i8> {
    return std::mem::take(&mut word);
}
pub fn return_reference_3(word: Ptr<Vec<i8>>) -> Vec<i8> {
    return word.with_mut(|__v| std::mem::replace(__v, vec![0]));
}
pub fn return_rvalue_reference_4(word: Ptr<Vec<i8>>) -> Vec<i8> {
    return word.with_mut(|__v| std::mem::replace(__v, vec![0]));
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let empty: Value<Vec<i8>> = Rc::new(RefCell::new(vec![0]));
    let from_empty: Value<Vec<i8>> = Rc::new(RefCell::new(
        ({ std::mem::replace(&mut (*empty.borrow_mut()), vec![0]) }),
    ));
    ({
        print_moved_from_0(
            Ptr::<i8>::from_string_literal(b"construct empty"),
            empty.as_pointer(),
            from_empty.as_pointer(),
        )
    });
    let word: Value<Vec<i8>> = Rc::new(RefCell::new({
        let mut __bytes = Ptr::<i8>::from_string_literal(b"hello").to_c_bytes();
        __bytes.push(0);
        __bytes
    }));
    let from_word: Value<Vec<i8>> = Rc::new(RefCell::new(
        ({ std::mem::replace(&mut (*word.borrow_mut()), vec![0]) }),
    ));
    ({
        print_moved_from_0(
            Ptr::<i8>::from_string_literal(b"construct word"),
            word.as_pointer(),
            from_word.as_pointer(),
        )
    });
    let long_word: Value<Vec<i8>> = Rc::new(RefCell::new({
        let mut __bytes =
            Ptr::<i8>::from_string_literal(b"a string that does not fit in place").to_c_bytes();
        __bytes.push(0);
        __bytes
    }));
    let assigned: Value<Vec<i8>> = Rc::new(RefCell::new({
        let mut __bytes = Ptr::<i8>::from_string_literal(b"x").to_c_bytes();
        __bytes.push(0);
        __bytes
    }));
    assigned
        .as_pointer()
        .write(({ std::mem::replace(&mut (*long_word.borrow_mut()), vec![0]) }));
    ({
        print_moved_from_0(
            Ptr::<i8>::from_string_literal(b"assign long"),
            long_word.as_pointer(),
            assigned.as_pointer(),
        )
    });
    let short_word: Value<Vec<i8>> = Rc::new(RefCell::new({
        let mut __bytes = Ptr::<i8>::from_string_literal(b"ab").to_c_bytes();
        __bytes.push(0);
        __bytes
    }));
    let assigned_short: Value<Vec<i8>> = Rc::new(RefCell::new(vec![0]));
    assigned_short
        .as_pointer()
        .write(({ std::mem::replace(&mut (*short_word.borrow_mut()), vec![0]) }));
    ({
        print_moved_from_0(
            Ptr::<i8>::from_string_literal(b"assign short"),
            short_word.as_pointer(),
            assigned_short.as_pointer(),
        )
    });
    {
        word.as_pointer().with_mut(|__v: &mut Vec<i8>| {
            __v.pop();
            Ptr::<i8>::from_string_literal(b"again")
                .with_slice(5_usize as usize, |__s| __v.extend_from_slice(__s));
            __v.push(0);
        });
        word.as_pointer()
    };
    println!("reuse: {} {}", ((*word.borrow()).len() - 1), word.decay());
    let local: Value<Vec<i8>> = Rc::new(RefCell::new(({ return_local_1() })));
    let parameter: Value<Vec<i8>> = Rc::new(RefCell::new(
        ({
            return_parameter_2({
                let mut __bytes =
                    Ptr::<i8>::from_string_literal(b"returned parameter").to_c_bytes();
                __bytes.push(0);
                __bytes
            })
        }),
    ));
    println!("returns: {} {}", local.decay(), parameter.decay());
    let source: Value<Vec<i8>> = Rc::new(RefCell::new({
        let mut __bytes = Ptr::<i8>::from_string_literal(b"returned reference").to_c_bytes();
        __bytes.push(0);
        __bytes
    }));
    let reference: Value<Vec<i8>> =
        Rc::new(RefCell::new(({ return_reference_3(source.as_pointer()) })));
    ({
        print_moved_from_0(
            Ptr::<i8>::from_string_literal(b"return reference"),
            source.as_pointer(),
            reference.as_pointer(),
        )
    });
    {
        source.as_pointer().with_mut(|__v: &mut Vec<i8>| {
            __v.pop();
            Ptr::<i8>::from_string_literal(b"rvalue reference")
                .with_slice(16_usize as usize, |__s| __v.extend_from_slice(__s));
            __v.push(0);
        });
        source.as_pointer()
    };
    let rvalue_reference: Value<Vec<i8>> = Rc::new(RefCell::new(
        ({ return_rvalue_reference_4(source.as_pointer()) }),
    ));
    ({
        print_moved_from_0(
            Ptr::<i8>::from_string_literal(b"return rvalue reference"),
            source.as_pointer(),
            rvalue_reference.as_pointer(),
        )
    });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
