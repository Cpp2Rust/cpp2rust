extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Complex {
    pub re: f64,
    pub img: f64,
}
pub unsafe fn Product_0(mut z1: Complex, mut z2: Complex) -> Complex {
    let mut ac: f64 = ((z1.re) * (z2.re));
    let mut bd: f64 = ((z1.img) * (z2.img));
    let mut ad: f64 = ((z1.re) * (z2.img));
    let mut bc: f64 = ((z1.img) * (z2.re));
    return Complex {
        re: ((ac) - (bd)),
        img: ((ad) + (bc)),
    };
}
pub unsafe fn Sum_1(mut z1: Complex, mut z2: Complex) -> Complex {
    let mut ac: f64 = ((z1.re) + (z2.re));
    let mut bd: f64 = ((z1.img) + (z2.img));
    return Complex { re: ac, img: bd };
}
pub unsafe fn Neg_2(mut z1: Complex) -> Complex {
    return Complex {
        re: -z1.re,
        img: -z1.img,
    };
}
pub unsafe fn fft_3(a: *mut Option<Box<[Complex]>>, mut N: i32) -> Option<Box<[Complex]>> {
    let mut y: Option<Box<[Complex]>> = Some(
        (0..(N as u64))
            .map(|_| <Complex>::default())
            .collect::<Box<[_]>>(),
    );
    if ((N) == (1)) {
        y.as_mut().unwrap()[(0_u64) as usize] = Complex {
            re: (*a).as_mut().unwrap()[(0_u64) as usize].re,
            img: (*a).as_mut().unwrap()[(0_u64) as usize].img,
        };
        return y;
    }
    let mut w: Option<Box<[Complex]>> = Some(
        (0..(N as u64))
            .map(|_| <Complex>::default())
            .collect::<Box<[_]>>(),
    );
    let mut i: i32 = 0;
    'loop_: while ((i) < (N)) {
        let mut alpha: f64 = ((((-2_i32 as f64) * (3.141592654E+0)) * (i as f64)) / (N as f64));
        w.as_mut().unwrap()[(i as u64) as usize] = Complex {
            re: alpha.cos(),
            img: alpha.sin(),
        };
        i.postfix_inc();
    }
    let mut A0: Option<Box<[Complex]>> = Some(
        (0..(((N) / (2)) as u64))
            .map(|_| <Complex>::default())
            .collect::<Box<[_]>>(),
    );
    let mut A1: Option<Box<[Complex]>> = Some(
        (0..(((N) / (2)) as u64))
            .map(|_| <Complex>::default())
            .collect::<Box<[_]>>(),
    );
    let mut i: i32 = 0;
    'loop_: while ((i) < ((N) / (2))) {
        A0.as_mut().unwrap()[(i as u64) as usize] = Complex {
            re: (*a).as_mut().unwrap()[(((i) * (2)) as u64) as usize].re,
            img: (*a).as_mut().unwrap()[(((i) * (2)) as u64) as usize].img,
        };
        A1.as_mut().unwrap()[(i as u64) as usize] = Complex {
            re: (*a).as_mut().unwrap()[((((i) * (2)) + (1)) as u64) as usize].re,
            img: (*a).as_mut().unwrap()[((((i) * (2)) + (1)) as u64) as usize].img,
        };
        i.postfix_inc();
    }
    let mut y0: Option<Box<[Complex]>> = (unsafe {
        let _a: *mut Option<Box<[Complex]>> = &mut A0 as *mut Option<Box<[Complex]>>;
        let _N: i32 = ((N) / (2));
        fft_3(_a, _N)
    });
    let mut y1: Option<Box<[Complex]>> = (unsafe {
        let _a: *mut Option<Box<[Complex]>> = &mut A1 as *mut Option<Box<[Complex]>>;
        let _N: i32 = ((N) / (2));
        fft_3(_a, _N)
    });
    let mut k: i32 = 0;
    'loop_: while ((k) < ((N) / (2))) {
        let mut yk: Complex = (unsafe {
            let _z1: Complex = y0.as_mut().unwrap()[(k as u64) as usize].clone();
            let _z2: Complex = (unsafe {
                let _z1: Complex = w.as_mut().unwrap()[(k as u64) as usize].clone();
                let _z2: Complex = y1.as_mut().unwrap()[(k as u64) as usize].clone();
                Product_0(_z1, _z2)
            });
            Sum_1(_z1, _z2)
        });
        y.as_mut().unwrap()[(k as u64) as usize] = Complex {
            re: yk.re,
            img: yk.img,
        };
        let mut yk_n2: Complex = (unsafe {
            let _z1: Complex = y0.as_mut().unwrap()[(k as u64) as usize].clone();
            let _z2: Complex = (unsafe {
                let _z1: Complex = (unsafe {
                    let _z1: Complex = w.as_mut().unwrap()[(k as u64) as usize].clone();
                    let _z2: Complex = y1.as_mut().unwrap()[(k as u64) as usize].clone();
                    Product_0(_z1, _z2)
                });
                Neg_2(_z1)
            });
            Sum_1(_z1, _z2)
        });
        y.as_mut().unwrap()[(((k) + ((N) / (2))) as u64) as usize] = Complex {
            re: yk_n2.re,
            img: yk_n2.img,
        };
        k.postfix_inc();
    }
    return y;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut N: i32 = 4;
    let mut a: Option<Box<[Complex]>> = Some(
        (0..(N as u64))
            .map(|_| <Complex>::default())
            .collect::<Box<[_]>>(),
    );
    let mut i: i32 = 0;
    'loop_: while ((i) < (N)) {
        a.as_mut().unwrap()[(i as u64) as usize] = Complex {
            re: ((i as f64) + (1_f64)),
            img: 0_f64,
        };
        i.postfix_inc();
    }
    let mut b: Option<Box<[Complex]>> = (unsafe {
        let _a: *mut Option<Box<[Complex]>> = &mut a as *mut Option<Box<[Complex]>>;
        let _N: i32 = N;
        fft_3(_a, _N)
    });
    let mut reals: Option<Box<[i32]>> = Some(
        (0..(N as u64))
            .map(|_| <i32>::default())
            .collect::<Box<[_]>>(),
    );
    let mut imgs: Option<Box<[i32]>> = Some(
        (0..(N as u64))
            .map(|_| <i32>::default())
            .collect::<Box<[_]>>(),
    );
    let mut i: i32 = 0;
    'loop_: while ((i) < (N)) {
        reals.as_mut().unwrap()[(i as u64) as usize] =
            (b.as_mut().unwrap()[(i as u64) as usize].re.round() as i32);
        imgs.as_mut().unwrap()[(i as u64) as usize] =
            (b.as_mut().unwrap()[(i as u64) as usize].img.round() as i32);
        i.prefix_inc();
    }
    return (((((((reals.as_mut().unwrap()[(0_u64) as usize]) == (10))
        && ((imgs.as_mut().unwrap()[(0_u64) as usize]) == (0)))
        && (((reals.as_mut().unwrap()[(1_u64) as usize]) == (-2_i32))
            && ((imgs.as_mut().unwrap()[(1_u64) as usize]) == (2))))
        && (((reals.as_mut().unwrap()[(2_u64) as usize]) == (-2_i32))
            && ((imgs.as_mut().unwrap()[(2_u64) as usize]) == (0))))
        && (((reals.as_mut().unwrap()[(3_u64) as usize]) == (-2_i32))
            && ((imgs.as_mut().unwrap()[(3_u64) as usize]) == (-2_i32)))) as i32);
}
