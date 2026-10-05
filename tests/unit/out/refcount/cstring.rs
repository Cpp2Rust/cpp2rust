extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_memcpy_0() {
    let src: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"hello\0")));
    let dst: Value<Box<[i8]>> =
        Rc::new(RefCell::new(Box::new([0_i8, 0_i8, 0_i8, 0_i8, 0_i8, 0_i8])));
    let mut r: AnyPtr = {
        ((dst.as_pointer() as Ptr<i8>) as Ptr<i8>).to_any().memcpy(
            &((src.as_pointer() as Ptr<i8>) as Ptr<i8>).to_any(),
            6_usize as usize,
        );
        ((dst.as_pointer() as Ptr<i8>) as Ptr<i8>).to_any()
    };
    assert!(({ (r).clone() } == { ((dst.as_pointer() as Ptr::<i8>) as Ptr::<i8>).to_any() }));
    assert!(
        ((((*dst.borrow())[(0) as usize] as i32) == (('h' as i8) as i32))
            && (((*dst.borrow())[(1) as usize] as i32) == (('e' as i8) as i32)))
            && (((*dst.borrow())[(2) as usize] as i32) == (('l' as i8) as i32))
    );
    assert!(
        ((((*dst.borrow())[(3) as usize] as i32) == (('l' as i8) as i32))
            && (((*dst.borrow())[(4) as usize] as i32) == (('o' as i8) as i32)))
            && (((*dst.borrow())[(5) as usize] as i32) == (('\0' as i8) as i32))
    );
}
pub fn test_memset_1() {
    let buf: Value<Box<[i8]>> = Rc::new(RefCell::new((0..4).map(|_| 0_i8).collect::<Box<[i8]>>()));
    let mut r: AnyPtr = {
        ((buf.as_pointer() as Ptr<i8>) as Ptr<i8>)
            .to_any()
            .memset((('x' as i8) as i32) as u8, 4_usize as usize);
        ((buf.as_pointer() as Ptr<i8>) as Ptr<i8>).to_any()
    };
    assert!(({ (r).clone() } == { ((buf.as_pointer() as Ptr::<i8>) as Ptr::<i8>).to_any() }));
    assert!(
        (((((*buf.borrow())[(0) as usize] as i32) == (('x' as i8) as i32))
            && (((*buf.borrow())[(1) as usize] as i32) == (('x' as i8) as i32)))
            && (((*buf.borrow())[(2) as usize] as i32) == (('x' as i8) as i32)))
            && (((*buf.borrow())[(3) as usize] as i32) == (('x' as i8) as i32))
    );
}
pub fn test_memcmp_2() {
    let a: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([1_i8, 2_i8, 3_i8, 4_i8])));
    let b: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([1_i8, 2_i8, 3_i8, 4_i8])));
    let c: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([1_i8, 2_i8, 9_i8, 4_i8])));
    assert!(
        (((a.as_pointer() as Ptr::<i8>) as Ptr::<i8>)
            .to_any()
            .memcmp(
                &((b.as_pointer() as Ptr::<i8>) as Ptr::<i8>).to_any(),
                4_usize
            )
            == 0)
    );
    assert!(
        (((a.as_pointer() as Ptr::<i8>) as Ptr::<i8>)
            .to_any()
            .memcmp(
                &((c.as_pointer() as Ptr::<i8>) as Ptr::<i8>).to_any(),
                4_usize
            )
            < 0)
    );
    assert!(
        (((c.as_pointer() as Ptr::<i8>) as Ptr::<i8>)
            .to_any()
            .memcmp(
                &((a.as_pointer() as Ptr::<i8>) as Ptr::<i8>).to_any(),
                4_usize
            )
            > 0)
    );
}
pub fn test_memmove_3() {
    let buf: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
        ('a' as i8),
        ('b' as i8),
        ('c' as i8),
        ('d' as i8),
        ('e' as i8),
        ('\0' as i8),
    ])));
    let mut r: AnyPtr = {
        ((buf.as_pointer() as Ptr<i8>).offset((1) as isize) as Ptr<i8>)
            .to_any()
            .memcpy(
                &((buf.as_pointer() as Ptr<i8>) as Ptr<i8>).to_any(),
                4_usize as usize,
            );
        ((buf.as_pointer() as Ptr<i8>).offset((1) as isize) as Ptr<i8>).to_any()
    };
    assert!(
        ({ (r).clone() } == {
            ((buf.as_pointer() as Ptr<i8>).offset((1) as isize) as Ptr<i8>).to_any()
        })
    );
    assert!(
        ((((*buf.borrow())[(0) as usize] as i32) == (('a' as i8) as i32))
            && (((*buf.borrow())[(1) as usize] as i32) == (('a' as i8) as i32)))
            && (((*buf.borrow())[(2) as usize] as i32) == (('b' as i8) as i32))
    );
    assert!(
        ((((*buf.borrow())[(3) as usize] as i32) == (('c' as i8) as i32))
            && (((*buf.borrow())[(4) as usize] as i32) == (('d' as i8) as i32)))
            && (((*buf.borrow())[(5) as usize] as i32) == (('\0' as i8) as i32))
    );
}
pub fn test_strchr_4() {
    let mut s: Ptr<i8> = Ptr::<i8>::from_string_literal(b"hello world");
    let mut r: Ptr<i8> = {
        let __s = (s).clone();
        let __t = (('w' as i8) as i32) as i8;
        match __s.to_c_string_iterator().position(|__c| __c == __t) {
            Some(__i) => __s.offset(__i),
            None => {
                if __t == 0 {
                    __s.offset(__s.to_c_string_iterator().count())
                } else {
                    Ptr::null()
                }
            }
        }
    };
    assert!(!((r).is_null()));
    assert!((((r.read()) as i32) == (('w' as i8) as i32)));
    assert!(
        ({
            let __s = (s).clone();
            let __t = (('z' as i8) as i32) as i8;
            match __s.to_c_string_iterator().position(|__c| __c == __t) {
                Some(__i) => __s.offset(__i),
                None => {
                    if __t == 0 {
                        __s.offset(__s.to_c_string_iterator().count())
                    } else {
                        Ptr::null()
                    }
                }
            }
        })
        .is_null()
    );
}
pub fn test_strlen_5() {
    assert!(
        (Ptr::<i8>::from_string_literal(b"")
            .to_c_string_iterator()
            .count()
            == 0_usize)
    );
    assert!(
        (Ptr::<i8>::from_string_literal(b"hello")
            .to_c_string_iterator()
            .count()
            == 5_usize)
    );
    assert!(
        (Ptr::<i8>::from_string_literal(b"hello world")
            .to_c_string_iterator()
            .count()
            == 11_usize)
    );
}
pub fn test_strcmp_6() {
    assert!(
        ({
            let mut __it1 = Ptr::<i8>::from_string_literal(b"abc").to_c_string_iterator();
            let mut __it2 = Ptr::<i8>::from_string_literal(b"abc").to_c_string_iterator();
            loop {
                let __c1 = __it1.next();
                let __c2 = __it2.next();
                if __c1 != __c2 {
                    break (__c1.unwrap_or(0) as u8 as i32) - (__c2.unwrap_or(0) as u8 as i32);
                }
                if __c1.is_none() {
                    break 0;
                }
            }
        } == 0)
    );
    assert!(
        ({
            let mut __it1 = Ptr::<i8>::from_string_literal(b"abc").to_c_string_iterator();
            let mut __it2 = Ptr::<i8>::from_string_literal(b"abd").to_c_string_iterator();
            loop {
                let __c1 = __it1.next();
                let __c2 = __it2.next();
                if __c1 != __c2 {
                    break (__c1.unwrap_or(0) as u8 as i32) - (__c2.unwrap_or(0) as u8 as i32);
                }
                if __c1.is_none() {
                    break 0;
                }
            }
        } < 0)
    );
    assert!(
        ({
            let mut __it1 = Ptr::<i8>::from_string_literal(b"abd").to_c_string_iterator();
            let mut __it2 = Ptr::<i8>::from_string_literal(b"abc").to_c_string_iterator();
            loop {
                let __c1 = __it1.next();
                let __c2 = __it2.next();
                if __c1 != __c2 {
                    break (__c1.unwrap_or(0) as u8 as i32) - (__c2.unwrap_or(0) as u8 as i32);
                }
                if __c1.is_none() {
                    break 0;
                }
            }
        } > 0)
    );
    let mut p: Ptr<i8> = Ptr::<i8>::from_string_literal(b"abc");
    let mut q: Ptr<i8> = Ptr::<i8>::from_string_literal(b"abd");
    let buf: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
        ('a' as i8),
        ('b' as i8),
        ('c' as i8),
        ('\0' as i8),
    ])));
    assert!(
        ({
            let mut __it1 = p.to_c_string_iterator();
            let mut __it2 = p.to_c_string_iterator();
            loop {
                let __c1 = __it1.next();
                let __c2 = __it2.next();
                if __c1 != __c2 {
                    break (__c1.unwrap_or(0) as u8 as i32) - (__c2.unwrap_or(0) as u8 as i32);
                }
                if __c1.is_none() {
                    break 0;
                }
            }
        } == 0)
    );
    assert!(
        ({
            let mut __it1 = p.to_c_string_iterator();
            let mut __it2 = q.to_c_string_iterator();
            loop {
                let __c1 = __it1.next();
                let __c2 = __it2.next();
                if __c1 != __c2 {
                    break (__c1.unwrap_or(0) as u8 as i32) - (__c2.unwrap_or(0) as u8 as i32);
                }
                if __c1.is_none() {
                    break 0;
                }
            }
        } < 0)
    );
    assert!(
        ({
            let mut __it1 = (buf.as_pointer() as Ptr<i8>).to_c_string_iterator();
            let mut __it2 = p.to_c_string_iterator();
            loop {
                let __c1 = __it1.next();
                let __c2 = __it2.next();
                if __c1 != __c2 {
                    break (__c1.unwrap_or(0) as u8 as i32) - (__c2.unwrap_or(0) as u8 as i32);
                }
                if __c1.is_none() {
                    break 0;
                }
            }
        } == 0)
    );
}
pub fn test_strncmp_7() {
    assert!(
        ({
            let __n = 3_usize;
            let mut __it1 = Ptr::<i8>::from_string_literal(b"abcdef")
                .to_c_string_iterator()
                .take(__n);
            let mut __it2 = Ptr::<i8>::from_string_literal(b"abcxyz")
                .to_c_string_iterator()
                .take(__n);
            loop {
                let __c1 = __it1.next();
                let __c2 = __it2.next();
                if __c1 != __c2 {
                    break (__c1.unwrap_or(0) as u8 as i32) - (__c2.unwrap_or(0) as u8 as i32);
                }
                if __c1.is_none() {
                    break 0;
                }
            }
        } == 0)
    );
    assert!(
        ({
            let __n = 4_usize;
            let mut __it1 = Ptr::<i8>::from_string_literal(b"abcdef")
                .to_c_string_iterator()
                .take(__n);
            let mut __it2 = Ptr::<i8>::from_string_literal(b"abcxyz")
                .to_c_string_iterator()
                .take(__n);
            loop {
                let __c1 = __it1.next();
                let __c2 = __it2.next();
                if __c1 != __c2 {
                    break (__c1.unwrap_or(0) as u8 as i32) - (__c2.unwrap_or(0) as u8 as i32);
                }
                if __c1.is_none() {
                    break 0;
                }
            }
        } < 0)
    );
    assert!(
        ({
            let __n = 4_usize;
            let mut __it1 = Ptr::<i8>::from_string_literal(b"abcxyz")
                .to_c_string_iterator()
                .take(__n);
            let mut __it2 = Ptr::<i8>::from_string_literal(b"abcdef")
                .to_c_string_iterator()
                .take(__n);
            loop {
                let __c1 = __it1.next();
                let __c2 = __it2.next();
                if __c1 != __c2 {
                    break (__c1.unwrap_or(0) as u8 as i32) - (__c2.unwrap_or(0) as u8 as i32);
                }
                if __c1.is_none() {
                    break 0;
                }
            }
        } > 0)
    );
    let mut p: Ptr<i8> = Ptr::<i8>::from_string_literal(b"abcdef");
    let mut q: Ptr<i8> = Ptr::<i8>::from_string_literal(b"abcxyz");
    let buf: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
        ('a' as i8),
        ('b' as i8),
        ('c' as i8),
        ('d' as i8),
        ('e' as i8),
        ('f' as i8),
        ('\0' as i8),
    ])));
    let mut n: usize = 3_usize;
    assert!(
        ({
            let __n = n;
            let mut __it1 = p.to_c_string_iterator().take(__n);
            let mut __it2 = q.to_c_string_iterator().take(__n);
            loop {
                let __c1 = __it1.next();
                let __c2 = __it2.next();
                if __c1 != __c2 {
                    break (__c1.unwrap_or(0) as u8 as i32) - (__c2.unwrap_or(0) as u8 as i32);
                }
                if __c1.is_none() {
                    break 0;
                }
            }
        } == 0)
    );
    assert!(
        ({
            let __n = (n).wrapping_add(1_usize);
            let mut __it1 = p.to_c_string_iterator().take(__n);
            let mut __it2 = q.to_c_string_iterator().take(__n);
            loop {
                let __c1 = __it1.next();
                let __c2 = __it2.next();
                if __c1 != __c2 {
                    break (__c1.unwrap_or(0) as u8 as i32) - (__c2.unwrap_or(0) as u8 as i32);
                }
                if __c1.is_none() {
                    break 0;
                }
            }
        } < 0)
    );
    assert!(
        ({
            let __n = 6_usize;
            let mut __it1 = (buf.as_pointer() as Ptr<i8>)
                .to_c_string_iterator()
                .take(__n);
            let mut __it2 = p.to_c_string_iterator().take(__n);
            loop {
                let __c1 = __it1.next();
                let __c2 = __it2.next();
                if __c1 != __c2 {
                    break (__c1.unwrap_or(0) as u8 as i32) - (__c2.unwrap_or(0) as u8 as i32);
                }
                if __c1.is_none() {
                    break 0;
                }
            }
        } == 0)
    );
}
pub fn test_memchr_8() {
    let data: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([16_i8, 32_i8, 48_i8, 64_i8])));
    let mut r: AnyPtr = {
        let mut __p = ((data.as_pointer() as Ptr<i8>) as Ptr<i8>)
            .to_any()
            .reinterpret_cast::<u8>();
        let mut __i: usize = 0;
        loop {
            if __i == 4_usize {
                break Ptr::<u8>::null().to_any();
            }
            if __p.read() == 48 as u8 {
                break __p.to_any();
            }
            __p += 1;
            __i += 1;
        }
    };
    assert!(
        ({ (r).clone() } == { (((data.as_pointer() as Ptr<i8>).offset(2)) as Ptr::<i8>).to_any() })
    );
    assert!(
        ({
            let mut __p = ((data.as_pointer() as Ptr<i8>) as Ptr<i8>)
                .to_any()
                .reinterpret_cast::<u8>();
            let mut __i: usize = 0;
            loop {
                if __i == 4_usize {
                    break Ptr::<u8>::null().to_any();
                }
                if __p.read() == 153 as u8 {
                    break __p.to_any();
                }
                __p += 1;
                __i += 1;
            }
        })
        .is_null()
    );
    let mut p: AnyPtr = ((data.as_pointer() as Ptr<i8>) as Ptr<i8>).to_any();
    let mut n: usize = 4_usize;
    assert!(
        ({
            {
                let mut __p = p.reinterpret_cast::<u8>();
                let mut __i: usize = 0;
                loop {
                    if __i == n {
                        break Ptr::<u8>::null().to_any();
                    }
                    if __p.read() == 16 as u8 {
                        break __p.to_any();
                    }
                    __p += 1;
                    __i += 1;
                }
            }
        } == { (p).clone() })
    );
}
pub fn test_strrchr_9() {
    let mut s: Ptr<i8> = Ptr::<i8>::from_string_literal(b"hello world");
    let mut r: Ptr<i8> = {
        let __s = (s).clone();
        let __t = (('l' as i8) as i32) as i8;
        match __s
            .to_c_string_iterator()
            .enumerate()
            .filter(|__e| __e.1 == __t)
            .last()
        {
            Some((__i, _)) => __s.offset(__i),
            None => {
                if __t == 0 {
                    __s.offset(__s.to_c_string_iterator().count())
                } else {
                    Ptr::null()
                }
            }
        }
    };
    assert!(!((r).is_null()));
    assert!((((r.read()) as i32) == (('l' as i8) as i32)));
    assert!(({ (r).clone() } == { s.offset((9) as isize) }));
    assert!(
        ({
            let __s = (s).clone();
            let __t = (('z' as i8) as i32) as i8;
            match __s
                .to_c_string_iterator()
                .enumerate()
                .filter(|__e| __e.1 == __t)
                .last()
            {
                Some((__i, _)) => __s.offset(__i),
                None => {
                    if __t == 0 {
                        __s.offset(__s.to_c_string_iterator().count())
                    } else {
                        Ptr::null()
                    }
                }
            }
        })
        .is_null()
    );
    let buf: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
        ('a' as i8),
        ('b' as i8),
        ('a' as i8),
        ('\0' as i8),
    ])));
    assert!(
        ({
            let __s = (buf.as_pointer() as Ptr<i8>);
            let __t = (('a' as i8) as i32) as i8;
            match __s
                .to_c_string_iterator()
                .enumerate()
                .filter(|__e| __e.1 == __t)
                .last()
            {
                Some((__i, _)) => __s.offset(__i),
                None => {
                    if __t == 0 {
                        __s.offset(__s.to_c_string_iterator().count())
                    } else {
                        Ptr::null()
                    }
                }
            }
        } == ((buf.as_pointer() as Ptr<i8>).offset(2)))
    );
}
pub fn test_strdup_10() {
    let mut d: Ptr<i8> = libcc2rs::strdup_refcount(Ptr::<i8>::from_string_literal(b"hello"));
    assert!(!((d).is_null()));
    assert!(
        ({
            let mut __it1 = d.to_c_string_iterator();
            let mut __it2 = Ptr::<i8>::from_string_literal(b"hello").to_c_string_iterator();
            loop {
                let __c1 = __it1.next();
                let __c2 = __it2.next();
                if __c1 != __c2 {
                    break (__c1.unwrap_or(0) as u8 as i32) - (__c2.unwrap_or(0) as u8 as i32);
                }
                if __c1.is_none() {
                    break 0;
                }
            }
        } == 0)
    );
    libcc2rs::free_refcount((d).to_any());
    let mut p: Ptr<i8> = Ptr::<i8>::from_string_literal(b"world");
    let buf: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
        ('a' as i8),
        ('b' as i8),
        ('c' as i8),
        ('\0' as i8),
    ])));
    let mut d2: Ptr<i8> = libcc2rs::strdup_refcount((p).clone());
    assert!(!((d2).is_null()));
    assert!(
        ({
            let mut __it1 = d2.to_c_string_iterator();
            let mut __it2 = p.to_c_string_iterator();
            loop {
                let __c1 = __it1.next();
                let __c2 = __it2.next();
                if __c1 != __c2 {
                    break (__c1.unwrap_or(0) as u8 as i32) - (__c2.unwrap_or(0) as u8 as i32);
                }
                if __c1.is_none() {
                    break 0;
                }
            }
        } == 0)
    );
    libcc2rs::free_refcount((d2).to_any());
    let mut d3: Ptr<i8> = libcc2rs::strdup_refcount((buf.as_pointer() as Ptr<i8>));
    assert!(!((d3).is_null()));
    assert!(
        ({
            let mut __it1 = d3.to_c_string_iterator();
            let mut __it2 = (buf.as_pointer() as Ptr<i8>).to_c_string_iterator();
            loop {
                let __c1 = __it1.next();
                let __c2 = __it2.next();
                if __c1 != __c2 {
                    break (__c1.unwrap_or(0) as u8 as i32) - (__c2.unwrap_or(0) as u8 as i32);
                }
                if __c1.is_none() {
                    break 0;
                }
            }
        } == 0)
    );
    libcc2rs::free_refcount((d3).to_any());
}
pub fn test_strcspn_11() {
    assert!(
        ({
            let __set = Ptr::<i8>::from_string_literal(b"el");
            Ptr::<i8>::from_string_literal(b"hello")
                .to_c_string_iterator()
                .take_while(|__c| !__set.to_c_string_iterator().any(|__r| __r == *__c))
                .count()
        } == 1_usize)
    );
    assert!(
        ({
            let __set = Ptr::<i8>::from_string_literal(b"xyz");
            Ptr::<i8>::from_string_literal(b"abc")
                .to_c_string_iterator()
                .take_while(|__c| !__set.to_c_string_iterator().any(|__r| __r == *__c))
                .count()
        } == 3_usize)
    );
    assert!(
        ({
            let __set = Ptr::<i8>::from_string_literal(b"abc");
            Ptr::<i8>::from_string_literal(b"")
                .to_c_string_iterator()
                .take_while(|__c| !__set.to_c_string_iterator().any(|__r| __r == *__c))
                .count()
        } == 0_usize)
    );
    let mut s: Ptr<i8> = Ptr::<i8>::from_string_literal(b"hello");
    let mut rej: Ptr<i8> = Ptr::<i8>::from_string_literal(b"el");
    assert!(
        ({
            let __set = (rej).clone();
            s.to_c_string_iterator()
                .take_while(|__c| !__set.to_c_string_iterator().any(|__r| __r == *__c))
                .count()
        } == 1_usize)
    );
}
pub fn test_strspn_12() {
    assert!(
        ({
            let __set = Ptr::<i8>::from_string_literal(b"hel");
            Ptr::<i8>::from_string_literal(b"hello")
                .to_c_string_iterator()
                .take_while(|__c| __set.to_c_string_iterator().any(|__r| __r == *__c))
                .count()
        } == 4_usize)
    );
    assert!(
        ({
            let __set = Ptr::<i8>::from_string_literal(b"xyz");
            Ptr::<i8>::from_string_literal(b"abc")
                .to_c_string_iterator()
                .take_while(|__c| __set.to_c_string_iterator().any(|__r| __r == *__c))
                .count()
        } == 0_usize)
    );
    assert!(
        ({
            let __set = Ptr::<i8>::from_string_literal(b"a");
            Ptr::<i8>::from_string_literal(b"aaa")
                .to_c_string_iterator()
                .take_while(|__c| __set.to_c_string_iterator().any(|__r| __r == *__c))
                .count()
        } == 3_usize)
    );
    let mut s: Ptr<i8> = Ptr::<i8>::from_string_literal(b"hello");
    let mut acc: Ptr<i8> = Ptr::<i8>::from_string_literal(b"hel");
    assert!(
        ({
            let __set = (acc).clone();
            s.to_c_string_iterator()
                .take_while(|__c| __set.to_c_string_iterator().any(|__r| __r == *__c))
                .count()
        } == 4_usize)
    );
}
pub fn test_strstr_13() {
    let mut h: Ptr<i8> = Ptr::<i8>::from_string_literal(b"hello world");
    let mut r: Ptr<i8> = {
        let __needle = Ptr::<i8>::from_string_literal(b"world");
        let mut __p = (h).clone();
        loop {
            let mut __h = __p.to_c_string_iterator();
            if __needle
                .to_c_string_iterator()
                .all(|__c| __h.next() == Some(__c))
            {
                break __p;
            }
            if __p.read() == 0 {
                break Ptr::null();
            }
            __p += 1;
        }
    };
    assert!(!((r).is_null()));
    assert!(({ (r).clone() } == { h.offset((6) as isize) }));
    assert!(
        ({
            let __needle = Ptr::<i8>::from_string_literal(b"xyz");
            let mut __p = (h).clone();
            loop {
                let mut __h = __p.to_c_string_iterator();
                if __needle
                    .to_c_string_iterator()
                    .all(|__c| __h.next() == Some(__c))
                {
                    break __p;
                }
                if __p.read() == 0 {
                    break Ptr::null();
                }
                __p += 1;
            }
        })
        .is_null()
    );
    let buf: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
        ('h' as i8),
        ('e' as i8),
        ('l' as i8),
        ('l' as i8),
        ('o' as i8),
        ('\0' as i8),
    ])));
    assert!(
        ({
            let __needle = Ptr::<i8>::from_string_literal(b"ll");
            let mut __p = (buf.as_pointer() as Ptr<i8>);
            loop {
                let mut __h = __p.to_c_string_iterator();
                if __needle
                    .to_c_string_iterator()
                    .all(|__c| __h.next() == Some(__c))
                {
                    break __p;
                }
                if __p.read() == 0 {
                    break Ptr::null();
                }
                __p += 1;
            }
        } == ((buf.as_pointer() as Ptr<i8>).offset(2)))
    );
}
pub fn test_strpbrk_14() {
    let mut s: Ptr<i8> = Ptr::<i8>::from_string_literal(b"hello world");
    let mut r: Ptr<i8> = {
        let __s = (s).clone();
        let __set = Ptr::<i8>::from_string_literal(b"wo");
        match __s
            .to_c_string_iterator()
            .position(|__c| __set.to_c_string_iterator().any(|__r| __r == __c))
        {
            Some(__i) => __s.offset(__i),
            None => Ptr::null(),
        }
    };
    assert!(!((r).is_null()));
    assert!(({ (r).clone() } == { s.offset((4) as isize) }));
    assert!(
        ({
            let __s = (s).clone();
            let __set = Ptr::<i8>::from_string_literal(b"xyz");
            match __s
                .to_c_string_iterator()
                .position(|__c| __set.to_c_string_iterator().any(|__r| __r == __c))
            {
                Some(__i) => __s.offset(__i),
                None => Ptr::null(),
            }
        })
        .is_null()
    );
    let buf: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
        ('a' as i8),
        ('b' as i8),
        ('c' as i8),
        ('\0' as i8),
    ])));
    assert!(
        ({
            let __s = (buf.as_pointer() as Ptr<i8>);
            let __set = Ptr::<i8>::from_string_literal(b"b");
            match __s
                .to_c_string_iterator()
                .position(|__c| __set.to_c_string_iterator().any(|__r| __r == __c))
            {
                Some(__i) => __s.offset(__i),
                None => Ptr::null(),
            }
        } == ((buf.as_pointer() as Ptr<i8>).offset(1)))
    );
}
pub fn test_strcasecmp_15() {
    assert!(
        ({
            let mut __it1 = Ptr::<i8>::from_string_literal(b"HELLO")
                .to_c_string_iterator()
                .map(|__c| (__c as u8).to_ascii_lowercase());
            let mut __it2 = Ptr::<i8>::from_string_literal(b"hello")
                .to_c_string_iterator()
                .map(|__c| (__c as u8).to_ascii_lowercase());
            loop {
                let __c1 = __it1.next();
                let __c2 = __it2.next();
                if __c1 != __c2 {
                    break (__c1.unwrap_or(0) as u8 as i32) - (__c2.unwrap_or(0) as u8 as i32);
                }
                if __c1.is_none() {
                    break 0;
                }
            }
        } == 0)
    );
    assert!(
        ({
            let mut __it1 = Ptr::<i8>::from_string_literal(b"abc")
                .to_c_string_iterator()
                .map(|__c| (__c as u8).to_ascii_lowercase());
            let mut __it2 = Ptr::<i8>::from_string_literal(b"abd")
                .to_c_string_iterator()
                .map(|__c| (__c as u8).to_ascii_lowercase());
            loop {
                let __c1 = __it1.next();
                let __c2 = __it2.next();
                if __c1 != __c2 {
                    break (__c1.unwrap_or(0) as u8 as i32) - (__c2.unwrap_or(0) as u8 as i32);
                }
                if __c1.is_none() {
                    break 0;
                }
            }
        } < 0)
    );
    assert!(
        ({
            let mut __it1 = Ptr::<i8>::from_string_literal(b"abd")
                .to_c_string_iterator()
                .map(|__c| (__c as u8).to_ascii_lowercase());
            let mut __it2 = Ptr::<i8>::from_string_literal(b"abc")
                .to_c_string_iterator()
                .map(|__c| (__c as u8).to_ascii_lowercase());
            loop {
                let __c1 = __it1.next();
                let __c2 = __it2.next();
                if __c1 != __c2 {
                    break (__c1.unwrap_or(0) as u8 as i32) - (__c2.unwrap_or(0) as u8 as i32);
                }
                if __c1.is_none() {
                    break 0;
                }
            }
        } > 0)
    );
    let mut p: Ptr<i8> = Ptr::<i8>::from_string_literal(b"FOO");
    let mut q: Ptr<i8> = Ptr::<i8>::from_string_literal(b"foo");
    assert!(
        ({
            let mut __it1 = p
                .to_c_string_iterator()
                .map(|__c| (__c as u8).to_ascii_lowercase());
            let mut __it2 = q
                .to_c_string_iterator()
                .map(|__c| (__c as u8).to_ascii_lowercase());
            loop {
                let __c1 = __it1.next();
                let __c2 = __it2.next();
                if __c1 != __c2 {
                    break (__c1.unwrap_or(0) as u8 as i32) - (__c2.unwrap_or(0) as u8 as i32);
                }
                if __c1.is_none() {
                    break 0;
                }
            }
        } == 0)
    );
}
pub fn main() {
    __cpp2rust_init_globals();
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
    ({ test_strdup_10() });
    ({ test_strcspn_11() });
    ({ test_strspn_12() });
    ({ test_strstr_13() });
    ({ test_strpbrk_14() });
    ({ test_strcasecmp_15() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
