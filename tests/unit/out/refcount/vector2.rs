extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn fn_0(v: Ptr<Vec<i32>>, v3: Vec<i32>) {
    let v3: Value<Vec<i32>> = Rc::new(RefCell::new(v3));
    {
        let __a1 = 20;
        v.with_mut(|__v: &mut Vec<i32>| __v.push(__a1))
    };
    let mut x: i32 = 0_i32;
    let v2: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
    let mut v4: Ptr<Vec<i32>> = (v3.as_pointer());
    {
        let __a1 = 0;
        (*v2.borrow_mut()).push(__a1)
    };
    {
        let __a1 = 1;
        (*v2.borrow_mut()).push(__a1)
    };
    {
        let __a1 = 3;
        (*v2.borrow_mut()).push(__a1)
    };
    x = (elem!((Ptr::<Vec<i32>>::decay(&(v)) as Ptr<i32>), 2_usize).read());
    (*v2.borrow_mut())[0_usize] = 1;
    elem!(
        ((if true {
            v3.as_pointer()
        } else {
            Ptr::<Vec<i32>>::decay(&(v))
        }) as Ptr<i32>),
        0_usize
    )
    .write(7);
    (v2.as_pointer() as Ptr<Vec<i32>>).write((*v.upgrade().deref()).clone());
    elem!(((Ptr::<Vec<i32>>::decay(&(v4))) as Ptr<i32>), 1_usize).write(13);
    assert!((x == 6));
    assert!((((Ptr::<Vec<i32>>::decay(&(v)) as Ptr<i32>).read()) == 4));
    assert!(((elem!((Ptr::<Vec<i32>>::decay(&(v)) as Ptr<i32>), 1_usize).read()) == 5));
    assert!(((elem!((Ptr::<Vec<i32>>::decay(&(v)) as Ptr<i32>), 2_usize).read()) == 6));
    assert!((((Ptr::<Vec<i32>>::decay(&(v)) as Ptr<i32>).to_last().read()) == 20));
    assert!(({ (*v2.borrow())[0_usize] } == 4));
    assert!(({ (*v2.borrow())[1_usize] } == 5));
    assert!(({ (*v2.borrow())[2_usize] } == 6));
    assert!(({ (*v3.borrow())[0_usize] } == 7));
    assert!(({ (*v3.borrow())[1_usize] } == 13));
    {
        let __a1 = 20;
        v.with_mut(|__v: &mut Vec<i32>| __v.push(__a1))
    };
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let v: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
    let mut v2: Vec<i32> = Vec::new();
    {
        let __a1 = 4;
        (*v.borrow_mut()).push(__a1)
    };
    {
        let __a1 = 5;
        (*v.borrow_mut()).push(__a1)
    };
    {
        let __a1 = 6;
        (*v.borrow_mut()).push(__a1)
    };
    {
        let __a1 = 8;
        v2.push(__a1)
    };
    {
        let __a1 = 9;
        v2.push(__a1)
    };
    ({ fn_0(v.as_pointer(), v2.clone()) });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
