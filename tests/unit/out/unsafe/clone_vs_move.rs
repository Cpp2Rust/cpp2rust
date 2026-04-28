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
pub struct Bar {
    pub w: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Foo {
    pub x: i32,
    pub y: *mut i32,
    pub z: *mut i32,
    pub a: [i32; 3],
    pub bar: Bar,
}
impl Default for Foo {
    fn default() -> Self {
        Foo {
            x: 0_i32,
            y: <*mut i32>::default(),
            z: Default::default(),
            a: [0_i32; 3],
            bar: <Bar>::default(),
        }
    }
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut x1: i32 = 1;
    let mut x2: i32 = x1;
    x2.prefix_inc();
    assert!(((x1) == (1)));
    assert!(((x2) == (2)));
    let mut x3: f64 = 3.0E+0;
    let mut x4: f64 = x3;
    x4.prefix_inc();
    assert!(((x3) == (3.0E+0)));
    assert!(((x4) == (4.0E+0)));
    let reference: *mut i32 = &mut x1 as *mut i32;
    let mut x5: i32 = (*reference);
    x5.prefix_inc();
    assert!(((*reference) == (1)));
    assert!(((x5) == (2)));
    let mut pointer: *mut i32 = (&mut x1 as *mut i32);
    let mut x6: i32 = (*pointer);
    x6.prefix_inc();
    assert!(((*pointer) == (1)));
    assert!(((x6) == (2)));
    let mut other_pointer: *mut i32 = pointer;
    assert!(((other_pointer) == (pointer)));
    (*other_pointer).prefix_inc();
    assert!(((*other_pointer) == (*pointer)));
    let mut f1: Foo = Foo {
        x: 1,
        y: &mut x1 as *mut i32,
        z: (&mut x1 as *mut i32),
        a: [0, 1, 2],
        bar: Bar { w: 10 },
    };
    assert!(((f1.x) == (1)));
    assert!(((*f1.y) == (2)));
    assert!(((f1.z) == (&mut x1 as *mut i32)));
    assert!(((*f1.z) == (2)));
    let mut f2: Foo = f1.clone();
    f2.x.prefix_inc();
    (*f2.y).prefix_inc();
    assert!(((f2.x) == (2)));
    assert!(((*f2.y) == (3)));
    assert!(((f1.x) == (1)));
    assert!(((*f1.y) == (3)));
    (*f2.z).prefix_inc();
    assert!(((*f2.y) == (4)));
    assert!(((f2.z) == (&mut x1 as *mut i32)));
    assert!(((*f2.z) == (4)));
    assert!(((*f1.y) == (4)));
    assert!(((f1.z) == (&mut x1 as *mut i32)));
    assert!(((*f1.z) == (4)));
    f2.a[(0) as usize].prefix_inc();
    f2.a[(1) as usize].prefix_inc();
    f2.a[(2) as usize].prefix_inc();
    assert!(((f2.a[(0) as usize]) == (1)));
    assert!(((f2.a[(1) as usize]) == (2)));
    assert!(((f2.a[(2) as usize]) == (3)));
    assert!(((f1.a[(0) as usize]) == (0)));
    assert!(((f1.a[(1) as usize]) == (1)));
    assert!(((f1.a[(2) as usize]) == (2)));
    f2.bar.w = 20;
    assert!(((f2.bar.w) == (20)));
    assert!(((f1.bar.w) == (10)));
    let mut N: i32 = 5;
    let mut v1: Vec<i32> = Vec::new();
    let mut i: i32 = 0;
    'loop_: while ((i) < (N)) {
        {
            let a0_clone = i.clone();
            v1.push(a0_clone)
        };
        i.prefix_inc();
    }
    let mut v2: Vec<i32> = v1.clone();
    let mut i: i32 = 0;
    'loop_: while ((i) < (N)) {
        assert!(((v2[(i as u64) as usize]) == (i)));
        i.prefix_inc();
    }
    let mut i: i32 = 0;
    'loop_: while ((i) < (N)) {
        v2[(i as u64) as usize].prefix_inc();
        i.prefix_inc();
    }
    let mut i: i32 = 0;
    'loop_: while ((i) < (N)) {
        assert!(((v2[(i as u64) as usize]) == ((i) + (1))));
        assert!(((v1[(i as u64) as usize]) == (i)));
        i.prefix_inc();
    }
    let mut m1: Vec<Vec<i32>> = Vec::new();
    let mut i: i32 = 0;
    'loop_: while ((i) < (N)) {
        m1.push(
            (0..(10_u64) as usize)
                .map(|_| <i32>::default())
                .collect::<Vec<_>>(),
        );
        i.prefix_inc();
    }
    let mut m2: Vec<Vec<i32>> = m1.clone();
    let mut i: i32 = 0;
    'loop_: while ((i) < (N)) {
        assert!(((m1[(i as u64) as usize].len() as u64) == (10_u64)));
        assert!(((m2[(i as u64) as usize].len() as u64) == (10_u64)));
        let mut j: i32 = 0;
        'loop_: while ((j) < (10)) {
            assert!(((m1[(i as u64) as usize][(j as u64) as usize]) == (0)));
            assert!(((m2[(i as u64) as usize][(j as u64) as usize]) == (0)));
            j.prefix_inc();
        }
        i.prefix_inc();
    }
    let mut i: i32 = 0;
    'loop_: while ((i) < (N)) {
        let mut j: i32 = 0;
        'loop_: while ((j) < (10)) {
            m2[(i as u64) as usize][(j as u64) as usize].postfix_inc();
            j.prefix_inc();
        }
        i.prefix_inc();
    }
    let mut i: i32 = 0;
    'loop_: while ((i) < (N)) {
        assert!(((m1[(i as u64) as usize].len() as u64) == (10_u64)));
        assert!(((m2[(i as u64) as usize].len() as u64) == (10_u64)));
        let mut j: i32 = 0;
        'loop_: while ((j) < (10)) {
            assert!(((m1[(i as u64) as usize][(j as u64) as usize]) == (0)));
            assert!(((m2[(i as u64) as usize][(j as u64) as usize]) == (1)));
            j.prefix_inc();
        }
        i.prefix_inc();
    }
    let mut map1: BTreeMap<i32, Box<i32>> = BTreeMap::new();
    let mut i: i32 = 0;
    'loop_: while ((i) < (N)) {
        (*map1.entry(i).or_default().as_mut()) = i;
        i.prefix_inc();
    }
    let mut map2: BTreeMap<i32, Box<i32>> = map1.clone();
    let mut i: i32 = 0;
    'loop_: while ((i) < (N)) {
        assert!(((*map2.entry(i).or_default().as_mut()) == (i)));
        (*map2.entry(i).or_default().as_mut()).prefix_inc();
        i.prefix_inc();
    }
    let mut i: i32 = 0;
    'loop_: while ((i) < (N)) {
        assert!(((*map1.entry(i).or_default().as_mut()) == (i)));
        assert!(((*map2.entry(i).or_default().as_mut()) == ((i) + (1))));
        i.prefix_inc();
    }
    let mut pair1: (i32, i32) = (1.into(), 2.into());
    let mut pair2: (i32, i32) = pair1.clone();
    pair2.0 = ((pair2.0) * (10));
    pair2.1 = ((pair2.1) * (10));
    assert!(((pair2.0) == (10)));
    assert!(((pair2.1) == (20)));
    assert!(((pair1.0) == (1)));
    assert!(((pair1.1) == (2)));
    let mut pair3: (Vec<i32>, i32) = (
        (0..(0_u64) as usize)
            .map(|_| <i32>::default())
            .collect::<Vec<_>>()
            .into(),
        0.into(),
    );
    let mut pair4: (Vec<i32>, i32) = pair3.clone();
    pair4.0.push(1);
    pair4.1 = 1;
    assert!(((pair4.0.len() as u64) == (1_u64)));
    assert!(((pair4.1) == (1)));
    assert!(((pair3.0.len() as u64) == (0_u64)));
    assert!(((pair3.1) == (0)));
    let mut s1: Vec<u8> = vec![('a' as u8); (3_u64) as usize]
        .iter()
        .cloned()
        .chain(std::iter::once(0))
        .collect();
    let mut s2: Vec<u8> = s1.clone();
    s2[(0_u64) as usize] = ('b' as u8);
    s2[(1_u64) as usize] = ('b' as u8);
    s2[(2_u64) as usize] = ('b' as u8);
    assert!(((s2[(0_u64) as usize] as i32) == (('b' as u8) as i32)));
    assert!(((s2[(1_u64) as usize] as i32) == (('b' as u8) as i32)));
    assert!(((s2[(2_u64) as usize] as i32) == (('b' as u8) as i32)));
    assert!(((s1[(0_u64) as usize] as i32) == (('a' as u8) as i32)));
    assert!(((s1[(1_u64) as usize] as i32) == (('a' as u8) as i32)));
    assert!(((s1[(2_u64) as usize] as i32) == (('a' as u8) as i32)));
    let mut b1: Bar = Bar { w: 1 };
    let mut b2: Bar = Bar { w: 2 };
    b2 = b1;
    b2.w.postfix_inc();
    assert!(((b1.w) == (1)));
    assert!(((b2.w) == (2)));
    let mut v4: Vec<i32> = Vec::new();
    v4 = v2.clone();
    let mut i: i32 = 0;
    'loop_: while ((i) < (N)) {
        assert!(((v4[(i as u64) as usize]) == ((i) + (1))));
        v4[(i as u64) as usize].prefix_inc();
        i.prefix_inc();
    }
    let mut i: i32 = 0;
    'loop_: while ((i) < (N)) {
        assert!(((v4[(i as u64) as usize]) == ((i) + (2))));
        assert!(((v2[(i as u64) as usize]) == ((i) + (1))));
        i.prefix_inc();
    }
    return 0;
}
