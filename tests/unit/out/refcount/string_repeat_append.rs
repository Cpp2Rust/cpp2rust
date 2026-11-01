extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn append_to_reference_0(s: Ptr<Vec<i8>>) {
    {
        s.with_mut(|__v: &mut Vec<i8>| __v.pop());
        s.with_mut(|__v: &mut Vec<i8>| {
            __v.resize(
                (*s.upgrade().deref()).len() + (1_usize) as usize,
                ('r' as i8),
            )
        });
        s.with_mut(|__v: &mut Vec<i8>| __v.push(0));
        (*s.upgrade().deref()).clone()
    };
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let lengths: Value<Box<[usize]>> = Rc::new(RefCell::new(Box::new([
        0_usize, 1_usize, 2_usize, 3_usize, 7_usize, 8_usize, 15_usize, 16_usize, 31_usize,
        64_usize,
    ])));
    let counts: Value<Box<[usize]>> = Rc::new(RefCell::new(Box::new([
        0_usize, 1_usize, 2_usize, 3_usize, 8_usize, 33_usize,
    ])));
    let characters: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
        0_i8,
        1_i8,
        2_i8,
        7_i8,
        9_i8,
        10_i8,
        13_i8,
        31_i8,
        32_i8,
        42_i8,
        65_i8,
        90_i8,
        97_i8,
        100_i8,
        127_i8,
        (-128_i32 as i8),
        (-127_i32 as i8),
        (-64_i32 as i8),
        (-2_i32 as i8),
        (-1_i32 as i8),
    ])));
    let mut initial: [i8; 5] = [
        ('a' as i8),
        0_i8,
        ('b' as i8),
        (-128_i32 as i8),
        (-1_i32 as i8),
    ];
    let mut cases: usize = 0_usize;
    'loop_: for length in IterRange::new(
        (lengths.as_pointer() as Ptr<usize>),
        (lengths.as_pointer() as Ptr<usize>).offset((10_i64) as isize),
    ) {
        let mut length: usize = (length.read());
        'loop_: for count in IterRange::new(
            (counts.as_pointer() as Ptr<usize>),
            (counts.as_pointer() as Ptr<usize>).offset((6_i64) as isize),
        ) {
            let mut count: usize = (count.read());
            'loop_: for character in IterRange::new(
                (characters.as_pointer() as Ptr<i8>),
                (characters.as_pointer() as Ptr<i8>).offset((20_i64) as isize),
            ) {
                let character: Value<i8> = Rc::new(RefCell::new((character.read())));
                let mut shrink: i32 = 0;
                'loop_: while (shrink < 2) {
                    let s: Value<Vec<i8>> = Rc::new(RefCell::new(
                        vec![('x' as i8); (length) as usize]
                            .iter()
                            .cloned()
                            .chain(std::iter::once(0))
                            .collect(),
                    ));
                    let mut i: usize = 0_usize;
                    'loop_: while (i < length) {
                        let __rhs = initial[((i as u64)
                            .wrapping_rem((::std::mem::size_of::<[i8; 5]>() as u64)))
                            as usize];
                        (*s.borrow_mut())[i] = __rhs;
                        i.prefix_inc();
                    }
                    if (shrink != 0) {
                        (*s.borrow_mut()).shrink_to_fit();
                    }
                    {
                        (*s.borrow_mut()).pop();
                        (*s.borrow_mut()).resize(
                            (*s.borrow()).len() + (count) as usize,
                            ({ (*character.borrow()) }),
                        );
                        (*s.borrow_mut()).push(0);
                        (*s.borrow()).clone()
                    };
                    assert!(({ (((*s.borrow()).len() - 1) == (length).wrapping_add(count)) }));
                    let mut data: Ptr<i8> = s.decay();
                    let mut i: usize = 0_usize;
                    'loop_: while (i < (length).wrapping_add(count)) {
                        assert!(
                            ({
                                ({ ((elem!(data, i).read()) as i32) } == {
                                    ((if (i < length) {
                                        initial[((i as u64).wrapping_rem(
                                            (::std::mem::size_of::<[i8; 5]>() as u64),
                                        )) as usize]
                                    } else {
                                        (*character.borrow())
                                    }) as i32)
                                })
                            })
                        );
                        i.prefix_inc();
                    }
                    assert!(({ (((elem!(data, ((*s.borrow()).len() - 1)).read()) as i32) == 0) }));
                    cases.prefix_inc();
                    shrink.prefix_inc();
                }
            }
        }
    }
    let mut unboxed: Vec<i8> = {
        let mut __bytes = Ptr::<i8>::from_string_literal(b"a").to_c_bytes();
        __bytes.push(0);
        __bytes
    };
    {
        unboxed.pop();
        unboxed.resize(unboxed.len() + (2_usize) as usize, ('b' as i8));
        unboxed.push(0);
        unboxed.clone()
    };
    assert!(
        Ptr::<i8>::from_string_literal(b"abb")
            .with_c_str(|__s| unboxed[..unboxed.len().saturating_sub(1)] == *__s)
    );
    let indirect: Value<Vec<i8>> = Rc::new(RefCell::new({
        let mut __bytes = Ptr::<i8>::from_string_literal(b"c").to_c_bytes();
        __bytes.push(0);
        __bytes
    }));
    ({ append_to_reference_0(indirect.as_pointer()) });
    assert!(
        ({
            Ptr::<i8>::from_string_literal(b"cr").with_c_str(|__s| {
                (*indirect.borrow())[..(*indirect.borrow()).len().saturating_sub(1)] == *__s
            })
        })
    );
    let mut pointer: Ptr<Vec<i8>> = (indirect.as_pointer());
    {
        pointer.with_mut(|__v: &mut Vec<i8>| __v.pop());
        pointer.with_mut(|__v: &mut Vec<i8>| {
            __v.resize(
                (*pointer.upgrade().deref()).len() + (0_usize) as usize,
                ('\0' as i8),
            )
        });
        pointer.with_mut(|__v: &mut Vec<i8>| __v.push(0));
        (*pointer.upgrade().deref()).clone()
    };
    assert!(
        ({
            Ptr::<i8>::from_string_literal(b"cr").with_c_str(|__s| {
                (*indirect.borrow())[..(*indirect.borrow()).len().saturating_sub(1)] == *__s
            })
        })
    );
    println!("checked {} appends", cases);
    return 0;
}
pub fn __cpp2rust_init_globals() {}
