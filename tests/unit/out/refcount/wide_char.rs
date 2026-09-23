extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn get_0(s: Ptr<u8>) -> Ptr<u8> {
    let s: Value<Ptr<u8>> = Rc::new(RefCell::new(s));
    return (*s.borrow()).clone();
}
pub fn get_1(s: Ptr<i32>) -> Ptr<i32> {
    let s: Value<Ptr<i32>> = Rc::new(RefCell::new(s));
    return (*s.borrow()).clone();
}
pub fn get_2(s: Ptr<u8>) -> Ptr<u8> {
    let s: Value<Ptr<u8>> = Rc::new(RefCell::new(s));
    return (*s.borrow()).clone();
}
pub fn get_3(s: Ptr<u16>) -> Ptr<u16> {
    let s: Value<Ptr<u16>> = Rc::new(RefCell::new(s));
    return (*s.borrow()).clone();
}
pub fn get_4(s: Ptr<u32>) -> Ptr<u32> {
    let s: Value<Ptr<u32>> = Rc::new(RefCell::new(s));
    return (*s.borrow()).clone();
}
pub fn second_5(s: Ptr<u8>) -> u8 {
    return ((s).offset((1) as isize).read());
}
pub fn second_6(s: Ptr<i32>) -> i32 {
    return ((s).offset((1) as isize).read());
}
pub fn second_7(s: Ptr<u8>) -> u8 {
    return ((s).offset((1) as isize).read());
}
pub fn second_8(s: Ptr<u16>) -> u16 {
    return ((s).offset((1) as isize).read());
}
pub fn second_9(s: Ptr<u32>) -> u32 {
    return ((s).offset((1) as isize).read());
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let c: Value<Ptr<u8>> = Rc::new(RefCell::new(Ptr::<u8>::from_string_literal(b"A")));
    let w: Value<Ptr<i32>> = Rc::new(RefCell::new(Ptr::<i32>::from_string_literal(&[
        65 as i32, 258 as i32, 0 as i32,
    ])));
    let b: Value<Ptr<u8>> = Rc::new(RefCell::new(Ptr::<u8>::from_string_literal(&[
        196 as u8, 130 as u8, 0 as u8,
    ])));
    let s: Value<Ptr<u16>> = Rc::new(RefCell::new(Ptr::<u16>::from_string_literal(&[
        65 as u16, 258 as u16, 0 as u16,
    ])));
    let l: Value<Ptr<u32>> = Rc::new(RefCell::new(Ptr::<u32>::from_string_literal(&[
        65 as u32, 258 as u32, 0 as u32,
    ])));
    assert!(((((*c.borrow()).offset((0) as isize).read()) as i32) == (('A' as u8) as i32)));
    assert!(
        ((((*b.borrow()).offset((0) as isize).read()) as i32) == 196)
            && ((((*b.borrow()).offset((1) as isize).read()) as i32) == 130)
    );
    assert!((((*w.borrow()).offset((1) as isize).read()) == 258));
    assert!(((((*s.borrow()).offset((1) as isize).read()) as i32) == 258));
    assert!((((*l.borrow()).offset((1) as isize).read()) == 258_u32));
    assert!(
        ((((*w.borrow()).offset((2) as isize).read()) == 0)
            && ((((*s.borrow()).offset((2) as isize).read()) as i32) == 0))
            && (((*l.borrow()).offset((2) as isize).read()) == 0_u32)
    );
    assert!(
        (((({ get_0(Ptr::<u8>::from_string_literal(b"A"),) })
            .offset((0) as isize)
            .read()) as i32)
            == (('A' as u8) as i32))
    );
    assert!(
        ((({ get_1(Ptr::<i32>::from_string_literal(&[258 as i32, 0 as i32,]),) })
            .offset((0) as isize)
            .read())
            == 258)
    );
    assert!(
        (((({
            get_2(Ptr::<u8>::from_string_literal(&[
                196 as u8, 130 as u8, 0 as u8,
            ]))
        })
        .offset((0) as isize)
        .read()) as i32)
            == 196)
    );
    assert!(
        (((({ get_3(Ptr::<u16>::from_string_literal(&[258 as u16, 0 as u16,]),) })
            .offset((0) as isize)
            .read()) as i32)
            == 258)
    );
    assert!(
        ((({ get_4(Ptr::<u32>::from_string_literal(&[258 as u32, 0 as u32,]),) })
            .offset((0) as isize)
            .read())
            == 258_u32)
    );
    assert!(
        ((({ second_5(Ptr::<u8>::from_string_literal(b"AB"),) }) as i32) == (('B' as u8) as i32))
    );
    assert!(
        (({
            second_6(Ptr::<i32>::from_string_literal(&[
                65 as i32, 258 as i32, 0 as i32,
            ]))
        }) == 258)
    );
    assert!(
        ((({
            second_7(Ptr::<u8>::from_string_literal(&[
                196 as u8, 130 as u8, 0 as u8,
            ]))
        }) as i32)
            == 130)
    );
    assert!(
        ((({
            second_8(Ptr::<u16>::from_string_literal(&[
                65 as u16, 258 as u16, 0 as u16,
            ]))
        }) as i32)
            == 258)
    );
    assert!(
        (({
            second_9(Ptr::<u32>::from_string_literal(&[
                65 as u32, 258 as u32, 0 as u32,
            ]))
        }) == 258_u32)
    );
    let nw: Value<usize> = Rc::new(RefCell::new(
        ((::std::mem::size_of::<[i32; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize)) as usize)
            .wrapping_sub(1_usize),
    ));
    let nb: Value<usize> = Rc::new(RefCell::new(
        ((::std::mem::size_of::<[u8; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<u8>() as usize)) as usize)
            .wrapping_sub(1_usize),
    ));
    let ns: Value<usize> = Rc::new(RefCell::new(
        ((::std::mem::size_of::<[u16; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<u16>() as usize)) as usize)
            .wrapping_sub(1_usize),
    ));
    let nl: Value<usize> = Rc::new(RefCell::new(
        ((::std::mem::size_of::<[u32; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<u32>() as usize)) as usize)
            .wrapping_sub(1_usize),
    ));
    assert!(
        ((((*nw.borrow()) == 3_usize) && ((*nb.borrow()) == 3_usize))
            && ((*ns.borrow()) == 3_usize))
            && ((*nl.borrow()) == 3_usize)
    );
    let pw: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::from([
        258 as i32, 0 as i32, 0 as i32, 0 as i32,
    ])));
    let ps: Value<Box<[u16]>> = Rc::new(RefCell::new(Box::from([
        258 as u16, 0 as u16, 0 as u16, 0 as u16,
    ])));
    assert!(
        (((*pw.borrow())[(0) as usize] == 258) && ((*pw.borrow())[(1) as usize] == 0))
            && ((*pw.borrow())[(3) as usize] == 0)
    );
    assert!(
        ((((*ps.borrow())[(0) as usize] as i32) == 258)
            && (((*ps.borrow())[(1) as usize] as i32) == 0))
            && (((*ps.borrow())[(3) as usize] as i32) == 0)
    );
    let ew: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::from([0 as i32, 0 as i32])));
    assert!(((*ew.borrow())[(0) as usize] == 0) && ((*ew.borrow())[(1) as usize] == 0));
    let wc: Value<i32> = Rc::new(RefCell::new((258 as i32)));
    let bc: Value<u8> = Rc::new(RefCell::new((65 as u8)));
    let sc: Value<u16> = Rc::new(RefCell::new((258 as u16)));
    let lc: Value<u32> = Rc::new(RefCell::new((258 as u32)));
    assert!(
        ((((*wc.borrow()) == 258) && (((*bc.borrow()) as i32) == 65))
            && (((*sc.borrow()) as i32) == 258))
            && ((*lc.borrow()) == 258_u32)
    );
    assert!(
        (((::std::mem::size_of::<i32>() == ::std::mem::size_of::<i32>())
            && (::std::mem::size_of::<u8>() == 1_usize))
            && (::std::mem::size_of::<u16>() == 2_usize))
            && (::std::mem::size_of::<u32>() == 4_usize)
    );
    assert!(
        ((((*w.borrow()).offset((1) as isize).read()) == (258 as i32))
            && ((((*s.borrow()).offset((1) as isize).read()) as i32) == ((258 as u16) as i32)))
            && (((*l.borrow()).offset((1) as isize).read()) == (258 as u32))
    );
    assert!(
        ((((*b.borrow()).offset((0) as isize).read()) as i32) == ((196 as u8) as i32))
            && ((((*b.borrow()).offset((1) as isize).read()) as i32) == ((130 as u8) as i32))
    );
    assert!(
        (({
            second_6(Ptr::<i32>::from_string_literal(&[
                65 as i32, 258 as i32, 0 as i32,
            ]))
        }) == (258 as i32))
            && ((({
                second_8(Ptr::<u16>::from_string_literal(&[
                    65 as u16, 258 as u16, 0 as u16,
                ]))
            }) as i32)
                == ((258 as u16) as i32))
    );
    assert!(
        ((({ get_4(Ptr::<u32>::from_string_literal(&[258 as u32, 0 as u32,]),) })
            .offset((0) as isize)
            .read())
            == (258 as u32))
            && ((({ get_4(Ptr::<u32>::from_string_literal(&[258 as u32, 0 as u32,]),) })
                .offset((1) as isize)
                .read())
                == (0 as u32))
    );
    assert!((((10 as i32) == 10) && (((9 as u16) as i32) == 9)) && ((92 as u32) == 92_u32));
    assert!((((97 as i32) + 1) == (98 as i32)));
    assert!(((122 as u32).wrapping_sub((97 as u32)) == 25_u32));
    let wa: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([
        (65 as i32),
        (258 as i32),
        (0 as i32),
    ])));
    assert!(
        (((*wa.borrow())[(0) as usize] == 65) && ((*wa.borrow())[(1) as usize] == 258))
            && ((*wa.borrow())[(2) as usize] == 0)
    );
    (*wa.borrow_mut())[(0) as usize] = (66 as i32);
    assert!(((*wa.borrow())[(0) as usize] == (('B' as u8) as i32)));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
