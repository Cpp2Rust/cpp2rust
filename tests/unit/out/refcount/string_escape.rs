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
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut special: Ptr<i8> = Ptr::<i8>::from_string_literal(
        b"\x07\x08\t\n\x0b\x0c\r !\"#$%&\'()*+,-./:;<=>?@[\\]^_`{|}~\xff",
    );
    thread_local!(
        static expected_0: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
            7_i8,
            8_i8,
            9_i8,
            10_i8,
            11_i8,
            12_i8,
            13_i8,
            32_i8,
            33_i8,
            34_i8,
            35_i8,
            36_i8,
            37_i8,
            38_i8,
            39_i8,
            40_i8,
            41_i8,
            42_i8,
            43_i8,
            44_i8,
            45_i8,
            46_i8,
            47_i8,
            58_i8,
            59_i8,
            60_i8,
            61_i8,
            62_i8,
            63_i8,
            64_i8,
            91_i8,
            92_i8,
            93_i8,
            94_i8,
            95_i8,
            96_i8,
            123_i8,
            124_i8,
            125_i8,
            126_i8,
            (b'\xff' as i8),
        ])));
    );
    let mut i: i32 = 0;
    'loop_: while (i
        < (((::std::mem::size_of::<[i8; 41]>() as usize)
            .wrapping_div((::std::mem::size_of::<i8>() as usize))) as i32))
    {
        assert!(
            ({ ((elem!(special, i).read()) as i32) } == {
                (({
                    let __idx = (i) as usize;
                    expected_0.with(|rc| rc.borrow()[__idx])
                }) as i32)
            })
        );
        i.postfix_inc();
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
