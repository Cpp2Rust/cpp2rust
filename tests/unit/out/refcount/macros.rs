extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn base_name_0(mut path: Ptr<i8>) -> Ptr<i8> {
    let mut slash: Ptr<i8> = {
        let __s = (path).clone();
        let __t = (('/' as i8) as i32) as i8;
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
    return if !(slash).is_null() {
        slash.offset((1) as isize)
    } else {
        path
    };
}
pub fn log_1(mut file: Ptr<i8>, mut line: i32, mut func: Ptr<i8>) {
    println!("{} {} {}", file, line, func);
}
pub fn line_2() -> i32 {
    return (line!() as u32 as i32);
}
pub fn function_3() -> Ptr<i8> {
    return Ptr::<i8>::from_string_literal(b"function");
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    println!(
        "{} {} {}",
        Ptr::<i8>::from_string_literal(b"macros.cpp"),
        19,
        Ptr::<i8>::from_string_literal(b"main")
    );
    ({
        log_1(
            Ptr::<i8>::from_string_literal(b"macros.cpp"),
            20,
            Ptr::<i8>::from_string_literal(b"main"),
        )
    });
    assert!((line!() as u32 > 0_u32));
    assert!(
        (((elem!(
            ({
                base_name_0(Ptr::<i8>::from_string_literal(
                    concat!(file!(), "\0").as_bytes(),
                ))
            }),
            0
        )
        .read()) as i32)
            != (('\0' as i8) as i32))
    );
    assert!((({ line_2() }) > 0));
    println!("{}", ({ function_3() }));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
