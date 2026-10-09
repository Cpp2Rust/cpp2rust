extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct Complex {
    #[offset(0)]
    pub re: f64,
    #[offset(8)]
    pub img: f64,
}
pub fn Product_0(mut z1: Complex, mut z2: Complex) -> Complex {
    let mut ac: f64 = (z1.re * z2.re);
    let mut bd: f64 = (z1.img * z2.img);
    let mut ad: f64 = (z1.re * z2.img);
    let mut bc: f64 = (z1.img * z2.re);
    return Complex {
        re: (ac - bd),
        img: (ad + bc),
    };
}
pub fn Sum_1(mut z1: Complex, mut z2: Complex) -> Complex {
    let mut ac: f64 = (z1.re + z2.re);
    let mut bd: f64 = (z1.img + z2.img);
    return Complex { re: ac, img: bd };
}
pub fn Neg_2(mut z1: Complex) -> Complex {
    return Complex {
        re: -z1.re,
        img: -z1.img,
    };
}
pub fn fft_3(a: Ptr<Option<Value<Box<[Complex]>>>>, mut N: i32) -> Option<Value<Box<[Complex]>>> {
    let mut y: Option<Value<Box<[Complex]>>> = Some(Rc::new(RefCell::new(
        (0..(N as usize))
            .map(|_| <Complex>::default())
            .collect::<Box<[_]>>(),
    )));
    if (N == 1) {
        let __rhs = Complex {
            re: { (*a.upgrade().deref()).as_ref().unwrap().borrow()[(0_usize) as usize].re },
            img: { (*a.upgrade().deref()).as_ref().unwrap().borrow()[(0_usize) as usize].img },
        };
        y.as_ref().unwrap().borrow_mut()[(0_usize) as usize] = __rhs;
        return y.take();
    }
    let mut w: Option<Value<Box<[Complex]>>> = Some(Rc::new(RefCell::new(
        (0..(N as usize))
            .map(|_| <Complex>::default())
            .collect::<Box<[_]>>(),
    )));
    let mut i: i32 = 0;
    'loop_: while (i < N) {
        let mut alpha: f64 =
            ((((-2_i32 as f64) * 3.141592653589793_f64) * (i as f64)) / (N as f64));
        let __rhs = Complex {
            re: alpha.cos(),
            img: alpha.sin(),
        };
        w.as_ref().unwrap().borrow_mut()[(i as usize) as usize] = __rhs;
        i.postfix_inc();
    }
    let A0: Value<Option<Value<Box<[Complex]>>>> =
        Rc::new(RefCell::new(Some(Rc::new(RefCell::new(
            (0..((N / 2) as usize))
                .map(|_| <Complex>::default())
                .collect::<Box<[_]>>(),
        )))));
    let A1: Value<Option<Value<Box<[Complex]>>>> =
        Rc::new(RefCell::new(Some(Rc::new(RefCell::new(
            (0..((N / 2) as usize))
                .map(|_| <Complex>::default())
                .collect::<Box<[_]>>(),
        )))));
    let mut i: i32 = 0;
    'loop_: while (i < (N / 2)) {
        let __rhs = Complex {
            re: {
                (*a.upgrade().deref()).as_ref().unwrap().borrow()[((i * 2) as usize) as usize].re
            },
            img: {
                (*a.upgrade().deref()).as_ref().unwrap().borrow()[((i * 2) as usize) as usize].img
            },
        };
        (*A0.borrow()).as_ref().unwrap().borrow_mut()[(i as usize) as usize] = __rhs;
        let __rhs = Complex {
            re: {
                (*a.upgrade().deref()).as_ref().unwrap().borrow()[(((i * 2) + 1) as usize) as usize]
                    .re
            },
            img: {
                (*a.upgrade().deref()).as_ref().unwrap().borrow()[(((i * 2) + 1) as usize) as usize]
                    .img
            },
        };
        (*A1.borrow()).as_ref().unwrap().borrow_mut()[(i as usize) as usize] = __rhs;
        i.postfix_inc();
    }
    let mut y0: Option<Value<Box<[Complex]>>> = ({ fft_3(A0.as_pointer(), (N / 2)) });
    let mut y1: Option<Value<Box<[Complex]>>> = ({ fft_3(A1.as_pointer(), (N / 2)) });
    let mut k: i32 = 0;
    'loop_: while (k < (N / 2)) {
        let mut yk: Complex = ({
            let _z1: Complex = (y0.as_ref().unwrap().borrow()[(k as usize) as usize]).clone();
            let _z2: Complex = ({
                let _z1: Complex = (w.as_ref().unwrap().borrow()[(k as usize) as usize]).clone();
                let _z2: Complex = (y1.as_ref().unwrap().borrow()[(k as usize) as usize]).clone();
                Product_0(_z1, _z2)
            });
            Sum_1(_z1, _z2)
        });
        y.as_ref().unwrap().borrow_mut()[(k as usize) as usize] = Complex {
            re: yk.re,
            img: yk.img,
        };
        let mut yk_n2: Complex = ({
            let _z1: Complex = (y0.as_ref().unwrap().borrow()[(k as usize) as usize]).clone();
            let _z2: Complex = ({
                Neg_2(
                    ({
                        let _z1: Complex =
                            (w.as_ref().unwrap().borrow()[(k as usize) as usize]).clone();
                        let _z2: Complex =
                            (y1.as_ref().unwrap().borrow()[(k as usize) as usize]).clone();
                        Product_0(_z1, _z2)
                    }),
                )
            });
            Sum_1(_z1, _z2)
        });
        y.as_ref().unwrap().borrow_mut()[((k + (N / 2)) as usize) as usize] = Complex {
            re: yk_n2.re,
            img: yk_n2.img,
        };
        k.postfix_inc();
    }
    return y.take();
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut N: i32 = 4;
    let a: Value<Option<Value<Box<[Complex]>>>> =
        Rc::new(RefCell::new(Some(Rc::new(RefCell::new(
            (0..(N as usize))
                .map(|_| <Complex>::default())
                .collect::<Box<[_]>>(),
        )))));
    let mut i: i32 = 0;
    'loop_: while (i < N) {
        let __rhs = Complex {
            re: ((i as f64) + 1_f64),
            img: 0_f64,
        };
        (*a.borrow()).as_ref().unwrap().borrow_mut()[(i as usize) as usize] = __rhs;
        i.postfix_inc();
    }
    let mut b: Option<Value<Box<[Complex]>>> = ({ fft_3(a.as_pointer(), N) });
    let mut reals: Option<Value<Box<[i32]>>> = Some(Rc::new(RefCell::new(
        (0..(N as usize))
            .map(|_| <i32>::default())
            .collect::<Box<[_]>>(),
    )));
    let mut imgs: Option<Value<Box<[i32]>>> = Some(Rc::new(RefCell::new(
        (0..(N as usize))
            .map(|_| <i32>::default())
            .collect::<Box<[_]>>(),
    )));
    let mut i: i32 = 0;
    'loop_: while (i < N) {
        let __rhs = ({ b.as_ref().unwrap().borrow()[(i as usize) as usize].re }.round() as i32);
        reals.as_ref().unwrap().borrow_mut()[(i as usize) as usize] = __rhs;
        let __rhs = ({ b.as_ref().unwrap().borrow()[(i as usize) as usize].img }.round() as i32);
        imgs.as_ref().unwrap().borrow_mut()[(i as usize) as usize] = __rhs;
        i.prefix_inc();
    }
    assert!(
        (((((((reals.as_ref().unwrap().borrow()[(0_usize) as usize] == 10)
            && (imgs.as_ref().unwrap().borrow()[(0_usize) as usize] == 0))
            && (reals.as_ref().unwrap().borrow()[(1_usize) as usize] == -2_i32))
            && (imgs.as_ref().unwrap().borrow()[(1_usize) as usize] == 2))
            && (reals.as_ref().unwrap().borrow()[(2_usize) as usize] == -2_i32))
            && (imgs.as_ref().unwrap().borrow()[(2_usize) as usize] == 0))
            && (reals.as_ref().unwrap().borrow()[(3_usize) as usize] == -2_i32))
            && (imgs.as_ref().unwrap().borrow()[(3_usize) as usize] == -2_i32)
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
