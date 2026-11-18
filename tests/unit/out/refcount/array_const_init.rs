extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(20)]
pub struct S {
    #[offset(0)]
    pub head: i32,
    #[offset(4)]
    #[byte_size(12)]
    pub tail: Box<[i32]>,
    #[offset(16)]
    #[byte_size(4)]
    pub buf: Box<[i8]>,
}
impl Default for S {
    fn default() -> Self {
        {
            S {
                head: 0_i32,
                tail: (0..3).map(|_| 0_i32).collect::<Box<[i32]>>(),
                buf: (0..4).map(|_| 0_i8).collect::<Box<[i8]>>(),
            }
        }
    }
}
thread_local!(
    pub static s_0: Value<S> = Rc::new(RefCell::new(S {
        head: 5,
        tail: Box::new([0; 3]),
        buf: Box::new([0; 4]),
    }));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct P {
    #[offset(0)]
    pub x: i32,
    #[offset(4)]
    pub y: i32,
}
thread_local!(
    pub static g_1: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0_i32; 5])));
);
thread_local!(
    pub static k_2: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([
        3, 0, 4, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32,
    ])));
);
thread_local!(
    pub static counter_3: Value<i32> = Rc::new(RefCell::new(0));
);
pub fn next_counter_4() -> i32 {
    return (*(*counter_3.with(Value::clone).borrow_mut()).prefix_inc());
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!(({ ({ (*s_0.with(Value::clone).borrow()).head } == 5) }));
    {
        let mut i: i32 = 0;
        'loop_: while (i < 3) {
            assert!(((array_elem!(s_0.with(|v| v.as_pointer()), tail, i).read()) == 0));
            i.postfix_inc();
        }
    }
    {
        let mut i: i32 = 0;
        'loop_: while (i < 4) {
            assert!((((array_elem!(s_0.with(|v| v.as_pointer()), buf, i).read()) as i32) == 0));
            i.postfix_inc();
        }
    }
    let mut a: [i32; 10] = [0_i32; 10];
    {
        let mut i: i32 = 0;
        'loop_: while (i < 10) {
            assert!((a[(i) as usize] == 0));
            i.postfix_inc();
        }
    }
    let mut seven: [i32; 4] = [7, 0_i32, 0_i32, 0_i32];
    assert!((seven[(0) as usize] == 7));
    {
        let mut i: i32 = 1;
        'loop_: while (i < 4) {
            assert!((seven[(i) as usize] == 0));
            i.postfix_inc();
        }
    }
    let mut b: [i32; 6] = [1, 0, 2, 0_i32, 0_i32, 0_i32];
    assert!((((b[(0) as usize] == 1) && (b[(1) as usize] == 0)) && (b[(2) as usize] == 2)));
    {
        let mut i: i32 = 3;
        'loop_: while (i < 6) {
            assert!((b[(i) as usize] == 0));
            i.postfix_inc();
        }
    }
    let mut d: [f64; 4] = [-0_f64, 0_f64, 0_f64, 0_f64];
    assert!(((d[(0) as usize] == 0_f64) && (d[(3) as usize] == 0_f64)));
    let mut p: [P; 4] = [
        P { x: 1, y: 2 },
        <P>::default(),
        <P>::default(),
        <P>::default(),
    ];
    assert!(((p[(0) as usize].x == 1) && (p[(0) as usize].y == 2)));
    assert!(((p[(3) as usize].x == 0) && (p[(3) as usize].y == 0)));
    let mut c: [i32; 5] = [
        ({ next_counter_4() }),
        ({ next_counter_4() }),
        0_i32,
        0_i32,
        0_i32,
    ];
    assert!((((c[(0) as usize] + c[(1) as usize]) == 3) && (c[(4) as usize] == 0)));
    let mut t: [i32; 2] = [0_i32; 2];
    assert!(((t[(0) as usize] == 0) && (t[(1) as usize] == 0)));
    let mut u: [i32; 3] = [5, 6, 0_i32];
    assert!((((u[(0) as usize] == 5) && (u[(1) as usize] == 6)) && (u[(2) as usize] == 0)));
    assert!(
        (({
            let __idx0 = (4) as usize;
            g_1.with(|rc| rc.borrow()[__idx0])
        }) == 0)
    );
    assert!(
        ((((({
            let __idx0 = (0) as usize;
            k_2.with(|rc| rc.borrow()[__idx0])
        }) == 3)
            && (({
                let __idx0 = (1) as usize;
                k_2.with(|rc| rc.borrow()[__idx0])
            }) == 0))
            && (({
                let __idx0 = (2) as usize;
                k_2.with(|rc| rc.borrow()[__idx0])
            }) == 4))
            && (({
                let __idx0 = (7) as usize;
                k_2.with(|rc| rc.borrow()[__idx0])
            }) == 0))
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = s_0.with(|_| ());
    let _ = g_1.with(|_| ());
    let _ = k_2.with(|_| ());
    let _ = counter_3.with(|_| ());
}
