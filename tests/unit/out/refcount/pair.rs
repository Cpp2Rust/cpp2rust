extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let p1: Value<(Value<f64>, Value<i32>)> = Rc::new(RefCell::new((
        Rc::new(RefCell::new((1 as f64))),
        Rc::new(RefCell::new(2)),
    )));
    assert!(({ ((*(*p1.borrow()).0.borrow()) == 1_f64) }));
    assert!(({ ((*(*p1.borrow()).1.borrow()) == 2) }));
    let p2: Value<(Value<f64>, Value<i32>)> = Rc::new(RefCell::new((
        Rc::new(RefCell::new((3 as f64))),
        Rc::new(RefCell::new(4)),
    )));
    assert!(({ ((*(*p2.borrow()).0.borrow()) == 3_f64) }));
    assert!(({ ((*(*p2.borrow()).1.borrow()) == 4) }));
    let p3: Value<(Value<f64>, Value<i32>)> = Rc::new(RefCell::new(
        match (Rc::new(RefCell::new((
            Rc::new(RefCell::new(5)),
            Rc::new(RefCell::new(6)),
        )))
        .as_pointer(),)
        {
            (__tmp_0,) => (
                Rc::new(RefCell::new(
                    (((*__tmp_0.upgrade().deref()).0.as_pointer().read()) as f64),
                )),
                Rc::new(RefCell::new(
                    ((*__tmp_0.upgrade().deref()).1.as_pointer().read()),
                )),
            ),
        },
    ));
    assert!(({ ((*(*p3.borrow()).0.borrow()) == 5_f64) }));
    assert!(({ ((*(*p3.borrow()).1.borrow()) == 6) }));
    let arr: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2])));
    let mut n: u32 = 2_u32;
    let const_ptr_pairs: Value<Vec<(Value<Ptr<i32>>, Value<u64>)>> = Rc::new(RefCell::new(
        (0..(1_usize) as usize)
            .map(|_| {
                (
                    Rc::new(RefCell::new(Ptr::<i32>::null())),
                    Rc::new(RefCell::new(0_u64)),
                )
            })
            .collect::<Vec<_>>(),
    ));
    match (Rc::new(RefCell::new((
        Rc::new(RefCell::new(arr.decay().offset((1) as isize))),
        Rc::new(RefCell::new(n)),
    )))
    .as_pointer(),)
    {
        (__tmp_1,) => {
            let __first = <Ptr<i32> as CppFrom<Ptr<i32>>>::cpp_from_move(
                (*__tmp_1.upgrade().deref()).0.as_pointer(),
            );
            let __second = (((*__tmp_1.upgrade().deref()).1.as_pointer().read()) as u64);
            const_ptr_pairs.decay().offset(0_usize).with(|__dst| {
                *__dst.0.borrow_mut() = __first;
                *__dst.1.borrow_mut() = __second;
            })
        }
    };
    assert!(
        ({
            (((*(*const_ptr_pairs.borrow())[0_usize].0.borrow()).read()) == 2)
                && ((*(*const_ptr_pairs.borrow())[0_usize].1.borrow()) == 2_u64)
        })
    );
    let p: Value<(Value<i32>, Value<i32>)> = Rc::new(RefCell::new((
        Rc::new(RefCell::new(1)),
        Rc::new(RefCell::new(2)),
    )));
    let q: Value<(Value<i32>, Value<i32>)> = Rc::new(RefCell::new((
        Rc::new(RefCell::new(3)),
        Rc::new(RefCell::new(4)),
    )));
    let mut f: Ptr<i32> = ((*p.borrow()).0.as_pointer());
    {
        let __first = (*q.borrow()).0.borrow().clone();
        let __second = (*q.borrow()).1.borrow().clone();
        p.as_pointer().with(|__dst| {
            *__dst.0.borrow_mut() = __first;
            *__dst.1.borrow_mut() = __second;
        })
    };
    assert!(((f.read()) == 3));
    let alias: Ptr<(Value<i32>, Value<i32>)> = p.as_pointer();
    {
        let __first = (*alias.upgrade().deref()).0.borrow().clone();
        let __second = (*alias.upgrade().deref()).1.borrow().clone();
        p.as_pointer().with(|__dst| {
            *__dst.0.borrow_mut() = __first;
            *__dst.1.borrow_mut() = __second;
        })
    };
    assert!(((f.read()) == 3));
    let a: Value<(Value<i32>, Value<i32>)> = Rc::new(RefCell::new((
        Rc::new(RefCell::new(1)),
        Rc::new(RefCell::new(2)),
    )));
    let mut pa: Ptr<i32> = ((*a.borrow()).0.as_pointer());
    let b: Value<(Value<i32>, Value<i32>)> = Rc::new(RefCell::new({
        let __first: Ptr<i32> = (*a.borrow()).0.as_pointer();
        let __second: Ptr<i32> = (*a.borrow()).1.as_pointer();
        (
            Rc::new(RefCell::new(cpp_move!(__first))),
            Rc::new(RefCell::new(cpp_move!(__second))),
        )
    }));
    pa.write(9);
    assert!(({ ((*(*b.borrow()).0.borrow()) == 1) }));
    assert!(({ ((*(*a.borrow()).0.borrow()) == 9) }));
    let d: Value<(Value<i32>, Value<i32>)> = Rc::new(RefCell::new((
        Rc::new(RefCell::new(1)),
        Rc::new(RefCell::new(2)),
    )));
    let s: Value<(Value<i32>, Value<i32>)> = Rc::new(RefCell::new((
        Rc::new(RefCell::new(3)),
        Rc::new(RefCell::new(4)),
    )));
    let mut pd: Ptr<i32> = ((*d.borrow()).0.as_pointer());
    let mut ps: Ptr<i32> = ((*s.borrow()).0.as_pointer());
    {
        let __src_first: Ptr<i32> = (*s.borrow()).0.as_pointer();
        let __src_second: Ptr<i32> = (*s.borrow()).1.as_pointer();
        let __first = cpp_move!(__src_first);
        let __second = cpp_move!(__src_second);
        d.as_pointer().with(|__dst| {
            *__dst.0.borrow_mut() = __first;
            *__dst.1.borrow_mut() = __second;
        })
    };
    assert!(((pd.read()) == 3));
    ps.write(5);
    assert!(({ ((*(*d.borrow()).0.borrow()) == 3) }));
    assert!(({ ((*(*s.borrow()).0.borrow()) == 5) }));
    let w: Value<(Value<i64>, Value<i64>)> = Rc::new(RefCell::new((
        Rc::new(RefCell::new((1 as i64))),
        Rc::new(RefCell::new((2 as i64))),
    )));
    let narrow: Value<(Value<i32>, Value<i32>)> = Rc::new(RefCell::new((
        Rc::new(RefCell::new(3)),
        Rc::new(RefCell::new(4)),
    )));
    let mut pw: Ptr<i64> = ((*w.borrow()).0.as_pointer());
    {
        let __first = (((*narrow.borrow()).0.as_pointer().read()) as i64);
        let __second = (((*narrow.borrow()).1.as_pointer().read()) as i64);
        w.as_pointer().with(|__dst| {
            *__dst.0.borrow_mut() = __first;
            *__dst.1.borrow_mut() = __second;
        })
    };
    assert!(((pw.read()) == 3_i64));
    let m: Value<(Value<i32>, Value<i32>)> = Rc::new(RefCell::new((
        Rc::new(RefCell::new(5)),
        Rc::new(RefCell::new(6)),
    )));
    {
        let __first = (((*m.borrow()).0.as_pointer().read()) as i64);
        let __second = (((*m.borrow()).1.as_pointer().read()) as i64);
        w.as_pointer().with(|__dst| {
            *__dst.0.borrow_mut() = __first;
            *__dst.1.borrow_mut() = __second;
        })
    };
    assert!(((pw.read()) == 5_i64));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
