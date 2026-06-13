extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let s1: Value<Vec<core::ffi::c_char>> = Rc::new(RefCell::new(
        Ptr::from_string_literal(b"hello")
            .to_c_string_iterator()
            .chain(std::iter::once(0))
            .collect::<Vec<core::ffi::c_char>>(),
    ));
    assert!((((*s1.borrow()).len() - 1) == 5_usize));
    assert!((((*s1.borrow()).len() - 1) == ((*s1.borrow()).len() - 1)));
    assert!(
        ((((s1.as_pointer() as Ptr<core::ffi::c_char>)
            .offset(0_usize as isize)
            .read()) as i32)
            == (('h' as core::ffi::c_char) as i32))
    );
    assert!(
        ((((s1.as_pointer() as Ptr<core::ffi::c_char>)
            .offset(1_usize as isize)
            .read()) as i32)
            == (('e' as core::ffi::c_char) as i32))
    );
    assert!(
        ((((s1.as_pointer() as Ptr<core::ffi::c_char>)
            .offset(2_usize as isize)
            .read()) as i32)
            == (('l' as core::ffi::c_char) as i32))
    );
    assert!(
        ((((s1.as_pointer() as Ptr<core::ffi::c_char>)
            .offset(3_usize as isize)
            .read()) as i32)
            == (('l' as core::ffi::c_char) as i32))
    );
    assert!(
        ((((s1.as_pointer() as Ptr<core::ffi::c_char>)
            .offset(4_usize as isize)
            .read()) as i32)
            == (('o' as core::ffi::c_char) as i32))
    );
    assert!(
        (*s1.borrow())
            .iter()
            .copied()
            .take((*s1.borrow()).len().saturating_sub(1))
            .eq(Ptr::from_string_literal(b"hello").to_c_string_iterator())
    );
    let p1: Value<Ptr<core::ffi::c_char>> =
        Rc::new(RefCell::new((s1.as_pointer() as Ptr<core::ffi::c_char>)));
    assert!(
        ((((*p1.borrow()).offset((0) as isize).read()) as i32)
            == (('h' as core::ffi::c_char) as i32))
    );
    assert!(
        ((((*p1.borrow()).offset((1) as isize).read()) as i32)
            == (('e' as core::ffi::c_char) as i32))
    );
    assert!(
        ((((*p1.borrow()).offset((2) as isize).read()) as i32)
            == (('l' as core::ffi::c_char) as i32))
    );
    assert!(
        ((((*p1.borrow()).offset((3) as isize).read()) as i32)
            == (('l' as core::ffi::c_char) as i32))
    );
    assert!(
        ((((*p1.borrow()).offset((4) as isize).read()) as i32)
            == (('o' as core::ffi::c_char) as i32))
    );
    let s2: Value<Vec<core::ffi::c_char>> = Rc::new(RefCell::new(
        vec![('a' as core::ffi::c_char); (10_usize) as usize]
            .iter()
            .cloned()
            .chain(std::iter::once(0))
            .collect(),
    ));
    let p2: Value<Ptr<core::ffi::c_char>> =
        Rc::new(RefCell::new((s2.as_pointer() as Ptr<core::ffi::c_char>)));
    let i: Value<u32> = Rc::new(RefCell::new(0_u32));
    'loop_: while (((*i.borrow()) as usize) < ((*s2.borrow()).len() - 1)) {
        assert!(
            ((((*p2.borrow()).offset((*i.borrow()) as isize).read()) as i32)
                == (('a' as core::ffi::c_char) as i32))
                && ((((s2.as_pointer() as Ptr<core::ffi::c_char>)
                    .offset(((*i.borrow()) as usize) as isize)
                    .read()) as i32)
                    == (('a' as core::ffi::c_char) as i32))
        );
        (*i.borrow_mut()).prefix_inc();
    }
    assert!((((*s2.borrow()).len() - 1) == 10_usize));
    assert!((((*s2.borrow()).len() - 1) == ((*s2.borrow()).len() - 1)));
    (s2.as_pointer() as Ptr<core::ffi::c_char>)
        .offset(0_usize as isize)
        .write(('b' as core::ffi::c_char));
    (s2.as_pointer() as Ptr<core::ffi::c_char>)
        .offset(1_usize as isize)
        .write(('c' as core::ffi::c_char));
    assert!(
        ((((s2.as_pointer() as Ptr<core::ffi::c_char>)
            .offset(0_usize as isize)
            .read()) as i32)
            == (('b' as core::ffi::c_char) as i32))
    );
    assert!(
        ((((s2.as_pointer() as Ptr<core::ffi::c_char>)
            .offset(1_usize as isize)
            .read()) as i32)
            == (('c' as core::ffi::c_char) as i32))
    );
    let i: Value<u32> = Rc::new(RefCell::new(2_u32));
    'loop_: while (((*i.borrow()) as usize) < ((*s2.borrow()).len() - 1)) {
        assert!(
            ((((*p2.borrow()).offset((*i.borrow()) as isize).read()) as i32)
                == (('a' as core::ffi::c_char) as i32))
                && ((((s2.as_pointer() as Ptr<core::ffi::c_char>)
                    .offset(((*i.borrow()) as usize) as isize)
                    .read()) as i32)
                    == (('a' as core::ffi::c_char) as i32))
        );
        (*i.borrow_mut()).prefix_inc();
    }
    let s3: Value<Vec<core::ffi::c_char>> = Rc::new(RefCell::new({
        let mut __tmp1 = (*s2.borrow())[(2_usize) as usize
            ..::std::cmp::min(
                (2_usize + 5_usize) as usize,
                (*s2.borrow()).len().saturating_sub(1),
            )]
            .to_vec();
        __tmp1.push(0);
        __tmp1
    }));
    assert!((((*s3.borrow()).len() - 1) == 5_usize));
    assert!((((*s3.borrow()).len() - 1) == ((*s3.borrow()).len() - 1)));
    let p3: Value<Ptr<core::ffi::c_char>> =
        Rc::new(RefCell::new((s3.as_pointer() as Ptr<core::ffi::c_char>)));
    let i: Value<u32> = Rc::new(RefCell::new(0_u32));
    'loop_: while (((*i.borrow()) as usize) < ((*s3.borrow()).len() - 1)) {
        assert!({
            let _lhs = (((*p3.borrow()).offset((*i.borrow()) as isize).read()) as i32);
            _lhs == (((s3.as_pointer() as Ptr<core::ffi::c_char>)
                .offset(((*i.borrow()) as usize) as isize)
                .read()) as i32)
        });
        (*i.borrow_mut()).prefix_inc();
    }
    let s4: Value<Vec<core::ffi::c_char>> = Rc::new(RefCell::new({
        let mut __tmp1 = (*s1.borrow())[(1_usize) as usize
            ..::std::cmp::min(
                (1_usize + {
                    let __lookup: Vec<core::ffi::c_char> = Ptr::from_string_literal(b"l")
                        .to_c_string_iterator()
                        .collect();
                    (*s1.borrow())
                        .iter()
                        .take((*s1.borrow()).len().saturating_sub(1))
                        .rposition(|&x| __lookup.contains(&x))
                        .unwrap_or(usize::MAX)
                }) as usize,
                (*s1.borrow()).len().saturating_sub(1),
            )]
            .to_vec();
        __tmp1.push(0);
        __tmp1
    }));
    assert!((((*s4.borrow()).len() - 1) == 3_usize));
    assert!((((*s4.borrow()).len() - 1) == ((*s4.borrow()).len() - 1)));
    let p4: Value<Ptr<core::ffi::c_char>> =
        Rc::new(RefCell::new((s4.as_pointer() as Ptr<core::ffi::c_char>)));
    let i: Value<u32> = Rc::new(RefCell::new(0_u32));
    'loop_: while (((*i.borrow()) as usize) < ((*s4.borrow()).len() - 1)) {
        assert!({
            let _lhs = (((*p4.borrow()).offset((*i.borrow()) as isize).read()) as i32);
            _lhs == (((s4.as_pointer() as Ptr<core::ffi::c_char>)
                .offset(((*i.borrow()) as usize) as isize)
                .read()) as i32)
        });
        (*i.borrow_mut()).prefix_inc();
    }
    let s5: Value<Vec<core::ffi::c_char>> = Rc::new(RefCell::new({
        let mut r = (*s1.borrow()).clone();
        r.pop();
        r.extend(Ptr::from_string_literal(b", world").to_c_string_iterator());
        r.push(0);
        r
    }));
    assert!((((*s5.borrow()).len() - 1) == 12_usize));
    assert!((((*s5.borrow()).len() - 1) == ((*s5.borrow()).len() - 1)));
    let p5: Value<Ptr<core::ffi::c_char>> =
        Rc::new(RefCell::new((s5.as_pointer() as Ptr<core::ffi::c_char>)));
    let i: Value<u32> = Rc::new(RefCell::new(0_u32));
    'loop_: while (((*i.borrow()) as usize) < ((*s5.borrow()).len() - 1)) {
        assert!({
            let _lhs = (((*p5.borrow()).offset((*i.borrow()) as isize).read()) as i32);
            _lhs == (((s5.as_pointer() as Ptr<core::ffi::c_char>)
                .offset(((*i.borrow()) as usize) as isize)
                .read()) as i32)
        });
        (*i.borrow_mut()).prefix_inc();
    }
    let arr: Value<Box<[core::ffi::c_char]>> = Rc::new(RefCell::new(Box::new([
        ('b' as core::ffi::c_char),
        ('a' as core::ffi::c_char),
        ('r' as core::ffi::c_char),
        (' ' as core::ffi::c_char),
        ('f' as core::ffi::c_char),
        ('o' as core::ffi::c_char),
        ('o' as core::ffi::c_char),
    ])));
    let string: Value<Vec<core::ffi::c_char>> = Rc::new(RefCell::new(
        (arr.as_pointer() as Ptr<core::ffi::c_char>)
            .map(|c| c.read())
            .take(3_usize as usize)
            .chain(std::iter::once(0))
            .collect::<Vec<core::ffi::c_char>>(),
    ));
    assert!((((*string.borrow()).len() - 1) == 3_usize));
    assert!(
        ((((string.as_pointer() as Ptr<core::ffi::c_char>)
            .offset(0_usize as isize)
            .read()) as i32)
            == (('b' as core::ffi::c_char) as i32))
    );
    assert!(
        ((((string.as_pointer() as Ptr<core::ffi::c_char>)
            .offset(1_usize as isize)
            .read()) as i32)
            == (('a' as core::ffi::c_char) as i32))
    );
    assert!(
        ((((string.as_pointer() as Ptr<core::ffi::c_char>)
            .offset(2_usize as isize)
            .read()) as i32)
            == (('r' as core::ffi::c_char) as i32))
    );
    assert!(
        (*string.borrow())
            .iter()
            .copied()
            .take((*string.borrow()).len().saturating_sub(1))
            .eq(Ptr::from_string_literal(b"bar").to_c_string_iterator())
    );
    {
        (*string.borrow_mut()).pop();
        (*string.borrow_mut()).resize((3_usize) as usize, 0);
        (*string.borrow_mut()).push(0)
    };
    assert!((((*string.borrow()).len() - 1) == 3_usize));
    assert!(
        ((((string.as_pointer() as Ptr<core::ffi::c_char>)
            .offset(0_usize as isize)
            .read()) as i32)
            == (('b' as core::ffi::c_char) as i32))
    );
    assert!(
        ((((string.as_pointer() as Ptr<core::ffi::c_char>)
            .offset(1_usize as isize)
            .read()) as i32)
            == (('a' as core::ffi::c_char) as i32))
    );
    assert!(
        ((((string.as_pointer() as Ptr<core::ffi::c_char>)
            .offset(2_usize as isize)
            .read()) as i32)
            == (('r' as core::ffi::c_char) as i32))
    );
    assert!(
        (*string.borrow())
            .iter()
            .copied()
            .take((*string.borrow()).len().saturating_sub(1))
            .eq(Ptr::from_string_literal(b"bar").to_c_string_iterator())
    );
    {
        (*string.borrow_mut()).pop();
        (*string.borrow_mut()).resize((5_usize) as usize, 0);
        (*string.borrow_mut()).push(0)
    };
    assert!((((*string.borrow()).len() - 1) == 5_usize));
    assert!(
        ((((string.as_pointer() as Ptr<core::ffi::c_char>)
            .offset(0_usize as isize)
            .read()) as i32)
            == (('b' as core::ffi::c_char) as i32))
    );
    assert!(
        ((((string.as_pointer() as Ptr<core::ffi::c_char>)
            .offset(1_usize as isize)
            .read()) as i32)
            == (('a' as core::ffi::c_char) as i32))
    );
    assert!(
        ((((string.as_pointer() as Ptr<core::ffi::c_char>)
            .offset(2_usize as isize)
            .read()) as i32)
            == (('r' as core::ffi::c_char) as i32))
    );
    assert!(
        ((((string.as_pointer() as Ptr<core::ffi::c_char>)
            .offset(3_usize as isize)
            .read()) as i32)
            == 0)
    );
    assert!(
        ((((string.as_pointer() as Ptr<core::ffi::c_char>)
            .offset(4_usize as isize)
            .read()) as i32)
            == 0)
    );
    (string.as_pointer() as Ptr<core::ffi::c_char>)
        .offset(3_usize as isize)
        .write(('a' as core::ffi::c_char));
    (string.as_pointer() as Ptr<core::ffi::c_char>)
        .offset(4_usize as isize)
        .write(('b' as core::ffi::c_char));
    assert!(
        ((((string.as_pointer() as Ptr<core::ffi::c_char>)
            .offset(3_usize as isize)
            .read()) as i32)
            == (('a' as core::ffi::c_char) as i32))
    );
    assert!(
        ((((string.as_pointer() as Ptr<core::ffi::c_char>)
            .offset(4_usize as isize)
            .read()) as i32)
            == (('b' as core::ffi::c_char) as i32))
    );
    (string.as_pointer() as Ptr<core::ffi::c_char>)
        .offset(3_usize as isize)
        .write((0 as core::ffi::c_char));
    (string.as_pointer() as Ptr<core::ffi::c_char>)
        .offset(4_usize as isize)
        .write((0 as core::ffi::c_char));
    {
        (*string.borrow_mut()).pop();
        (*string.borrow_mut()).resize((4_usize) as usize, 0);
        (*string.borrow_mut()).push(0)
    };
    assert!((((*string.borrow()).len() - 1) == 4_usize));
    assert!(
        ((((string.as_pointer() as Ptr<core::ffi::c_char>)
            .offset(0_usize as isize)
            .read()) as i32)
            == (('b' as core::ffi::c_char) as i32))
    );
    assert!(
        ((((string.as_pointer() as Ptr<core::ffi::c_char>)
            .offset(1_usize as isize)
            .read()) as i32)
            == (('a' as core::ffi::c_char) as i32))
    );
    assert!(
        ((((string.as_pointer() as Ptr<core::ffi::c_char>)
            .offset(2_usize as isize)
            .read()) as i32)
            == (('r' as core::ffi::c_char) as i32))
    );
    assert!(
        ((((string.as_pointer() as Ptr<core::ffi::c_char>)
            .offset(3_usize as isize)
            .read()) as i32)
            == 0)
    );
    let result: Value<Vec<core::ffi::c_char>> = Rc::new(RefCell::new({
        let mut r = (*string.borrow()).clone();
        r.pop();
        r.extend(Ptr::from_string_literal(b" foo").to_c_string_iterator());
        r.push(0);
        r
    }));
    assert!((((*result.borrow()).len() - 1) == 8_usize));
    assert!(
        ((((result.as_pointer() as Ptr<core::ffi::c_char>)
            .offset(0_usize as isize)
            .read()) as i32)
            == (('b' as core::ffi::c_char) as i32))
    );
    assert!(
        ((((result.as_pointer() as Ptr<core::ffi::c_char>)
            .offset(1_usize as isize)
            .read()) as i32)
            == (('a' as core::ffi::c_char) as i32))
    );
    assert!(
        ((((result.as_pointer() as Ptr<core::ffi::c_char>)
            .offset(2_usize as isize)
            .read()) as i32)
            == (('r' as core::ffi::c_char) as i32))
    );
    assert!(
        ((((result.as_pointer() as Ptr<core::ffi::c_char>)
            .offset(3_usize as isize)
            .read()) as i32)
            == 0)
    );
    assert!(
        ((((result.as_pointer() as Ptr<core::ffi::c_char>)
            .offset(4_usize as isize)
            .read()) as i32)
            == ((' ' as core::ffi::c_char) as i32))
    );
    assert!(
        ((((result.as_pointer() as Ptr<core::ffi::c_char>)
            .offset(5_usize as isize)
            .read()) as i32)
            == (('f' as core::ffi::c_char) as i32))
    );
    assert!(
        ((((result.as_pointer() as Ptr<core::ffi::c_char>)
            .offset(6_usize as isize)
            .read()) as i32)
            == (('o' as core::ffi::c_char) as i32))
    );
    assert!(
        ((((result.as_pointer() as Ptr<core::ffi::c_char>)
            .offset(7_usize as isize)
            .read()) as i32)
            == (('o' as core::ffi::c_char) as i32))
    );
    let substr_0: Value<Vec<core::ffi::c_char>> = Rc::new(RefCell::new({
        let mut __tmp1 = (*result.borrow())[(5_usize) as usize
            ..::std::cmp::min(
                (5_usize + 3_usize) as usize,
                (*result.borrow()).len().saturating_sub(1),
            )]
            .to_vec();
        __tmp1.push(0);
        __tmp1
    }));
    assert!((((*substr_0.borrow()).len() - 1) == 3_usize));
    assert!(
        ((((substr_0.as_pointer() as Ptr<core::ffi::c_char>)
            .offset(0_usize as isize)
            .read()) as i32)
            == (('f' as core::ffi::c_char) as i32))
    );
    assert!(
        ((((substr_0.as_pointer() as Ptr<core::ffi::c_char>)
            .offset(1_usize as isize)
            .read()) as i32)
            == (('o' as core::ffi::c_char) as i32))
    );
    assert!(
        ((((substr_0.as_pointer() as Ptr<core::ffi::c_char>)
            .offset(2_usize as isize)
            .read()) as i32)
            == (('o' as core::ffi::c_char) as i32))
    );
    let substr_1: Value<Vec<core::ffi::c_char>> = Rc::new(RefCell::new({
        let mut __tmp1 = (*result.borrow())[(0_usize) as usize
            ..::std::cmp::min(
                (0_usize + 5_usize) as usize,
                (*result.borrow()).len().saturating_sub(1),
            )]
            .to_vec();
        __tmp1.push(0);
        __tmp1
    }));
    assert!((((*substr_1.borrow()).len() - 1) == 5_usize));
    assert!(
        ((((substr_1.as_pointer() as Ptr<core::ffi::c_char>)
            .offset(0_usize as isize)
            .read()) as i32)
            == (('b' as core::ffi::c_char) as i32))
    );
    assert!(
        ((((substr_1.as_pointer() as Ptr<core::ffi::c_char>)
            .offset(1_usize as isize)
            .read()) as i32)
            == (('a' as core::ffi::c_char) as i32))
    );
    assert!(
        ((((substr_1.as_pointer() as Ptr<core::ffi::c_char>)
            .offset(2_usize as isize)
            .read()) as i32)
            == (('r' as core::ffi::c_char) as i32))
    );
    assert!(
        ((((substr_1.as_pointer() as Ptr<core::ffi::c_char>)
            .offset(3_usize as isize)
            .read()) as i32)
            == 0)
    );
    assert!(
        ((((substr_1.as_pointer() as Ptr<core::ffi::c_char>)
            .offset(4_usize as isize)
            .read()) as i32)
            == ((' ' as core::ffi::c_char) as i32))
    );
    let substr_2: Value<Vec<core::ffi::c_char>> = Rc::new(RefCell::new({
        let mut __tmp1 = (*result.borrow())[(0_usize) as usize
            ..::std::cmp::min(
                (0_usize + 15_usize) as usize,
                (*result.borrow()).len().saturating_sub(1),
            )]
            .to_vec();
        __tmp1.push(0);
        __tmp1
    }));
    assert!((((*substr_2.borrow()).len() - 1) == 8_usize));
    assert!(
        ((((substr_2.as_pointer() as Ptr<core::ffi::c_char>)
            .offset(0_usize as isize)
            .read()) as i32)
            == (('b' as core::ffi::c_char) as i32))
    );
    assert!(
        ((((substr_2.as_pointer() as Ptr<core::ffi::c_char>)
            .offset(1_usize as isize)
            .read()) as i32)
            == (('a' as core::ffi::c_char) as i32))
    );
    assert!(
        ((((substr_2.as_pointer() as Ptr<core::ffi::c_char>)
            .offset(2_usize as isize)
            .read()) as i32)
            == (('r' as core::ffi::c_char) as i32))
    );
    assert!(
        ((((substr_2.as_pointer() as Ptr<core::ffi::c_char>)
            .offset(3_usize as isize)
            .read()) as i32)
            == 0)
    );
    assert!(
        ((((substr_2.as_pointer() as Ptr<core::ffi::c_char>)
            .offset(4_usize as isize)
            .read()) as i32)
            == ((' ' as core::ffi::c_char) as i32))
    );
    assert!(
        ((((substr_2.as_pointer() as Ptr<core::ffi::c_char>)
            .offset(5_usize as isize)
            .read()) as i32)
            == (('f' as core::ffi::c_char) as i32))
    );
    assert!(
        ((((substr_2.as_pointer() as Ptr<core::ffi::c_char>)
            .offset(6_usize as isize)
            .read()) as i32)
            == (('o' as core::ffi::c_char) as i32))
    );
    assert!(
        ((((substr_2.as_pointer() as Ptr<core::ffi::c_char>)
            .offset(7_usize as isize)
            .read()) as i32)
            == (('o' as core::ffi::c_char) as i32))
    );
    let pos: Value<usize> = Rc::new(RefCell::new({
        let __lookup: Vec<core::ffi::c_char> = Ptr::from_string_literal(b"b")
            .to_c_string_iterator()
            .collect();
        (*result.borrow())
            .iter()
            .take((*result.borrow()).len().saturating_sub(1))
            .rposition(|&x| __lookup.contains(&x))
            .unwrap_or(usize::MAX)
    }));
    assert!(((*pos.borrow()) == 0_usize));
    (*pos.borrow_mut()) = {
        let __lookup: Vec<core::ffi::c_char> = Ptr::from_string_literal(b"f")
            .to_c_string_iterator()
            .collect();
        (*result.borrow())
            .iter()
            .take((*result.borrow()).len().saturating_sub(1))
            .rposition(|&x| __lookup.contains(&x))
            .unwrap_or(usize::MAX)
    };
    assert!(((*pos.borrow()) == 5_usize));
    (*pos.borrow_mut()) = {
        let __lookup: Vec<core::ffi::c_char> = Ptr::from_string_literal(b"o")
            .to_c_string_iterator()
            .collect();
        (*result.borrow())
            .iter()
            .take((*result.borrow()).len().saturating_sub(1))
            .rposition(|&x| __lookup.contains(&x))
            .unwrap_or(usize::MAX)
    };
    assert!(((*pos.borrow()) == 7_usize));
    (*pos.borrow_mut()) = {
        let __lookup: Vec<core::ffi::c_char> = Ptr::from_string_literal(b"x")
            .to_c_string_iterator()
            .collect();
        (*result.borrow())
            .iter()
            .take((*result.borrow()).len().saturating_sub(1))
            .rposition(|&x| __lookup.contains(&x))
            .unwrap_or(usize::MAX)
    };
    assert!(((*pos.borrow()) == ((-1_i64 as u64) as usize)));
    let string_to_cast: Value<Vec<core::ffi::c_char>> = Rc::new(RefCell::new(
        Ptr::from_string_literal(b"cast")
            .to_c_string_iterator()
            .chain(std::iter::once(0))
            .collect::<Vec<core::ffi::c_char>>(),
    ));
    let output_data: Value<Ptr<u8>> = Rc::new(RefCell::new(
        ((string_to_cast.as_pointer() as Ptr<core::ffi::c_char>).offset(0_usize as isize))
            .reinterpret_cast::<u8>(),
    ));
    assert!(((((*output_data.borrow()).read()) as i32) == (('c' as core::ffi::c_char) as i32)));
    assert!(
        ((((*output_data.borrow()).offset((1) as isize).read()) as i32)
            == (('a' as core::ffi::c_char) as i32))
    );
    assert!(
        ((((*output_data.borrow()).offset((2) as isize).read()) as i32)
            == (('s' as core::ffi::c_char) as i32))
    );
    assert!(
        ((((*output_data.borrow()).offset((3) as isize).read()) as i32)
            == (('t' as core::ffi::c_char) as i32))
    );
    let t0: Value<usize> = Rc::new(RefCell::new(((*s1.borrow()).len() - 1)));
    let t1: Value<usize> = Rc::new(RefCell::new(
        (*t0.borrow()).wrapping_add((((*p1.borrow()).read()) as usize)),
    ));
    let t2: Value<usize> = Rc::new(RefCell::new(
        (*t1.borrow()).wrapping_add(((*s2.borrow()).len() - 1)),
    ));
    let t3: Value<usize> = Rc::new(RefCell::new(
        (*t2.borrow()).wrapping_add(((*s3.borrow()).len() - 1)),
    ));
    let t4: Value<usize> = Rc::new(RefCell::new(
        (*t3.borrow()).wrapping_add(((*s4.borrow()).len() - 1)),
    ));
    return (((*t4.borrow()).wrapping_add(((*s5.borrow()).len() - 1))) as i32);
}
