extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let buf: Value<Box<[i8]>> = Rc::new(RefCell::new((0..32).map(|_| 0_i8).collect::<Box<[i8]>>()));
    assert!(
        ({
            let mut __tmp_0 = { buf.decay() };
            let __b = Ptr::<i8>::from_string_literal(b"x=%d y=%u").with_c_bytes(|__fmt| {
                libcc2rs::format_c(
                    __fmt,
                    &Box::<[VaArg]>::from([(-3_i32).into(), (7_u32).into()]),
                )
            });
            if ::std::mem::size_of::<[i8; 32]>() > 0 {
                let __n = ::std::cmp::min(__b.len(), ::std::mem::size_of::<[i8; 32]>() - 1);
                __tmp_0.write_c_bytes(&__b[..__n]);
                elem!(__tmp_0, __n).write(0);
            }
            __b.len() as i32
        } == 8)
    );
    assert!(
        ({
            let mut __it1 = buf.decay().to_c_string_iterator();
            let mut __it2 = Ptr::<i8>::from_string_literal(b"x=-3 y=7").to_c_string_iterator();
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
            let mut __tmp_1 = { buf.decay() };
            let __b = Ptr::<i8>::from_string_literal(b"%s").with_c_bytes(|__fmt| {
                libcc2rs::format_c(
                    __fmt,
                    &Box::<[VaArg]>::from([(Ptr::<i8>::from_string_literal(b"hello")).into()]),
                )
            });
            if 4_usize > 0 {
                let __n = ::std::cmp::min(__b.len(), 4_usize - 1);
                __tmp_1.write_c_bytes(&__b[..__n]);
                elem!(__tmp_1, __n).write(0);
            }
            __b.len() as i32
        } == 5)
    );
    assert!(
        ({
            let mut __it1 = buf.decay().to_c_string_iterator();
            let mut __it2 = Ptr::<i8>::from_string_literal(b"hel").to_c_string_iterator();
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
            let mut __tmp_2 = { buf.decay() };
            let __b = Ptr::<i8>::from_string_literal(b"%05d|%x|%X").with_c_bytes(|__fmt| {
                libcc2rs::format_c(
                    __fmt,
                    &Box::<[VaArg]>::from([(42).into(), (255).into(), (255).into()]),
                )
            });
            if ::std::mem::size_of::<[i8; 32]>() > 0 {
                let __n = ::std::cmp::min(__b.len(), ::std::mem::size_of::<[i8; 32]>() - 1);
                __tmp_2.write_c_bytes(&__b[..__n]);
                elem!(__tmp_2, __n).write(0);
            }
            __b.len() as i32
        } == 11)
    );
    assert!(
        ({
            let mut __it1 = buf.decay().to_c_string_iterator();
            let mut __it2 = Ptr::<i8>::from_string_literal(b"00042|ff|FF").to_c_string_iterator();
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
            let mut __tmp_3 = { buf.decay() };
            let __b = Ptr::<i8>::from_string_literal(b"%.2f").with_c_bytes(|__fmt| {
                libcc2rs::format_c(__fmt, &Box::<[VaArg]>::from([(3.14159_f64).into()]))
            });
            if ::std::mem::size_of::<[i8; 32]>() > 0 {
                let __n = ::std::cmp::min(__b.len(), ::std::mem::size_of::<[i8; 32]>() - 1);
                __tmp_3.write_c_bytes(&__b[..__n]);
                elem!(__tmp_3, __n).write(0);
            }
            __b.len() as i32
        } == 4)
    );
    assert!(
        ({
            let mut __it1 = buf.decay().to_c_string_iterator();
            let mut __it2 = Ptr::<i8>::from_string_literal(b"3.14").to_c_string_iterator();
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
            let mut __tmp_4 = { buf.decay() };
            let __b = Ptr::<i8>::from_string_literal(b"%-6s|").with_c_bytes(|__fmt| {
                libcc2rs::format_c(
                    __fmt,
                    &Box::<[VaArg]>::from([(Ptr::<i8>::from_string_literal(b"ab")).into()]),
                )
            });
            if ::std::mem::size_of::<[i8; 32]>() > 0 {
                let __n = ::std::cmp::min(__b.len(), ::std::mem::size_of::<[i8; 32]>() - 1);
                __tmp_4.write_c_bytes(&__b[..__n]);
                elem!(__tmp_4, __n).write(0);
            }
            __b.len() as i32
        } == 7)
    );
    assert!(
        ({
            let mut __it1 = buf.decay().to_c_string_iterator();
            let mut __it2 = Ptr::<i8>::from_string_literal(b"ab    |").to_c_string_iterator();
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
            let mut __tmp_5 = { buf.decay() };
            let __b = Ptr::<i8>::from_string_literal(b"%c%%").with_c_bytes(|__fmt| {
                libcc2rs::format_c(__fmt, &Box::<[VaArg]>::from([(65).into()]))
            });
            if ::std::mem::size_of::<[i8; 32]>() > 0 {
                let __n = ::std::cmp::min(__b.len(), ::std::mem::size_of::<[i8; 32]>() - 1);
                __tmp_5.write_c_bytes(&__b[..__n]);
                elem!(__tmp_5, __n).write(0);
            }
            __b.len() as i32
        } == 2)
    );
    assert!(
        ({
            let mut __it1 = buf.decay().to_c_string_iterator();
            let mut __it2 = Ptr::<i8>::from_string_literal(b"A%").to_c_string_iterator();
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
            let mut __tmp_6 = { buf.decay() };
            let __b = Ptr::<i8>::from_string_literal(b"%+d % d").with_c_bytes(|__fmt| {
                libcc2rs::format_c(__fmt, &Box::<[VaArg]>::from([(5).into(), (5).into()]))
            });
            if ::std::mem::size_of::<[i8; 32]>() > 0 {
                let __n = ::std::cmp::min(__b.len(), ::std::mem::size_of::<[i8; 32]>() - 1);
                __tmp_6.write_c_bytes(&__b[..__n]);
                elem!(__tmp_6, __n).write(0);
            }
            __b.len() as i32
        } == 5)
    );
    assert!(
        ({
            let mut __it1 = buf.decay().to_c_string_iterator();
            let mut __it2 = Ptr::<i8>::from_string_literal(b"+5  5").to_c_string_iterator();
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
            let mut __tmp_7 = { buf.decay() };
            let __b = Ptr::<i8>::from_string_literal(b"%ld %lu %zu").with_c_bytes(|__fmt| {
                libcc2rs::format_c(
                    __fmt,
                    &Box::<[VaArg]>::from([(-1_i64).into(), (1_u64).into(), (9 as usize).into()]),
                )
            });
            if ::std::mem::size_of::<[i8; 32]>() > 0 {
                let __n = ::std::cmp::min(__b.len(), ::std::mem::size_of::<[i8; 32]>() - 1);
                __tmp_7.write_c_bytes(&__b[..__n]);
                elem!(__tmp_7, __n).write(0);
            }
            __b.len() as i32
        } == 6)
    );
    assert!(
        ({
            let mut __it1 = buf.decay().to_c_string_iterator();
            let mut __it2 = Ptr::<i8>::from_string_literal(b"-1 1 9").to_c_string_iterator();
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
            let mut __tmp_8 = { buf.decay() };
            let __b = Ptr::<i8>::from_string_literal(b"%e").with_c_bytes(|__fmt| {
                libcc2rs::format_c(__fmt, &Box::<[VaArg]>::from([(1234.5678_f64).into()]))
            });
            if ::std::mem::size_of::<[i8; 32]>() > 0 {
                let __n = ::std::cmp::min(__b.len(), ::std::mem::size_of::<[i8; 32]>() - 1);
                __tmp_8.write_c_bytes(&__b[..__n]);
                elem!(__tmp_8, __n).write(0);
            }
            __b.len() as i32
        } == 12)
    );
    assert!(
        ({
            let mut __it1 = buf.decay().to_c_string_iterator();
            let mut __it2 = Ptr::<i8>::from_string_literal(b"1.234568e+03").to_c_string_iterator();
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
            let mut __tmp_9 = { buf.decay() };
            let __b = Ptr::<i8>::from_string_literal(b"%g").with_c_bytes(|__fmt| {
                libcc2rs::format_c(__fmt, &Box::<[VaArg]>::from([(1234567_f64).into()]))
            });
            if ::std::mem::size_of::<[i8; 32]>() > 0 {
                let __n = ::std::cmp::min(__b.len(), ::std::mem::size_of::<[i8; 32]>() - 1);
                __tmp_9.write_c_bytes(&__b[..__n]);
                elem!(__tmp_9, __n).write(0);
            }
            __b.len() as i32
        } == 11)
    );
    assert!(
        ({
            let mut __it1 = buf.decay().to_c_string_iterator();
            let mut __it2 = Ptr::<i8>::from_string_literal(b"1.23457e+06").to_c_string_iterator();
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
    (*buf.borrow_mut())[(0) as usize] = (('Z' as i32) as i8);
    assert!(
        ({
            let mut __tmp_10 = { buf.decay() };
            let __b = Ptr::<i8>::from_string_literal(b"%d").with_c_bytes(|__fmt| {
                libcc2rs::format_c(__fmt, &Box::<[VaArg]>::from([(123).into()]))
            });
            if 0_usize > 0 {
                let __n = ::std::cmp::min(__b.len(), 0_usize - 1);
                __tmp_10.write_c_bytes(&__b[..__n]);
                elem!(__tmp_10, __n).write(0);
            }
            __b.len() as i32
        } == 3)
    );
    assert!(({ (((*buf.borrow())[(0) as usize] as i32) == ('Z' as i32)) }));
    let fmt: Value<Box<[i8]>> = Rc::new(RefCell::new((0..8).map(|_| 0_i8).collect::<Box<[i8]>>()));
    (*fmt.borrow_mut())[(0) as usize] = (('%' as i32) as i8);
    (*fmt.borrow_mut())[(1) as usize] = (('5' as i32) as i8);
    (*fmt.borrow_mut())[(2) as usize] = (('.' as i32) as i8);
    (*fmt.borrow_mut())[(3) as usize] = (('1' as i32) as i8);
    (*fmt.borrow_mut())[(4) as usize] = (('f' as i32) as i8);
    (*fmt.borrow_mut())[(5) as usize] = 0_i8;
    assert!(
        ({
            let mut __tmp_11 = { buf.decay() };
            let __b = fmt.decay().with_c_bytes(|__fmt| {
                libcc2rs::format_c(__fmt, &Box::<[VaArg]>::from([(3.26_f64).into()]))
            });
            if ::std::mem::size_of::<[i8; 32]>() > 0 {
                let __n = ::std::cmp::min(__b.len(), ::std::mem::size_of::<[i8; 32]>() - 1);
                __tmp_11.write_c_bytes(&__b[..__n]);
                elem!(__tmp_11, __n).write(0);
            }
            __b.len() as i32
        } == 5)
    );
    assert!(
        ({
            let mut __it1 = buf.decay().to_c_string_iterator();
            let mut __it2 = Ptr::<i8>::from_string_literal(b"  3.3").to_c_string_iterator();
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
            let mut __tmp_12 = { buf.decay() };
            let __b = Ptr::<i8>::from_string_literal(b"\xff%s|%4s|%.1s|%c").with_c_bytes(|__fmt| {
                libcc2rs::format_c(
                    __fmt,
                    &Box::<[VaArg]>::from([
                        (Ptr::<i8>::from_string_literal(b"\xe9")).into(),
                        (Ptr::<i8>::from_string_literal(b"\xc3\xa9")).into(),
                        (Ptr::<i8>::from_string_literal(b"\xc3\xa9")).into(),
                        (233).into(),
                    ]),
                )
            });
            if ::std::mem::size_of::<[i8; 32]>() > 0 {
                let __n = ::std::cmp::min(__b.len(), ::std::mem::size_of::<[i8; 32]>() - 1);
                __tmp_12.write_c_bytes(&__b[..__n]);
                elem!(__tmp_12, __n).write(0);
            }
            __b.len() as i32
        } == 11)
    );
    assert!(
        ({
            let mut __it1 = buf.decay().to_c_string_iterator();
            let mut __it2 = Ptr::<i8>::from_string_literal(b"\xff\xe9|  \xc3\xa9|\xc3|\xe9")
                .to_c_string_iterator();
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
    return 0;
}
pub fn __cpp2rust_init_globals() {}
