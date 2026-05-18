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
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    let mut square: i32 = 0;
    'loop_: for e1 in 0..(v.len()) {
        let mut e1 = v[e1].clone();
        'loop_: for e2 in 0..(v.len()) {
            let mut e2 = v[e2].clone();
            square += ((e1) * (e2));
        }
    }
    'loop_: for e1 in 0..(v.len()) {
        let mut e1 = v.as_mut_ptr().add(e1);
        'loop_: for e2 in 0..(v.len()) {
            let mut e2 = v.as_ptr().add(e2);
            square += ((*e1) * (*e2));
        }
    }
    'loop_: for e1 in 0..(v.len()) {
        let mut e1 = v.as_ptr().add(e1);
        'loop_: for e2 in 0..(v.len()) {
            let mut e2 = v.as_mut_ptr().add(e2);
            square += ((*e1) * (*e2));
        }
    }
    'loop_: for e1 in 0..(v.len()) {
        let mut e1 = v.as_mut_ptr().add(e1);
        'loop_: for e2 in 0..(v.len()) {
            let mut e2 = v.as_mut_ptr().add(e2);
            square += ((*e1) * (*e2));
        }
    }
    let mut m: Vec<Vec<i32>> = Vec::new();
    let mut v1: Vec<i32> = Vec::new();
    m.push(v1);
    let mut v2: Vec<i32> = Vec::new();
    m.push(v2);
    let mut v3: Vec<i32> = Vec::new();
    m.push(v3);
    'loop_: for row in 0..(m.len()) {
        let mut row = m.as_mut_ptr().add(row);
        'loop_: for col in 0..((*row).len()) {
            let mut col = (*row).as_mut_ptr().add(col);
            square += (*col);
        }
    }
    return square;
}
