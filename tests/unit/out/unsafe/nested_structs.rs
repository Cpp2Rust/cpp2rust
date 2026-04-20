extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[derive(Copy, Clone, Default)]
pub struct Level0_Level1_1_Level2_1_Level3_1 {
    pub x1: i32,
}
#[derive(Copy, Clone, Default)]
pub struct Level0_Level1_1_Level2_1_Level3_2 {
    pub x1: i32,
    pub x2: i32,
}
#[derive(Copy, Clone, Default)]
pub struct Level0_Level1_1_Level2_1 {
    pub x1: i32,
}
#[derive(Copy, Clone, Default)]
pub struct Level0_Level1_1 {
    pub x1: i32,
}
#[derive(Copy, Clone, Default)]
pub struct Level0_Level1_2 {
    pub x1: i32,
    pub x2: i32,
}
#[derive(Copy, Clone, Default)]
pub struct Level0 {}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut x1: Level0_Level1_1 = Level0_Level1_1 { x1: 0 };
    let mut x2: Level0_Level1_2 = Level0_Level1_2 { x1: 1, x2: 2 };
    let mut x3: Level0_Level1_1_Level2_1 = Level0_Level1_1_Level2_1 { x1: 3 };
    let mut x4: Level0_Level1_1_Level2_1_Level3_1 = Level0_Level1_1_Level2_1_Level3_1 { x1: 4 };
    let mut x5: Level0_Level1_1_Level2_1_Level3_2 =
        Level0_Level1_1_Level2_1_Level3_2 { x1: 5, x2: 6 };
    return 0;
}
