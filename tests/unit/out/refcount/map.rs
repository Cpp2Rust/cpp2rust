extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn foo_0(mut x: u32) {
    x = { (x).wrapping_add(1_u32) };
}
pub fn bar_1(x: Ptr<u32>) {
    x.write({ (x.read()).wrapping_add(1_u32) });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let m: Value<Map<i16, (Value<i16>, Value<u32>)>> = Rc::new(RefCell::new(Map::new()));
    Map::subscript_move(
        (m.as_pointer() as Ptr<Map<i16, (Value<i16>, Value<u32>)>>),
        Rc::new(RefCell::new(0_i16)).as_pointer(),
        |__k| cpp_move!(__k),
    )
    .write(1_u32);
    Map::subscript_move(
        (m.as_pointer() as Ptr<Map<i16, (Value<i16>, Value<u32>)>>),
        Rc::new(RefCell::new(1_i16)).as_pointer(),
        |__k| cpp_move!(__k),
    )
    .write(2_u32);
    Map::subscript_move(
        (m.as_pointer() as Ptr<Map<i16, (Value<i16>, Value<u32>)>>),
        Rc::new(RefCell::new(2_i16)).as_pointer(),
        |__k| cpp_move!(__k),
    )
    .write(3_u32);
    assert!(((*m.borrow()).len() == 3_usize));
    assert!(
        ((Map::subscript_move(
            (m.as_pointer() as Ptr<Map<i16, (Value<i16>, Value<u32>)>>),
            Rc::new(RefCell::new(0_i16)).as_pointer(),
            |__k| cpp_move!(__k)
        )
        .read())
            == 1_u32)
    );
    assert!(
        ((Map::subscript_move(
            (m.as_pointer() as Ptr<Map<i16, (Value<i16>, Value<u32>)>>),
            Rc::new(RefCell::new(1_i16)).as_pointer(),
            |__k| cpp_move!(__k)
        )
        .read())
            == 2_u32)
    );
    assert!(
        ((Map::subscript_move(
            (m.as_pointer() as Ptr<Map<i16, (Value<i16>, Value<u32>)>>),
            Rc::new(RefCell::new(2_i16)).as_pointer(),
            |__k| cpp_move!(__k)
        )
        .read())
            == 3_u32)
    );
    let mut x: i32 = 4;
    Map::subscript_move(
        (m.as_pointer() as Ptr<Map<i16, (Value<i16>, Value<u32>)>>),
        Rc::new(RefCell::new(1_i16)).as_pointer(),
        |__k| cpp_move!(__k),
    )
    .write((x as u32));
    assert!(((*m.borrow()).len() == 3_usize));
    assert!(
        ((Map::subscript_move(
            (m.as_pointer() as Ptr<Map<i16, (Value<i16>, Value<u32>)>>),
            Rc::new(RefCell::new(0_i16)).as_pointer(),
            |__k| cpp_move!(__k)
        )
        .read())
            == 1_u32)
    );
    assert!(
        ((Map::subscript_move(
            (m.as_pointer() as Ptr<Map<i16, (Value<i16>, Value<u32>)>>),
            Rc::new(RefCell::new(1_i16)).as_pointer(),
            |__k| cpp_move!(__k)
        )
        .read())
            == 4_u32)
    );
    assert!(
        ((Map::subscript_move(
            (m.as_pointer() as Ptr<Map<i16, (Value<i16>, Value<u32>)>>),
            Rc::new(RefCell::new(2_i16)).as_pointer(),
            |__k| cpp_move!(__k)
        )
        .read())
            == 3_u32)
    );
    ({
        foo_0(
            (Map::subscript_move(
                (m.as_pointer() as Ptr<Map<i16, (Value<i16>, Value<u32>)>>),
                Rc::new(RefCell::new(0_i16)).as_pointer(),
                |__k| cpp_move!(__k),
            )
            .read()),
        )
    });
    assert!(
        ((Map::subscript_move(
            (m.as_pointer() as Ptr<Map<i16, (Value<i16>, Value<u32>)>>),
            Rc::new(RefCell::new(0_i16)).as_pointer(),
            |__k| cpp_move!(__k)
        )
        .read())
            == 1_u32)
    );
    ({
        bar_1(Map::subscript_move(
            (m.as_pointer() as Ptr<Map<i16, (Value<i16>, Value<u32>)>>),
            Rc::new(RefCell::new(2_i16)).as_pointer(),
            |__k| cpp_move!(__k),
        ))
    });
    assert!(
        ((Map::subscript_move(
            (m.as_pointer() as Ptr<Map<i16, (Value<i16>, Value<u32>)>>),
            Rc::new(RefCell::new(2_i16)).as_pointer(),
            |__k| cpp_move!(__k)
        )
        .read())
            == 4_u32)
    );
    let __rhs = (Map::subscript_move(
        (m.as_pointer() as Ptr<Map<i16, (Value<i16>, Value<u32>)>>),
        Rc::new(RefCell::new(0_i16)).as_pointer(),
        |__k| cpp_move!(__k),
    )
    .read())
    .wrapping_add(
        (Map::subscript_move(
            (m.as_pointer() as Ptr<Map<i16, (Value<i16>, Value<u32>)>>),
            Rc::new(RefCell::new(2_i16)).as_pointer(),
            |__k| cpp_move!(__k),
        )
        .read()),
    );
    Map::subscript_move(
        (m.as_pointer() as Ptr<Map<i16, (Value<i16>, Value<u32>)>>),
        Rc::new(RefCell::new(0_i16)).as_pointer(),
        |__k| cpp_move!(__k),
    )
    .write(__rhs);
    assert!(
        ((Map::subscript_move(
            (m.as_pointer() as Ptr<Map<i16, (Value<i16>, Value<u32>)>>),
            Rc::new(RefCell::new(0_i16)).as_pointer(),
            |__k| cpp_move!(__k)
        )
        .read())
            == 5_u32)
    );
    let end: Value<BidirectionalIterator<(Value<i16>, Value<u32>)>> = Rc::new(RefCell::new(
        (m.as_pointer() as Ptr<Map<i16, (Value<i16>, Value<u32>)>>)
            .with(|__m: &Map<i16, (Value<i16>, Value<u32>)>| __m.end()),
    ));
    let it: Value<BidirectionalIterator<(Value<i16>, Value<u32>)>> = Rc::new(RefCell::new(
        (m.as_pointer() as Ptr<Map<i16, (Value<i16>, Value<u32>)>>)
            .with(|__m: &Map<i16, (Value<i16>, Value<u32>)>| __m.find(&1_i16)),
    ));
    let const_it: Value<BidirectionalIterator<(Value<i16>, Value<u32>)>> = Rc::new(RefCell::new(
        <BidirectionalIterator<(Value<i16>, Value<u32>)>>::from(
            (m.as_pointer() as Ptr<Map<i16, (Value<i16>, Value<u32>)>>)
                .with(|__m: &Map<i16, (Value<i16>, Value<u32>)>| __m.find(&10_i16)),
        ),
    ));
    let mut x1: u32 = if (*it.borrow()) == (*end.borrow()) {
        0_u32
    } else {
        (*(*(*it.borrow()).iter_deref().upgrade().deref()).1.borrow())
    };
    assert!((x1 == 4_u32));
    let mut x2: u32 = if (*const_it.borrow())
        == <BidirectionalIterator<(Value<i16>, Value<u32>)>>::from((*end.borrow()).clone())
    {
        0_u32
    } else {
        (*(*(*const_it.borrow()).iter_deref().upgrade().deref())
            .1
            .borrow())
    };
    assert!((x2 == 0_u32));
    let mut x3: u32 = if (*it.borrow())
        == (m.as_pointer() as Ptr<Map<i16, (Value<i16>, Value<u32>)>>)
            .with(|__m: &Map<i16, (Value<i16>, Value<u32>)>| __m.end())
    {
        0_u32
    } else {
        (*(*(*it.borrow()).iter_deref().upgrade().deref()).1.borrow())
    };
    assert!((x3 == 4_u32));
    let mut x4: u32 = if (*const_it.borrow())
        == <BidirectionalIterator<(Value<i16>, Value<u32>)>>::from(
            (m.as_pointer() as Ptr<Map<i16, (Value<i16>, Value<u32>)>>)
                .with(|__m: &Map<i16, (Value<i16>, Value<u32>)>| __m.end()),
        ) {
        0_u32
    } else {
        (*(*(*const_it.borrow()).iter_deref().upgrade().deref())
            .1
            .borrow())
    };
    assert!((x4 == 0_u32));
    Map::subscript_move(
        (m.as_pointer() as Ptr<Map<i16, (Value<i16>, Value<u32>)>>),
        Rc::new(RefCell::new(4_i16)).as_pointer(),
        |__k| cpp_move!(__k),
    )
    .write(5_u32);
    let it4: Value<BidirectionalIterator<(Value<i16>, Value<u32>)>> = Rc::new(RefCell::new(
        (m.as_pointer() as Ptr<Map<i16, (Value<i16>, Value<u32>)>>)
            .with(|__m: &Map<i16, (Value<i16>, Value<u32>)>| __m.find(&4_i16)),
    ));
    let mut p: Ptr<u32> = ((*(*it4.borrow()).iter_deref().upgrade().deref())
        .1
        .as_pointer());
    let mut x5: u32 = (p.read());
    assert!(
        ((Map::subscript_move(
            (m.as_pointer() as Ptr<Map<i16, (Value<i16>, Value<u32>)>>),
            Rc::new(RefCell::new(4_i16)).as_pointer(),
            |__k| cpp_move!(__k)
        )
        .read())
            == 5_u32)
    );
    assert!(((*(*(*it4.borrow()).iter_deref().upgrade().deref()).1.borrow()) == 5_u32));
    assert!(((p.read()) == 5_u32));
    assert!((x5 == 5_u32));
    p.with_mut(|__v| __v.prefix_inc());
    assert!(
        ((Map::subscript_move(
            (m.as_pointer() as Ptr<Map<i16, (Value<i16>, Value<u32>)>>),
            Rc::new(RefCell::new(4_i16)).as_pointer(),
            |__k| cpp_move!(__k)
        )
        .read())
            == 6_u32)
    );
    assert!(((*(*(*it4.borrow()).iter_deref().upgrade().deref()).1.borrow()) == 6_u32));
    assert!(((p.read()) == 6_u32));
    assert!((x5 == 5_u32));
    let const_it4: Value<BidirectionalIterator<(Value<i16>, Value<u32>)>> = Rc::new(RefCell::new(
        <BidirectionalIterator<(Value<i16>, Value<u32>)>>::from(
            (m.as_pointer() as Ptr<Map<i16, (Value<i16>, Value<u32>)>>)
                .with(|__m: &Map<i16, (Value<i16>, Value<u32>)>| __m.find(&4_i16)),
        ),
    ));
    assert!(
        ((*(*(*const_it4.borrow()).iter_deref().upgrade().deref())
            .1
            .borrow())
            == 6_u32)
    );
    let r: Ptr<Map<i16, (Value<i16>, Value<u32>)>> = m.as_pointer();
    assert!(((*r.upgrade().deref()).len() == 4_usize));
    assert!(
        (m.as_pointer() as Ptr<Map<i16, (Value<i16>, Value<u32>)>>)
            .with(|__m: &Map<i16, (Value<i16>, Value<u32>)>| __m.find(&4_i16))
            != (m.as_pointer() as Ptr<Map<i16, (Value<i16>, Value<u32>)>>)
                .with(|__m: &Map<i16, (Value<i16>, Value<u32>)>| __m.end())
    );
    {
        let __it = (*it4.borrow()).clone();
        Map::erase(
            ((r).clone() as Ptr<Map<i16, (Value<i16>, Value<u32>)>>),
            &__it,
        )
    };
    assert!(((*r.upgrade().deref()).len() == 3_usize));
    assert!(
        (m.as_pointer() as Ptr<Map<i16, (Value<i16>, Value<u32>)>>)
            .with(|__m: &Map<i16, (Value<i16>, Value<u32>)>| __m.find(&4_i16))
            == (m.as_pointer() as Ptr<Map<i16, (Value<i16>, Value<u32>)>>)
                .with(|__m: &Map<i16, (Value<i16>, Value<u32>)>| __m.end())
    );
    Map::subscript_move(
        (m.as_pointer() as Ptr<Map<i16, (Value<i16>, Value<u32>)>>),
        Rc::new(RefCell::new(7_i16)).as_pointer(),
        |__k| cpp_move!(__k),
    )
    .write(7_u32);
    Map::subscript_move(
        (m.as_pointer() as Ptr<Map<i16, (Value<i16>, Value<u32>)>>),
        Rc::new(RefCell::new(8_i16)).as_pointer(),
        |__k| cpp_move!(__k),
    )
    .write(8_u32);
    Map::subscript_move(
        (m.as_pointer() as Ptr<Map<i16, (Value<i16>, Value<u32>)>>),
        Rc::new(RefCell::new(9_i16)).as_pointer(),
        |__k| cpp_move!(__k),
    )
    .write(9_u32);
    assert!(((*r.upgrade().deref()).len() == 6_usize));
    assert!(
        (Map::erase_key(
            (m.as_pointer() as Ptr<Map<i16, (Value<i16>, Value<u32>)>>),
            &10_i16
        ) == 0_usize)
    );
    let mut key9: i32 = 9;
    assert!(
        (Map::erase_key(
            (m.as_pointer() as Ptr<Map<i16, (Value<i16>, Value<u32>)>>),
            &(key9 as i16)
        ) == 1_usize)
    );
    assert!(((*r.upgrade().deref()).len() == 5_usize));
    let cit: Value<BidirectionalIterator<(Value<i16>, Value<u32>)>> = Rc::new(RefCell::new(
        <BidirectionalIterator<(Value<i16>, Value<u32>)>>::from(
            (m.as_pointer() as Ptr<Map<i16, (Value<i16>, Value<u32>)>>)
                .with(|__m: &Map<i16, (Value<i16>, Value<u32>)>| __m.find(&7_i16)),
        ),
    ));
    let next: Value<BidirectionalIterator<(Value<i16>, Value<u32>)>> = Rc::new(RefCell::new({
        let __it = (*cit.borrow()).clone();
        Map::erase(
            (m.as_pointer() as Ptr<Map<i16, (Value<i16>, Value<u32>)>>),
            &__it,
        )
    }));
    assert!(
        (*next.borrow())
            == (m.as_pointer() as Ptr<Map<i16, (Value<i16>, Value<u32>)>>)
                .with(|__m: &Map<i16, (Value<i16>, Value<u32>)>| __m.find(&8_i16))
    );
    (*next.borrow_mut()) = {
        let __it = (*next.borrow()).clone();
        Map::erase(
            (m.as_pointer() as Ptr<Map<i16, (Value<i16>, Value<u32>)>>),
            &__it,
        )
    };
    assert!(
        (*next.borrow())
            == (m.as_pointer() as Ptr<Map<i16, (Value<i16>, Value<u32>)>>)
                .with(|__m: &Map<i16, (Value<i16>, Value<u32>)>| __m.end())
    );
    assert!(((*r.upgrade().deref()).len() == 3_usize));
    assert!(
        (m.as_pointer() as Ptr<Map<i16, (Value<i16>, Value<u32>)>>)
            .with(|__m: &Map<i16, (Value<i16>, Value<u32>)>| __m.find(&8_i16))
            == (m.as_pointer() as Ptr<Map<i16, (Value<i16>, Value<u32>)>>)
                .with(|__m: &Map<i16, (Value<i16>, Value<u32>)>| __m.end())
    );
    let other_map: Value<
        Map<(Value<i32>, Value<i64>), (Value<(Value<i32>, Value<i64>)>, Value<f64>)>,
    > = Rc::new(RefCell::new(Map::new()));
    assert!(((*other_map.borrow()).len() == 0_usize));
    let key0: Value<(Value<i32>, Value<i64>)> = Rc::new(RefCell::new((
        Rc::new(RefCell::new(1.try_into().expect("failed conversion"))),
        Rc::new(RefCell::new(1.try_into().expect("failed conversion"))),
    )));
    let mut value: f64 = 2_f64;
    Map::subscript(
        (other_map.as_pointer()
            as Ptr<Map<(Value<i32>, Value<i64>), (Value<(Value<i32>, Value<i64>)>, Value<f64>)>>),
        &(*key0.borrow()),
    )
    .write(value);
    value = (Map::subscript(
        (other_map.as_pointer()
            as Ptr<Map<(Value<i32>, Value<i64>), (Value<(Value<i32>, Value<i64>)>, Value<f64>)>>),
        &(*key0.borrow()),
    )
    .read());
    assert!(((*other_map.borrow()).len() == 1_usize));
    assert!(
        ((Map::subscript(
            (other_map.as_pointer()
                as Ptr<
                    Map<(Value<i32>, Value<i64>), (Value<(Value<i32>, Value<i64>)>, Value<f64>)>,
                >),
            &(*key0.borrow())
        )
        .read())
            == value)
    );
    assert!(((*m.borrow()).len() == 3_usize));
    let mut k: i32 = 0;
    assert!(
        (((*m.borrow())
            .get(&(k as i16))
            .expect("out of range!")
            .borrow()
            .1
            .as_pointer()
            .read())
            == 5_u32)
    );
    k.prefix_inc();
    assert!(
        (((*m.borrow())
            .get(&(k as i16))
            .expect("out of range!")
            .borrow()
            .1
            .as_pointer()
            .read())
            == 4_u32)
    );
    k.prefix_inc();
    assert!(
        (((*m.borrow())
            .get(&(k as i16))
            .expect("out of range!")
            .borrow()
            .1
            .as_pointer()
            .read())
            == 4_u32)
    );
    let m2: Value<Map<i32, (Value<i32>, Value<bool>)>> = Rc::new(RefCell::new(Map::new()));
    assert!(((*m2.borrow()).len() == 0_usize));
    let mut indexes: Vec<i32> = Vec::new();
    let mut i: u32 = 60_u32;
    'loop_: while (i > 30_u32) {
        {
            let __a1 = (i as i32);
            indexes.push(__a1)
        };
        i.prefix_dec();
    }
    let mut i: u32 = 100_u32;
    'loop_: while (i > 60_u32) {
        {
            let __a1 = (i as i32);
            indexes.push(__a1)
        };
        i.prefix_dec();
    }
    let mut i: u32 = 30_u32;
    'loop_: while (i > 0_u32) {
        {
            let __a1 = (i as i32);
            indexes.push(__a1)
        };
        i.prefix_dec();
    }
    let mut i: u32 = 0_u32;
    'loop_: while ((i as usize) < indexes.len()) {
        let __rhs = ((i).wrapping_rem(2_u32) != 0);
        Map::subscript(
            (m2.as_pointer() as Ptr<Map<i32, (Value<i32>, Value<bool>)>>),
            &indexes[(i as usize)],
        )
        .write(__rhs);
        i.prefix_inc();
    }
    assert!(((*m2.borrow()).len() == indexes.len()));
    let mut last: i32 = -1_i32;
    'loop_: for pair in IterRange::new(
        (m2.as_pointer() as Ptr<Map<i32, (Value<i32>, Value<bool>)>>)
            .with(|__m: &Map<i32, (Value<i32>, Value<bool>)>| __m.begin()),
        (m2.as_pointer() as Ptr<Map<i32, (Value<i32>, Value<bool>)>>)
            .with(|__m: &Map<i32, (Value<i32>, Value<bool>)>| __m.end()),
    ) {
        let pair: Ptr<(Value<i32>, Value<bool>)> = pair.iter_deref();
        assert!(({ (*(*pair.upgrade().deref()).0.borrow()) } > { last }));
        assert!(
            ({ ((*(*pair.upgrade().deref()).1.borrow()) as i32) } == {
                ((*(*pair.upgrade().deref()).0.borrow()) % 2)
            })
        );
        last = (*(*pair.upgrade().deref()).0.borrow());
    }
    k = 0;
    let value_0: Ptr<u32> = (*m.borrow())
        .get(&(k as i16))
        .expect("out of range!")
        .borrow()
        .1
        .as_pointer();
    assert!(
        ((((((((*m.borrow()).len()).wrapping_add((x1 as usize))).wrapping_add((x2 as usize)))
            .wrapping_add((x3 as usize)))
        .wrapping_add((x4 as usize)))
        .wrapping_add((x5 as usize)))
        .wrapping_add(((value_0.read()) as usize))
            == 21_usize)
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
