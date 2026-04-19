extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::Seek;
use std::io::{Read, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_promotions_0(mut count: i32, args: &[VaArg]) -> i32 {
    let mut ap: VaList = <VaList>::default();
    ap = VaList::new(args);
    let mut a: i32 = ap.arg::<i32>();
    let mut b: i32 = ap.arg::<i32>();
    let mut c: f64 = ap.arg::<f64>();
    assert!(((a) == (65)));
    assert!(((b) == (10)));
    assert!(((c) == (3.0E+0)));
    return (((a) + (b)) + (c as i32));
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut x: u8 = (('A' as i32) as u8);
    let mut y: i16 = 10_i16;
    let mut z: f32 = 3.0E+0;
    assert!(
        ((unsafe {
            let _count: i32 = 3;
            test_promotions_0(
                _count,
                &[(x as i32).into(), (y as i32).into(), (z as f64).into()],
            )
        }) == (78))
    );
    return 0;
}
