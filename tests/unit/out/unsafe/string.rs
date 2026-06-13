extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut s1: Vec<::core::ffi::c_char> = {
        let s = c"hello".as_ptr();
        std::slice::from_raw_parts(s, (0..).take_while(|&i| *s.add(i) != 0).count() + 1).to_vec()
    };
    assert!(((s1.len() - 1) == (5_usize)));
    assert!(((s1.len() - 1) == (s1.len() - 1)));
    assert!(((s1[(0_usize)] as i32) == (('h' as ::core::ffi::c_char) as i32)));
    assert!(((s1[(1_usize)] as i32) == (('e' as ::core::ffi::c_char) as i32)));
    assert!(((s1[(2_usize)] as i32) == (('l' as ::core::ffi::c_char) as i32)));
    assert!(((s1[(3_usize)] as i32) == (('l' as ::core::ffi::c_char) as i32)));
    assert!(((s1[(4_usize)] as i32) == (('o' as ::core::ffi::c_char) as i32)));
    assert!(
        s1 == {
            let s = c"hello".as_ptr();
            std::slice::from_raw_parts(s, (0..).take_while(|&i| *s.add(i) != 0).count() + 1)
                .to_vec()
        }
    );
    let mut p1: *const ::core::ffi::c_char = (s1.as_mut_ptr()).cast_const();
    assert!((((*p1.offset((0) as isize)) as i32) == (('h' as ::core::ffi::c_char) as i32)));
    assert!((((*p1.offset((1) as isize)) as i32) == (('e' as ::core::ffi::c_char) as i32)));
    assert!((((*p1.offset((2) as isize)) as i32) == (('l' as ::core::ffi::c_char) as i32)));
    assert!((((*p1.offset((3) as isize)) as i32) == (('l' as ::core::ffi::c_char) as i32)));
    assert!((((*p1.offset((4) as isize)) as i32) == (('o' as ::core::ffi::c_char) as i32)));
    let mut s2: Vec<::core::ffi::c_char> = vec![('a' as ::core::ffi::c_char); (10_usize) as usize]
        .iter()
        .cloned()
        .chain(std::iter::once(0))
        .collect();
    let mut p2: *const ::core::ffi::c_char = (s2.as_mut_ptr()).cast_const();
    let mut i: u32 = 0_u32;
    'loop_: while ((i as usize) < (s2.len() - 1)) {
        assert!(
            (((*p2.offset((i) as isize)) as i32) == (('a' as ::core::ffi::c_char) as i32))
                && ((s2[(i as usize)] as i32) == (('a' as ::core::ffi::c_char) as i32))
        );
        i.prefix_inc();
    }
    assert!(((s2.len() - 1) == (10_usize)));
    assert!(((s2.len() - 1) == (s2.len() - 1)));
    s2[(0_usize)] = ('b' as ::core::ffi::c_char);
    s2[(1_usize)] = ('c' as ::core::ffi::c_char);
    assert!(((s2[(0_usize)] as i32) == (('b' as ::core::ffi::c_char) as i32)));
    assert!(((s2[(1_usize)] as i32) == (('c' as ::core::ffi::c_char) as i32)));
    let mut i: u32 = 2_u32;
    'loop_: while ((i as usize) < (s2.len() - 1)) {
        assert!(
            (((*p2.offset((i) as isize)) as i32) == (('a' as ::core::ffi::c_char) as i32))
                && ((s2[(i as usize)] as i32) == (('a' as ::core::ffi::c_char) as i32))
        );
        i.prefix_inc();
    }
    let mut s3: Vec<::core::ffi::c_char> = {
        let mut __tmp1 = s2
            [(2_usize) as usize..::std::cmp::min((2_usize + 5_usize) as usize, s2.len() - 1)]
            .to_vec();
        __tmp1.push(0);
        __tmp1
    };
    assert!(((s3.len() - 1) == (5_usize)));
    assert!(((s3.len() - 1) == (s3.len() - 1)));
    let mut p3: *const ::core::ffi::c_char = (s3.as_mut_ptr()).cast_const();
    let mut i: u32 = 0_u32;
    'loop_: while ((i as usize) < (s3.len() - 1)) {
        assert!((((*p3.offset((i) as isize)) as i32) == (s3[(i as usize)] as i32)));
        i.prefix_inc();
    }
    let mut s4: Vec<::core::ffi::c_char> = {
        let mut __tmp1 = s1[(1_usize) as usize
            ..::std::cmp::min(
                (1_usize
                    + match s1.iter().rposition(|&c| {
                        ::std::ffi::CStr::from_ptr(c"l".as_ptr())
                            .to_str()
                            .unwrap()
                            .contains(c as u8 as char)
                    }) {
                        Some(idx) => idx,
                        None => usize::MAX,
                    }) as usize,
                s1.len() - 1,
            )]
            .to_vec();
        __tmp1.push(0);
        __tmp1
    };
    assert!(((s4.len() - 1) == (3_usize)));
    assert!(((s4.len() - 1) == (s4.len() - 1)));
    let mut p4: *const ::core::ffi::c_char = (s4.as_mut_ptr()).cast_const();
    let mut i: u32 = 0_u32;
    'loop_: while ((i as usize) < (s4.len() - 1)) {
        assert!((((*p4.offset((i) as isize)) as i32) == (s4[(i as usize)] as i32)));
        i.prefix_inc();
    }
    let mut s5: Vec<::core::ffi::c_char> = {
        let mut __tmp2 = s1.clone();
        __tmp2.pop();
        let __from = c", world".as_ptr();
        __tmp2.extend_from_slice(::std::slice::from_raw_parts(
            __from,
            (0..).position(|i| *__from.add(i) == 0).unwrap(),
        ));
        __tmp2.push(0);
        __tmp2
    };
    assert!(((s5.len() - 1) == (12_usize)));
    assert!(((s5.len() - 1) == (s5.len() - 1)));
    let mut p5: *const ::core::ffi::c_char = (s5.as_mut_ptr()).cast_const();
    let mut i: u32 = 0_u32;
    'loop_: while ((i as usize) < (s5.len() - 1)) {
        assert!((((*p5.offset((i) as isize)) as i32) == (s5[(i as usize)] as i32)));
        i.prefix_inc();
    }
    let mut arr: [::core::ffi::c_char; 7] = [
        ('b' as ::core::ffi::c_char),
        ('a' as ::core::ffi::c_char),
        ('r' as ::core::ffi::c_char),
        (' ' as ::core::ffi::c_char),
        ('f' as ::core::ffi::c_char),
        ('o' as ::core::ffi::c_char),
        ('o' as ::core::ffi::c_char),
    ];
    let mut string: Vec<::core::ffi::c_char> =
        std::slice::from_raw_parts((arr.as_mut_ptr()).cast_const(), 3_usize as usize)
            .to_vec()
            .iter()
            .copied()
            .chain(std::iter::once(0))
            .collect();
    assert!(((string.len() - 1) == (3_usize)));
    assert!(((string[(0_usize)] as i32) == (('b' as ::core::ffi::c_char) as i32)));
    assert!(((string[(1_usize)] as i32) == (('a' as ::core::ffi::c_char) as i32)));
    assert!(((string[(2_usize)] as i32) == (('r' as ::core::ffi::c_char) as i32)));
    assert!(
        string == {
            let s = c"bar".as_ptr();
            std::slice::from_raw_parts(s, (0..).take_while(|&i| *s.add(i) != 0).count() + 1)
                .to_vec()
        }
    );
    {
        string.pop();
        string.resize((3_usize) as usize, 0);
        string.push(0)
    };
    assert!(((string.len() - 1) == (3_usize)));
    assert!(((string[(0_usize)] as i32) == (('b' as ::core::ffi::c_char) as i32)));
    assert!(((string[(1_usize)] as i32) == (('a' as ::core::ffi::c_char) as i32)));
    assert!(((string[(2_usize)] as i32) == (('r' as ::core::ffi::c_char) as i32)));
    assert!(
        string == {
            let s = c"bar".as_ptr();
            std::slice::from_raw_parts(s, (0..).take_while(|&i| *s.add(i) != 0).count() + 1)
                .to_vec()
        }
    );
    {
        string.pop();
        string.resize((5_usize) as usize, 0);
        string.push(0)
    };
    assert!(((string.len() - 1) == (5_usize)));
    assert!(((string[(0_usize)] as i32) == (('b' as ::core::ffi::c_char) as i32)));
    assert!(((string[(1_usize)] as i32) == (('a' as ::core::ffi::c_char) as i32)));
    assert!(((string[(2_usize)] as i32) == (('r' as ::core::ffi::c_char) as i32)));
    assert!(((string[(3_usize)] as i32) == (0)));
    assert!(((string[(4_usize)] as i32) == (0)));
    string[(3_usize)] = ('a' as ::core::ffi::c_char);
    string[(4_usize)] = ('b' as ::core::ffi::c_char);
    assert!(((string[(3_usize)] as i32) == (('a' as ::core::ffi::c_char) as i32)));
    assert!(((string[(4_usize)] as i32) == (('b' as ::core::ffi::c_char) as i32)));
    string[(3_usize)] = (0 as ::core::ffi::c_char);
    string[(4_usize)] = (0 as ::core::ffi::c_char);
    {
        string.pop();
        string.resize((4_usize) as usize, 0);
        string.push(0)
    };
    assert!(((string.len() - 1) == (4_usize)));
    assert!(((string[(0_usize)] as i32) == (('b' as ::core::ffi::c_char) as i32)));
    assert!(((string[(1_usize)] as i32) == (('a' as ::core::ffi::c_char) as i32)));
    assert!(((string[(2_usize)] as i32) == (('r' as ::core::ffi::c_char) as i32)));
    assert!(((string[(3_usize)] as i32) == (0)));
    let mut result: Vec<::core::ffi::c_char> = {
        let mut __tmp2 = string.clone();
        __tmp2.pop();
        let __from = c" foo".as_ptr();
        __tmp2.extend_from_slice(::std::slice::from_raw_parts(
            __from,
            (0..).position(|i| *__from.add(i) == 0).unwrap(),
        ));
        __tmp2.push(0);
        __tmp2
    };
    assert!(((result.len() - 1) == (8_usize)));
    assert!(((result[(0_usize)] as i32) == (('b' as ::core::ffi::c_char) as i32)));
    assert!(((result[(1_usize)] as i32) == (('a' as ::core::ffi::c_char) as i32)));
    assert!(((result[(2_usize)] as i32) == (('r' as ::core::ffi::c_char) as i32)));
    assert!(((result[(3_usize)] as i32) == (0)));
    assert!(((result[(4_usize)] as i32) == ((' ' as ::core::ffi::c_char) as i32)));
    assert!(((result[(5_usize)] as i32) == (('f' as ::core::ffi::c_char) as i32)));
    assert!(((result[(6_usize)] as i32) == (('o' as ::core::ffi::c_char) as i32)));
    assert!(((result[(7_usize)] as i32) == (('o' as ::core::ffi::c_char) as i32)));
    let mut substr_0: Vec<::core::ffi::c_char> = {
        let mut __tmp1 = result
            [(5_usize) as usize..::std::cmp::min((5_usize + 3_usize) as usize, result.len() - 1)]
            .to_vec();
        __tmp1.push(0);
        __tmp1
    };
    assert!(((substr_0.len() - 1) == (3_usize)));
    assert!(((substr_0[(0_usize)] as i32) == (('f' as ::core::ffi::c_char) as i32)));
    assert!(((substr_0[(1_usize)] as i32) == (('o' as ::core::ffi::c_char) as i32)));
    assert!(((substr_0[(2_usize)] as i32) == (('o' as ::core::ffi::c_char) as i32)));
    let mut substr_1: Vec<::core::ffi::c_char> = {
        let mut __tmp1 = result
            [(0_usize) as usize..::std::cmp::min((0_usize + 5_usize) as usize, result.len() - 1)]
            .to_vec();
        __tmp1.push(0);
        __tmp1
    };
    assert!(((substr_1.len() - 1) == (5_usize)));
    assert!(((substr_1[(0_usize)] as i32) == (('b' as ::core::ffi::c_char) as i32)));
    assert!(((substr_1[(1_usize)] as i32) == (('a' as ::core::ffi::c_char) as i32)));
    assert!(((substr_1[(2_usize)] as i32) == (('r' as ::core::ffi::c_char) as i32)));
    assert!(((substr_1[(3_usize)] as i32) == (0)));
    assert!(((substr_1[(4_usize)] as i32) == ((' ' as ::core::ffi::c_char) as i32)));
    let mut substr_2: Vec<::core::ffi::c_char> = {
        let mut __tmp1 = result
            [(0_usize) as usize..::std::cmp::min((0_usize + 15_usize) as usize, result.len() - 1)]
            .to_vec();
        __tmp1.push(0);
        __tmp1
    };
    assert!(((substr_2.len() - 1) == (8_usize)));
    assert!(((substr_2[(0_usize)] as i32) == (('b' as ::core::ffi::c_char) as i32)));
    assert!(((substr_2[(1_usize)] as i32) == (('a' as ::core::ffi::c_char) as i32)));
    assert!(((substr_2[(2_usize)] as i32) == (('r' as ::core::ffi::c_char) as i32)));
    assert!(((substr_2[(3_usize)] as i32) == (0)));
    assert!(((substr_2[(4_usize)] as i32) == ((' ' as ::core::ffi::c_char) as i32)));
    assert!(((substr_2[(5_usize)] as i32) == (('f' as ::core::ffi::c_char) as i32)));
    assert!(((substr_2[(6_usize)] as i32) == (('o' as ::core::ffi::c_char) as i32)));
    assert!(((substr_2[(7_usize)] as i32) == (('o' as ::core::ffi::c_char) as i32)));
    let mut pos: usize = match result.iter().rposition(|&c| {
        ::std::ffi::CStr::from_ptr(c"b".as_ptr())
            .to_str()
            .unwrap()
            .contains(c as u8 as char)
    }) {
        Some(idx) => idx,
        None => usize::MAX,
    };
    assert!(((pos) == (0_usize)));
    pos = match result.iter().rposition(|&c| {
        ::std::ffi::CStr::from_ptr(c"f".as_ptr())
            .to_str()
            .unwrap()
            .contains(c as u8 as char)
    }) {
        Some(idx) => idx,
        None => usize::MAX,
    };
    assert!(((pos) == (5_usize)));
    pos = match result.iter().rposition(|&c| {
        ::std::ffi::CStr::from_ptr(c"o".as_ptr())
            .to_str()
            .unwrap()
            .contains(c as u8 as char)
    }) {
        Some(idx) => idx,
        None => usize::MAX,
    };
    assert!(((pos) == (7_usize)));
    pos = match result.iter().rposition(|&c| {
        ::std::ffi::CStr::from_ptr(c"x".as_ptr())
            .to_str()
            .unwrap()
            .contains(c as u8 as char)
    }) {
        Some(idx) => idx,
        None => usize::MAX,
    };
    assert!(((pos) == ((-1_i64 as u64) as usize)));
    let mut string_to_cast: Vec<::core::ffi::c_char> = {
        let s = c"cast".as_ptr();
        std::slice::from_raw_parts(s, (0..).take_while(|&i| *s.add(i) != 0).count() + 1).to_vec()
    };
    let mut output_data: *mut u8 = ((&mut string_to_cast[(0_usize)] as *mut ::core::ffi::c_char)
        as *mut ::core::ffi::c_char as *mut u8);
    assert!((((*output_data) as i32) == (('c' as ::core::ffi::c_char) as i32)));
    assert!(
        (((*output_data.offset((1) as isize)) as i32) == (('a' as ::core::ffi::c_char) as i32))
    );
    assert!(
        (((*output_data.offset((2) as isize)) as i32) == (('s' as ::core::ffi::c_char) as i32))
    );
    assert!(
        (((*output_data.offset((3) as isize)) as i32) == (('t' as ::core::ffi::c_char) as i32))
    );
    let mut t0: usize = (s1.len() - 1);
    let mut t1: usize = (t0).wrapping_add(((*p1) as usize));
    let mut t2: usize = (t1).wrapping_add((s2.len() - 1));
    let mut t3: usize = (t2).wrapping_add((s3.len() - 1));
    let mut t4: usize = (t3).wrapping_add((s4.len() - 1));
    return (((t4).wrapping_add((s5.len() - 1))) as i32);
}
