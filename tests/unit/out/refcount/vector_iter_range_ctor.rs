extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let src: Value<Box<[u32]>> = Rc::new(RefCell::new(Box::new([1_u32, 2_u32, 3_u32])));
    let v1: Value<Vec<u32>> = Rc::new(RefCell::new({
        let __count = (src.as_pointer() as Ptr<u32>)
            .offset((3) as isize)
            .get_offset()
            - (src.as_pointer() as Ptr<u32>).get_offset();
        PtrValueIter::new(&(src.as_pointer() as Ptr<u32>), __count)
            .map(|item| u32::try_from(item).ok().unwrap())
            .collect::<Vec<_>>()
    }));
    assert!(((*v1.borrow()).len() == 3_usize));
    assert!(
        ((((v1.as_pointer() as Ptr<u32>)
            .offset(0_usize as isize)
            .read())
            == 1_u32)
            && (((v1.as_pointer() as Ptr<u32>)
                .offset(1_usize as isize)
                .read())
                == 2_u32))
            && (((v1.as_pointer() as Ptr<u32>)
                .offset(2_usize as isize)
                .read())
                == 3_u32)
    );
    let v2: Value<Vec<u64>> = Rc::new(RefCell::new({
        let __count = (src.as_pointer() as Ptr<u32>)
            .offset((3) as isize)
            .get_offset()
            - (src.as_pointer() as Ptr<u32>).get_offset();
        PtrValueIter::new(&(src.as_pointer() as Ptr<u32>), __count)
            .map(|item| u64::try_from(item).ok().unwrap())
            .collect::<Vec<_>>()
    }));
    assert!(((*v2.borrow()).len() == 3_usize));
    assert!(
        ((((v2.as_pointer() as Ptr<u64>)
            .offset(0_usize as isize)
            .read())
            == 1_u64)
            && (((v2.as_pointer() as Ptr<u64>)
                .offset(1_usize as isize)
                .read())
                == 2_u64))
            && (((v2.as_pointer() as Ptr<u64>)
                .offset(2_usize as isize)
                .read())
                == 3_u64)
    );
    let v3: Value<Vec<i32>> = Rc::new(RefCell::new({
        let __count = (src.as_pointer() as Ptr<u32>)
            .offset((3) as isize)
            .get_offset()
            - (src.as_pointer() as Ptr<u32>).get_offset();
        PtrValueIter::new(&(src.as_pointer() as Ptr<u32>), __count)
            .map(|item| i32::try_from(item).ok().unwrap())
            .collect::<Vec<_>>()
    }));
    assert!(((*v3.borrow()).len() == 3_usize));
    assert!(
        ((((v3.as_pointer() as Ptr<i32>)
            .offset(0_usize as isize)
            .read())
            == 1)
            && (((v3.as_pointer() as Ptr<i32>)
                .offset(1_usize as isize)
                .read())
                == 2))
            && (((v3.as_pointer() as Ptr<i32>)
                .offset(2_usize as isize)
                .read())
                == 3)
    );
    let src1: Value<Box<[u32]>> = Rc::new(RefCell::new(Box::new([1_u32, 2_u32, 3_u32])));
    let v4: Value<Vec<u32>> = Rc::new(RefCell::new({
        let __count = (src1.as_pointer() as Ptr<u32>).to_end().get_offset()
            - (src1.as_pointer() as Ptr<u32>).get_offset();
        PtrValueIter::new(&(src1.as_pointer() as Ptr<u32>), __count).collect::<Vec<_>>()
    }));
    assert!(((*v4.borrow()).len() == 3_usize));
    assert!(
        ((((v4.as_pointer() as Ptr<u32>)
            .offset(0_usize as isize)
            .read())
            == 1_u32)
            && (((v4.as_pointer() as Ptr<u32>)
                .offset(1_usize as isize)
                .read())
                == 2_u32))
            && (((v4.as_pointer() as Ptr<u32>)
                .offset(2_usize as isize)
                .read())
                == 3_u32)
    );
    let buf: Value<Box<[u8]>> =
        Rc::new(RefCell::new(Box::new([10_u8, 20_u8, 30_u8, 40_u8, 50_u8])));
    let start: Value<Ptr<u8>> = Rc::new(RefCell::new((buf.as_pointer() as Ptr<u8>)));
    let len: Value<usize> = Rc::new(RefCell::new(5_usize));
    let v5: Value<Vec<u8>> = Rc::new(RefCell::new({
        let __count = (*start.borrow())
            .offset((*len.borrow()) as isize)
            .get_offset()
            - (*start.borrow()).get_offset();
        PtrValueIter::new(&(*start.borrow()), __count).collect::<Vec<_>>()
    }));
    assert!(((*v5.borrow()).len() == 5_usize));
    assert!(
        ((((v5.as_pointer() as Ptr<u8>).offset(0_usize as isize).read()) as i32) == 10)
            && ((((v5.as_pointer() as Ptr<u8>).offset(4_usize as isize).read()) as i32) == 50)
    );
    return 0;
}
