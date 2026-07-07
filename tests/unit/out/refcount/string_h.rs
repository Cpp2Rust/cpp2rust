extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_memcpy_0() {
    let src: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::from(*b"hello\0")));
    let dst: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([
        0_u8,
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
    ])));
    let r: Value<AnyPtr> = Rc::new(RefCell::new({
        ((dst.as_pointer() as Ptr<u8>) as Ptr<u8>).to_any().memcpy(
            &((src.as_pointer() as Ptr<u8>) as Ptr<u8>).to_any(),
            6_usize as usize,
        );
        ((dst.as_pointer() as Ptr<u8>) as Ptr<u8>).to_any().clone()
    }));
    assert!(
        ((({
            let _lhs = (*r.borrow()).clone();
            _lhs == ((dst.as_pointer() as Ptr<u8>) as Ptr<u8>).to_any()
        }) as i32)
            != 0)
    );
    assert!(
        (((((((((((*dst.borrow())[(0) as usize] as i32) == ('h' as i32)) as i32) != 0)
            && (((((*dst.borrow())[(1) as usize] as i32) == ('e' as i32)) as i32) != 0))
            as i32)
            != 0)
            && (((((*dst.borrow())[(2) as usize] as i32) == ('l' as i32)) as i32) != 0))
            as i32)
            != 0)
    );
    assert!(
        (((((((((((*dst.borrow())[(3) as usize] as i32) == ('l' as i32)) as i32) != 0)
            && (((((*dst.borrow())[(4) as usize] as i32) == ('o' as i32)) as i32) != 0))
            as i32)
            != 0)
            && (((((*dst.borrow())[(5) as usize] as i32) == ('\0' as i32)) as i32) != 0))
            as i32)
            != 0)
    );
}
pub fn test_memset_1() {
    let buf: Value<Box<[u8]>> = Rc::new(RefCell::new(
        (0..4).map(|_| <u8>::default()).collect::<Box<[u8]>>(),
    ));
    let r: Value<AnyPtr> = Rc::new(RefCell::new({
        ((buf.as_pointer() as Ptr<u8>) as Ptr<u8>)
            .to_any()
            .memset(('x' as i32) as u8, 4_usize as usize);
        ((buf.as_pointer() as Ptr<u8>) as Ptr<u8>).to_any().clone()
    }));
    assert!(
        ((({
            let _lhs = (*r.borrow()).clone();
            _lhs == ((buf.as_pointer() as Ptr<u8>) as Ptr<u8>).to_any()
        }) as i32)
            != 0)
    );
    assert!(
        ((((((((((((((*buf.borrow())[(0) as usize] as i32) == ('x' as i32)) as i32) != 0)
            && (((((*buf.borrow())[(1) as usize] as i32) == ('x' as i32)) as i32) != 0))
            as i32)
            != 0)
            && (((((*buf.borrow())[(2) as usize] as i32) == ('x' as i32)) as i32) != 0))
            as i32)
            != 0)
            && (((((*buf.borrow())[(3) as usize] as i32) == ('x' as i32)) as i32) != 0))
            as i32)
            != 0)
    );
}
pub fn test_memcmp_2() {
    let a: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([1_u8, 2_u8, 3_u8, 4_u8])));
    let b: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([1_u8, 2_u8, 3_u8, 4_u8])));
    let c: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([1_u8, 2_u8, 9_u8, 4_u8])));
    assert!(
        (((((a.as_pointer() as Ptr::<u8>) as Ptr::<u8>)
            .to_any()
            .memcmp(
                &((b.as_pointer() as Ptr::<u8>) as Ptr::<u8>).to_any(),
                4_usize
            )
            == 0) as i32)
            != 0)
    );
    assert!(
        (((((a.as_pointer() as Ptr::<u8>) as Ptr::<u8>)
            .to_any()
            .memcmp(
                &((c.as_pointer() as Ptr::<u8>) as Ptr::<u8>).to_any(),
                4_usize
            )
            < 0) as i32)
            != 0)
    );
    assert!(
        (((((c.as_pointer() as Ptr::<u8>) as Ptr::<u8>)
            .to_any()
            .memcmp(
                &((a.as_pointer() as Ptr::<u8>) as Ptr::<u8>).to_any(),
                4_usize
            )
            > 0) as i32)
            != 0)
    );
}
pub fn test_memmove_3() {
    let buf: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([
        (('a' as i32) as u8),
        (('b' as i32) as u8),
        (('c' as i32) as u8),
        (('d' as i32) as u8),
        (('e' as i32) as u8),
        (('\0' as i32) as u8),
    ])));
    let r: Value<AnyPtr> = Rc::new(RefCell::new({
        let tmp: Vec<u8> = (0..4_usize)
            .map(|i| {
                ((buf.as_pointer() as Ptr<u8>) as Ptr<u8>)
                    .to_any()
                    .reinterpret_cast::<u8>()
                    .offset(i)
                    .read()
            })
            .collect();
        for i in 0..4_usize {
            ((buf.as_pointer() as Ptr<u8>).offset((1) as isize) as Ptr<u8>)
                .to_any()
                .reinterpret_cast::<u8>()
                .offset(i)
                .write(tmp[i]);
        }
        ((buf.as_pointer() as Ptr<u8>).offset((1) as isize) as Ptr<u8>)
            .to_any()
            .clone()
    }));
    assert!(
        ((({
            let _lhs = (*r.borrow()).clone();
            _lhs == ((buf.as_pointer() as Ptr<u8>).offset((1) as isize) as Ptr<u8>).to_any()
        }) as i32)
            != 0)
    );
    assert!(
        (((((((((((*buf.borrow())[(0) as usize] as i32) == ('a' as i32)) as i32) != 0)
            && (((((*buf.borrow())[(1) as usize] as i32) == ('a' as i32)) as i32) != 0))
            as i32)
            != 0)
            && (((((*buf.borrow())[(2) as usize] as i32) == ('b' as i32)) as i32) != 0))
            as i32)
            != 0)
    );
    assert!(
        (((((((((((*buf.borrow())[(3) as usize] as i32) == ('c' as i32)) as i32) != 0)
            && (((((*buf.borrow())[(4) as usize] as i32) == ('d' as i32)) as i32) != 0))
            as i32)
            != 0)
            && (((((*buf.borrow())[(5) as usize] as i32) == ('\0' as i32)) as i32) != 0))
            as i32)
            != 0)
    );
}
pub fn test_strchr_4() {
    let s: Value<Ptr<u8>> = Rc::new(RefCell::new(Ptr::from_string_literal(b"hello world")));
    let r: Value<Ptr<u8>> = Rc::new(RefCell::new({
        let mut i: usize = 0;
        loop {
            let c = (*s.borrow()).reinterpret_cast::<u8>().offset(i).read();
            if c == ('w' as i32) as u8 {
                break (*s.borrow()).reinterpret_cast::<u8>().offset(i);
            }
            if c == 0 {
                break Ptr::null();
            }
            i += 1;
        }
    }));
    assert!((((!((*r.borrow()).is_null())) as i32) != 0));
    assert!(((((((*r.borrow()).read()) as i32) == ('w' as i32)) as i32) != 0));
    assert!(
        (((({
            let mut i: usize = 0;
            loop {
                let c = (*s.borrow()).reinterpret_cast::<u8>().offset(i).read();
                if c == ('z' as i32) as u8 {
                    break (*s.borrow()).reinterpret_cast::<u8>().offset(i);
                }
                if c == 0 {
                    break Ptr::null();
                }
                i += 1;
            }
        })
        .is_null()) as i32)
            != 0)
    );
}
pub fn test_strlen_5() {
    assert!(
        ((({
            let mut i: usize = 0;
            while Ptr::from_string_literal(b"").offset(i).read() != 0 {
                i += 1;
            }
            i
        } == 0_usize) as i32)
            != 0)
    );
    assert!(
        ((({
            let mut i: usize = 0;
            while Ptr::from_string_literal(b"hello").offset(i).read() != 0 {
                i += 1;
            }
            i
        } == 5_usize) as i32)
            != 0)
    );
    assert!(
        ((({
            let mut i: usize = 0;
            while Ptr::from_string_literal(b"hello world").offset(i).read() != 0 {
                i += 1;
            }
            i
        } == 11_usize) as i32)
            != 0)
    );
    let buf: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::from(*b"one\0two\0")));
    let first: Value<Ptr<u8>> = Rc::new(RefCell::new((buf.as_pointer() as Ptr<u8>)));
    let second: Value<Ptr<u8>> = Rc::new(RefCell::new(
        ((buf.as_pointer() as Ptr<u8>).offset(
            ({
                let mut i: usize = 0;
                while (*first.borrow()).offset(i).read() != 0 {
                    i += 1;
                }
                i
            })
            .wrapping_add(1_usize),
        )),
    ));
    assert!(
        ((({
            let mut i: usize = 0;
            loop {
                let c1 = (*second.borrow()).offset(i).read();
                let c2 = Ptr::from_string_literal(b"two").offset(i).read();
                if c1 != c2 {
                    break (c1 as i32) - (c2 as i32);
                }
                if c1 == 0 {
                    break 0;
                }
                i += 1;
            }
        } == 0) as i32)
            != 0)
    );
    let  big : Value<Box<[ u8  ]> > = Rc::new(RefCell::new(Box::from(*b"hi\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0") )) ;
    assert!(
        ((({
            let mut i: usize = 0;
            while (big.as_pointer() as Ptr<u8>).offset(i).read() != 0 {
                i += 1;
            }
            i
        } == 2_usize) as i32)
            != 0)
    );
    (*big.borrow_mut())[(2) as usize] = (('x' as i32) as u8);
    (*big.borrow_mut())[(3) as usize] = (('\0' as i32) as u8);
    assert!(
        ((({
            let mut i: usize = 0;
            while (big.as_pointer() as Ptr<u8>).offset(i).read() != 0 {
                i += 1;
            }
            i
        } == 3_usize) as i32)
            != 0)
    );
}
pub fn test_strcmp_6() {
    assert!(
        ((({
            let mut i: usize = 0;
            loop {
                let c1 = Ptr::from_string_literal(b"abc").offset(i).read();
                let c2 = Ptr::from_string_literal(b"abc").offset(i).read();
                if c1 != c2 {
                    break (c1 as i32) - (c2 as i32);
                }
                if c1 == 0 {
                    break 0;
                }
                i += 1;
            }
        } == 0) as i32)
            != 0)
    );
    assert!(
        ((({
            let mut i: usize = 0;
            loop {
                let c1 = Ptr::from_string_literal(b"abc").offset(i).read();
                let c2 = Ptr::from_string_literal(b"abd").offset(i).read();
                if c1 != c2 {
                    break (c1 as i32) - (c2 as i32);
                }
                if c1 == 0 {
                    break 0;
                }
                i += 1;
            }
        } < 0) as i32)
            != 0)
    );
    assert!(
        ((({
            let mut i: usize = 0;
            loop {
                let c1 = Ptr::from_string_literal(b"abd").offset(i).read();
                let c2 = Ptr::from_string_literal(b"abc").offset(i).read();
                if c1 != c2 {
                    break (c1 as i32) - (c2 as i32);
                }
                if c1 == 0 {
                    break 0;
                }
                i += 1;
            }
        } > 0) as i32)
            != 0)
    );
    let p: Value<Ptr<u8>> = Rc::new(RefCell::new(Ptr::from_string_literal(b"abc")));
    let q: Value<Ptr<u8>> = Rc::new(RefCell::new(Ptr::from_string_literal(b"abd")));
    let buf: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([
        (('a' as i32) as u8),
        (('b' as i32) as u8),
        (('c' as i32) as u8),
        (('\0' as i32) as u8),
    ])));
    assert!(
        ((({
            let mut i: usize = 0;
            loop {
                let c1 = (*p.borrow()).offset(i).read();
                let c2 = (*p.borrow()).offset(i).read();
                if c1 != c2 {
                    break (c1 as i32) - (c2 as i32);
                }
                if c1 == 0 {
                    break 0;
                }
                i += 1;
            }
        } == 0) as i32)
            != 0)
    );
    assert!(
        ((({
            let mut i: usize = 0;
            loop {
                let c1 = (*p.borrow()).offset(i).read();
                let c2 = (*q.borrow()).offset(i).read();
                if c1 != c2 {
                    break (c1 as i32) - (c2 as i32);
                }
                if c1 == 0 {
                    break 0;
                }
                i += 1;
            }
        } < 0) as i32)
            != 0)
    );
    assert!(
        ((({
            let mut i: usize = 0;
            loop {
                let c1 = (buf.as_pointer() as Ptr<u8>).offset(i).read();
                let c2 = (*p.borrow()).offset(i).read();
                if c1 != c2 {
                    break (c1 as i32) - (c2 as i32);
                }
                if c1 == 0 {
                    break 0;
                }
                i += 1;
            }
        } == 0) as i32)
            != 0)
    );
}
pub fn test_strncmp_7() {
    assert!(
        ((({
            let mut i: usize = 0;
            loop {
                if i == 3_usize {
                    break 0;
                }
                let c1 = Ptr::from_string_literal(b"abcdef").offset(i).read();
                let c2 = Ptr::from_string_literal(b"abcxyz").offset(i).read();
                if c1 != c2 {
                    break (c1 as i32) - (c2 as i32);
                }
                if c1 == 0 {
                    break 0;
                }
                i += 1;
            }
        } == 0) as i32)
            != 0)
    );
    assert!(
        ((({
            let mut i: usize = 0;
            loop {
                if i == 4_usize {
                    break 0;
                }
                let c1 = Ptr::from_string_literal(b"abcdef").offset(i).read();
                let c2 = Ptr::from_string_literal(b"abcxyz").offset(i).read();
                if c1 != c2 {
                    break (c1 as i32) - (c2 as i32);
                }
                if c1 == 0 {
                    break 0;
                }
                i += 1;
            }
        } < 0) as i32)
            != 0)
    );
    assert!(
        ((({
            let mut i: usize = 0;
            loop {
                if i == 4_usize {
                    break 0;
                }
                let c1 = Ptr::from_string_literal(b"abcxyz").offset(i).read();
                let c2 = Ptr::from_string_literal(b"abcdef").offset(i).read();
                if c1 != c2 {
                    break (c1 as i32) - (c2 as i32);
                }
                if c1 == 0 {
                    break 0;
                }
                i += 1;
            }
        } > 0) as i32)
            != 0)
    );
    let p: Value<Ptr<u8>> = Rc::new(RefCell::new(Ptr::from_string_literal(b"abcdef")));
    let q: Value<Ptr<u8>> = Rc::new(RefCell::new(Ptr::from_string_literal(b"abcxyz")));
    let buf: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([
        (('a' as i32) as u8),
        (('b' as i32) as u8),
        (('c' as i32) as u8),
        (('d' as i32) as u8),
        (('e' as i32) as u8),
        (('f' as i32) as u8),
        (('\0' as i32) as u8),
    ])));
    let n: Value<usize> = Rc::new(RefCell::new(3_usize));
    assert!(
        ((({
            let mut i: usize = 0;
            loop {
                if i == (*n.borrow()) {
                    break 0;
                }
                let c1 = (*p.borrow()).offset(i).read();
                let c2 = (*q.borrow()).offset(i).read();
                if c1 != c2 {
                    break (c1 as i32) - (c2 as i32);
                }
                if c1 == 0 {
                    break 0;
                }
                i += 1;
            }
        } == 0) as i32)
            != 0)
    );
    assert!(
        ((({
            let mut i: usize = 0;
            loop {
                if i == (*n.borrow()).wrapping_add(1_usize) {
                    break 0;
                }
                let c1 = (*p.borrow()).offset(i).read();
                let c2 = (*q.borrow()).offset(i).read();
                if c1 != c2 {
                    break (c1 as i32) - (c2 as i32);
                }
                if c1 == 0 {
                    break 0;
                }
                i += 1;
            }
        } < 0) as i32)
            != 0)
    );
    assert!(
        ((({
            let mut i: usize = 0;
            loop {
                if i == 6_usize {
                    break 0;
                }
                let c1 = (buf.as_pointer() as Ptr<u8>).offset(i).read();
                let c2 = (*p.borrow()).offset(i).read();
                if c1 != c2 {
                    break (c1 as i32) - (c2 as i32);
                }
                if c1 == 0 {
                    break 0;
                }
                i += 1;
            }
        } == 0) as i32)
            != 0)
    );
}
pub fn test_memchr_8() {
    let data: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([16_u8, 32_u8, 48_u8, 64_u8])));
    let r: Value<AnyPtr> = Rc::new(RefCell::new(
        match (0..4_usize).find(|&i| {
            ((data.as_pointer() as Ptr<u8>) as Ptr<u8>)
                .to_any()
                .reinterpret_cast::<u8>()
                .offset(i)
                .read()
                == 48 as u8
        }) {
            Some(i) => ((data.as_pointer() as Ptr<u8>) as Ptr<u8>)
                .to_any()
                .reinterpret_cast::<u8>()
                .offset(i)
                .to_any(),
            None => Ptr::<u8>::null().to_any(),
        },
    ));
    assert!(
        ((({
            let _lhs = (*r.borrow()).clone();
            _lhs == (((data.as_pointer() as Ptr<u8>).offset(2)) as Ptr<u8>).to_any()
        }) as i32)
            != 0)
    );
    assert!(
        ((((match (0..4_usize).find(|&i| ((data.as_pointer() as Ptr::<u8>) as Ptr::<u8>)
            .to_any()
            .reinterpret_cast::<u8>()
            .offset(i)
            .read()
            == 153 as u8)
        {
            Some(i) => ((data.as_pointer() as Ptr::<u8>) as Ptr::<u8>)
                .to_any()
                .reinterpret_cast::<u8>()
                .offset(i)
                .to_any(),
            None => Ptr::<u8>::null().to_any(),
        })
        .is_null()) as i32)
            != 0)
    );
    let p: Value<AnyPtr> = Rc::new(RefCell::new(
        ((data.as_pointer() as Ptr<u8>) as Ptr<u8>).to_any(),
    ));
    let n: Value<usize> = Rc::new(RefCell::new(4_usize));
    assert!(
        ((({
            let _lhs = match (0..(*n.borrow()))
                .find(|&i| (*p.borrow()).reinterpret_cast::<u8>().offset(i).read() == 16 as u8)
            {
                Some(i) => (*p.borrow()).reinterpret_cast::<u8>().offset(i).to_any(),
                None => Ptr::<u8>::null().to_any(),
            };
            _lhs == (*p.borrow()).clone()
        }) as i32)
            != 0)
    );
}
pub fn test_strrchr_9() {
    let s: Value<Ptr<u8>> = Rc::new(RefCell::new(Ptr::from_string_literal(b"hello world")));
    let r: Value<Ptr<u8>> = Rc::new(RefCell::new({
        let mut i: usize = 0;
        let mut found = Ptr::null();
        loop {
            let c = (*s.borrow()).reinterpret_cast::<u8>().offset(i).read();
            if c == ('l' as i32) as u8 {
                found = (*s.borrow()).reinterpret_cast::<u8>().offset(i);
            }
            if c == 0 {
                break found;
            }
            i += 1;
        }
    }));
    assert!((((!((*r.borrow()).is_null())) as i32) != 0));
    assert!(((((((*r.borrow()).read()) as i32) == ('l' as i32)) as i32) != 0));
    assert!(
        ((({
            let _lhs = (*r.borrow()).clone();
            _lhs == (*s.borrow()).offset((9) as isize)
        }) as i32)
            != 0)
    );
    assert!(
        (((({
            let mut i: usize = 0;
            let mut found = Ptr::null();
            loop {
                let c = (*s.borrow()).reinterpret_cast::<u8>().offset(i).read();
                if c == ('z' as i32) as u8 {
                    found = (*s.borrow()).reinterpret_cast::<u8>().offset(i);
                }
                if c == 0 {
                    break found;
                }
                i += 1;
            }
        })
        .is_null()) as i32)
            != 0)
    );
    let buf: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([
        (('a' as i32) as u8),
        (('b' as i32) as u8),
        (('a' as i32) as u8),
        (('\0' as i32) as u8),
    ])));
    assert!(
        ((({
            let mut i: usize = 0;
            let mut found = Ptr::null();
            loop {
                let c = (buf.as_pointer() as Ptr<u8>).offset(i).read();
                if c == ('a' as i32) as u8 {
                    found = (buf.as_pointer() as Ptr<u8>).offset(i);
                }
                if c == 0 {
                    break found;
                }
                i += 1;
            }
        } == ((buf.as_pointer() as Ptr<u8>).offset(2))) as i32)
            != 0)
    );
}
pub fn test_strcspn_10() {
    assert!(
        ((({
            let mut i: usize = 0;
            loop {
                let c = Ptr::from_string_literal(b"hello").offset(i).read();
                if c == 0 {
                    break i;
                }
                let mut j: usize = 0;
                let hit = loop {
                    let r = Ptr::from_string_literal(b"el").offset(j).read();
                    if r == 0 {
                        break false;
                    }
                    if r == c {
                        break true;
                    }
                    j += 1;
                };
                if hit {
                    break i;
                }
                i += 1;
            }
        } == 1_usize) as i32)
            != 0)
    );
    assert!(
        ((({
            let mut i: usize = 0;
            loop {
                let c = Ptr::from_string_literal(b"abc").offset(i).read();
                if c == 0 {
                    break i;
                }
                let mut j: usize = 0;
                let hit = loop {
                    let r = Ptr::from_string_literal(b"xyz").offset(j).read();
                    if r == 0 {
                        break false;
                    }
                    if r == c {
                        break true;
                    }
                    j += 1;
                };
                if hit {
                    break i;
                }
                i += 1;
            }
        } == 3_usize) as i32)
            != 0)
    );
    assert!(
        ((({
            let mut i: usize = 0;
            loop {
                let c = Ptr::from_string_literal(b"").offset(i).read();
                if c == 0 {
                    break i;
                }
                let mut j: usize = 0;
                let hit = loop {
                    let r = Ptr::from_string_literal(b"abc").offset(j).read();
                    if r == 0 {
                        break false;
                    }
                    if r == c {
                        break true;
                    }
                    j += 1;
                };
                if hit {
                    break i;
                }
                i += 1;
            }
        } == 0_usize) as i32)
            != 0)
    );
    let s: Value<Ptr<u8>> = Rc::new(RefCell::new(Ptr::from_string_literal(b"hello")));
    let rej: Value<Ptr<u8>> = Rc::new(RefCell::new(Ptr::from_string_literal(b"el")));
    assert!(
        ((({
            let mut i: usize = 0;
            loop {
                let c = (*s.borrow()).offset(i).read();
                if c == 0 {
                    break i;
                }
                let mut j: usize = 0;
                let hit = loop {
                    let r = (*rej.borrow()).offset(j).read();
                    if r == 0 {
                        break false;
                    }
                    if r == c {
                        break true;
                    }
                    j += 1;
                };
                if hit {
                    break i;
                }
                i += 1;
            }
        } == 1_usize) as i32)
            != 0)
    );
}
pub fn test_strspn_11() {
    assert!(
        ((({
            let mut i: usize = 0;
            loop {
                let c = Ptr::from_string_literal(b"hello").offset(i).read();
                if c == 0 {
                    break i;
                }
                let mut j: usize = 0;
                let hit = loop {
                    let r = Ptr::from_string_literal(b"hel").offset(j).read();
                    if r == 0 {
                        break false;
                    }
                    if r == c {
                        break true;
                    }
                    j += 1;
                };
                if !hit {
                    break i;
                }
                i += 1;
            }
        } == 4_usize) as i32)
            != 0)
    );
    assert!(
        ((({
            let mut i: usize = 0;
            loop {
                let c = Ptr::from_string_literal(b"abc").offset(i).read();
                if c == 0 {
                    break i;
                }
                let mut j: usize = 0;
                let hit = loop {
                    let r = Ptr::from_string_literal(b"xyz").offset(j).read();
                    if r == 0 {
                        break false;
                    }
                    if r == c {
                        break true;
                    }
                    j += 1;
                };
                if !hit {
                    break i;
                }
                i += 1;
            }
        } == 0_usize) as i32)
            != 0)
    );
    assert!(
        ((({
            let mut i: usize = 0;
            loop {
                let c = Ptr::from_string_literal(b"aaa").offset(i).read();
                if c == 0 {
                    break i;
                }
                let mut j: usize = 0;
                let hit = loop {
                    let r = Ptr::from_string_literal(b"a").offset(j).read();
                    if r == 0 {
                        break false;
                    }
                    if r == c {
                        break true;
                    }
                    j += 1;
                };
                if !hit {
                    break i;
                }
                i += 1;
            }
        } == 3_usize) as i32)
            != 0)
    );
    let s: Value<Ptr<u8>> = Rc::new(RefCell::new(Ptr::from_string_literal(b"hello")));
    let acc: Value<Ptr<u8>> = Rc::new(RefCell::new(Ptr::from_string_literal(b"hel")));
    assert!(
        ((({
            let mut i: usize = 0;
            loop {
                let c = (*s.borrow()).offset(i).read();
                if c == 0 {
                    break i;
                }
                let mut j: usize = 0;
                let hit = loop {
                    let r = (*acc.borrow()).offset(j).read();
                    if r == 0 {
                        break false;
                    }
                    if r == c {
                        break true;
                    }
                    j += 1;
                };
                if !hit {
                    break i;
                }
                i += 1;
            }
        } == 4_usize) as i32)
            != 0)
    );
}
pub fn test_strstr_12() {
    let h: Value<Ptr<u8>> = Rc::new(RefCell::new(Ptr::from_string_literal(b"hello world")));
    let r: Value<Ptr<u8>> = Rc::new(RefCell::new({
        let mut s: usize = 0;
        loop {
            let mut i: usize = 0;
            let matched = loop {
                let n = Ptr::from_string_literal(b"world").offset(i).read();
                if n == 0 {
                    break true;
                }
                if (*h.borrow()).reinterpret_cast::<u8>().offset(s + i).read() != n {
                    break false;
                }
                i += 1;
            };
            if matched {
                break (*h.borrow()).reinterpret_cast::<u8>().offset(s);
            }
            if (*h.borrow()).reinterpret_cast::<u8>().offset(s).read() == 0 {
                break Ptr::null();
            }
            s += 1;
        }
    }));
    assert!((((!((*r.borrow()).is_null())) as i32) != 0));
    assert!(
        ((({
            let _lhs = (*r.borrow()).clone();
            _lhs == (*h.borrow()).offset((6) as isize)
        }) as i32)
            != 0)
    );
    assert!(
        (((({
            let mut s: usize = 0;
            loop {
                let mut i: usize = 0;
                let matched = loop {
                    let n = Ptr::from_string_literal(b"xyz").offset(i).read();
                    if n == 0 {
                        break true;
                    }
                    if (*h.borrow()).reinterpret_cast::<u8>().offset(s + i).read() != n {
                        break false;
                    }
                    i += 1;
                };
                if matched {
                    break (*h.borrow()).reinterpret_cast::<u8>().offset(s);
                }
                if (*h.borrow()).reinterpret_cast::<u8>().offset(s).read() == 0 {
                    break Ptr::null();
                }
                s += 1;
            }
        })
        .is_null()) as i32)
            != 0)
    );
    let buf: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([
        (('h' as i32) as u8),
        (('e' as i32) as u8),
        (('l' as i32) as u8),
        (('l' as i32) as u8),
        (('o' as i32) as u8),
        (('\0' as i32) as u8),
    ])));
    assert!(
        ((({
            let mut s: usize = 0;
            loop {
                let mut i: usize = 0;
                let matched = loop {
                    let n = Ptr::from_string_literal(b"ll").offset(i).read();
                    if n == 0 {
                        break true;
                    }
                    if (buf.as_pointer() as Ptr<u8>).offset(s + i).read() != n {
                        break false;
                    }
                    i += 1;
                };
                if matched {
                    break (buf.as_pointer() as Ptr<u8>).offset(s);
                }
                if (buf.as_pointer() as Ptr<u8>).offset(s).read() == 0 {
                    break Ptr::null();
                }
                s += 1;
            }
        } == ((buf.as_pointer() as Ptr<u8>).offset(2))) as i32)
            != 0)
    );
}
pub fn test_strpbrk_13() {
    let s: Value<Ptr<u8>> = Rc::new(RefCell::new(Ptr::from_string_literal(b"hello world")));
    let r: Value<Ptr<u8>> = Rc::new(RefCell::new({
        let mut i: usize = 0;
        loop {
            let c = (*s.borrow()).reinterpret_cast::<u8>().offset(i).read();
            if c == 0 {
                break Ptr::null();
            }
            let mut j: usize = 0;
            let hit = loop {
                let r = Ptr::from_string_literal(b"wo").offset(j).read();
                if r == 0 {
                    break false;
                }
                if r == c {
                    break true;
                }
                j += 1;
            };
            if hit {
                break (*s.borrow()).reinterpret_cast::<u8>().offset(i);
            }
            i += 1;
        }
    }));
    assert!((((!((*r.borrow()).is_null())) as i32) != 0));
    assert!(
        ((({
            let _lhs = (*r.borrow()).clone();
            _lhs == (*s.borrow()).offset((4) as isize)
        }) as i32)
            != 0)
    );
    assert!(
        (((({
            let mut i: usize = 0;
            loop {
                let c = (*s.borrow()).reinterpret_cast::<u8>().offset(i).read();
                if c == 0 {
                    break Ptr::null();
                }
                let mut j: usize = 0;
                let hit = loop {
                    let r = Ptr::from_string_literal(b"xyz").offset(j).read();
                    if r == 0 {
                        break false;
                    }
                    if r == c {
                        break true;
                    }
                    j += 1;
                };
                if hit {
                    break (*s.borrow()).reinterpret_cast::<u8>().offset(i);
                }
                i += 1;
            }
        })
        .is_null()) as i32)
            != 0)
    );
    let buf: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([
        (('a' as i32) as u8),
        (('b' as i32) as u8),
        (('c' as i32) as u8),
        (('\0' as i32) as u8),
    ])));
    assert!(
        ((({
            let mut i: usize = 0;
            loop {
                let c = (buf.as_pointer() as Ptr<u8>).offset(i).read();
                if c == 0 {
                    break Ptr::null();
                }
                let mut j: usize = 0;
                let hit = loop {
                    let r = Ptr::from_string_literal(b"b").offset(j).read();
                    if r == 0 {
                        break false;
                    }
                    if r == c {
                        break true;
                    }
                    j += 1;
                };
                if hit {
                    break (buf.as_pointer() as Ptr<u8>).offset(i);
                }
                i += 1;
            }
        } == ((buf.as_pointer() as Ptr<u8>).offset(1))) as i32)
            != 0)
    );
}
pub fn test_strcasecmp_14() {
    assert!(
        ((({
            let mut i: usize = 0;
            loop {
                let c1 = Ptr::from_string_literal(b"HELLO")
                    .offset(i)
                    .read()
                    .to_ascii_lowercase();
                let c2 = Ptr::from_string_literal(b"hello")
                    .offset(i)
                    .read()
                    .to_ascii_lowercase();
                if c1 != c2 {
                    break (c1 as i32) - (c2 as i32);
                }
                if c1 == 0 {
                    break 0;
                }
                i += 1;
            }
        } == 0) as i32)
            != 0)
    );
    assert!(
        ((({
            let mut i: usize = 0;
            loop {
                let c1 = Ptr::from_string_literal(b"abc")
                    .offset(i)
                    .read()
                    .to_ascii_lowercase();
                let c2 = Ptr::from_string_literal(b"abd")
                    .offset(i)
                    .read()
                    .to_ascii_lowercase();
                if c1 != c2 {
                    break (c1 as i32) - (c2 as i32);
                }
                if c1 == 0 {
                    break 0;
                }
                i += 1;
            }
        } < 0) as i32)
            != 0)
    );
    assert!(
        ((({
            let mut i: usize = 0;
            loop {
                let c1 = Ptr::from_string_literal(b"abd")
                    .offset(i)
                    .read()
                    .to_ascii_lowercase();
                let c2 = Ptr::from_string_literal(b"abc")
                    .offset(i)
                    .read()
                    .to_ascii_lowercase();
                if c1 != c2 {
                    break (c1 as i32) - (c2 as i32);
                }
                if c1 == 0 {
                    break 0;
                }
                i += 1;
            }
        } > 0) as i32)
            != 0)
    );
    let p: Value<Ptr<u8>> = Rc::new(RefCell::new(Ptr::from_string_literal(b"FOO")));
    let q: Value<Ptr<u8>> = Rc::new(RefCell::new(Ptr::from_string_literal(b"foo")));
    assert!(
        ((({
            let mut i: usize = 0;
            loop {
                let c1 = (*p.borrow()).offset(i).read().to_ascii_lowercase();
                let c2 = (*q.borrow()).offset(i).read().to_ascii_lowercase();
                if c1 != c2 {
                    break (c1 as i32) - (c2 as i32);
                }
                if c1 == 0 {
                    break 0;
                }
                i += 1;
            }
        } == 0) as i32)
            != 0)
    );
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    ({ test_memcpy_0() });
    ({ test_memset_1() });
    ({ test_memcmp_2() });
    ({ test_memmove_3() });
    ({ test_strchr_4() });
    ({ test_strlen_5() });
    ({ test_strcmp_6() });
    ({ test_strncmp_7() });
    ({ test_memchr_8() });
    ({ test_strrchr_9() });
    ({ test_strcspn_10() });
    ({ test_strspn_11() });
    ({ test_strstr_12() });
    ({ test_strpbrk_13() });
    ({ test_strcasecmp_14() });
    return 0;
}
