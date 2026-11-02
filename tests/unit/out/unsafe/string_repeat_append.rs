extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn append_to_reference_0(s: *mut Vec<libc::c_char>) {
    (*s).splice(
        (*s).len() - 1..(*s).len() - 1,
        ::std::vec::from_elem(('r' as libc::c_char), 1_usize as usize),
    );
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let lengths: [usize; 10] = [
        0_usize, 1_usize, 2_usize, 3_usize, 7_usize, 8_usize, 15_usize, 16_usize, 31_usize,
        64_usize,
    ];
    let counts: [usize; 6] = [0_usize, 1_usize, 2_usize, 3_usize, 8_usize, 33_usize];
    let characters: [libc::c_char; 20] = [
        (0 as libc::c_char),
        (1 as libc::c_char),
        (2 as libc::c_char),
        (7 as libc::c_char),
        (9 as libc::c_char),
        (10 as libc::c_char),
        (13 as libc::c_char),
        (31 as libc::c_char),
        (32 as libc::c_char),
        (42 as libc::c_char),
        (65 as libc::c_char),
        (90 as libc::c_char),
        (97 as libc::c_char),
        (100 as libc::c_char),
        (127 as libc::c_char),
        (-128_i32 as libc::c_char),
        (-127_i32 as libc::c_char),
        (-64_i32 as libc::c_char),
        (-2_i32 as libc::c_char),
        (-1_i32 as libc::c_char),
    ];
    let initial: [libc::c_char; 5] = [
        ('a' as libc::c_char),
        (0 as libc::c_char),
        ('b' as libc::c_char),
        (-128_i32 as libc::c_char),
        (-1_i32 as libc::c_char),
    ];
    let mut cases: usize = 0_usize;
    'loop_: for length in
        IterRange::new(lengths.as_ptr(), lengths.as_ptr().offset((10_i64) as isize))
    {
        let mut length: usize = (*length);
        'loop_: for count in
            IterRange::new(counts.as_ptr(), counts.as_ptr().offset((6_i64) as isize))
        {
            let mut count: usize = (*count);
            'loop_: for character in IterRange::new(
                characters.as_ptr(),
                characters.as_ptr().offset((20_i64) as isize),
            ) {
                let mut character: libc::c_char = (*character);
                let mut shrink: i32 = 0;
                'loop_: while ((shrink) < (2)) {
                    let mut s: Vec<libc::c_char> = vec![('x' as libc::c_char); (length) as usize]
                        .iter()
                        .cloned()
                        .chain(std::iter::once(0))
                        .collect();
                    let mut i: usize = 0_usize;
                    'loop_: while ((i) < (length)) {
                        s[(i)] = initial[((i)
                            .wrapping_rem((::std::mem::size_of::<[libc::c_char; 5]>() as usize)))];
                        i.prefix_inc();
                    }
                    if (shrink != 0) {
                        s.shrink_to_fit();
                    }
                    s.splice(
                        s.len() - 1..s.len() - 1,
                        ::std::vec::from_elem(character, count as usize),
                    );
                    assert!(((s.len() - 1) == ((length).wrapping_add(count))));
                    let mut data: *const libc::c_char = (s.as_mut_ptr()).cast_const();
                    let mut i: usize = 0_usize;
                    'loop_: while ((i) < ((length).wrapping_add(count))) {
                        assert!(
                            (((*data.offset((i) as isize)) as i32)
                                == ((if ((i) < (length)) {
                                    initial[((i).wrapping_rem(
                                        (::std::mem::size_of::<[libc::c_char; 5]>() as usize),
                                    ))]
                                } else {
                                    character
                                }) as i32))
                        );
                        i.prefix_inc();
                    }
                    assert!((((*data.offset((s.len() - 1) as isize)) as i32) == (0)));
                    cases.prefix_inc();
                    shrink.prefix_inc();
                }
            }
        }
    }
    let mut unboxed: Vec<libc::c_char> = std::slice::from_raw_parts(
        c"a".as_ptr(),
        (0..).take_while(|&i| *c"a".as_ptr().add(i) != 0).count() + 1,
    )
    .to_vec();
    unboxed.splice(
        unboxed.len() - 1..unboxed.len() - 1,
        ::std::vec::from_elem(('b' as libc::c_char), 2_usize as usize),
    );
    assert!(
        unboxed == {
            std::slice::from_raw_parts(
                c"abb".as_ptr(),
                (0..).take_while(|&i| *c"abb".as_ptr().add(i) != 0).count() + 1,
            )
            .to_vec()
        }
    );
    let mut indirect: Vec<libc::c_char> = std::slice::from_raw_parts(
        c"c".as_ptr(),
        (0..).take_while(|&i| *c"c".as_ptr().add(i) != 0).count() + 1,
    )
    .to_vec();
    (unsafe { append_to_reference_0(&mut indirect) });
    assert!(
        indirect == {
            std::slice::from_raw_parts(
                c"cr".as_ptr(),
                (0..).take_while(|&i| *c"cr".as_ptr().add(i) != 0).count() + 1,
            )
            .to_vec()
        }
    );
    let mut pointer: *mut Vec<libc::c_char> = (&mut indirect as *mut Vec<libc::c_char>);
    (*pointer).splice(
        (*pointer).len() - 1..(*pointer).len() - 1,
        ::std::vec::from_elem(('\0' as libc::c_char), 0_usize as usize),
    );
    assert!(
        indirect == {
            std::slice::from_raw_parts(
                c"cr".as_ptr(),
                (0..).take_while(|&i| *c"cr".as_ptr().add(i) != 0).count() + 1,
            )
            .to_vec()
        }
    );
    (unsafe {
        libc::printf(
            c"checked %zu appends\n".as_ptr() as *const libc::c_char,
            (cases),
        )
    });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
