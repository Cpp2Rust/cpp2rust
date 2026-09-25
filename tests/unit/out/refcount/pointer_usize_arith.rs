extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type std_byte = u8;
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let arr: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([10, 11, 12, 13, 14, 15, 16, 17])));
    let p: Value<Ptr<i32>> = Rc::new(RefCell::new((arr.as_pointer() as Ptr<i32>)));
    let q: Value<Ptr<i32>> = Rc::new(RefCell::new((*p.borrow()).offset((1) as isize)));
    (if (((*q.borrow()).read()) == 11) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<u8>::from_string_literal(b"*q == 11"),
                Ptr::<u8>::from_string_literal(b"pointer_usize_arith.cpp"),
                9_u32,
                Ptr::<u8>::from_string_literal(b"int main()"),
            )
        });
    });
    let r: Value<Ptr<i32>> = Rc::new(RefCell::new((*p.borrow()).offset((3) as isize)));
    (if (((*r.borrow()).read()) == 13) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<u8>::from_string_literal(b"*r == 13"),
                Ptr::<u8>::from_string_literal(b"pointer_usize_arith.cpp"),
                12_u32,
                Ptr::<u8>::from_string_literal(b"int main()"),
            )
        });
    });
    let s: Value<Ptr<i32>> = Rc::new(RefCell::new((*r.borrow()).offset(-((2) as isize))));
    (if (((*s.borrow()).read()) == 11) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<u8>::from_string_literal(b"*s == 11"),
                Ptr::<u8>::from_string_literal(b"pointer_usize_arith.cpp"),
                15_u32,
                Ptr::<u8>::from_string_literal(b"int main()"),
            )
        });
    });
    let diff: Value<i64> = Rc::new(RefCell::new(
        ((*r.borrow()).clone() - (*p.borrow()).clone()) as i64,
    ));
    (if ((*diff.borrow()) == 3_i64) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<u8>::from_string_literal(b"diff == 3"),
                Ptr::<u8>::from_string_literal(b"pointer_usize_arith.cpp"),
                18_u32,
                Ptr::<u8>::from_string_literal(b"int main()"),
            )
        });
    });
    let idx: Value<usize> = Rc::new(RefCell::new(
        ((((*r.borrow()).clone() - (*p.borrow()).clone()) as i64) as usize),
    ));
    (if ((*idx.borrow()) == 3_usize) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<u8>::from_string_literal(b"idx == 3"),
                Ptr::<u8>::from_string_literal(b"pointer_usize_arith.cpp"),
                21_u32,
                Ptr::<u8>::from_string_literal(b"int main()"),
            )
        });
    });
    let q2: Value<Ptr<i32>> = Rc::new(RefCell::new((*p.borrow()).clone()));
    (*q2.borrow_mut()).prefix_inc();
    (if (((*q2.borrow()).read()) == 11) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<u8>::from_string_literal(b"*q2 == 11"),
                Ptr::<u8>::from_string_literal(b"pointer_usize_arith.cpp"),
                25_u32,
                Ptr::<u8>::from_string_literal(b"int main()"),
            )
        });
    });
    (*q2.borrow_mut()).postfix_inc();
    (if (((*q2.borrow()).read()) == 12) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<u8>::from_string_literal(b"*q2 == 12"),
                Ptr::<u8>::from_string_literal(b"pointer_usize_arith.cpp"),
                28_u32,
                Ptr::<u8>::from_string_literal(b"int main()"),
            )
        });
    });
    (*q2.borrow_mut()).prefix_dec();
    (if (((*q2.borrow()).read()) == 11) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<u8>::from_string_literal(b"*q2 == 11"),
                Ptr::<u8>::from_string_literal(b"pointer_usize_arith.cpp"),
                31_u32,
                Ptr::<u8>::from_string_literal(b"int main()"),
            )
        });
    });
    (*q2.borrow_mut()).postfix_dec();
    (if (((*q2.borrow()).read()) == 10) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<u8>::from_string_literal(b"*q2 == 10"),
                Ptr::<u8>::from_string_literal(b"pointer_usize_arith.cpp"),
                34_u32,
                Ptr::<u8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if {
        let _lhs = (*q2.borrow()).clone();
        _lhs == (*p.borrow()).clone()
    } {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<u8>::from_string_literal(b"q2 == p"),
                Ptr::<u8>::from_string_literal(b"pointer_usize_arith.cpp"),
                35_u32,
                Ptr::<u8>::from_string_literal(b"int main()"),
            )
        });
    });
    let q3: Value<Ptr<i32>> = Rc::new(RefCell::new((*p.borrow()).clone()));
    (*q3.borrow_mut()) += 4;
    (if (((*q3.borrow()).read()) == 14) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<u8>::from_string_literal(b"*q3 == 14"),
                Ptr::<u8>::from_string_literal(b"pointer_usize_arith.cpp"),
                39_u32,
                Ptr::<u8>::from_string_literal(b"int main()"),
            )
        });
    });
    (*q3.borrow_mut()) -= 2;
    (if (((*q3.borrow()).read()) == 12) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<u8>::from_string_literal(b"*q3 == 12"),
                Ptr::<u8>::from_string_literal(b"pointer_usize_arith.cpp"),
                41_u32,
                Ptr::<u8>::from_string_literal(b"int main()"),
            )
        });
    });
    let step: Value<usize> = Rc::new(RefCell::new(2_usize));
    let q4: Value<Ptr<i32>> = Rc::new(RefCell::new(
        (*p.borrow()).offset((*step.borrow()) as isize),
    ));
    (if (((*q4.borrow()).read()) == 12) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<u8>::from_string_literal(b"*q4 == 12"),
                Ptr::<u8>::from_string_literal(b"pointer_usize_arith.cpp"),
                45_u32,
                Ptr::<u8>::from_string_literal(b"int main()"),
            )
        });
    });
    let v: Value<i32> = Rc::new(RefCell::new(((*p.borrow()).offset((3) as isize).read())));
    (if ((*v.borrow()) == 13) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<u8>::from_string_literal(b"v == 13"),
                Ptr::<u8>::from_string_literal(b"pointer_usize_arith.cpp"),
                48_u32,
                Ptr::<u8>::from_string_literal(b"int main()"),
            )
        });
    });
    let v2: Value<i32> = Rc::new(RefCell::new((((*p.borrow()).offset((4) as isize)).read())));
    (if ((*v2.borrow()) == 14) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<u8>::from_string_literal(b"v2 == 14"),
                Ptr::<u8>::from_string_literal(b"pointer_usize_arith.cpp"),
                51_u32,
                Ptr::<u8>::from_string_literal(b"int main()"),
            )
        });
    });
    ((*p.borrow()).offset((5) as isize)).write(99);
    (if (((*p.borrow()).offset((5) as isize).read()) == 99) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<u8>::from_string_literal(b"p[5] == 99"),
                Ptr::<u8>::from_string_literal(b"pointer_usize_arith.cpp"),
                54_u32,
                Ptr::<u8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ((*arr.borrow())[(5) as usize] == 99) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<u8>::from_string_literal(b"arr[5] == 99"),
                Ptr::<u8>::from_string_literal(b"pointer_usize_arith.cpp"),
                55_u32,
                Ptr::<u8>::from_string_literal(b"int main()"),
            )
        });
    });
    let end: Value<Ptr<i32>> = Rc::new(RefCell::new((*p.borrow()).offset((8) as isize)));
    let sum: Value<i32> = Rc::new(RefCell::new(0));
    let it: Value<Ptr<i32>> = Rc::new(RefCell::new((*p.borrow()).clone()));
    'loop_: while {
        let _lhs = (*it.borrow()).clone();
        _lhs != (*end.borrow()).clone()
    } {
        let __rhs = ((*it.borrow()).read());
        (*sum.borrow_mut()) += __rhs;
        (*it.borrow_mut()).prefix_inc();
    }
    (if ((*sum.borrow()) == (((((((10 + 11) + 12) + 13) + 14) + 99) + 16) + 17)) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<u8>::from_string_literal(b"sum == 10 + 11 + 12 + 13 + 14 + 99 + 16 + 17"),
                Ptr::<u8>::from_string_literal(b"pointer_usize_arith.cpp"),
                62_u32,
                Ptr::<u8>::from_string_literal(b"int main()"),
            )
        });
    });
    let bytes: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([
        0_u8, 1_u8, 2_u8, 3_u8, 4_u8, 5_u8, 6_u8, 7_u8,
    ])));
    let bp: Value<Ptr<u8>> = Rc::new(RefCell::new((bytes.as_pointer() as Ptr<u8>)));
    let bq: Value<Ptr<u8>> = Rc::new(RefCell::new((*bp.borrow()).offset((4) as isize)));
    (if ((((*bq.borrow()).read()) as i32) == 4) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<u8>::from_string_literal(b"*bq == 4"),
                Ptr::<u8>::from_string_literal(b"pointer_usize_arith.cpp"),
                67_u32,
                Ptr::<u8>::from_string_literal(b"int main()"),
            )
        });
    });
    let bdiff: Value<i64> = Rc::new(RefCell::new(
        ((*bq.borrow()).clone() - (*bp.borrow()).clone()) as i64,
    ));
    (if ((*bdiff.borrow()) == 4_i64) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<u8>::from_string_literal(b"bdiff == 4"),
                Ptr::<u8>::from_string_literal(b"pointer_usize_arith.cpp"),
                70_u32,
                Ptr::<u8>::from_string_literal(b"int main()"),
            )
        });
    });
    let cp: Value<Ptr<i32>> = Rc::new(RefCell::new((arr.as_pointer() as Ptr<i32>)));
    let cq: Value<Ptr<i32>> = Rc::new(RefCell::new((*cp.borrow()).offset((2) as isize)));
    (if (((*cq.borrow()).read()) == 12) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<u8>::from_string_literal(b"*cq == 12"),
                Ptr::<u8>::from_string_literal(b"pointer_usize_arith.cpp"),
                74_u32,
                Ptr::<u8>::from_string_literal(b"int main()"),
            )
        });
    });
    let cdiff: Value<i64> = Rc::new(RefCell::new(
        ((*cq.borrow()).clone() - (*cp.borrow()).clone()) as i64,
    ));
    (if ((*cdiff.borrow()) == 2_i64) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<u8>::from_string_literal(b"cdiff == 2"),
                Ptr::<u8>::from_string_literal(b"pointer_usize_arith.cpp"),
                76_u32,
                Ptr::<u8>::from_string_literal(b"int main()"),
            )
        });
    });
    let n: Value<usize> = Rc::new(RefCell::new(3_usize));
    let q5: Value<Ptr<i32>> = Rc::new(RefCell::new(
        (arr.as_pointer() as Ptr<i32>).offset((*n.borrow()) as isize),
    ));
    (if (((*q5.borrow()).read()) == 13) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<u8>::from_string_literal(b"*q5 == 13"),
                Ptr::<u8>::from_string_literal(b"pointer_usize_arith.cpp"),
                80_u32,
                Ptr::<u8>::from_string_literal(b"int main()"),
            )
        });
    });
    let q6: Value<Ptr<i32>> = Rc::new(RefCell::new(
        ((arr.as_pointer() as Ptr<i32>).offset((*n.borrow()))),
    ));
    (if {
        let _lhs = (*q6.borrow()).clone();
        _lhs == (*q5.borrow()).clone()
    } {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<u8>::from_string_literal(b"q6 == q5"),
                Ptr::<u8>::from_string_literal(b"pointer_usize_arith.cpp"),
                83_u32,
                Ptr::<u8>::from_string_literal(b"int main()"),
            )
        });
    });
    let matrix: Value<Box<[Value<Box<[i32]>>]>> = Rc::new(RefCell::new(Box::new([
        Rc::new(RefCell::new(Box::new([0, 1, 2, 3]))),
        Rc::new(RefCell::new(Box::new([4, 5, 6, 7]))),
        Rc::new(RefCell::new(Box::new([8, 9, 10, 11]))),
    ])));
    let row1: Value<Ptr<i32>> = Rc::new(RefCell::new(
        ((((matrix.as_pointer() as Ptr<Value<Box<[i32]>>>)
            .offset(1)
            .read()
            .as_pointer()) as Ptr<i32>)
            .offset(0)),
    ));
    (if (((*row1.borrow()).offset((2) as isize).read()) == 6) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<u8>::from_string_literal(b"row1[2] == 6"),
                Ptr::<u8>::from_string_literal(b"pointer_usize_arith.cpp"),
                87_u32,
                Ptr::<u8>::from_string_literal(b"int main()"),
            )
        });
    });
    let back: Value<Ptr<i32>> = Rc::new(RefCell::new((*end.borrow()).offset(-((1) as isize))));
    (if (((*back.borrow()).read()) == 17) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<u8>::from_string_literal(b"*back == 17"),
                Ptr::<u8>::from_string_literal(b"pointer_usize_arith.cpp"),
                90_u32,
                Ptr::<u8>::from_string_literal(b"int main()"),
            )
        });
    });
    return 0;
}
pub fn __assert_fail_0(__assertion: Ptr<u8>, __file: Ptr<u8>, __line: u32, __function: Ptr<u8>) {
    unimplemented!()
}
pub fn __cpp2rust_init_globals() {}
