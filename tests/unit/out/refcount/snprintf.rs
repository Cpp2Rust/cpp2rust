extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let buf: Value<Box<[u8]>> = Rc::new(RefCell::new(
        (0..32).map(|_| <u8>::default()).collect::<Box<[u8]>>(),
    ));
    assert!(
        ((({
            let __s = libcc2rs::format_c(
                &Ptr::from_string_literal(b"x=%d y=%u").to_rust_string(),
                &[(-3_i32).into(), (7_u32).into()],
            );
            let __b = __s.as_bytes();
            if ::std::mem::size_of::<[u8; 32]>() > 0 {
                let __n = ::std::cmp::min(__b.len(), ::std::mem::size_of::<[u8; 32]>() - 1);
                (buf.as_pointer() as Ptr<u8>).with_slice_mut(__n + 1, |__dst| {
                    __dst[..__n].copy_from_slice(&__b[..__n]);
                    __dst[__n] = 0;
                });
            }
            __b.len() as i32
        } == 8) as i32)
            != 0)
    );
    assert!(
        ((({
            let mut __it1 = (buf.as_pointer() as Ptr<u8>).to_c_string_iterator();
            let mut __it2 = Ptr::from_string_literal(b"x=-3 y=7").to_c_string_iterator();
            loop {
                let __c1 = __it1.next();
                let __c2 = __it2.next();
                if __c1 != __c2 {
                    break (__c1.unwrap_or(0) as i32) - (__c2.unwrap_or(0) as i32);
                }
                if __c1.is_none() {
                    break 0;
                }
            }
        } == 0) as i32)
            != 0)
    );
    assert!(
        ((({
            let __s = libcc2rs::format_c(
                &Ptr::from_string_literal(b"%s").to_rust_string(),
                &[(Ptr::from_string_literal(b"hello")).into()],
            );
            let __b = __s.as_bytes();
            if 4_usize > 0 {
                let __n = ::std::cmp::min(__b.len(), 4_usize - 1);
                (buf.as_pointer() as Ptr<u8>).with_slice_mut(__n + 1, |__dst| {
                    __dst[..__n].copy_from_slice(&__b[..__n]);
                    __dst[__n] = 0;
                });
            }
            __b.len() as i32
        } == 5) as i32)
            != 0)
    );
    assert!(
        ((({
            let mut __it1 = (buf.as_pointer() as Ptr<u8>).to_c_string_iterator();
            let mut __it2 = Ptr::from_string_literal(b"hel").to_c_string_iterator();
            loop {
                let __c1 = __it1.next();
                let __c2 = __it2.next();
                if __c1 != __c2 {
                    break (__c1.unwrap_or(0) as i32) - (__c2.unwrap_or(0) as i32);
                }
                if __c1.is_none() {
                    break 0;
                }
            }
        } == 0) as i32)
            != 0)
    );
    assert!(
        ((({
            let __s = libcc2rs::format_c(
                &Ptr::from_string_literal(b"%05d|%x|%X").to_rust_string(),
                &[(42).into(), (255).into(), (255).into()],
            );
            let __b = __s.as_bytes();
            if ::std::mem::size_of::<[u8; 32]>() > 0 {
                let __n = ::std::cmp::min(__b.len(), ::std::mem::size_of::<[u8; 32]>() - 1);
                (buf.as_pointer() as Ptr<u8>).with_slice_mut(__n + 1, |__dst| {
                    __dst[..__n].copy_from_slice(&__b[..__n]);
                    __dst[__n] = 0;
                });
            }
            __b.len() as i32
        } == 11) as i32)
            != 0)
    );
    assert!(
        ((({
            let mut __it1 = (buf.as_pointer() as Ptr<u8>).to_c_string_iterator();
            let mut __it2 = Ptr::from_string_literal(b"00042|ff|FF").to_c_string_iterator();
            loop {
                let __c1 = __it1.next();
                let __c2 = __it2.next();
                if __c1 != __c2 {
                    break (__c1.unwrap_or(0) as i32) - (__c2.unwrap_or(0) as i32);
                }
                if __c1.is_none() {
                    break 0;
                }
            }
        } == 0) as i32)
            != 0)
    );
    assert!(
        ((({
            let __s = libcc2rs::format_c(
                &Ptr::from_string_literal(b"%.2f").to_rust_string(),
                &[(3.14159E+0).into()],
            );
            let __b = __s.as_bytes();
            if ::std::mem::size_of::<[u8; 32]>() > 0 {
                let __n = ::std::cmp::min(__b.len(), ::std::mem::size_of::<[u8; 32]>() - 1);
                (buf.as_pointer() as Ptr<u8>).with_slice_mut(__n + 1, |__dst| {
                    __dst[..__n].copy_from_slice(&__b[..__n]);
                    __dst[__n] = 0;
                });
            }
            __b.len() as i32
        } == 4) as i32)
            != 0)
    );
    assert!(
        ((({
            let mut __it1 = (buf.as_pointer() as Ptr<u8>).to_c_string_iterator();
            let mut __it2 = Ptr::from_string_literal(b"3.14").to_c_string_iterator();
            loop {
                let __c1 = __it1.next();
                let __c2 = __it2.next();
                if __c1 != __c2 {
                    break (__c1.unwrap_or(0) as i32) - (__c2.unwrap_or(0) as i32);
                }
                if __c1.is_none() {
                    break 0;
                }
            }
        } == 0) as i32)
            != 0)
    );
    assert!(
        ((({
            let __s = libcc2rs::format_c(
                &Ptr::from_string_literal(b"%-6s|").to_rust_string(),
                &[(Ptr::from_string_literal(b"ab")).into()],
            );
            let __b = __s.as_bytes();
            if ::std::mem::size_of::<[u8; 32]>() > 0 {
                let __n = ::std::cmp::min(__b.len(), ::std::mem::size_of::<[u8; 32]>() - 1);
                (buf.as_pointer() as Ptr<u8>).with_slice_mut(__n + 1, |__dst| {
                    __dst[..__n].copy_from_slice(&__b[..__n]);
                    __dst[__n] = 0;
                });
            }
            __b.len() as i32
        } == 7) as i32)
            != 0)
    );
    assert!(
        ((({
            let mut __it1 = (buf.as_pointer() as Ptr<u8>).to_c_string_iterator();
            let mut __it2 = Ptr::from_string_literal(b"ab    |").to_c_string_iterator();
            loop {
                let __c1 = __it1.next();
                let __c2 = __it2.next();
                if __c1 != __c2 {
                    break (__c1.unwrap_or(0) as i32) - (__c2.unwrap_or(0) as i32);
                }
                if __c1.is_none() {
                    break 0;
                }
            }
        } == 0) as i32)
            != 0)
    );
    assert!(
        ((({
            let __s = libcc2rs::format_c(
                &Ptr::from_string_literal(b"%c%%").to_rust_string(),
                &[(65).into()],
            );
            let __b = __s.as_bytes();
            if ::std::mem::size_of::<[u8; 32]>() > 0 {
                let __n = ::std::cmp::min(__b.len(), ::std::mem::size_of::<[u8; 32]>() - 1);
                (buf.as_pointer() as Ptr<u8>).with_slice_mut(__n + 1, |__dst| {
                    __dst[..__n].copy_from_slice(&__b[..__n]);
                    __dst[__n] = 0;
                });
            }
            __b.len() as i32
        } == 2) as i32)
            != 0)
    );
    assert!(
        ((({
            let mut __it1 = (buf.as_pointer() as Ptr<u8>).to_c_string_iterator();
            let mut __it2 = Ptr::from_string_literal(b"A%").to_c_string_iterator();
            loop {
                let __c1 = __it1.next();
                let __c2 = __it2.next();
                if __c1 != __c2 {
                    break (__c1.unwrap_or(0) as i32) - (__c2.unwrap_or(0) as i32);
                }
                if __c1.is_none() {
                    break 0;
                }
            }
        } == 0) as i32)
            != 0)
    );
    assert!(
        ((({
            let __s = libcc2rs::format_c(
                &Ptr::from_string_literal(b"%+d % d").to_rust_string(),
                &[(5).into(), (5).into()],
            );
            let __b = __s.as_bytes();
            if ::std::mem::size_of::<[u8; 32]>() > 0 {
                let __n = ::std::cmp::min(__b.len(), ::std::mem::size_of::<[u8; 32]>() - 1);
                (buf.as_pointer() as Ptr<u8>).with_slice_mut(__n + 1, |__dst| {
                    __dst[..__n].copy_from_slice(&__b[..__n]);
                    __dst[__n] = 0;
                });
            }
            __b.len() as i32
        } == 5) as i32)
            != 0)
    );
    assert!(
        ((({
            let mut __it1 = (buf.as_pointer() as Ptr<u8>).to_c_string_iterator();
            let mut __it2 = Ptr::from_string_literal(b"+5  5").to_c_string_iterator();
            loop {
                let __c1 = __it1.next();
                let __c2 = __it2.next();
                if __c1 != __c2 {
                    break (__c1.unwrap_or(0) as i32) - (__c2.unwrap_or(0) as i32);
                }
                if __c1.is_none() {
                    break 0;
                }
            }
        } == 0) as i32)
            != 0)
    );
    assert!(
        ((({
            let __s = libcc2rs::format_c(
                &Ptr::from_string_literal(b"%ld %lu %zu").to_rust_string(),
                &[(-1_i64).into(), (1_u64).into(), (9 as usize).into()],
            );
            let __b = __s.as_bytes();
            if ::std::mem::size_of::<[u8; 32]>() > 0 {
                let __n = ::std::cmp::min(__b.len(), ::std::mem::size_of::<[u8; 32]>() - 1);
                (buf.as_pointer() as Ptr<u8>).with_slice_mut(__n + 1, |__dst| {
                    __dst[..__n].copy_from_slice(&__b[..__n]);
                    __dst[__n] = 0;
                });
            }
            __b.len() as i32
        } == 6) as i32)
            != 0)
    );
    assert!(
        ((({
            let mut __it1 = (buf.as_pointer() as Ptr<u8>).to_c_string_iterator();
            let mut __it2 = Ptr::from_string_literal(b"-1 1 9").to_c_string_iterator();
            loop {
                let __c1 = __it1.next();
                let __c2 = __it2.next();
                if __c1 != __c2 {
                    break (__c1.unwrap_or(0) as i32) - (__c2.unwrap_or(0) as i32);
                }
                if __c1.is_none() {
                    break 0;
                }
            }
        } == 0) as i32)
            != 0)
    );
    assert!(
        ((({
            let __s = libcc2rs::format_c(
                &Ptr::from_string_literal(b"%e").to_rust_string(),
                &[(1.2345678E+3).into()],
            );
            let __b = __s.as_bytes();
            if ::std::mem::size_of::<[u8; 32]>() > 0 {
                let __n = ::std::cmp::min(__b.len(), ::std::mem::size_of::<[u8; 32]>() - 1);
                (buf.as_pointer() as Ptr<u8>).with_slice_mut(__n + 1, |__dst| {
                    __dst[..__n].copy_from_slice(&__b[..__n]);
                    __dst[__n] = 0;
                });
            }
            __b.len() as i32
        } == 12) as i32)
            != 0)
    );
    assert!(
        ((({
            let mut __it1 = (buf.as_pointer() as Ptr<u8>).to_c_string_iterator();
            let mut __it2 = Ptr::from_string_literal(b"1.234568e+03").to_c_string_iterator();
            loop {
                let __c1 = __it1.next();
                let __c2 = __it2.next();
                if __c1 != __c2 {
                    break (__c1.unwrap_or(0) as i32) - (__c2.unwrap_or(0) as i32);
                }
                if __c1.is_none() {
                    break 0;
                }
            }
        } == 0) as i32)
            != 0)
    );
    assert!(
        ((({
            let __s = libcc2rs::format_c(
                &Ptr::from_string_literal(b"%g").to_rust_string(),
                &[(1.234567E+6).into()],
            );
            let __b = __s.as_bytes();
            if ::std::mem::size_of::<[u8; 32]>() > 0 {
                let __n = ::std::cmp::min(__b.len(), ::std::mem::size_of::<[u8; 32]>() - 1);
                (buf.as_pointer() as Ptr<u8>).with_slice_mut(__n + 1, |__dst| {
                    __dst[..__n].copy_from_slice(&__b[..__n]);
                    __dst[__n] = 0;
                });
            }
            __b.len() as i32
        } == 11) as i32)
            != 0)
    );
    assert!(
        ((({
            let mut __it1 = (buf.as_pointer() as Ptr<u8>).to_c_string_iterator();
            let mut __it2 = Ptr::from_string_literal(b"1.23457e+06").to_c_string_iterator();
            loop {
                let __c1 = __it1.next();
                let __c2 = __it2.next();
                if __c1 != __c2 {
                    break (__c1.unwrap_or(0) as i32) - (__c2.unwrap_or(0) as i32);
                }
                if __c1.is_none() {
                    break 0;
                }
            }
        } == 0) as i32)
            != 0)
    );
    (*buf.borrow_mut())[(0) as usize] = (('Z' as i32) as u8);
    assert!(
        ((({
            let __s = libcc2rs::format_c(
                &Ptr::from_string_literal(b"%d").to_rust_string(),
                &[(123).into()],
            );
            let __b = __s.as_bytes();
            if 0_usize > 0 {
                let __n = ::std::cmp::min(__b.len(), 0_usize - 1);
                (buf.as_pointer() as Ptr<u8>).with_slice_mut(__n + 1, |__dst| {
                    __dst[..__n].copy_from_slice(&__b[..__n]);
                    __dst[__n] = 0;
                });
            }
            __b.len() as i32
        } == 3) as i32)
            != 0)
    );
    assert!((((((*buf.borrow())[(0) as usize] as i32) == ('Z' as i32)) as i32) != 0));
    let fmt: Value<Box<[u8]>> = Rc::new(RefCell::new(
        (0..8).map(|_| <u8>::default()).collect::<Box<[u8]>>(),
    ));
    (*fmt.borrow_mut())[(0) as usize] = (('%' as i32) as u8);
    (*fmt.borrow_mut())[(1) as usize] = (('5' as i32) as u8);
    (*fmt.borrow_mut())[(2) as usize] = (('.' as i32) as u8);
    (*fmt.borrow_mut())[(3) as usize] = (('1' as i32) as u8);
    (*fmt.borrow_mut())[(4) as usize] = (('f' as i32) as u8);
    (*fmt.borrow_mut())[(5) as usize] = 0_u8;
    assert!(
        ((({
            let __s = libcc2rs::format_c(
                &(fmt.as_pointer() as Ptr<u8>).to_rust_string(),
                &[(3.26E+0).into()],
            );
            let __b = __s.as_bytes();
            if ::std::mem::size_of::<[u8; 32]>() > 0 {
                let __n = ::std::cmp::min(__b.len(), ::std::mem::size_of::<[u8; 32]>() - 1);
                (buf.as_pointer() as Ptr<u8>).with_slice_mut(__n + 1, |__dst| {
                    __dst[..__n].copy_from_slice(&__b[..__n]);
                    __dst[__n] = 0;
                });
            }
            __b.len() as i32
        } == 5) as i32)
            != 0)
    );
    assert!(
        ((({
            let mut __it1 = (buf.as_pointer() as Ptr<u8>).to_c_string_iterator();
            let mut __it2 = Ptr::from_string_literal(b"  3.3").to_c_string_iterator();
            loop {
                let __c1 = __it1.next();
                let __c2 = __it2.next();
                if __c1 != __c2 {
                    break (__c1.unwrap_or(0) as i32) - (__c2.unwrap_or(0) as i32);
                }
                if __c1.is_none() {
                    break 0;
                }
            }
        } == 0) as i32)
            != 0)
    );
    return 0;
}
