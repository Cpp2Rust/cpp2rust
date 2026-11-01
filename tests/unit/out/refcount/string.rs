extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn counted_string_0(calls: Ptr<i32>) -> Vec<i8> {
    calls.with_mut(|__v| __v.prefix_inc());
    return {
        let mut __bytes = Ptr::<i8>::from_string_literal(b"hello").to_c_bytes();
        __bytes.push(0);
        __bytes
    };
}
pub fn counted_ref_1(s: Ptr<Vec<i8>>, calls: Ptr<i32>) -> Ptr<Vec<i8>> {
    calls.with_mut(|__v| __v.prefix_inc());
    return (s).clone();
}
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
    assert!(({ (((*s1.borrow()).len() - 1) == 5_usize) }));
    assert!(({ (((*s1.borrow()).len() - 1) == ((*s1.borrow()).len() - 1)) }));
    assert!(({ (({ (*s1.borrow())[0_usize] } as i32) == (('h' as i8) as i32)) }));
    assert!(({ (({ (*s1.borrow())[1_usize] } as i32) == (('e' as i8) as i32)) }));
    assert!(({ (({ (*s1.borrow())[2_usize] } as i32) == (('l' as i8) as i32)) }));
    assert!(({ (({ (*s1.borrow())[3_usize] } as i32) == (('l' as i8) as i32)) }));
    assert!(({ (({ (*s1.borrow())[4_usize] } as i32) == (('o' as i8) as i32)) }));
    assert!(
        ({
            Ptr::<i8>::from_string_literal(b"hello")
                .with_c_str(|__s| (*s1.borrow())[..(*s1.borrow()).len().saturating_sub(1)] == *__s)
        })
    );
    assert!(
        ({
            Ptr::<i8>::from_string_literal(b"world")
                .with_c_str(|__s| (*s1.borrow())[..(*s1.borrow()).len().saturating_sub(1)] != *__s)
        })
    );
    assert!(
        ({
            !(Ptr::<i8>::from_string_literal(b"hello")
                .with_c_str(|__s| (*s1.borrow())[..(*s1.borrow()).len().saturating_sub(1)] != *__s))
        })
    );
    let mut p1: Ptr<i8> = s1.decay();
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
    let mut p2: Ptr<i8> = s2.decay();
    let mut i: u32 = 0_u32;
    'loop_: while ((i as usize) < ((*s2.borrow()).len() - 1)) {
        assert!(
            ({
                (((elem!(p2, i).read()) as i32) == (('a' as i8) as i32))
                    && (({ (*s2.borrow())[(i as usize)] } as i32) == (('a' as i8) as i32))
            })
        );
        i.prefix_inc();
    }
    assert!(({ (((*s2.borrow()).len() - 1) == 10_usize) }));
    assert!(({ (((*s2.borrow()).len() - 1) == ((*s2.borrow()).len() - 1)) }));
    (*s2.borrow_mut())[0_usize] = ('b' as i8);
    (*s2.borrow_mut())[1_usize] = ('c' as i8);
    assert!(({ (({ (*s2.borrow())[0_usize] } as i32) == (('b' as i8) as i32)) }));
    assert!(({ (({ (*s2.borrow())[1_usize] } as i32) == (('c' as i8) as i32)) }));
    let mut i: u32 = 2_u32;
    'loop_: while ((i as usize) < ((*s2.borrow()).len() - 1)) {
        assert!(
            ({
                (((elem!(p2, i).read()) as i32) == (('a' as i8) as i32))
                    && (({ (*s2.borrow())[(i as usize)] } as i32) == (('a' as i8) as i32))
            })
        );
        i.prefix_inc();
    }
    let s3: Value<Vec<i8>> = Rc::new(RefCell::new(s2.as_pointer().with(|__s: &Vec<i8>| {
        let mut __tmp1 =
            __s[2_usize..::std::cmp::min(2_usize + 5_usize, __s.len().saturating_sub(1))].to_vec();
        __tmp1.push(0);
        __tmp1
    })));
    assert!(({ (((*s3.borrow()).len() - 1) == 5_usize) }));
    assert!(({ (((*s3.borrow()).len() - 1) == ((*s3.borrow()).len() - 1)) }));
    let mut p3: Ptr<i8> = s3.decay();
    let mut i: u32 = 0_u32;
    'loop_: while ((i as usize) < ((*s3.borrow()).len() - 1)) {
        assert!(
            ({
                ({ ((elem!(p3, i).read()) as i32) } == {
                    ({ (*s3.borrow())[(i as usize)] } as i32)
                })
            })
        );
        i.prefix_inc();
    }
    let s4: Value<Vec<i8>> = Rc::new(RefCell::new(
        match ({
            Ptr::<i8>::from_string_literal(b"l").with_c_str(|__lookup| {
                (*s1.borrow())
                    .iter()
                    .take((*s1.borrow()).len().saturating_sub(1))
                    .rposition(|&x| __lookup.contains(&x))
                    .unwrap_or(usize::MAX)
            })
        },)
        {
            (mut __tmp_0,) => s1.as_pointer().with(|__s: &Vec<i8>| {
                let mut __tmp1 = __s
                    [1_usize..::std::cmp::min(1_usize + __tmp_0, __s.len().saturating_sub(1))]
                    .to_vec();
                __tmp1.push(0);
                __tmp1
            }),
        },
    ));
    assert!(({ (((*s4.borrow()).len() - 1) == 3_usize) }));
    assert!(({ (((*s4.borrow()).len() - 1) == ((*s4.borrow()).len() - 1)) }));
    let mut p4: Ptr<i8> = s4.decay();
    let mut i: u32 = 0_u32;
    'loop_: while ((i as usize) < ((*s4.borrow()).len() - 1)) {
        assert!(
            ({
                ({ ((elem!(p4, i).read()) as i32) } == {
                    ({ (*s4.borrow())[(i as usize)] } as i32)
                })
            })
        );
        i.prefix_inc();
    }
    let s5: Value<Vec<i8>> = Rc::new(RefCell::new({
        let mut r = ({ (*s1.borrow()).clone() });
        r.pop();
        Ptr::<i8>::from_string_literal(b", world").with_c_str(|__s| r.extend_from_slice(__s));
        r.push(0);
        r
    }));
    assert!(({ (((*s5.borrow()).len() - 1) == 12_usize) }));
    assert!(({ (((*s5.borrow()).len() - 1) == ((*s5.borrow()).len() - 1)) }));
    let mut p5: Ptr<i8> = s5.decay();
    let mut i: u32 = 0_u32;
    'loop_: while ((i as usize) < ((*s5.borrow()).len() - 1)) {
        assert!(
            ({
                ({ ((elem!(p5, i).read()) as i32) } == {
                    ({ (*s5.borrow())[(i as usize)] } as i32)
                })
            })
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
    let result: Value<Vec<i8>> = Rc::new(RefCell::new({
        let mut r = (string).clone();
        r.pop();
        Ptr::<i8>::from_string_literal(b" foo").with_c_str(|__s| r.extend_from_slice(__s));
        r.push(0);
        r
    }));
    assert!(({ (((*result.borrow()).len() - 1) == 8_usize) }));
    assert!(({ (({ (*result.borrow())[0_usize] } as i32) == (('b' as i8) as i32)) }));
    assert!(({ (({ (*result.borrow())[1_usize] } as i32) == (('a' as i8) as i32)) }));
    assert!(({ (({ (*result.borrow())[2_usize] } as i32) == (('r' as i8) as i32)) }));
    assert!(({ (({ (*result.borrow())[3_usize] } as i32) == 0) }));
    assert!(({ (({ (*result.borrow())[4_usize] } as i32) == ((' ' as i8) as i32)) }));
    assert!(({ (({ (*result.borrow())[5_usize] } as i32) == (('f' as i8) as i32)) }));
    assert!(({ (({ (*result.borrow())[6_usize] } as i32) == (('o' as i8) as i32)) }));
    assert!(({ (({ (*result.borrow())[7_usize] } as i32) == (('o' as i8) as i32)) }));
    let mut substr_0: Vec<i8> = result.as_pointer().with(|__s: &Vec<i8>| {
        let mut __tmp1 =
            __s[5_usize..::std::cmp::min(5_usize + 3_usize, __s.len().saturating_sub(1))].to_vec();
        __tmp1.push(0);
        __tmp1
    });
    assert!(((substr_0.len() - 1) == 3_usize));
    assert!(((substr_0[0_usize] as i32) == (('f' as i8) as i32)));
    assert!(((substr_0[1_usize] as i32) == (('o' as i8) as i32)));
    assert!(((substr_0[2_usize] as i32) == (('o' as i8) as i32)));
    let mut substr_1: Vec<i8> = result.as_pointer().with(|__s: &Vec<i8>| {
        let mut __tmp1 =
            __s[0_usize..::std::cmp::min(0_usize + 5_usize, __s.len().saturating_sub(1))].to_vec();
        __tmp1.push(0);
        __tmp1
    });
    assert!(((substr_1.len() - 1) == 5_usize));
    assert!(((substr_1[0_usize] as i32) == (('b' as i8) as i32)));
    assert!(((substr_1[1_usize] as i32) == (('a' as i8) as i32)));
    assert!(((substr_1[2_usize] as i32) == (('r' as i8) as i32)));
    assert!(((substr_1[3_usize] as i32) == 0));
    assert!(((substr_1[4_usize] as i32) == ((' ' as i8) as i32)));
    let mut substr_2: Vec<i8> = result.as_pointer().with(|__s: &Vec<i8>| {
        let mut __tmp1 =
            __s[0_usize..::std::cmp::min(0_usize + 15_usize, __s.len().saturating_sub(1))].to_vec();
        __tmp1.push(0);
        __tmp1
    });
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
        (*result.borrow())
            .iter()
            .take((*result.borrow()).len().saturating_sub(1))
            .rposition(|&x| __lookup.contains(&x))
            .unwrap_or(usize::MAX)
    });
    assert!((pos == 0_usize));
    pos = Ptr::<i8>::from_string_literal(b"f").with_c_str(|__lookup| {
        (*result.borrow())
            .iter()
            .take((*result.borrow()).len().saturating_sub(1))
            .rposition(|&x| __lookup.contains(&x))
            .unwrap_or(usize::MAX)
    });
    assert!((pos == 5_usize));
    pos = Ptr::<i8>::from_string_literal(b"o").with_c_str(|__lookup| {
        (*result.borrow())
            .iter()
            .take((*result.borrow()).len().saturating_sub(1))
            .rposition(|&x| __lookup.contains(&x))
            .unwrap_or(usize::MAX)
    });
    assert!((pos == 7_usize));
    pos = Ptr::<i8>::from_string_literal(b"x").with_c_str(|__lookup| {
        (*result.borrow())
            .iter()
            .take((*result.borrow()).len().saturating_sub(1))
            .rposition(|&x| __lookup.contains(&x))
            .unwrap_or(usize::MAX)
    });
    assert!((pos == ((-1_i64 as u64) as usize)));
    pos = {
        let __hay = &(*result.borrow())[..(*result.borrow()).len().saturating_sub(1)];
        Ptr::<i8>::from_string_literal(b"f").with_c_str(|__needle| {
            (0_usize..=__hay.len().saturating_sub(__needle.len()))
                .find(|&__i| __hay[__i..].starts_with(__needle))
                .unwrap_or(usize::MAX)
        })
    };
    assert!((pos == 5_usize));
    pos = {
        let __hay = &(*result.borrow())[..(*result.borrow()).len().saturating_sub(1)];
        Ptr::<i8>::from_string_literal(b"oo").with_c_str(|__needle| {
            (0_usize..=__hay.len().saturating_sub(__needle.len()))
                .find(|&__i| __hay[__i..].starts_with(__needle))
                .unwrap_or(usize::MAX)
        })
    };
    assert!((pos == 6_usize));
    pos = {
        let __hay = &(*result.borrow())[..(*result.borrow()).len().saturating_sub(1)];
        Ptr::<i8>::from_string_literal(b"x").with_c_str(|__needle| {
            (0_usize..=__hay.len().saturating_sub(__needle.len()))
                .find(|&__i| __hay[__i..].starts_with(__needle))
                .unwrap_or(usize::MAX)
        })
    };
    assert!((pos == ((-1_i64 as u64) as usize)));
    let string_to_cast: Value<Vec<i8>> = Rc::new(RefCell::new({
        let mut __bytes = Ptr::<i8>::from_string_literal(b"cast").to_c_bytes();
        __bytes.push(0);
        __bytes
    }));
    let mut output_data: Ptr<u8> =
        (string_to_cast.decay().offset(0_usize)).reinterpret_cast::<u8>();
    assert!((((output_data.read()) as i32) == (('c' as i8) as i32)));
    assert!((((elem!(output_data, 1).read()) as i32) == (('a' as i8) as i32)));
    assert!((((elem!(output_data, 2).read()) as i32) == (('s' as i8) as i32)));
    assert!((((elem!(output_data, 3).read()) as i32) == (('t' as i8) as i32)));
    let mut diff: i64 = (-3_i32 as i64);
    assert!((diff < 0_i64));
    diff += 6_i64;
    assert!((diff == 3_i64));
    let mut substr_pos: usize = 1_usize;
    let once: Value<Vec<i8>> = Rc::new(RefCell::new(match ({ substr_pos.postfix_inc() },) {
        (mut __tmp_1,) => s1.as_pointer().with(|__s: &Vec<i8>| {
            let mut __tmp1 = __s
                [__tmp_1..::std::cmp::min(__tmp_1 + 2_usize, __s.len().saturating_sub(1))]
                .to_vec();
            __tmp1.push(0);
            __tmp1
        }),
    }));
    println!("position: {} {}", once.decay(), substr_pos);
    let mut substr_count: usize = 2_usize;
    once.as_pointer()
        .write(match ({ substr_count.postfix_inc() },) {
            (mut __tmp_2,) => s1.as_pointer().with(|__s: &Vec<i8>| {
                let mut __tmp1 = __s
                    [1_usize..::std::cmp::min(1_usize + __tmp_2, __s.len().saturating_sub(1))]
                    .to_vec();
                __tmp1.push(0);
                __tmp1
            }),
        });
    println!("count: {} {}", once.decay(), substr_count);
    let receiver_calls: Value<i32> = Rc::new(RefCell::new(0));
    once.as_pointer().write(
        Rc::new(RefCell::new(
            ({ counted_string_0(receiver_calls.as_pointer()) }),
        ))
        .as_pointer()
        .with(|__s: &Vec<i8>| {
            let mut __tmp1 = __s
                [1_usize..::std::cmp::min(1_usize + 2_usize, __s.len().saturating_sub(1))]
                .to_vec();
            __tmp1.push(0);
            __tmp1
        }),
    );
    println!("receiver: {} {}", once.decay(), (*receiver_calls.borrow()));
    let ref_calls: Value<i32> = Rc::new(RefCell::new(0));
    let target: Value<Vec<i8>> = Rc::new(RefCell::new({
        let mut __bytes = Ptr::<i8>::from_string_literal(b"ab").to_c_bytes();
        __bytes.push(0);
        __bytes
    }));
    match (({ counted_ref_1(target.as_pointer(), ref_calls.as_pointer()) }),) {
        (__tmp_3,) => {
            __tmp_3.with_mut(|__v: &mut Vec<i8>| __v.pop());
            __tmp_3.with_mut(|__v: &mut Vec<i8>| __v.resize((1_usize) as usize, 0));
            __tmp_3.with_mut(|__v: &mut Vec<i8>| __v.push(0))
        }
    };
    match (({ counted_ref_1(target.as_pointer(), ref_calls.as_pointer()) }),) {
        (__tmp_4,) => {
            if 0_usize as usize >= (*__tmp_4.upgrade().deref()).len().saturating_sub(1) {
                panic!("out of bounds access")
            } else {
                __tmp_4.decay().offset(0_usize as isize)
            }
        }
    }
    .write(('A' as i8));
    println!(
        "reference receiver: {} {}",
        target.decay(),
        (*ref_calls.borrow())
    );
    let targets: Value<Box<[Vec<i8>]>> = Rc::new(RefCell::new(Box::new([
        {
            let mut __bytes = Ptr::<i8>::from_string_literal(b"cd").to_c_bytes();
            __bytes.push(0);
            __bytes
        },
        {
            let mut __bytes = Ptr::<i8>::from_string_literal(b"ef").to_c_bytes();
            __bytes.push(0);
            __bytes
        },
    ])));
    let mut target_index: i32 = 0;
    match ((targets.as_pointer() as Ptr<Vec<i8>>).offset(target_index.postfix_inc()),) {
        (__tmp_5,) => {
            __tmp_5.with_mut(|__v: &mut Vec<i8>| __v.pop());
            __tmp_5.with_mut(|__v: &mut Vec<i8>| __v.resize((1_usize) as usize, 0));
            __tmp_5.with_mut(|__v: &mut Vec<i8>| __v.push(0))
        }
    };
    println!(
        "element receiver: {} {} {}",
        (targets.as_pointer() as Ptr<Vec<i8>>).offset(0).decay(),
        (targets.as_pointer() as Ptr<Vec<i8>>).offset(1).decay(),
        target_index
    );
    (*receiver_calls.borrow_mut()) = 0;
    let mut before_end: i8 = ((RandomAccessIterator::Vec(
        Rc::new(RefCell::new(
            ({ counted_string_0(receiver_calls.as_pointer()) }),
        ))
        .decay()
        .to_last(),
    )
    .iter_add(-2_isize))
    .iter_deref()
    .read());
    println!(
        "temporary end: {} {}",
        (before_end as i32) as u8 as char,
        (*receiver_calls.borrow())
    );
    let mut t0: usize = ((*s1.borrow()).len() - 1);
    let mut t1: usize = (t0).wrapping_add(((p1.read()) as usize));
    let mut t2: usize = (t1).wrapping_add(((*s2.borrow()).len() - 1));
    let mut t3: usize = (t2).wrapping_add(((*s3.borrow()).len() - 1));
    let mut t4: usize = (t3).wrapping_add(((*s4.borrow()).len() - 1));
    assert!(({ ((t4).wrapping_add(((*s5.borrow()).len() - 1)) == 139_usize) }));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
