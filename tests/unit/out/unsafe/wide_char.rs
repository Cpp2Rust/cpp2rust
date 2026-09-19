extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn get_0(mut s: *const libc::c_char) -> *const libc::c_char {
    return s;
}
pub unsafe fn get_1(mut s: *const i32) -> *const i32 {
    return s;
}
pub unsafe fn get_2(mut s: *const u8) -> *const u8 {
    return s;
}
pub unsafe fn get_3(mut s: *const u16) -> *const u16 {
    return s;
}
pub unsafe fn get_4(mut s: *const u32) -> *const u32 {
    return s;
}
pub unsafe fn second_5(s: *const [libc::c_char; 3]) -> libc::c_char {
    return (*s)[(1) as usize];
}
pub unsafe fn second_6(s: *const [i32; 3]) -> i32 {
    return (*s)[(1) as usize];
}
pub unsafe fn second_7(s: *const [u8; 3]) -> u8 {
    return (*s)[(1) as usize];
}
pub unsafe fn second_8(s: *const [u16; 3]) -> u16 {
    return (*s)[(1) as usize];
}
pub unsafe fn second_9(s: *const [u32; 3]) -> u32 {
    return (*s)[(1) as usize];
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut c: *const libc::c_char = c"A".as_ptr();
    let mut w: *const i32 = [65 as i32, 258 as i32, 0 as i32].as_ptr();
    let mut b: *const u8 = [196 as u8, 130 as u8, 0 as u8].as_ptr();
    let mut s: *const u16 = [65 as u16, 258 as u16, 0 as u16].as_ptr();
    let mut l: *const u32 = [65 as u32, 258 as u32, 0 as u32].as_ptr();
    assert!((((*c.offset((0) as isize)) as i32) == (('A' as libc::c_char) as i32)));
    assert!(
        (((*b.offset((0) as isize)) as i32) == (196))
            && (((*b.offset((1) as isize)) as i32) == (130))
    );
    assert!(((*w.offset((1) as isize)) == (258)));
    assert!((((*s.offset((1) as isize)) as i32) == (258)));
    assert!(((*l.offset((1) as isize)) == (258_u32)));
    assert!(
        (((*w.offset((2) as isize)) == (0)) && (((*s.offset((2) as isize)) as i32) == (0)))
            && ((*l.offset((2) as isize)) == (0_u32))
    );
    assert!(
        (((*(unsafe { get_0(c"A".as_ptr(),) }).offset((0) as isize)) as i32)
            == (('A' as libc::c_char) as i32))
    );
    assert!(
        ((*(unsafe { get_1([258 as i32, 0 as i32,].as_ptr(),) }).offset((0) as isize)) == (258))
    );
    assert!(
        (((*(unsafe { get_2([196 as u8, 130 as u8, 0 as u8,].as_ptr(),) }).offset((0) as isize))
            as i32)
            == (196))
    );
    assert!(
        (((*(unsafe { get_3([258 as u16, 0 as u16,].as_ptr(),) }).offset((0) as isize)) as i32)
            == (258))
    );
    assert!(
        ((*(unsafe { get_4([258 as u32, 0 as u32,].as_ptr(),) }).offset((0) as isize))
            == (258_u32))
    );
    assert!(
        (((unsafe { second_5(&std::mem::transmute(*b"AB\0"),) }) as i32)
            == (('B' as libc::c_char) as i32))
    );
    assert!(((unsafe { second_6(&[65 as i32, 258 as i32, 0 as i32,],) }) == (258)));
    assert!((((unsafe { second_7(&[196 as u8, 130 as u8, 0 as u8,],) }) as i32) == (130)));
    assert!((((unsafe { second_8(&[65 as u16, 258 as u16, 0 as u16,],) }) as i32) == (258)));
    assert!(((unsafe { second_9(&[65 as u32, 258 as u32, 0 as u32,],) }) == (258_u32)));
    let nw: usize = ((::std::mem::size_of::<[i32; 4]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize)) as usize)
        .wrapping_sub(1_usize);
    let nb: usize = ((::std::mem::size_of::<[u8; 4]>() as usize)
        .wrapping_div((::std::mem::size_of::<u8>() as usize)) as usize)
        .wrapping_sub(1_usize);
    let ns: usize = ((::std::mem::size_of::<[u16; 4]>() as usize)
        .wrapping_div((::std::mem::size_of::<u16>() as usize)) as usize)
        .wrapping_sub(1_usize);
    let nl: usize = ((::std::mem::size_of::<[u32; 4]>() as usize)
        .wrapping_div((::std::mem::size_of::<u32>() as usize)) as usize)
        .wrapping_sub(1_usize);
    assert!(
        ((((nw) == (3_usize)) && ((nb) == (3_usize))) && ((ns) == (3_usize)))
            && ((nl) == (3_usize))
    );
    let pw: [i32; 4] = [258 as i32, 0 as i32, 0 as i32, 0 as i32];
    let ps: [u16; 4] = [258 as u16, 0 as u16, 0 as u16, 0 as u16];
    assert!(
        (((pw[(0) as usize]) == (258)) && ((pw[(1) as usize]) == (0)))
            && ((pw[(3) as usize]) == (0))
    );
    assert!(
        (((ps[(0) as usize] as i32) == (258)) && ((ps[(1) as usize] as i32) == (0)))
            && ((ps[(3) as usize] as i32) == (0))
    );
    let ew: [i32; 2] = [0 as i32, 0 as i32];
    assert!(((ew[(0) as usize]) == (0)) && ((ew[(1) as usize]) == (0)));
    let mut wc: i32 = (258 as i32);
    let mut bc: u8 = (65 as u8);
    let mut sc: u16 = (258 as u16);
    let mut lc: u32 = (258 as u32);
    assert!(
        ((((wc) == (258)) && ((bc as i32) == (65))) && ((sc as i32) == (258)))
            && ((lc) == (258_u32))
    );
    assert!(
        ((((::std::mem::size_of::<i32>()) == (::std::mem::size_of::<i32>()))
            && ((::std::mem::size_of::<u8>()) == (1_usize)))
            && ((::std::mem::size_of::<u16>()) == (2_usize)))
            && ((::std::mem::size_of::<u32>()) == (4_usize))
    );
    assert!(
        (((*w.offset((1) as isize)) == (258 as i32))
            && (((*s.offset((1) as isize)) as i32) == ((258 as u16) as i32)))
            && ((*l.offset((1) as isize)) == (258 as u32))
    );
    assert!(
        (((*b.offset((0) as isize)) as i32) == ((196 as u8) as i32))
            && (((*b.offset((1) as isize)) as i32) == ((130 as u8) as i32))
    );
    assert!(
        ((unsafe { second_6(&[65 as i32, 258 as i32, 0 as i32,],) }) == (258 as i32))
            && (((unsafe { second_8(&[65 as u16, 258 as u16, 0 as u16,],) }) as i32)
                == ((258 as u16) as i32))
    );
    assert!(
        ((*(unsafe { get_4([258 as u32, 0 as u32,].as_ptr(),) }).offset((0) as isize))
            == (258 as u32))
            && ((*(unsafe { get_4([258 as u32, 0 as u32,].as_ptr(),) }).offset((1) as isize))
                == (0 as u32))
    );
    assert!((((10 as i32) == (10)) && (((9 as u16) as i32) == (9))) && ((92 as u32) == (92_u32)));
    assert!((((97 as i32) + (1)) == (98 as i32)));
    assert!((((122 as u32).wrapping_sub((97 as u32))) == (25_u32)));
    let mut wa: [i32; 3] = [(65 as i32), (258 as i32), (0 as i32)];
    assert!(
        (((wa[(0) as usize]) == (65)) && ((wa[(1) as usize]) == (258)))
            && ((wa[(2) as usize]) == (0))
    );
    wa[(0) as usize] = (66 as i32);
    assert!(((wa[(0) as usize]) == (('B' as libc::c_char) as i32)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
