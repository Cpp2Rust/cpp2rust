extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn append_to_reference_0(s: Ptr<Vec<i8>>) {
    s.with_mut(|__b0| {
        (*__b0).pop();
        let __len = (*__b0).len() + 1_usize;
        (*__b0).resize(__len, ('r' as i8));
        (*__b0).push(0);
    });
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
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
    'loop_: for length in IterRange::new(lengths.decay(), lengths.decay().offset((10_i64) as isize))
    {
        let mut length: usize = (length.read());
        'loop_: for count in IterRange::new(counts.decay(), counts.decay().offset((6_i64) as isize))
        {
            let mut count: usize = (count.read());
            'loop_: for character in IterRange::new(
                characters.decay(),
                characters.decay().offset((20_i64) as isize),
            ) {
                let mut character: i8 = (character.read());
                {
                    let mut shrink: i32 = 0;
                    'loop_: while (shrink < 2) {
                        let s: Value<Vec<i8>> = Rc::new(RefCell::new(
                            vec![('x' as i8); (length) as usize]
                                .iter()
                                .cloned()
                                .chain(std::iter::once(0))
                                .collect(),
                        ));
                        {
                            let mut i: usize = 0_usize;
                            'loop_: while (i < length) {
                                let __rhs = initial[((i)
                                    .wrapping_rem((::std::mem::size_of::<[i8; 5]>() as usize)))
                                    as usize];
                                (*s.borrow_mut())[i] = __rhs;
                                i.prefix_inc();
                            }
                        }
                        if (shrink != 0) {
                            (*s.borrow_mut()).shrink_to_fit();
                        }
                        {
                            (*s.borrow_mut()).pop();
                            let __len = (*s.borrow()).len() + count;
                            (*s.borrow_mut()).resize(__len, character);
                            (*s.borrow_mut()).push(0);
                        };
                        assert!(({ (((*s.borrow()).len() - 1) == (length).wrapping_add(count)) }));
                        let mut data: Ptr<i8> = s.decay();
                        {
                            let mut i: usize = 0_usize;
                            'loop_: while (i < (length).wrapping_add(count)) {
                                assert!(
                                    ({ ((elem!(data, i).read()) as i32) } == {
                                        ((if (i < length) {
                                            initial[((i).wrapping_rem(
                                                (::std::mem::size_of::<[i8; 5]>() as usize),
                                            ))
                                                as usize]
                                        } else {
                                            character
                                        }) as i32)
                                    })
                                );
                                i.prefix_inc();
                            }
                        }
                        assert!(
                            ({ (((elem!(data, ((*s.borrow()).len() - 1)).read()) as i32) == 0) })
                        );
                        cases.prefix_inc();
                        shrink.prefix_inc();
                    }
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
        let __len = unboxed.len() + 2_usize;
        unboxed.resize(__len, ('b' as i8));
        unboxed.push(0);
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
    pointer.with_mut(|__b0| {
        (*__b0).pop();
        let __len = (*__b0).len() + 0_usize;
        (*__b0).resize(__len, ('\0' as i8));
        (*__b0).push(0);
    });
    assert!(
        ({
            Ptr::<i8>::from_string_literal(b"cr").with_c_str(|__s| {
                (*indirect.borrow())[..(*indirect.borrow()).len().saturating_sub(1)] == *__s
            })
        })
    );
    let chained: Value<Vec<i8>> = Rc::new(RefCell::new({
        let mut __bytes = Ptr::<i8>::from_string_literal(b"a").to_c_bytes();
        __bytes.push(0);
        __bytes
    }));
    {
        {
            (*chained.borrow_mut()).pop();
            let __len = (*chained.borrow()).len() + 3_usize;
            (*chained.borrow_mut()).resize(__len, ('x' as i8));
            (*chained.borrow_mut()).push(0);
        };
        {
            (*chained.borrow_mut()).pop();
            let __len = (*chained.borrow()).len() + 2_usize;
            (*chained.borrow_mut()).resize(__len, ('y' as i8));
            (*chained.borrow_mut()).push(0);
        }
    };
    assert!(
        ({
            Ptr::<i8>::from_string_literal(b"axxxyy").with_c_str(|__s| {
                (*chained.borrow())[..(*chained.borrow()).len().saturating_sub(1)] == *__s
            })
        })
    );
    println!("checked {} appends", cases);
    return 0;
}
pub fn __cpp2rust_init_globals() {}
