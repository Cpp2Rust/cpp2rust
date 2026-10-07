extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut empty: Vec<i8> = vec![0];
    assert!(((empty.len() - 1) == 0_usize));
    let s1: Value<Vec<i8>> = Rc::new(RefCell::new({
        let mut __bytes = Ptr::<i8>::from_string_literal(b"hello").to_c_bytes();
        __bytes.push(0);
        __bytes
    }));
    assert!((((*s1.borrow()).len() - 1) == 5_usize));
    assert!((((*s1.borrow()).len() - 1) == ((*s1.borrow()).len() - 1)));
    assert!((({ (*s1.borrow())[0_usize] } as i32) == (('h' as i8) as i32)));
    assert!((({ (*s1.borrow())[1_usize] } as i32) == (('e' as i8) as i32)));
    assert!((({ (*s1.borrow())[2_usize] } as i32) == (('l' as i8) as i32)));
    assert!((({ (*s1.borrow())[3_usize] } as i32) == (('l' as i8) as i32)));
    assert!((({ (*s1.borrow())[4_usize] } as i32) == (('o' as i8) as i32)));
    assert!(
        Ptr::<i8>::from_string_literal(b"hello")
            .with_c_str(|__s| (*s1.borrow())[..(*s1.borrow()).len().saturating_sub(1)] == *__s)
    );
    let mut p1: Ptr<i8> = (s1.as_pointer() as Ptr<i8>);
    assert!((((elem!(p1, 0).read()) as i32) == (('h' as i8) as i32)));
    assert!((((elem!(p1, 1).read()) as i32) == (('e' as i8) as i32)));
    assert!((((elem!(p1, 2).read()) as i32) == (('l' as i8) as i32)));
    assert!((((elem!(p1, 3).read()) as i32) == (('l' as i8) as i32)));
    assert!((((elem!(p1, 4).read()) as i32) == (('o' as i8) as i32)));
    let s2: Value<Vec<i8>> = Rc::new(RefCell::new(
        vec![('a' as i8); (10_usize) as usize]
            .iter()
            .cloned()
            .chain(std::iter::once(0))
            .collect(),
    ));
    let mut p2: Ptr<i8> = (s2.as_pointer() as Ptr<i8>);
    let mut i: u32 = 0_u32;
    'loop_: while ((i as usize) < ((*s2.borrow()).len() - 1)) {
        assert!(
            (((elem!(p2, i).read()) as i32) == (('a' as i8) as i32))
                && (({ (*s2.borrow())[(i as usize)] } as i32) == (('a' as i8) as i32))
        );
        i.prefix_inc();
    }
    assert!((((*s2.borrow()).len() - 1) == 10_usize));
    assert!((((*s2.borrow()).len() - 1) == ((*s2.borrow()).len() - 1)));
    (*s2.borrow_mut())[0_usize] = ('b' as i8);
    (*s2.borrow_mut())[1_usize] = ('c' as i8);
    assert!((({ (*s2.borrow())[0_usize] } as i32) == (('b' as i8) as i32)));
    assert!((({ (*s2.borrow())[1_usize] } as i32) == (('c' as i8) as i32)));
    let mut i: u32 = 2_u32;
    'loop_: while ((i as usize) < ((*s2.borrow()).len() - 1)) {
        assert!(
            (((elem!(p2, i).read()) as i32) == (('a' as i8) as i32))
                && (({ (*s2.borrow())[(i as usize)] } as i32) == (('a' as i8) as i32))
        );
        i.prefix_inc();
    }
    let s3: Value<Vec<i8>> = Rc::new(RefCell::new({
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
    let mut p3: Ptr<i8> = (s3.as_pointer() as Ptr<i8>);
    let mut i: u32 = 0_u32;
    'loop_: while ((i as usize) < ((*s3.borrow()).len() - 1)) {
        assert!(
            ({ ((elem!(p3, i).read()) as i32) } == { ({ (*s3.borrow())[(i as usize)] } as i32) })
        );
        i.prefix_inc();
    }
    let s4: Value<Vec<i8>> = Rc::new(RefCell::new({
        let mut __tmp1 = (*s1.borrow())[(1_usize) as usize
            ..::std::cmp::min(
                (1_usize
                    + Ptr::<i8>::from_string_literal(b"l").with_c_str(|__lookup| {
                        (*s1.borrow())
                            .iter()
                            .take((*s1.borrow()).len().saturating_sub(1))
                            .rposition(|&x| __lookup.contains(&x))
                            .unwrap_or(usize::MAX)
                    })) as usize,
                (*s1.borrow()).len().saturating_sub(1),
            )]
            .to_vec();
        __tmp1.push(0);
        __tmp1
    }));
    assert!((((*s4.borrow()).len() - 1) == 3_usize));
    assert!((((*s4.borrow()).len() - 1) == ((*s4.borrow()).len() - 1)));
    let mut p4: Ptr<i8> = (s4.as_pointer() as Ptr<i8>);
    let mut i: u32 = 0_u32;
    'loop_: while ((i as usize) < ((*s4.borrow()).len() - 1)) {
        assert!(
            ({ ((elem!(p4, i).read()) as i32) } == { ({ (*s4.borrow())[(i as usize)] } as i32) })
        );
        i.prefix_inc();
    }
    let s5: Value<Vec<i8>> = Rc::new(RefCell::new({
        let mut r = (*s1.borrow()).clone();
        r.pop();
        Ptr::<i8>::from_string_literal(b", world").with_c_str(|__s| r.extend_from_slice(__s));
        r.push(0);
        r
    }));
    assert!((((*s5.borrow()).len() - 1) == 12_usize));
    assert!((((*s5.borrow()).len() - 1) == ((*s5.borrow()).len() - 1)));
    let mut p5: Ptr<i8> = (s5.as_pointer() as Ptr<i8>);
    let mut i: u32 = 0_u32;
    'loop_: while ((i as usize) < ((*s5.borrow()).len() - 1)) {
        assert!(
            ({ ((elem!(p5, i).read()) as i32) } == { ({ (*s5.borrow())[(i as usize)] } as i32) })
        );
        i.prefix_inc();
    }
    let arr: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
        ('b' as i8),
        ('a' as i8),
        ('r' as i8),
        (' ' as i8),
        ('f' as i8),
        ('o' as i8),
        ('o' as i8),
    ])));
    let mut string: Vec<i8> = {
        let mut __v = Vec::with_capacity(3_usize as usize + 1);
        (arr.as_pointer() as Ptr<i8>)
            .with_slice(3_usize as usize, |__s| __v.extend_from_slice(__s));
        __v.push(0);
        __v
    };
    assert!(((string.len() - 1) == 3_usize));
    assert!(((string[0_usize] as i32) == (('b' as i8) as i32)));
    assert!(((string[1_usize] as i32) == (('a' as i8) as i32)));
    assert!(((string[2_usize] as i32) == (('r' as i8) as i32)));
    assert!(
        Ptr::<i8>::from_string_literal(b"bar")
            .with_c_str(|__s| string[..string.len().saturating_sub(1)] == *__s)
    );
    {
        string.pop();
        string.resize((3_usize) as usize, 0);
        string.push(0)
    };
    assert!(((string.len() - 1) == 3_usize));
    assert!(((string[0_usize] as i32) == (('b' as i8) as i32)));
    assert!(((string[1_usize] as i32) == (('a' as i8) as i32)));
    assert!(((string[2_usize] as i32) == (('r' as i8) as i32)));
    assert!(
        Ptr::<i8>::from_string_literal(b"bar")
            .with_c_str(|__s| string[..string.len().saturating_sub(1)] == *__s)
    );
    {
        string.pop();
        string.resize((5_usize) as usize, 0);
        string.push(0)
    };
    assert!(((string.len() - 1) == 5_usize));
    assert!(((string[0_usize] as i32) == (('b' as i8) as i32)));
    assert!(((string[1_usize] as i32) == (('a' as i8) as i32)));
    assert!(((string[2_usize] as i32) == (('r' as i8) as i32)));
    assert!(((string[3_usize] as i32) == 0));
    assert!(((string[4_usize] as i32) == 0));
    string[3_usize] = ('a' as i8);
    string[4_usize] = ('b' as i8);
    assert!(((string[3_usize] as i32) == (('a' as i8) as i32)));
    assert!(((string[4_usize] as i32) == (('b' as i8) as i32)));
    string[3_usize] = 0_i8;
    string[4_usize] = 0_i8;
    {
        string.pop();
        string.resize((4_usize) as usize, 0);
        string.push(0)
    };
    assert!(((string.len() - 1) == 4_usize));
    assert!(((string[0_usize] as i32) == (('b' as i8) as i32)));
    assert!(((string[1_usize] as i32) == (('a' as i8) as i32)));
    assert!(((string[2_usize] as i32) == (('r' as i8) as i32)));
    assert!(((string[3_usize] as i32) == 0));
    let mut result: Vec<i8> = {
        let mut r = (string).clone();
        r.pop();
        Ptr::<i8>::from_string_literal(b" foo").with_c_str(|__s| r.extend_from_slice(__s));
        r.push(0);
        r
    };
    assert!(((result.len() - 1) == 8_usize));
    assert!(((result[0_usize] as i32) == (('b' as i8) as i32)));
    assert!(((result[1_usize] as i32) == (('a' as i8) as i32)));
    assert!(((result[2_usize] as i32) == (('r' as i8) as i32)));
    assert!(((result[3_usize] as i32) == 0));
    assert!(((result[4_usize] as i32) == ((' ' as i8) as i32)));
    assert!(((result[5_usize] as i32) == (('f' as i8) as i32)));
    assert!(((result[6_usize] as i32) == (('o' as i8) as i32)));
    assert!(((result[7_usize] as i32) == (('o' as i8) as i32)));
    let mut substr_0: Vec<i8> = {
        let mut __tmp1 = result[(5_usize) as usize
            ..::std::cmp::min((5_usize + 3_usize) as usize, result.len().saturating_sub(1))]
            .to_vec();
        __tmp1.push(0);
        __tmp1
    };
    assert!(((substr_0.len() - 1) == 3_usize));
    assert!(((substr_0[0_usize] as i32) == (('f' as i8) as i32)));
    assert!(((substr_0[1_usize] as i32) == (('o' as i8) as i32)));
    assert!(((substr_0[2_usize] as i32) == (('o' as i8) as i32)));
    let mut substr_1: Vec<i8> = {
        let mut __tmp1 = result[(0_usize) as usize
            ..::std::cmp::min((0_usize + 5_usize) as usize, result.len().saturating_sub(1))]
            .to_vec();
        __tmp1.push(0);
        __tmp1
    };
    assert!(((substr_1.len() - 1) == 5_usize));
    assert!(((substr_1[0_usize] as i32) == (('b' as i8) as i32)));
    assert!(((substr_1[1_usize] as i32) == (('a' as i8) as i32)));
    assert!(((substr_1[2_usize] as i32) == (('r' as i8) as i32)));
    assert!(((substr_1[3_usize] as i32) == 0));
    assert!(((substr_1[4_usize] as i32) == ((' ' as i8) as i32)));
    let mut substr_2: Vec<i8> = {
        let mut __tmp1 = result[(0_usize) as usize
            ..::std::cmp::min(
                (0_usize + 15_usize) as usize,
                result.len().saturating_sub(1),
            )]
            .to_vec();
        __tmp1.push(0);
        __tmp1
    };
    assert!(((substr_2.len() - 1) == 8_usize));
    assert!(((substr_2[0_usize] as i32) == (('b' as i8) as i32)));
    assert!(((substr_2[1_usize] as i32) == (('a' as i8) as i32)));
    assert!(((substr_2[2_usize] as i32) == (('r' as i8) as i32)));
    assert!(((substr_2[3_usize] as i32) == 0));
    assert!(((substr_2[4_usize] as i32) == ((' ' as i8) as i32)));
    assert!(((substr_2[5_usize] as i32) == (('f' as i8) as i32)));
    assert!(((substr_2[6_usize] as i32) == (('o' as i8) as i32)));
    assert!(((substr_2[7_usize] as i32) == (('o' as i8) as i32)));
    let mut pos: usize = Ptr::<i8>::from_string_literal(b"b").with_c_str(|__lookup| {
        result
            .iter()
            .take(result.len().saturating_sub(1))
            .rposition(|&x| __lookup.contains(&x))
            .unwrap_or(usize::MAX)
    });
    assert!((pos == 0_usize));
    pos = Ptr::<i8>::from_string_literal(b"f").with_c_str(|__lookup| {
        result
            .iter()
            .take(result.len().saturating_sub(1))
            .rposition(|&x| __lookup.contains(&x))
            .unwrap_or(usize::MAX)
    });
    assert!((pos == 5_usize));
    pos = Ptr::<i8>::from_string_literal(b"o").with_c_str(|__lookup| {
        result
            .iter()
            .take(result.len().saturating_sub(1))
            .rposition(|&x| __lookup.contains(&x))
            .unwrap_or(usize::MAX)
    });
    assert!((pos == 7_usize));
    pos = Ptr::<i8>::from_string_literal(b"x").with_c_str(|__lookup| {
        result
            .iter()
            .take(result.len().saturating_sub(1))
            .rposition(|&x| __lookup.contains(&x))
            .unwrap_or(usize::MAX)
    });
    assert!((pos == ((-1_i64 as u64) as usize)));
    let string_to_cast: Value<Vec<i8>> = Rc::new(RefCell::new({
        let mut __bytes = Ptr::<i8>::from_string_literal(b"cast").to_c_bytes();
        __bytes.push(0);
        __bytes
    }));
    let mut output_data: Ptr<u8> =
        ((string_to_cast.as_pointer() as Ptr<i8>).offset(0_usize)).reinterpret_cast::<u8>();
    assert!((((output_data.read()) as i32) == (('c' as i8) as i32)));
    assert!((((elem!(output_data, 1).read()) as i32) == (('a' as i8) as i32)));
    assert!((((elem!(output_data, 2).read()) as i32) == (('s' as i8) as i32)));
    assert!((((elem!(output_data, 3).read()) as i32) == (('t' as i8) as i32)));
    let mut t0: usize = ((*s1.borrow()).len() - 1);
    let mut t1: usize = (t0).wrapping_add(((p1.read()) as usize));
    let mut t2: usize = (t1).wrapping_add(((*s2.borrow()).len() - 1));
    let mut t3: usize = (t2).wrapping_add(((*s3.borrow()).len() - 1));
    let mut t4: usize = (t3).wrapping_add(((*s4.borrow()).len() - 1));
    assert!(((t4).wrapping_add(((*s5.borrow()).len() - 1)) == 139_usize));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
