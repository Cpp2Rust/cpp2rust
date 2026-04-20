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
impl ByteRepr for Complex {}
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
            (0..((*N.borrow()) as u64))
                .map(|_| <Complex>::default())
                .collect::<Box<[_]>>(),
        )))));
    if ((*N.borrow()) == 1) {
        let __rhs = Complex {
            re: Rc::new(RefCell::new(
                (*(*a.upgrade().deref()).as_ref().unwrap().borrow()[(0_u64) as usize]
                    .re
                    .borrow()),
            )),
            img: Rc::new(RefCell::new(
                (*(*a.upgrade().deref()).as_ref().unwrap().borrow()[(0_u64) as usize]
                    .img
                    .borrow()),
            )),
        };
        (*y.borrow()).as_ref().unwrap().borrow_mut()[(0_u64) as usize] = __rhs;
        return (*y.borrow_mut()).take();
    }
    let w: Value<Option<Value<Box<[Complex]>>>> =
        Rc::new(RefCell::new(Some(Rc::new(RefCell::new(
            (0..((*N.borrow()) as u64))
                .map(|_| <Complex>::default())
                .collect::<Box<[_]>>(),
        )))));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < (*N.borrow())) {
        let alpha: Value<f64> = Rc::new(RefCell::new(
            ((((-2_i32 as f64) * 3.141592654E+0) * ((*i.borrow()) as f64))
                / ((*N.borrow()) as f64)),
        ));
        (*w.borrow()).as_ref().unwrap().borrow_mut()[((*i.borrow()) as u64) as usize] = Complex {
            re: Rc::new(RefCell::new((*alpha.borrow()).cos())),
            img: Rc::new(RefCell::new((*alpha.borrow()).sin())),
        };
        (*i.borrow_mut()).postfix_inc();
    }
    let A0: Value<Option<Value<Box<[Complex]>>>> =
        Rc::new(RefCell::new(Some(Rc::new(RefCell::new(
            (0..(((*N.borrow()) / 2) as u64))
                .map(|_| <Complex>::default())
                .collect::<Box<[_]>>(),
        )))));
    let A1: Value<Option<Value<Box<[Complex]>>>> =
        Rc::new(RefCell::new(Some(Rc::new(RefCell::new(
            (0..(((*N.borrow()) / 2) as u64))
                .map(|_| <Complex>::default())
                .collect::<Box<[_]>>(),
        )))));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < ((*N.borrow()) / 2)) {
        let __rhs = Complex {
            re: Rc::new(RefCell::new(
                (*(*a.upgrade().deref()).as_ref().unwrap().borrow()
                    [(((*i.borrow()) * 2) as u64) as usize]
                    .re
                    .borrow()),
            )),
            img: Rc::new(RefCell::new(
                (*(*a.upgrade().deref()).as_ref().unwrap().borrow()
                    [(((*i.borrow()) * 2) as u64) as usize]
                    .img
                    .borrow()),
            )),
        };
        (*A0.borrow()).as_ref().unwrap().borrow_mut()[((*i.borrow()) as u64) as usize] = __rhs;
        let __rhs = Complex {
            re: Rc::new(RefCell::new(
                (*(*a.upgrade().deref()).as_ref().unwrap().borrow()
                    [((((*i.borrow()) * 2) + 1) as u64) as usize]
                    .re
                    .borrow()),
            )),
            img: Rc::new(RefCell::new(
                (*(*a.upgrade().deref()).as_ref().unwrap().borrow()
                    [((((*i.borrow()) * 2) + 1) as u64) as usize]
                    .img
                    .borrow()),
            )),
        };
        (*A1.borrow()).as_ref().unwrap().borrow_mut()[((*i.borrow()) as u64) as usize] = __rhs;
        (*i.borrow_mut()).postfix_inc();
    }
    let y0: Value<Option<Value<Box<[Complex]>>>> = Rc::new(RefCell::new(
        ({
            let _a: Ptr<Option<Value<Box<[Complex]>>>> = A0.as_pointer();
            let _N: i32 = ((*N.borrow()) / 2);
            fft_3(_a, _N)
        }),
    ));
    let y1: Value<Option<Value<Box<[Complex]>>>> = Rc::new(RefCell::new(
        ({
            let _a: Ptr<Option<Value<Box<[Complex]>>>> = A1.as_pointer();
            let _N: i32 = ((*N.borrow()) / 2);
            fft_3(_a, _N)
        }),
    ));
    let k: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*k.borrow()) < ((*N.borrow()) / 2)) {
        let yk: Value<Complex> = Rc::new(RefCell::new(
            ({
                let _z1: Complex = ((*y0.borrow()).as_ref().unwrap().borrow()
                    [((*k.borrow()) as u64) as usize])
                    .clone();
                let _z2: Complex = ({
                    let _z1: Complex = ((*w.borrow()).as_ref().unwrap().borrow()
                        [((*k.borrow()) as u64) as usize])
                        .clone();
                    let _z2: Complex = ((*y1.borrow()).as_ref().unwrap().borrow()
                        [((*k.borrow()) as u64) as usize])
                        .clone();
                    Product_0(_z1, _z2)
                });
                Sum_1(_z1, _z2)
            }),
        ));
        (*y.borrow()).as_ref().unwrap().borrow_mut()[((*k.borrow()) as u64) as usize] = Complex {
            re: Rc::new(RefCell::new((*(*yk.borrow()).re.borrow()))),
            img: Rc::new(RefCell::new((*(*yk.borrow()).img.borrow()))),
        };
        let yk_n2: Value<Complex> = Rc::new(RefCell::new(
            ({
                let _z1: Complex = ((*y0.borrow()).as_ref().unwrap().borrow()
                    [((*k.borrow()) as u64) as usize])
                    .clone();
                let _z2: Complex = ({
                    let _z1: Complex = ({
                        let _z1: Complex = ((*w.borrow()).as_ref().unwrap().borrow()
                            [((*k.borrow()) as u64) as usize])
                            .clone();
                        let _z2: Complex = ((*y1.borrow()).as_ref().unwrap().borrow()
                            [((*k.borrow()) as u64) as usize])
                            .clone();
                        Product_0(_z1, _z2)
                    });
                    Neg_2(_z1)
                });
                Sum_1(_z1, _z2)
            }),
        ));
        (*y.borrow()).as_ref().unwrap().borrow_mut()
            [(((*k.borrow()) + ((*N.borrow()) / 2)) as u64) as usize] = Complex {
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
            (0..((*N.borrow()) as u64))
                .map(|_| <Complex>::default())
                .collect::<Box<[_]>>(),
        )))));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < (*N.borrow())) {
        let __rhs = Complex {
            re: Rc::new(RefCell::new((((*i.borrow()) as f64) + 1_f64))),
            img: Rc::new(RefCell::new(0_f64)),
        };
        (*a.borrow()).as_ref().unwrap().borrow_mut()[((*i.borrow()) as u64) as usize] = __rhs;
        (*i.borrow_mut()).postfix_inc();
    }
    let b: Value<Option<Value<Box<[Complex]>>>> = Rc::new(RefCell::new(
        ({
            let _a: Ptr<Option<Value<Box<[Complex]>>>> = a.as_pointer();
            let _N: i32 = (*N.borrow());
            fft_3(_a, _N)
        }),
    ));
    let reals: Value<Option<Value<Box<[i32]>>>> =
        Rc::new(RefCell::new(Some(Rc::new(RefCell::new(
            (0..((*N.borrow()) as u64))
                .map(|_| <i32>::default())
                .collect::<Box<[_]>>(),
        )))));
    let imgs: Value<Option<Value<Box<[i32]>>>> =
        Rc::new(RefCell::new(Some(Rc::new(RefCell::new(
            (0..((*N.borrow()) as u64))
                .map(|_| <i32>::default())
                .collect::<Box<[_]>>(),
        )))));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < (*N.borrow())) {
        let __rhs = ((*(*b.borrow()).as_ref().unwrap().borrow()[((*i.borrow()) as u64) as usize]
            .re
            .borrow())
        .round() as i32);
        (*reals.borrow()).as_ref().unwrap().borrow_mut()[((*i.borrow()) as u64) as usize] = __rhs;
        let __rhs = ((*(*b.borrow()).as_ref().unwrap().borrow()[((*i.borrow()) as u64) as usize]
            .img
            .borrow())
        .round() as i32);
        (*imgs.borrow()).as_ref().unwrap().borrow_mut()[((*i.borrow()) as u64) as usize] = __rhs;
        (*i.borrow_mut()).prefix_inc();
    }
    return ((((((((*reals.borrow()).as_ref().unwrap().borrow()[(0_u64) as usize] == 10)
        && ((*imgs.borrow()).as_ref().unwrap().borrow()[(0_u64) as usize] == 0))
        as bool)
        && ((((*reals.borrow()).as_ref().unwrap().borrow()[(1_u64) as usize] == -2_i32)
            && ((*imgs.borrow()).as_ref().unwrap().borrow()[(1_u64) as usize] == 2))
            as bool))
        && ((((*reals.borrow()).as_ref().unwrap().borrow()[(2_u64) as usize] == -2_i32)
            && ((*imgs.borrow()).as_ref().unwrap().borrow()[(2_u64) as usize] == 0))
            as bool))
        && ((((*reals.borrow()).as_ref().unwrap().borrow()[(3_u64) as usize] == -2_i32)
            && ((*imgs.borrow()).as_ref().unwrap().borrow()[(3_u64) as usize] == -2_i32))
            as bool)) as i32);
}
