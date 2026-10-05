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
pub fn Product_0(z1: Complex, z2: Complex) -> Complex {
    let z1: Value<Complex> = Rc::new(RefCell::new(z1));
    let z2: Value<Complex> = Rc::new(RefCell::new(z2));
    let mut ac: f64 = ({ (*z1.borrow()).re } * { (*z2.borrow()).re });
    let mut bd: f64 = ({ (*z1.borrow()).img } * { (*z2.borrow()).img });
    let mut ad: f64 = ({ (*z1.borrow()).re } * { (*z2.borrow()).img });
    let mut bc: f64 = ({ (*z1.borrow()).img } * { (*z2.borrow()).re });
    return Complex {
        re: (ac - bd),
        img: (ad + bc),
    };
}
pub fn Sum_1(z1: Complex, z2: Complex) -> Complex {
    let z1: Value<Complex> = Rc::new(RefCell::new(z1));
    let z2: Value<Complex> = Rc::new(RefCell::new(z2));
    let ac: Value<f64> = Rc::new(RefCell::new(
        ({ (*z1.borrow()).re } + { (*z2.borrow()).re }),
    ));
    let bd: Value<f64> = Rc::new(RefCell::new(
        ({ (*z1.borrow()).img } + { (*z2.borrow()).img }),
    ));
    return Complex {
        re: (*ac.borrow()),
        img: (*bd.borrow()),
    };
}
pub fn Neg_2(z1: Complex) -> Complex {
    let z1: Value<Complex> = Rc::new(RefCell::new(z1));
    return Complex {
        re: -{ (*z1.borrow()).re },
        img: -{ (*z1.borrow()).img },
    };
}
pub fn fft_3(a: Ptr<Option<Value<Box<[Complex]>>>>, mut N: i32) -> Option<Value<Box<[Complex]>>> {
    let y: Value<Option<Value<Box<[Complex]>>>> =
        Rc::new(RefCell::new(Some(Rc::new(RefCell::new(
            (0..(N as usize))
                .map(|_| <Complex>::default())
                .collect::<Box<[_]>>(),
        )))));
    if (N == 1) {
        let __rhs = Complex {
            re: { (*a.upgrade().deref()).as_ref().unwrap().borrow()[(0_usize) as usize].re },
            img: { (*a.upgrade().deref()).as_ref().unwrap().borrow()[(0_usize) as usize].img },
        };
        (*y.borrow()).as_ref().unwrap().borrow_mut()[(0_usize) as usize] = __rhs;
        return (*y.borrow_mut()).take();
    }
    let w: Value<Option<Value<Box<[Complex]>>>> =
        Rc::new(RefCell::new(Some(Rc::new(RefCell::new(
            (0..(N as usize))
                .map(|_| <Complex>::default())
                .collect::<Box<[_]>>(),
        )))));
    let mut i: i32 = 0;
    'loop_: while (i < N) {
        let mut alpha: f64 = ((((-2_i32 as f64) * 3.141592654E+0) * (i as f64)) / (N as f64));
        let __rhs = Complex {
            re: alpha.cos(),
            img: alpha.sin(),
        };
        (*w.borrow()).as_ref().unwrap().borrow_mut()[(i as usize) as usize] = __rhs;
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
    let y0: Value<Option<Value<Box<[Complex]>>>> =
        Rc::new(RefCell::new(({ fft_3(A0.as_pointer(), (N / 2)) })));
    let y1: Value<Option<Value<Box<[Complex]>>>> =
        Rc::new(RefCell::new(({ fft_3(A1.as_pointer(), (N / 2)) })));
    let mut k: i32 = 0;
    'loop_: while (k < (N / 2)) {
        let yk: Value<Complex> = Rc::new(RefCell::new(
            ({
                let _z1: Complex =
                    ((*y0.borrow()).as_ref().unwrap().borrow()[(k as usize) as usize]).clone();
                let _z2: Complex = ({
                    let _z1: Complex =
                        ((*w.borrow()).as_ref().unwrap().borrow()[(k as usize) as usize]).clone();
                    let _z2: Complex =
                        ((*y1.borrow()).as_ref().unwrap().borrow()[(k as usize) as usize]).clone();
                    Product_0(_z1, _z2)
                });
                Sum_1(_z1, _z2)
            }),
        ));
        (*y.borrow()).as_ref().unwrap().borrow_mut()[(k as usize) as usize] = Complex {
            re: { (*yk.borrow()).re },
            img: { (*yk.borrow()).img },
        };
        let yk_n2: Value<Complex> = Rc::new(RefCell::new(
            ({
                let _z1: Complex =
                    ((*y0.borrow()).as_ref().unwrap().borrow()[(k as usize) as usize]).clone();
                let _z2: Complex = ({
                    Neg_2(
                        ({
                            let _z1: Complex = ((*w.borrow()).as_ref().unwrap().borrow()
                                [(k as usize) as usize])
                                .clone();
                            let _z2: Complex = ((*y1.borrow()).as_ref().unwrap().borrow()
                                [(k as usize) as usize])
                                .clone();
                            Product_0(_z1, _z2)
                        }),
                    )
                });
                Sum_1(_z1, _z2)
            }),
        ));
        (*y.borrow()).as_ref().unwrap().borrow_mut()[((k + (N / 2)) as usize) as usize] = Complex {
            re: { (*yk_n2.borrow()).re },
            img: { (*yk_n2.borrow()).img },
        };
        k.postfix_inc();
    }
    return (*y.borrow_mut()).take();
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
    let b: Value<Option<Value<Box<[Complex]>>>> =
        Rc::new(RefCell::new(({ fft_3(a.as_pointer(), N) })));
    let reals: Value<Option<Value<Box<[i32]>>>> =
        Rc::new(RefCell::new(Some(Rc::new(RefCell::new(
            (0..(N as usize))
                .map(|_| <i32>::default())
                .collect::<Box<[_]>>(),
        )))));
    let imgs: Value<Option<Value<Box<[i32]>>>> =
        Rc::new(RefCell::new(Some(Rc::new(RefCell::new(
            (0..(N as usize))
                .map(|_| <i32>::default())
                .collect::<Box<[_]>>(),
        )))));
    let mut i: i32 = 0;
    'loop_: while (i < N) {
        let __rhs =
            ({ (*b.borrow()).as_ref().unwrap().borrow()[(i as usize) as usize].re }.round() as i32);
        (*reals.borrow()).as_ref().unwrap().borrow_mut()[(i as usize) as usize] = __rhs;
        let __rhs = ({ (*b.borrow()).as_ref().unwrap().borrow()[(i as usize) as usize].img }.round()
            as i32);
        (*imgs.borrow()).as_ref().unwrap().borrow_mut()[(i as usize) as usize] = __rhs;
        i.prefix_inc();
    }
    assert!(
        ((((((((*reals.borrow()).as_ref().unwrap().borrow()[(0_usize) as usize] == 10)
            && ((*imgs.borrow()).as_ref().unwrap().borrow()[(0_usize) as usize] == 0))
            && ((*reals.borrow()).as_ref().unwrap().borrow()[(1_usize) as usize] == -2_i32))
            && ((*imgs.borrow()).as_ref().unwrap().borrow()[(1_usize) as usize] == 2))
            && ((*reals.borrow()).as_ref().unwrap().borrow()[(2_usize) as usize] == -2_i32))
            && ((*imgs.borrow()).as_ref().unwrap().borrow()[(2_usize) as usize] == 0))
            && ((*reals.borrow()).as_ref().unwrap().borrow()[(3_usize) as usize] == -2_i32))
            && ((*imgs.borrow()).as_ref().unwrap().borrow()[(3_usize) as usize] == -2_i32)
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
