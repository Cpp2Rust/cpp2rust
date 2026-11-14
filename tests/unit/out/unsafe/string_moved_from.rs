extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn print_moved_from_0(
    mut label: *const libc::c_char,
    from: *const Vec<libc::c_char>,
    to: *const Vec<libc::c_char>,
) {
    let mut sub: Vec<libc::c_char> = {
        let __range = 0_usize
            ..::std::cmp::min(
                0_usize + 0_usize,
                (*(from as *const Vec<libc::c_char>)).len() - 1,
            );
        let mut __tmp1 = (&*(from as *const Vec<libc::c_char>))[__range].to_vec();
        __tmp1.push(0);
        __tmp1
    };
    (unsafe {
        libc::printf(
            c"%s: %zu %zu %d %zu %d [%s]\n".as_ptr() as *const libc::c_char,
            (label),
            ((*from).len() - 1),
            (sub.len() - 1),
            ((*(*from).as_ptr().offset((0) as isize)) as i32),
            ((*to).len() - 1),
            ((*sub.as_ptr().offset((0) as isize)) as i32),
            ((*to).as_ptr()),
        )
    });
}
pub unsafe fn return_local_1() -> Vec<libc::c_char> {
    let mut word: Vec<libc::c_char> = std::slice::from_raw_parts(
        c"returned local".as_ptr(),
        (0..)
            .take_while(|&i| *c"returned local".as_ptr().add(i) != 0)
            .count()
            + 1,
    )
    .to_vec();
    return std::mem::take(&mut (word));
}
pub unsafe fn return_parameter_2(mut word: Vec<libc::c_char>) -> Vec<libc::c_char> {
    return std::mem::take(&mut word);
}
pub unsafe fn return_reference_3(word: *mut Vec<libc::c_char>) -> Vec<libc::c_char> {
    return std::mem::replace(&mut (*word), vec![0]);
}
pub unsafe fn return_rvalue_reference_4(word: *mut Vec<libc::c_char>) -> Vec<libc::c_char> {
    return std::mem::replace(&mut (*word), vec![0]);
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut empty: Vec<libc::c_char> = vec![0];
    let mut from_empty: Vec<libc::c_char> = std::mem::replace(&mut empty, vec![0]);
    (unsafe { print_moved_from_0(c"construct empty".as_ptr(), &empty, &from_empty) });
    let mut word: Vec<libc::c_char> = std::slice::from_raw_parts(
        c"hello".as_ptr(),
        (0..)
            .take_while(|&i| *c"hello".as_ptr().add(i) != 0)
            .count()
            + 1,
    )
    .to_vec();
    let mut from_word: Vec<libc::c_char> = std::mem::replace(&mut word, vec![0]);
    (unsafe { print_moved_from_0(c"construct word".as_ptr(), &word, &from_word) });
    let mut long_word: Vec<libc::c_char> = std::slice::from_raw_parts(
        c"a string that does not fit in place".as_ptr(),
        (0..)
            .take_while(|&i| *c"a string that does not fit in place".as_ptr().add(i) != 0)
            .count()
            + 1,
    )
    .to_vec();
    let mut assigned: Vec<libc::c_char> = std::slice::from_raw_parts(
        c"x".as_ptr(),
        (0..).take_while(|&i| *c"x".as_ptr().add(i) != 0).count() + 1,
    )
    .to_vec();
    assigned = std::mem::replace(&mut long_word, vec![0]);
    (unsafe { print_moved_from_0(c"assign long".as_ptr(), &long_word, &assigned) });
    let mut short_word: Vec<libc::c_char> = std::slice::from_raw_parts(
        c"ab".as_ptr(),
        (0..).take_while(|&i| *c"ab".as_ptr().add(i) != 0).count() + 1,
    )
    .to_vec();
    let mut assigned_short: Vec<libc::c_char> = vec![0];
    assigned_short = std::mem::replace(&mut short_word, vec![0]);
    (unsafe { print_moved_from_0(c"assign short".as_ptr(), &short_word, &assigned_short) });
    word.splice(word.len().saturating_sub(1)..word.len(), {
        let mut v = ::std::slice::from_raw_parts(c"again".as_ptr(), 5_usize as usize).to_vec();
        v.push(0);
        v
    });
    (unsafe {
        libc::printf(
            c"reuse: %zu %s\n".as_ptr() as *const libc::c_char,
            (word.len() - 1),
            (word.as_ptr()),
        )
    });
    let mut local: Vec<libc::c_char> = (unsafe { return_local_1() });
    let mut parameter: Vec<libc::c_char> = (unsafe {
        return_parameter_2(
            std::slice::from_raw_parts(
                c"returned parameter".as_ptr(),
                (0..)
                    .take_while(|&i| *c"returned parameter".as_ptr().add(i) != 0)
                    .count()
                    + 1,
            )
            .to_vec(),
        )
    });
    (unsafe {
        libc::printf(
            c"returns: %s %s\n".as_ptr() as *const libc::c_char,
            (local.as_ptr()),
            (parameter.as_ptr()),
        )
    });
    let mut source: Vec<libc::c_char> = std::slice::from_raw_parts(
        c"returned reference".as_ptr(),
        (0..)
            .take_while(|&i| *c"returned reference".as_ptr().add(i) != 0)
            .count()
            + 1,
    )
    .to_vec();
    let mut reference: Vec<libc::c_char> = (unsafe { return_reference_3(&mut source) });
    (unsafe { print_moved_from_0(c"return reference".as_ptr(), &source, &reference) });
    source.splice(source.len().saturating_sub(1)..source.len(), {
        let mut v =
            ::std::slice::from_raw_parts(c"rvalue reference".as_ptr(), 16_usize as usize).to_vec();
        v.push(0);
        v
    });
    let mut rvalue_reference: Vec<libc::c_char> =
        (unsafe { return_rvalue_reference_4(&mut source) });
    (unsafe {
        print_moved_from_0(
            c"return rvalue reference".as_ptr(),
            &source,
            &rvalue_reference,
        )
    });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
