extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn to_int_0(mut c: i8) -> i32 {
    return (c as i32);
}
pub fn is_negative_1(mut s: Ptr<i8>) -> bool {
    return (((elem!(s, 0).read()) as i32) < 0);
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut c: i8 = -56_i8;
    let mut widened: i32 = (c as i32);
    println!("{} {}", widened, ({ to_int_0((b'\xff' as i8),) }));
    println!("{}", (((c as i32) < 0) as i32));
    let lit: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"\xe9t\xe9\0")));
    let mut ulit: [u8; 4] = b"\xe9t\xe9\0".map(u8::from_byte);
    println!(
        "{} {}",
        ((*lit.borrow())[(0) as usize] as i32),
        (ulit[(0) as usize] as i32)
    );
    println!(
        "{}",
        (({ is_negative_1((lit.as_pointer() as Ptr::<i8>),) }) as i32)
    );
    let mut p: Ptr<i8> = Ptr::<i8>::from_string_literal(b"\x80");
    println!(
        "{} {}",
        ((elem!(p, 0).read()) as i32),
        (((elem!(p, 0).read()) as u8) as i32)
    );
    println!(
        "{}",
        (({
            let mut __it1 = Ptr::<i8>::from_string_literal(b"\x80").to_c_string_iterator();
            let mut __it2 = Ptr::<i8>::from_string_literal(b"a").to_c_string_iterator();
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
        } > 0) as i32)
    );
    let s: Value<Vec<i8>> = Rc::new(RefCell::new({
        let mut __bytes = Ptr::<i8>::from_string_literal(b"\xfe!").to_c_bytes();
        __bytes.push(0);
        __bytes
    }));
    println!(
        "{} {}",
        ((elem!((s.as_pointer() as Ptr<i8>), 0_usize).read()) as i32),
        ((elem!((s.as_pointer() as Ptr<i8>), 1_usize).read()) as i32)
    );
    let mut sum: i8 = (((c as i32) + (c as i32)) as i8);
    println!("{}", (sum as i32));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
