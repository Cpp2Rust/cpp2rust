extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut special: *const u8 =
        b"\x07\x08\t\n\x0b\x0c\r !\"#$%&\'()*+,-./:;<=>?@[\\]^_`{|}~\0".as_ptr();
    static mut expected: [u8; 40] = [
        7_u8, 8_u8, 9_u8, 10_u8, 11_u8, 12_u8, 13_u8, 32_u8, 33_u8, 34_u8, 35_u8, 36_u8, 37_u8,
        38_u8, 39_u8, 40_u8, 41_u8, 42_u8, 43_u8, 44_u8, 45_u8, 46_u8, 47_u8, 58_u8, 59_u8, 60_u8,
        61_u8, 62_u8, 63_u8, 64_u8, 91_u8, 92_u8, 93_u8, 94_u8, 95_u8, 96_u8, 123_u8, 124_u8,
        125_u8, 126_u8,
    ];;
    let mut i: i32 = 0;
    'loop_: while ((i)
        < (((::std::mem::size_of::<[u8; 40]>() as u64 as u64)
            .wrapping_div(::std::mem::size_of::<u8>() as u64 as u64)) as i32))
    {
        assert!((((*special.offset((i) as isize)) as i32) == (expected[(i) as usize] as i32)));
        i.postfix_inc();
    }
    return 0;
}
