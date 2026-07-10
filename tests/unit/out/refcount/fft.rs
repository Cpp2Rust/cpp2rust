extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct Complex {
    pub re: Value<f64>,
    pub img: Value<f64>,
}
impl Clone for Complex {
    fn clone(&self) -> Self {
        let mut this = Self {
            re: Rc::new(RefCell::new((*self.re.borrow()))),
            img: Rc::new(RefCell::new((*self.img.borrow()))),
        };
        this
    }
}
impl ByteRepr for Complex {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.re.borrow()).to_bytes(&mut buf[0..8]);
        (*self.img.borrow()).to_bytes(&mut buf[8..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            re: Rc::new(RefCell::new(<f64>::from_bytes(&buf[0..8]))),
            img: Rc::new(RefCell::new(<f64>::from_bytes(&buf[8..16]))),
        }
    }
}
pub fn Product_0(z1: Complex, z2: Complex) -> Complex {
    let z1: Value<Complex> = Rc::new(RefCell::new(z1));
    let z2: Value<Complex> = Rc::new(RefCell::new(z2));
    let ac: Value<f64> = Rc::new(RefCell::new(
        ((*(*z1.borrow()).re.borrow()) * (*(*z2.borrow()).re.borrow())),
    ));
    let bd: Value<f64> = Rc::new(RefCell::new(
        ((*(*z1.borrow()).img.borrow()) * (*(*z2.borrow()).img.borrow())),
    ));
    let ad: Value<f64> = Rc::new(RefCell::new(
        ((*(*z1.borrow()).re.borrow()) * (*(*z2.borrow()).img.borrow())),
    ));
    let bc: Value<f64> = Rc::new(RefCell::new(
        ((*(*z1.borrow()).img.borrow()) * (*(*z2.borrow()).re.borrow())),
    ));
    return Complex {
        re: Rc::new(RefCell::new(((*ac.borrow()) - (*bd.borrow())))),
        img: Rc::new(RefCell::new(((*ad.borrow()) + (*bc.borrow())))),
    };
}
pub fn Sum_1(z1: Complex, z2: Complex) -> Complex {
    let z1: Value<Complex> = Rc::new(RefCell::new(z1));
    let z2: Value<Complex> = Rc::new(RefCell::new(z2));
    let ac: Value<f64> = Rc::new(RefCell::new(
        ((*(*z1.borrow()).re.borrow()) + (*(*z2.borrow()).re.borrow())),
    ));
    let bd: Value<f64> = Rc::new(RefCell::new(
        ((*(*z1.borrow()).img.borrow()) + (*(*z2.borrow()).img.borrow())),
    ));
    return Complex {
        re: Rc::new(RefCell::new((*ac.borrow()))),
        img: Rc::new(RefCell::new((*bd.borrow()))),
    };
}
pub fn Neg_2(z1: Complex) -> Complex {
    let z1: Value<Complex> = Rc::new(RefCell::new(z1));
    return Complex {
        re: Rc::new(RefCell::new(-(*(*z1.borrow()).re.borrow()))),
        img: Rc::new(RefCell::new(-(*(*z1.borrow()).img.borrow()))),
    };
}
pub fn fft_3(a: Ptr<Option<Value<Box<[Complex]>>>>, N: i32) -> Option<Value<Box<[Complex]>>> {
    let N: Value<i32> = Rc::new(RefCell::new(N));
    let y: Value<Option<Value<Box<[Complex]>>>> =
        Rc::new(RefCell::new(Some(Rc::new(RefCell::new(
            (0..((*N.borrow()) as usize))
                .map(|_| <Complex>::default())
                .collect::<Box<[_]>>(),
        )))));
    if ((*N.borrow()) == 1) {
        let __rhs = Complex {
            re: Rc::new(RefCell::new(
                (*(*a.upgrade().deref()).as_ref().unwrap().borrow()[(0_usize) as usize]
                    .re
                    .borrow()),
            )),
            img: Rc::new(RefCell::new(
                (*(*a.upgrade().deref()).as_ref().unwrap().borrow()[(0_usize) as usize]
                    .img
                    .borrow()),
            )),
        };
        (*y.borrow()).as_ref().unwrap().borrow_mut()[(0_usize) as usize] = __rhs;
        return (*y.borrow_mut()).take();
    }
    let w: Value<Option<Value<Box<[Complex]>>>> =
        Rc::new(RefCell::new(Some(Rc::new(RefCell::new(
            (0..((*N.borrow()) as usize))
                .map(|_| <Complex>::default())
                .collect::<Box<[_]>>(),
        )))));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < (*N.borrow())) {
        let alpha: Value<f64> = Rc::new(RefCell::new(
            ((((-2_i32 as f64) * 3.141592654E+0) * ((*i.borrow()) as f64))
                / ((*N.borrow()) as f64)),
        ));
        (*w.borrow()).as_ref().unwrap().borrow_mut()[((*i.borrow()) as usize) as usize] = Complex {
            re: Rc::new(RefCell::new((*alpha.borrow()).cos())),
            img: Rc::new(RefCell::new((*alpha.borrow()).sin())),
        };
        (*i.borrow_mut()).postfix_inc();
    }
    let A0: Value<Option<Value<Box<[Complex]>>>> =
        Rc::new(RefCell::new(Some(Rc::new(RefCell::new(
            (0..(((*N.borrow()) / 2) as usize))
                .map(|_| <Complex>::default())
                .collect::<Box<[_]>>(),
        )))));
    let A1: Value<Option<Value<Box<[Complex]>>>> =
        Rc::new(RefCell::new(Some(Rc::new(RefCell::new(
            (0..(((*N.borrow()) / 2) as usize))
                .map(|_| <Complex>::default())
                .collect::<Box<[_]>>(),
        )))));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < ((*N.borrow()) / 2)) {
        let __rhs = Complex {
            re: Rc::new(RefCell::new(
                (*(*a.upgrade().deref()).as_ref().unwrap().borrow()
                    [(((*i.borrow()) * 2) as usize) as usize]
                    .re
                    .borrow()),
            )),
            img: Rc::new(RefCell::new(
                (*(*a.upgrade().deref()).as_ref().unwrap().borrow()
                    [(((*i.borrow()) * 2) as usize) as usize]
                    .img
                    .borrow()),
            )),
        };
        (*A0.borrow()).as_ref().unwrap().borrow_mut()[((*i.borrow()) as usize) as usize] = __rhs;
        let __rhs = Complex {
            re: Rc::new(RefCell::new(
                (*(*a.upgrade().deref()).as_ref().unwrap().borrow()
                    [((((*i.borrow()) * 2) + 1) as usize) as usize]
                    .re
                    .borrow()),
            )),
            img: Rc::new(RefCell::new(
                (*(*a.upgrade().deref()).as_ref().unwrap().borrow()
                    [((((*i.borrow()) * 2) + 1) as usize) as usize]
                    .img
                    .borrow()),
            )),
        };
        (*A1.borrow()).as_ref().unwrap().borrow_mut()[((*i.borrow()) as usize) as usize] = __rhs;
        (*i.borrow_mut()).postfix_inc();
    }
    let y0: Value<Option<Value<Box<[Complex]>>>> = Rc::new(RefCell::new(
        ({ fft_3(A0.as_pointer(), ((*N.borrow()) / 2)) }),
    ));
    let y1: Value<Option<Value<Box<[Complex]>>>> = Rc::new(RefCell::new(
        ({ fft_3(A1.as_pointer(), ((*N.borrow()) / 2)) }),
    ));
    let k: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*k.borrow()) < ((*N.borrow()) / 2)) {
        let yk: Value<Complex> = Rc::new(RefCell::new(
            ({
                let _z1: Complex = ((*y0.borrow()).as_ref().unwrap().borrow()
                    [((*k.borrow()) as usize) as usize])
                    .clone();
                let _z2: Complex = ({
                    let _z1: Complex = ((*w.borrow()).as_ref().unwrap().borrow()
                        [((*k.borrow()) as usize) as usize])
                        .clone();
                    let _z2: Complex = ((*y1.borrow()).as_ref().unwrap().borrow()
                        [((*k.borrow()) as usize) as usize])
                        .clone();
                    Product_0(_z1, _z2)
                });
                Sum_1(_z1, _z2)
            }),
        ));
        (*y.borrow()).as_ref().unwrap().borrow_mut()[((*k.borrow()) as usize) as usize] = Complex {
            re: Rc::new(RefCell::new((*(*yk.borrow()).re.borrow()))),
            img: Rc::new(RefCell::new((*(*yk.borrow()).img.borrow()))),
        };
        let yk_n2: Value<Complex> = Rc::new(RefCell::new(
            ({
                let _z1: Complex = ((*y0.borrow()).as_ref().unwrap().borrow()
                    [((*k.borrow()) as usize) as usize])
                    .clone();
                let _z2: Complex = ({
                    Neg_2(
                        ({
                            let _z1: Complex = ((*w.borrow()).as_ref().unwrap().borrow()
                                [((*k.borrow()) as usize) as usize])
                                .clone();
                            let _z2: Complex = ((*y1.borrow()).as_ref().unwrap().borrow()
                                [((*k.borrow()) as usize) as usize])
                                .clone();
                            Product_0(_z1, _z2)
                        }),
                    )
                });
                Sum_1(_z1, _z2)
            }),
        ));
        (*y.borrow()).as_ref().unwrap().borrow_mut()
            [(((*k.borrow()) + ((*N.borrow()) / 2)) as usize) as usize] = Complex {
            re: Rc::new(RefCell::new((*(*yk_n2.borrow()).re.borrow()))),
            img: Rc::new(RefCell::new((*(*yk_n2.borrow()).img.borrow()))),
        };
        (*k.borrow_mut()).postfix_inc();
    }
    return (*y.borrow_mut()).take();
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let N: Value<i32> = Rc::new(RefCell::new(4));
    let a: Value<Option<Value<Box<[Complex]>>>> =
        Rc::new(RefCell::new(Some(Rc::new(RefCell::new(
            (0..((*N.borrow()) as usize))
                .map(|_| <Complex>::default())
                .collect::<Box<[_]>>(),
        )))));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < (*N.borrow())) {
        let __rhs = Complex {
            re: Rc::new(RefCell::new((((*i.borrow()) as f64) + 1_f64))),
            img: Rc::new(RefCell::new(0_f64)),
        };
        (*a.borrow()).as_ref().unwrap().borrow_mut()[((*i.borrow()) as usize) as usize] = __rhs;
        (*i.borrow_mut()).postfix_inc();
    }
    let b: Value<Option<Value<Box<[Complex]>>>> =
        Rc::new(RefCell::new(({ fft_3(a.as_pointer(), (*N.borrow())) })));
    let reals: Value<Option<Value<Box<[i32]>>>> =
        Rc::new(RefCell::new(Some(Rc::new(RefCell::new(
            (0..((*N.borrow()) as usize))
                .map(|_| <i32>::default())
                .collect::<Box<[_]>>(),
        )))));
    let imgs: Value<Option<Value<Box<[i32]>>>> =
        Rc::new(RefCell::new(Some(Rc::new(RefCell::new(
            (0..((*N.borrow()) as usize))
                .map(|_| <i32>::default())
                .collect::<Box<[_]>>(),
        )))));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < (*N.borrow())) {
        let __rhs = ((*(*b.borrow()).as_ref().unwrap().borrow()[((*i.borrow()) as usize) as usize]
            .re
            .borrow())
        .round() as i32);
        (*reals.borrow()).as_ref().unwrap().borrow_mut()[((*i.borrow()) as usize) as usize] = __rhs;
        let __rhs = ((*(*b.borrow()).as_ref().unwrap().borrow()[((*i.borrow()) as usize) as usize]
            .img
            .borrow())
        .round() as i32);
        (*imgs.borrow()).as_ref().unwrap().borrow_mut()[((*i.borrow()) as usize) as usize] = __rhs;
        (*i.borrow_mut()).prefix_inc();
    }
    return (((((((*reals.borrow()).as_ref().unwrap().borrow()[(0_usize) as usize] == 10)
        && ((*imgs.borrow()).as_ref().unwrap().borrow()[(0_usize) as usize] == 0))
        && (((*reals.borrow()).as_ref().unwrap().borrow()[(1_usize) as usize] == -2_i32)
            && ((*imgs.borrow()).as_ref().unwrap().borrow()[(1_usize) as usize] == 2)))
        && (((*reals.borrow()).as_ref().unwrap().borrow()[(2_usize) as usize] == -2_i32)
            && ((*imgs.borrow()).as_ref().unwrap().borrow()[(2_usize) as usize] == 0)))
        && (((*reals.borrow()).as_ref().unwrap().borrow()[(3_usize) as usize] == -2_i32)
            && ((*imgs.borrow()).as_ref().unwrap().borrow()[(3_usize) as usize] == -2_i32)))
        as i32);
}
