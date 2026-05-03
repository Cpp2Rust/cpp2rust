extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[derive(Clone, Copy, PartialEq, Debug, Default)]
enum Choice {
    #[default]
    C_LIST = 1,
    C_LETTERS = 2,
    C_INTEGERS = 3,
}
impl From<i32> for Choice {
    fn from(n: i32) -> Choice {
        match n {
            1 => Choice::C_LIST,
            2 => Choice::C_LETTERS,
            3 => Choice::C_INTEGERS,
            _ => panic!("invalid Choice value: {}", n),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Branch_anon_0_anon_0 {
    pub items: *mut *mut u8,
    pub count: i64,
    pub cursor: i64,
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Branch_anon_0_anon_1 {
    pub lo: i32,
    pub hi: i32,
    pub curr: i32,
    pub step: u8,
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Branch_anon_0_anon_2 {
    pub lo: i64,
    pub hi: i64,
    pub curr: i64,
    pub step: i64,
    pub width: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union Branch_anon_0 {
    pub list: Branch_anon_0_anon_0,
    pub letters: Branch_anon_0_anon_1,
    pub integers: Branch_anon_0_anon_2,
}
impl Default for Branch_anon_0 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Branch {
    pub choice: Choice,
    pub index: i32,
    pub v: Branch_anon_0,
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    static mut items: [*mut u8; 3] = [
        b"a\0".as_ptr().cast_mut(),
        b"b\0".as_ptr().cast_mut(),
        b"c\0".as_ptr().cast_mut(),
    ];;
    let mut p_list: Branch = <Branch>::default();
    p_list.choice = Choice::from((Choice::C_LIST as i32));
    p_list.index = 0;
    p_list.v.list.items = items.as_mut_ptr();
    p_list.v.list.count = 3_i64;
    p_list.v.list.cursor = 1_i64;
    assert!(((((p_list.v.list.count) == (3_i64)) as i32) != 0));
    assert!(
        (((((*(*p_list.v.list.items.offset((1) as isize)).offset((0) as isize)) as i32)
            == ('b' as i32)) as i32)
            != 0)
    );
    let mut p_letters: Branch = <Branch>::default();
    p_letters.choice = Choice::from((Choice::C_LETTERS as i32));
    p_letters.index = 1;
    p_letters.v.letters.lo = ('a' as i32);
    p_letters.v.letters.hi = ('z' as i32);
    p_letters.v.letters.curr = ('m' as i32);
    p_letters.v.letters.step = 1_u8;
    assert!((((((p_letters.v.letters.hi) - (p_letters.v.letters.lo)) == (25)) as i32) != 0));
    let mut p_integers: Branch = <Branch>::default();
    p_integers.choice = Choice::from((Choice::C_INTEGERS as i32));
    p_integers.index = 2;
    p_integers.v.integers.lo = 1_i64;
    p_integers.v.integers.hi = 100_i64;
    p_integers.v.integers.curr = 1_i64;
    p_integers.v.integers.step = 1_i64;
    p_integers.v.integers.width = 3;
    assert!(((((p_integers.v.integers.hi) == (100_i64)) as i32) != 0));
    assert!(((((p_integers.v.integers.width) == (3)) as i32) != 0));
    return 0;
}
