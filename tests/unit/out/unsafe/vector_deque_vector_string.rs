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
        libcc2rs::cpp2rust_exit(main_0() as i32);
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
        let __a1 = std::slice::from_raw_parts(
            c"a".as_ptr(),
            (0..).take_while(|&i| *c"a".as_ptr().add(i) != 0).count() + 1,
        )
        .to_vec();
        row.push(__a1)
    };
    {
        let __a1 = std::slice::from_raw_parts(
            c"bc".as_ptr(),
            (0..).take_while(|&i| *c"bc".as_ptr().add(i) != 0).count() + 1,
        )
        .to_vec();
        row.push(__a1)
    };
    v[(0_usize)].push((row).clone());
    {
        let __a1 = std::slice::from_raw_parts(
            c"def".as_ptr(),
            (0..).take_while(|&i| *c"def".as_ptr().add(i) != 0).count() + 1,
        )
        .to_vec();
        row.push(__a1)
    };
    v[(1_usize)].push((row).clone());
    {
        let __init = Vec::new();
        v[(1_usize)].push(__init)
    };
    match (
        (match ((&mut v[(1_usize)] as *mut Vec<Vec<Vec<libc::c_char>>>),) {
            (__tmp_1,) => {
                let __n = (*__tmp_1).len() - 1;
                &raw mut (&mut (*__tmp_1))[__n]
            }
        } as *mut Vec<Vec<libc::c_char>>),
    ) {
        (__tmp_0,) => {
            let __a1 = {
                let mut __tmp2 = v[(0_usize)][0][(1_usize)].clone();
                __tmp2.pop();
                __tmp2.extend_from_slice(::std::slice::from_raw_parts(
                    c"x".as_ptr(),
                    (0..).position(|i| *c"x".as_ptr().add(i) == 0).unwrap(),
                ));
                __tmp2.push(0);
                __tmp2
            };
            (*__tmp_0).push(__a1)
        }
    };
    assert!(((v[(0_usize)][0].len()) == (2_usize)));
    assert!(
        ((v[(1_usize)][0].len()) == (3_usize))
            && (v[(1_usize)][0][(2_usize)] == {
                std::slice::from_raw_parts(
                    c"def".as_ptr(),
                    (0..).take_while(|&i| *c"def".as_ptr().add(i) != 0).count() + 1,
                )
                .to_vec()
            })
    );
    assert!(
        (*match ((&mut v[(1_usize)] as *mut Vec<Vec<Vec<libc::c_char>>>),) {
            (__tmp_2,) => {
                {
                    let __n = (*__tmp_2).len() - 1;
                    &mut (&mut (*__tmp_2))[__n]
                }
            }
        })[(0_usize)]
            == {
                std::slice::from_raw_parts(
                    c"bcx".as_ptr(),
                    (0..).take_while(|&i| *c"bcx".as_ptr().add(i) != 0).count() + 1,
                )
                .to_vec()
            }
    );
    let mut copy: Vec<Vec<Vec<Vec<libc::c_char>>>> = v.clone();
    match (
        (&mut copy[(0_usize)][0][(0_usize)] as *mut Vec<libc::c_char>),
        (&mut std::slice::from_raw_parts(
            c"z".as_ptr(),
            (0..).take_while(|&i| *c"z".as_ptr().add(i) != 0).count() + 1,
        )
        .to_vec() as *mut Vec<libc::c_char>),
    ) {
        (__tmp_3, __tmp_4) => {
            (*__tmp_3) = std::mem::replace(&mut (*__tmp_4), vec![0]);
        }
    };
    (*match ((&mut v[(0_usize)][0][(1_usize)] as *mut Vec<libc::c_char>),) {
        (__tmp_5,) => {
            if 0_usize >= (*__tmp_5).len() - 1 {
                panic!("out of bounds access")
            }
            &mut (&mut (*__tmp_5))[0_usize]
        }
    }) = ('B' as libc::c_char);
    assert!(
        (v[(0_usize)][0][(0_usize)] == {
            std::slice::from_raw_parts(
                c"a".as_ptr(),
                (0..).take_while(|&i| *c"a".as_ptr().add(i) != 0).count() + 1,
            )
            .to_vec()
        }) && (copy[(0_usize)][0][(0_usize)] == {
            std::slice::from_raw_parts(
                c"z".as_ptr(),
                (0..).take_while(|&i| *c"z".as_ptr().add(i) != 0).count() + 1,
            )
            .to_vec()
        })
    );
    assert!(
        (v[(0_usize)][0][(1_usize)] == {
            std::slice::from_raw_parts(
                c"Bc".as_ptr(),
                (0..).take_while(|&i| *c"Bc".as_ptr().add(i) != 0).count() + 1,
            )
            .to_vec()
        }) && (copy[(0_usize)][0][(1_usize)] == {
            std::slice::from_raw_parts(
                c"bc".as_ptr(),
                (0..).take_while(|&i| *c"bc".as_ptr().add(i) != 0).count() + 1,
            )
            .to_vec()
        })
    );
    v[(1_usize)].remove(0);
    assert!(
        v[(1_usize)][0][(0_usize)] == {
            std::slice::from_raw_parts(
                c"bcx".as_ptr(),
                (0..).take_while(|&i| *c"bcx".as_ptr().add(i) != 0).count() + 1,
            )
            .to_vec()
        }
    );
    assert!(((copy[(1_usize)][0].len()) == (3_usize)));
    assert!(((row.len()) == (3_usize)));
    let mut d: Vec<i32> = Vec::new();
    let mut first: i32 = 1;
    let mut last: i32 = 2;
    d.push(first);
    d.push(last);
    let r: *mut i32 = &raw mut d[0];
    (*r) = 5;
    let c: *const i32 = {
        let __n = d.len() - 1;
        &raw mut d[__n]
    };
    assert!(((d[0]) == (5)) && ((*c) == (2)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
