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
    let s1: Value<Vec<u8>> = Rc::new(RefCell::new(
        Ptr::from_string_literal("hello")
            .to_c_string_iterator()
            .chain(std::iter::once(0))
            .collect::<Vec<u8>>(),
    ));
    assert!((((*s1.borrow()).len() - 1) as u64 == 5_u64));
    assert!((((*s1.borrow()).len() - 1) as u64 == ((*s1.borrow()).len() - 1) as u64));
    assert!(
        ((((s1.as_pointer() as Ptr<u8>).offset(0_u64 as isize).read()) as i32)
            == (('h' as u8) as i32))
    );
    assert!(
        ((((s1.as_pointer() as Ptr<u8>).offset(1_u64 as isize).read()) as i32)
            == (('e' as u8) as i32))
    );
    assert!(
        ((((s1.as_pointer() as Ptr<u8>).offset(2_u64 as isize).read()) as i32)
            == (('l' as u8) as i32))
    );
    assert!(
        ((((s1.as_pointer() as Ptr<u8>).offset(3_u64 as isize).read()) as i32)
            == (('l' as u8) as i32))
    );
    assert!(
        ((((s1.as_pointer() as Ptr<u8>).offset(4_u64 as isize).read()) as i32)
            == (('o' as u8) as i32))
    );
    assert!((*s1.borrow())
        .iter()
        .copied()
        .take((*s1.borrow()).len() - 1)
        .eq(Ptr::from_string_literal("hello").to_c_string_iterator()));
    let p1: Value<Ptr<u8>> = Rc::new(RefCell::new((s1.as_pointer() as Ptr<u8>)));
    assert!(((((*p1.borrow()).offset((0) as isize).read()) as i32) == (('h' as u8) as i32)));
    assert!(((((*p1.borrow()).offset((1) as isize).read()) as i32) == (('e' as u8) as i32)));
    assert!(((((*p1.borrow()).offset((2) as isize).read()) as i32) == (('l' as u8) as i32)));
    assert!(((((*p1.borrow()).offset((3) as isize).read()) as i32) == (('l' as u8) as i32)));
    assert!(((((*p1.borrow()).offset((4) as isize).read()) as i32) == (('o' as u8) as i32)));
    let s2: Value<Vec<u8>> = Rc::new(RefCell::new(
        vec![('a' as u8); (10_u64) as usize]
            .iter()
            .cloned()
            .chain(std::iter::once(0))
            .collect(),
    ));
    let p2: Value<Ptr<u8>> = Rc::new(RefCell::new((s2.as_pointer() as Ptr<u8>)));
    let i: Value<u32> = Rc::new(RefCell::new(0_u32));
    'loop_: while (((*i.borrow()) as u64) < ((*s2.borrow()).len() - 1) as u64) {
        assert!(
            ((((*p2.borrow()).offset((*i.borrow()) as isize).read()) as i32)
                == (('a' as u8) as i32))
                && ((((s2.as_pointer() as Ptr<u8>)
                    .offset(((*i.borrow()) as u64) as isize)
                    .read()) as i32)
                    == (('a' as u8) as i32))
        );
        (*i.borrow_mut()).prefix_inc();
    }
    assert!((((*s2.borrow()).len() - 1) as u64 == 10_u64));
    assert!((((*s2.borrow()).len() - 1) as u64 == ((*s2.borrow()).len() - 1) as u64));
    (s2.as_pointer() as Ptr<u8>)
        .offset(0_u64 as isize)
        .write(('b' as u8));
    (s2.as_pointer() as Ptr<u8>)
        .offset(1_u64 as isize)
        .write(('c' as u8));
    assert!(
        ((((s2.as_pointer() as Ptr<u8>).offset(0_u64 as isize).read()) as i32)
            == (('b' as u8) as i32))
    );
    assert!(
        ((((s2.as_pointer() as Ptr<u8>).offset(1_u64 as isize).read()) as i32)
            == (('c' as u8) as i32))
    );
    let i: Value<u32> = Rc::new(RefCell::new(2_u32));
    'loop_: while (((*i.borrow()) as u64) < ((*s2.borrow()).len() - 1) as u64) {
        assert!(
            ((((*p2.borrow()).offset((*i.borrow()) as isize).read()) as i32)
                == (('a' as u8) as i32))
                && ((((s2.as_pointer() as Ptr<u8>)
                    .offset(((*i.borrow()) as u64) as isize)
                    .read()) as i32)
                    == (('a' as u8) as i32))
        );
        (*i.borrow_mut()).prefix_inc();
    }
    let s3: Value<Vec<u8>> = Rc::new(RefCell::new({
        let mut __tmp1 = (*s2.borrow())
            [(2_u64) as usize..::std::cmp::min((2_u64 + 5_u64) as usize, (*s2.borrow()).len() - 1)]
            .to_vec();
        __tmp1.push(0);
        __tmp1
    }));
    assert!((((*s3.borrow()).len() - 1) as u64 == 5_u64));
    assert!((((*s3.borrow()).len() - 1) as u64 == ((*s3.borrow()).len() - 1) as u64));
    let p3: Value<Ptr<u8>> = Rc::new(RefCell::new((s3.as_pointer() as Ptr<u8>)));
    let i: Value<u32> = Rc::new(RefCell::new(0_u32));
    'loop_: while (((*i.borrow()) as u64) < ((*s3.borrow()).len() - 1) as u64) {
        assert!({
            let _lhs = (((*p3.borrow()).offset((*i.borrow()) as isize).read()) as i32);
            _lhs == (((s3.as_pointer() as Ptr<u8>)
                .offset(((*i.borrow()) as u64) as isize)
                .read()) as i32)
        });
        (*i.borrow_mut()).prefix_inc();
    }
    let s4: Value<Vec<u8>> = Rc::new(RefCell::new({
        let mut __tmp1 = (*s1.borrow())[(1_u64) as usize
            ..::std::cmp::min(
                (1_u64
                    + match (*s1.borrow())
                        .iter()
                        .take((*s1.borrow()).len() - 1)
                        .rposition(|&x| {
                            Ptr::from_string_literal("l")
                                .to_c_string_iterator()
                                .position(|ch| ch == x)
                                .is_some()
                        }) {
                        Some(idx) => idx as u64,
                        None => u64::MAX,
                    }) as usize,
                (*s1.borrow()).len() - 1,
            )]
            .to_vec();
        __tmp1.push(0);
        __tmp1
    }));
    assert!((((*s4.borrow()).len() - 1) as u64 == 3_u64));
    assert!((((*s4.borrow()).len() - 1) as u64 == ((*s4.borrow()).len() - 1) as u64));
    let p4: Value<Ptr<u8>> = Rc::new(RefCell::new((s4.as_pointer() as Ptr<u8>)));
    let i: Value<u32> = Rc::new(RefCell::new(0_u32));
    'loop_: while (((*i.borrow()) as u64) < ((*s4.borrow()).len() - 1) as u64) {
        assert!({
            let _lhs = (((*p4.borrow()).offset((*i.borrow()) as isize).read()) as i32);
            _lhs == (((s4.as_pointer() as Ptr<u8>)
                .offset(((*i.borrow()) as u64) as isize)
                .read()) as i32)
        });
        (*i.borrow_mut()).prefix_inc();
    }
    let s5: Value<Vec<u8>> = Rc::new(RefCell::new({
        let mut r = (*s1.borrow()).clone();
        r.pop();
        r.extend(Ptr::from_string_literal(", world").to_c_string_iterator());
        r.push(0);
        r
    }));
    assert!((((*s5.borrow()).len() - 1) as u64 == 12_u64));
    assert!((((*s5.borrow()).len() - 1) as u64 == ((*s5.borrow()).len() - 1) as u64));
    let p5: Value<Ptr<u8>> = Rc::new(RefCell::new((s5.as_pointer() as Ptr<u8>)));
    let i: Value<u32> = Rc::new(RefCell::new(0_u32));
    'loop_: while (((*i.borrow()) as u64) < ((*s5.borrow()).len() - 1) as u64) {
        assert!({
            let _lhs = (((*p5.borrow()).offset((*i.borrow()) as isize).read()) as i32);
            _lhs == (((s5.as_pointer() as Ptr<u8>)
                .offset(((*i.borrow()) as u64) as isize)
                .read()) as i32)
        });
        (*i.borrow_mut()).prefix_inc();
    }
    let arr: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([
        ('b' as u8),
        ('a' as u8),
        ('r' as u8),
        (' ' as u8),
        ('f' as u8),
        ('o' as u8),
        ('o' as u8),
    ])));
    let string: Value<Vec<u8>> = Rc::new(RefCell::new(
        (arr.as_pointer() as Ptr<u8>)
            .map(|c| c.read())
            .take(3_u64 as usize)
            .chain(std::iter::once(0))
            .collect::<Vec<u8>>(),
    ));
    assert!((((*string.borrow()).len() - 1) as u64 == 3_u64));
    assert!(
        ((((string.as_pointer() as Ptr<u8>)
            .offset(0_u64 as isize)
            .read()) as i32)
            == (('b' as u8) as i32))
    );
    assert!(
        ((((string.as_pointer() as Ptr<u8>)
            .offset(1_u64 as isize)
            .read()) as i32)
            == (('a' as u8) as i32))
    );
    assert!(
        ((((string.as_pointer() as Ptr<u8>)
            .offset(2_u64 as isize)
            .read()) as i32)
            == (('r' as u8) as i32))
    );
    assert!((*string.borrow())
        .iter()
        .copied()
        .take((*string.borrow()).len() - 1)
        .eq(Ptr::from_string_literal("bar").to_c_string_iterator()));
    {
        (*string.borrow_mut()).pop();
        (*string.borrow_mut()).resize((3_u64) as usize, 0);
        (*string.borrow_mut()).push(0)
    };
    assert!((((*string.borrow()).len() - 1) as u64 == 3_u64));
    assert!(
        ((((string.as_pointer() as Ptr<u8>)
            .offset(0_u64 as isize)
            .read()) as i32)
            == (('b' as u8) as i32))
    );
    assert!(
        ((((string.as_pointer() as Ptr<u8>)
            .offset(1_u64 as isize)
            .read()) as i32)
            == (('a' as u8) as i32))
    );
    assert!(
        ((((string.as_pointer() as Ptr<u8>)
            .offset(2_u64 as isize)
            .read()) as i32)
            == (('r' as u8) as i32))
    );
    assert!((*string.borrow())
        .iter()
        .copied()
        .take((*string.borrow()).len() - 1)
        .eq(Ptr::from_string_literal("bar").to_c_string_iterator()));
    {
        (*string.borrow_mut()).pop();
        (*string.borrow_mut()).resize((5_u64) as usize, 0);
        (*string.borrow_mut()).push(0)
    };
    assert!((((*string.borrow()).len() - 1) as u64 == 5_u64));
    assert!(
        ((((string.as_pointer() as Ptr<u8>)
            .offset(0_u64 as isize)
            .read()) as i32)
            == (('b' as u8) as i32))
    );
    assert!(
        ((((string.as_pointer() as Ptr<u8>)
            .offset(1_u64 as isize)
            .read()) as i32)
            == (('a' as u8) as i32))
    );
    assert!(
        ((((string.as_pointer() as Ptr<u8>)
            .offset(2_u64 as isize)
            .read()) as i32)
            == (('r' as u8) as i32))
    );
    assert!(
        ((((string.as_pointer() as Ptr<u8>)
            .offset(3_u64 as isize)
            .read()) as i32)
            == 0)
    );
    assert!(
        ((((string.as_pointer() as Ptr<u8>)
            .offset(4_u64 as isize)
            .read()) as i32)
            == 0)
    );
    (string.as_pointer() as Ptr<u8>)
        .offset(3_u64 as isize)
        .write(('a' as u8));
    (string.as_pointer() as Ptr<u8>)
        .offset(4_u64 as isize)
        .write(('b' as u8));
    assert!(
        ((((string.as_pointer() as Ptr<u8>)
            .offset(3_u64 as isize)
            .read()) as i32)
            == (('a' as u8) as i32))
    );
    assert!(
        ((((string.as_pointer() as Ptr<u8>)
            .offset(4_u64 as isize)
            .read()) as i32)
            == (('b' as u8) as i32))
    );
    (string.as_pointer() as Ptr<u8>)
        .offset(3_u64 as isize)
        .write(0_u8);
    (string.as_pointer() as Ptr<u8>)
        .offset(4_u64 as isize)
        .write(0_u8);
    {
        (*string.borrow_mut()).pop();
        (*string.borrow_mut()).resize((4_u64) as usize, 0);
        (*string.borrow_mut()).push(0)
    };
    assert!((((*string.borrow()).len() - 1) as u64 == 4_u64));
    assert!(
        ((((string.as_pointer() as Ptr<u8>)
            .offset(0_u64 as isize)
            .read()) as i32)
            == (('b' as u8) as i32))
    );
    assert!(
        ((((string.as_pointer() as Ptr<u8>)
            .offset(1_u64 as isize)
            .read()) as i32)
            == (('a' as u8) as i32))
    );
    assert!(
        ((((string.as_pointer() as Ptr<u8>)
            .offset(2_u64 as isize)
            .read()) as i32)
            == (('r' as u8) as i32))
    );
    assert!(
        ((((string.as_pointer() as Ptr<u8>)
            .offset(3_u64 as isize)
            .read()) as i32)
            == 0)
    );
    let result: Value<Vec<u8>> = Rc::new(RefCell::new({
        let mut r = (*string.borrow()).clone();
        r.pop();
        r.extend(Ptr::from_string_literal(" foo").to_c_string_iterator());
        r.push(0);
        r
    }));
    assert!((((*result.borrow()).len() - 1) as u64 == 8_u64));
    assert!(
        ((((result.as_pointer() as Ptr<u8>)
            .offset(0_u64 as isize)
            .read()) as i32)
            == (('b' as u8) as i32))
    );
    assert!(
        ((((result.as_pointer() as Ptr<u8>)
            .offset(1_u64 as isize)
            .read()) as i32)
            == (('a' as u8) as i32))
    );
    assert!(
        ((((result.as_pointer() as Ptr<u8>)
            .offset(2_u64 as isize)
            .read()) as i32)
            == (('r' as u8) as i32))
    );
    assert!(
        ((((result.as_pointer() as Ptr<u8>)
            .offset(3_u64 as isize)
            .read()) as i32)
            == 0)
    );
    assert!(
        ((((result.as_pointer() as Ptr<u8>)
            .offset(4_u64 as isize)
            .read()) as i32)
            == ((' ' as u8) as i32))
    );
    assert!(
        ((((result.as_pointer() as Ptr<u8>)
            .offset(5_u64 as isize)
            .read()) as i32)
            == (('f' as u8) as i32))
    );
    assert!(
        ((((result.as_pointer() as Ptr<u8>)
            .offset(6_u64 as isize)
            .read()) as i32)
            == (('o' as u8) as i32))
    );
    assert!(
        ((((result.as_pointer() as Ptr<u8>)
            .offset(7_u64 as isize)
            .read()) as i32)
            == (('o' as u8) as i32))
    );
    let substr_0: Value<Vec<u8>> = Rc::new(RefCell::new({
        let mut __tmp1 = (*result.borrow())[(5_u64) as usize
            ..::std::cmp::min((5_u64 + 3_u64) as usize, (*result.borrow()).len() - 1)]
            .to_vec();
        __tmp1.push(0);
        __tmp1
    }));
    assert!((((*substr_0.borrow()).len() - 1) as u64 == 3_u64));
    assert!(
        ((((substr_0.as_pointer() as Ptr<u8>)
            .offset(0_u64 as isize)
            .read()) as i32)
            == (('f' as u8) as i32))
    );
    assert!(
        ((((substr_0.as_pointer() as Ptr<u8>)
            .offset(1_u64 as isize)
            .read()) as i32)
            == (('o' as u8) as i32))
    );
    assert!(
        ((((substr_0.as_pointer() as Ptr<u8>)
            .offset(2_u64 as isize)
            .read()) as i32)
            == (('o' as u8) as i32))
    );
    let substr_1: Value<Vec<u8>> = Rc::new(RefCell::new({
        let mut __tmp1 = (*result.borrow())[(0_u64) as usize
            ..::std::cmp::min((0_u64 + 5_u64) as usize, (*result.borrow()).len() - 1)]
            .to_vec();
        __tmp1.push(0);
        __tmp1
    }));
    assert!((((*substr_1.borrow()).len() - 1) as u64 == 5_u64));
    assert!(
        ((((substr_1.as_pointer() as Ptr<u8>)
            .offset(0_u64 as isize)
            .read()) as i32)
            == (('b' as u8) as i32))
    );
    assert!(
        ((((substr_1.as_pointer() as Ptr<u8>)
            .offset(1_u64 as isize)
            .read()) as i32)
            == (('a' as u8) as i32))
    );
    assert!(
        ((((substr_1.as_pointer() as Ptr<u8>)
            .offset(2_u64 as isize)
            .read()) as i32)
            == (('r' as u8) as i32))
    );
    assert!(
        ((((substr_1.as_pointer() as Ptr<u8>)
            .offset(3_u64 as isize)
            .read()) as i32)
            == 0)
    );
    assert!(
        ((((substr_1.as_pointer() as Ptr<u8>)
            .offset(4_u64 as isize)
            .read()) as i32)
            == ((' ' as u8) as i32))
    );
    let substr_2: Value<Vec<u8>> = Rc::new(RefCell::new({
        let mut __tmp1 = (*result.borrow())[(0_u64) as usize
            ..::std::cmp::min((0_u64 + 15_u64) as usize, (*result.borrow()).len() - 1)]
            .to_vec();
        __tmp1.push(0);
        __tmp1
    }));
    assert!((((*substr_2.borrow()).len() - 1) as u64 == 8_u64));
    assert!(
        ((((substr_2.as_pointer() as Ptr<u8>)
            .offset(0_u64 as isize)
            .read()) as i32)
            == (('b' as u8) as i32))
    );
    assert!(
        ((((substr_2.as_pointer() as Ptr<u8>)
            .offset(1_u64 as isize)
            .read()) as i32)
            == (('a' as u8) as i32))
    );
    assert!(
        ((((substr_2.as_pointer() as Ptr<u8>)
            .offset(2_u64 as isize)
            .read()) as i32)
            == (('r' as u8) as i32))
    );
    assert!(
        ((((substr_2.as_pointer() as Ptr<u8>)
            .offset(3_u64 as isize)
            .read()) as i32)
            == 0)
    );
    assert!(
        ((((substr_2.as_pointer() as Ptr<u8>)
            .offset(4_u64 as isize)
            .read()) as i32)
            == ((' ' as u8) as i32))
    );
    assert!(
        ((((substr_2.as_pointer() as Ptr<u8>)
            .offset(5_u64 as isize)
            .read()) as i32)
            == (('f' as u8) as i32))
    );
    assert!(
        ((((substr_2.as_pointer() as Ptr<u8>)
            .offset(6_u64 as isize)
            .read()) as i32)
            == (('o' as u8) as i32))
    );
    assert!(
        ((((substr_2.as_pointer() as Ptr<u8>)
            .offset(7_u64 as isize)
            .read()) as i32)
            == (('o' as u8) as i32))
    );
    let pos: Value<u64> = Rc::new(RefCell::new(
        match (*result.borrow())
            .iter()
            .take((*result.borrow()).len() - 1)
            .rposition(|&x| {
                Ptr::from_string_literal("b")
                    .to_c_string_iterator()
                    .position(|ch| ch == x)
                    .is_some()
            }) {
            Some(idx) => idx as u64,
            None => u64::MAX,
        },
    ));
    assert!(((*pos.borrow()) == 0_u64));
    (*pos.borrow_mut()) = match (*result.borrow())
        .iter()
        .take((*result.borrow()).len() - 1)
        .rposition(|&x| {
            Ptr::from_string_literal("f")
                .to_c_string_iterator()
                .position(|ch| ch == x)
                .is_some()
        }) {
        Some(idx) => idx as u64,
        None => u64::MAX,
    };
    assert!(((*pos.borrow()) == 5_u64));
    (*pos.borrow_mut()) = match (*result.borrow())
        .iter()
        .take((*result.borrow()).len() - 1)
        .rposition(|&x| {
            Ptr::from_string_literal("o")
                .to_c_string_iterator()
                .position(|ch| ch == x)
                .is_some()
        }) {
        Some(idx) => idx as u64,
        None => u64::MAX,
    };
    assert!(((*pos.borrow()) == 7_u64));
    (*pos.borrow_mut()) = match (*result.borrow())
        .iter()
        .take((*result.borrow()).len() - 1)
        .rposition(|&x| {
            Ptr::from_string_literal("x")
                .to_c_string_iterator()
                .position(|ch| ch == x)
                .is_some()
        }) {
        Some(idx) => idx as u64,
        None => u64::MAX,
    };
    assert!(((*pos.borrow()) == (-1_i64 as u64)));
    let string_to_cast: Value<Vec<u8>> = Rc::new(RefCell::new(
        Ptr::from_string_literal("cast")
            .to_c_string_iterator()
            .chain(std::iter::once(0))
            .collect::<Vec<u8>>(),
    ));
    let output_data: Value<Ptr<u8>> = Rc::new(RefCell::new(
        ((string_to_cast.as_pointer() as Ptr<u8>).offset(0_u64 as isize)).reinterpret_cast::<u8>(),
    ));
    assert!(((((*output_data.borrow()).read()) as i32) == (('c' as u8) as i32)));
    assert!(
        ((((*output_data.borrow()).offset((1) as isize).read()) as i32) == (('a' as u8) as i32))
    );
    assert!(
        ((((*output_data.borrow()).offset((2) as isize).read()) as i32) == (('s' as u8) as i32))
    );
    assert!(
        ((((*output_data.borrow()).offset((3) as isize).read()) as i32) == (('t' as u8) as i32))
    );
    let t0: Value<u64> = Rc::new(RefCell::new(((*s1.borrow()).len() - 1) as u64));
    let t1: Value<u64> = Rc::new(RefCell::new(
        (*t0.borrow()).wrapping_add((((*p1.borrow()).read()) as u64)),
    ));
    let t2: Value<u64> = Rc::new(RefCell::new(
        (*t1.borrow()).wrapping_add(((*s2.borrow()).len() - 1) as u64),
    ));
    let t3: Value<u64> = Rc::new(RefCell::new(
        (*t2.borrow()).wrapping_add(((*s3.borrow()).len() - 1) as u64),
    ));
    let t4: Value<u64> = Rc::new(RefCell::new(
        (*t3.borrow()).wrapping_add(((*s4.borrow()).len() - 1) as u64),
    ));
    return (((*t4.borrow()).wrapping_add(((*s5.borrow()).len() - 1) as u64)) as i32);
}
