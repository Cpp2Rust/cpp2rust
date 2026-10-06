extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static next_0: Value<i32> = Rc::new(RefCell::new(0));
);
pub fn bump_1(mut v: Ptr<Vec<i32>>, mut x: i32) -> i32 {
    {
        let __rhs = x;
        elem!(((Ptr::<Vec<i32>>::decay(&(v))) as Ptr<i32>), 0_usize)
            .with_mut(|__v| *__v = *__v + __rhs)
    };
    return (*next_0.with(Value::clone).borrow_mut()).postfix_inc();
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let v: Value<Vec<i32>> = Rc::new(RefCell::new(vec![1; 4_usize as usize]));
    let mut items: Vec<u32> = (0..(3_usize) as usize)
        .map(|_| <u32>::default())
        .collect::<Vec<_>>();
    let mut i: u32 = 0_u32;
    'loop_: while ((i as usize) < items.len()) {
        let __rhs = (i).wrapping_mul(2_u32);
        items[(i as usize)] = __rhs;
        i.prefix_inc();
    }
    {
        let rhs_0 = (items[1_usize]).wrapping_add(5_u32);
        items[1_usize] = rhs_0
    };
    items[2_usize].postfix_inc();
    assert!(((items[0_usize] == 0_u32) && (items[1_usize] == 7_u32)) && (items[2_usize] == 5_u32));
    elem!(
        (v.as_pointer() as Ptr<i32>),
        ({ (*v.borrow())[0_usize] } as usize)
    )
    .write(3);
    let __rhs = ({ (*v.borrow())[1_usize] } + 4);
    elem!(
        (v.as_pointer() as Ptr<i32>),
        ({ (*v.borrow())[1_usize] } as usize)
    )
    .write(__rhs);
    assert!(({ (*v.borrow())[1_usize] } == 3) && ({ (*v.borrow())[3_usize] } == 7));
    let mut p: Ptr<i32> = ((v.as_pointer() as Ptr<i32>).offset(0_usize));
    elem!((v.as_pointer() as Ptr<i32>), ((p.read()) as usize)).write(9);
    assert!(({ (*v.borrow())[1_usize] } == 9));
    elem!(
        (v.as_pointer() as Ptr<i32>),
        (({ bump_1((v.as_pointer()), 10,) }) as usize)
    )
    .write(2);
    assert!(({ (*v.borrow())[0_usize] } == 2));
    assert!(
        (({
            let _v: Ptr<Vec<i32>> = (v.as_pointer());
            let _x: i32 = { (*v.borrow())[2_usize] };
            bump_1(_v, _x)
        }) == 1)
            && ({ (*v.borrow())[0_usize] } == 3)
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = next_0.with(|_| ());
}
