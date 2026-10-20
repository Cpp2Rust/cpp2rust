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
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut v: Vec<Vec<Vec<Vec<libc::c_char>>>> = Vec::new();
    {
        let __init = Vec::new();
        v.push(__init)
    };
    {
        let __init = Vec::new();
        v.push(__init)
    };
    let mut row: Vec<Vec<libc::c_char>> = Vec::new();
    {
        let __a1 = {
            let s = c"a".as_ptr();
            std::slice::from_raw_parts(s, (0..).take_while(|&i| *s.add(i) != 0).count() + 1)
                .to_vec()
        };
        row.push(__a1)
    };
    {
        let __a1 = {
            let s = c"bc".as_ptr();
            std::slice::from_raw_parts(s, (0..).take_while(|&i| *s.add(i) != 0).count() + 1)
                .to_vec()
        };
        row.push(__a1)
    };
    v[(0_usize)].push((row).clone());
    {
        let __a1 = {
            let s = c"def".as_ptr();
            std::slice::from_raw_parts(s, (0..).take_while(|&i| *s.add(i) != 0).count() + 1)
                .to_vec()
        };
        row.push(__a1)
    };
    v[(1_usize)].push((row).clone());
    {
        let __init = Vec::new();
        v[(1_usize)].push(__init)
    };
    {
        let __a1 = {
            let mut __tmp2 = (*((v[(0_usize)]).first_mut().unwrap()))[(1_usize)].clone();
            __tmp2.pop();
            let __from = c"x".as_ptr();
            __tmp2.extend_from_slice(::std::slice::from_raw_parts(
                __from,
                (0..).position(|i| *__from.add(i) == 0).unwrap(),
            ));
            __tmp2.push(0);
            __tmp2
        };
        (*(v[(1_usize)].last_mut().unwrap())).push(__a1)
    };
    assert!((((*((v[(0_usize)]).first_mut().unwrap())).len()) == (2_usize)));
    assert!(
        (((*((v[(1_usize)]).first_mut().unwrap())).len()) == (3_usize))
            && ((*((v[(1_usize)]).first_mut().unwrap()))[(2_usize)] == {
                let s = c"def".as_ptr();
                std::slice::from_raw_parts(s, (0..).take_while(|&i| *s.add(i) != 0).count() + 1)
                    .to_vec()
            })
    );
    assert!(
        (*(v[(1_usize)].last_mut().unwrap()))[(0_usize)] == {
            let s = c"bcx".as_ptr();
            std::slice::from_raw_parts(s, (0..).take_while(|&i| *s.add(i) != 0).count() + 1)
                .to_vec()
        }
    );
    let mut copy: Vec<Vec<Vec<Vec<libc::c_char>>>> = v.clone();
    (*((copy[(0_usize)]).first_mut().unwrap()))[(0_usize)] = {
        let s = c"z".as_ptr();
        std::slice::from_raw_parts(s, (0..).take_while(|&i| *s.add(i) != 0).count() + 1).to_vec()
    };
    (*if 0_usize as usize >= (*((v[(0_usize)]).first_mut().unwrap()))[(1_usize)].len() - 1 {
        panic!("out of bounds access")
    } else {
        (&mut (*((v[(0_usize)]).first_mut().unwrap()))[(1_usize)][0_usize as usize]
            as *mut libc::c_char)
    }) = ('B' as libc::c_char);
    assert!(
        ((*((v[(0_usize)]).first_mut().unwrap()))[(0_usize)] == {
            let s = c"a".as_ptr();
            std::slice::from_raw_parts(s, (0..).take_while(|&i| *s.add(i) != 0).count() + 1)
                .to_vec()
        }) && ((*((copy[(0_usize)]).first_mut().unwrap()))[(0_usize)] == {
            let s = c"z".as_ptr();
            std::slice::from_raw_parts(s, (0..).take_while(|&i| *s.add(i) != 0).count() + 1)
                .to_vec()
        })
    );
    assert!(
        ((*((v[(0_usize)]).first_mut().unwrap()))[(1_usize)] == {
            let s = c"Bc".as_ptr();
            std::slice::from_raw_parts(s, (0..).take_while(|&i| *s.add(i) != 0).count() + 1)
                .to_vec()
        }) && ((*((copy[(0_usize)]).first_mut().unwrap()))[(1_usize)] == {
            let s = c"bc".as_ptr();
            std::slice::from_raw_parts(s, (0..).take_while(|&i| *s.add(i) != 0).count() + 1)
                .to_vec()
        })
    );
    v[(1_usize)].remove(0);
    assert!(
        (*((v[(1_usize)]).first_mut().unwrap()))[(0_usize)] == {
            let s = c"bcx".as_ptr();
            std::slice::from_raw_parts(s, (0..).take_while(|&i| *s.add(i) != 0).count() + 1)
                .to_vec()
        }
    );
    assert!((((*((copy[(1_usize)]).first_mut().unwrap())).len()) == (3_usize)));
    assert!(((row.len()) == (3_usize)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
