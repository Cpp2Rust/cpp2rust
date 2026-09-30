extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, VaArg, FnPtrArg, Default)]
pub struct S {
    #[offset(0)]
    pub as_: i32,
    #[offset(4)]
    pub async_: i32,
    #[offset(8)]
    pub await_: i32,
    #[offset(12)]
    pub crate_: i32,
    #[offset(16)]
    pub dyn_: i32,
    #[offset(20)]
    pub fn_: i32,
    #[offset(24)]
    pub impl_: i32,
    #[offset(28)]
    pub in_: i32,
    #[offset(32)]
    pub let_: i32,
    #[offset(36)]
    pub loop_: i32,
    #[offset(40)]
    pub match_: i32,
    #[offset(44)]
    pub mod_: i32,
    #[offset(48)]
    pub move_: i32,
    #[offset(52)]
    pub mut_: i32,
    #[offset(56)]
    pub pub_: i32,
    #[offset(60)]
    pub ref_: i32,
    #[offset(64)]
    pub self_: i32,
    #[offset(68)]
    pub Self_: i32,
    #[offset(72)]
    pub super_: i32,
    #[offset(76)]
    pub trait_: i32,
    #[offset(80)]
    pub type_: i32,
    #[offset(84)]
    pub unsafe_: i32,
    #[offset(88)]
    pub use_: i32,
    #[offset(92)]
    pub where_: i32,
    #[offset(96)]
    pub abstract_: i32,
    #[offset(100)]
    pub become_: i32,
    #[offset(104)]
    pub box_: i32,
    #[offset(108)]
    pub final_: i32,
    #[offset(112)]
    pub gen_: i32,
    #[offset(116)]
    pub macro_: i32,
    #[offset(120)]
    pub override_: i32,
    #[offset(124)]
    pub priv_: i32,
    #[offset(128)]
    pub unsized_: i32,
    #[offset(132)]
    pub yield_: i32,
    #[offset(136)]
    pub macro_rules_: i32,
    #[offset(140)]
    pub raw_: i32,
    #[offset(144)]
    pub safe_: i32,
    #[offset(148)]
    pub vec_: i32,
}
impl ByteRepr for S {
    fn byte_size() -> usize {
        152
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.as_.to_bytes(&mut buf[0..4]);
        self.async_.to_bytes(&mut buf[4..8]);
        self.await_.to_bytes(&mut buf[8..12]);
        self.crate_.to_bytes(&mut buf[12..16]);
        self.dyn_.to_bytes(&mut buf[16..20]);
        self.fn_.to_bytes(&mut buf[20..24]);
        self.impl_.to_bytes(&mut buf[24..28]);
        self.in_.to_bytes(&mut buf[28..32]);
        self.let_.to_bytes(&mut buf[32..36]);
        self.loop_.to_bytes(&mut buf[36..40]);
        self.match_.to_bytes(&mut buf[40..44]);
        self.mod_.to_bytes(&mut buf[44..48]);
        self.move_.to_bytes(&mut buf[48..52]);
        self.mut_.to_bytes(&mut buf[52..56]);
        self.pub_.to_bytes(&mut buf[56..60]);
        self.ref_.to_bytes(&mut buf[60..64]);
        self.self_.to_bytes(&mut buf[64..68]);
        self.Self_.to_bytes(&mut buf[68..72]);
        self.super_.to_bytes(&mut buf[72..76]);
        self.trait_.to_bytes(&mut buf[76..80]);
        self.type_.to_bytes(&mut buf[80..84]);
        self.unsafe_.to_bytes(&mut buf[84..88]);
        self.use_.to_bytes(&mut buf[88..92]);
        self.where_.to_bytes(&mut buf[92..96]);
        self.abstract_.to_bytes(&mut buf[96..100]);
        self.become_.to_bytes(&mut buf[100..104]);
        self.box_.to_bytes(&mut buf[104..108]);
        self.final_.to_bytes(&mut buf[108..112]);
        self.gen_.to_bytes(&mut buf[112..116]);
        self.macro_.to_bytes(&mut buf[116..120]);
        self.override_.to_bytes(&mut buf[120..124]);
        self.priv_.to_bytes(&mut buf[124..128]);
        self.unsized_.to_bytes(&mut buf[128..132]);
        self.yield_.to_bytes(&mut buf[132..136]);
        self.macro_rules_.to_bytes(&mut buf[136..140]);
        self.raw_.to_bytes(&mut buf[140..144]);
        self.safe_.to_bytes(&mut buf[144..148]);
        self.vec_.to_bytes(&mut buf[148..152]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            as_: <i32>::from_bytes(&buf[0..4]),
            async_: <i32>::from_bytes(&buf[4..8]),
            await_: <i32>::from_bytes(&buf[8..12]),
            crate_: <i32>::from_bytes(&buf[12..16]),
            dyn_: <i32>::from_bytes(&buf[16..20]),
            fn_: <i32>::from_bytes(&buf[20..24]),
            impl_: <i32>::from_bytes(&buf[24..28]),
            in_: <i32>::from_bytes(&buf[28..32]),
            let_: <i32>::from_bytes(&buf[32..36]),
            loop_: <i32>::from_bytes(&buf[36..40]),
            match_: <i32>::from_bytes(&buf[40..44]),
            mod_: <i32>::from_bytes(&buf[44..48]),
            move_: <i32>::from_bytes(&buf[48..52]),
            mut_: <i32>::from_bytes(&buf[52..56]),
            pub_: <i32>::from_bytes(&buf[56..60]),
            ref_: <i32>::from_bytes(&buf[60..64]),
            self_: <i32>::from_bytes(&buf[64..68]),
            Self_: <i32>::from_bytes(&buf[68..72]),
            super_: <i32>::from_bytes(&buf[72..76]),
            trait_: <i32>::from_bytes(&buf[76..80]),
            type_: <i32>::from_bytes(&buf[80..84]),
            unsafe_: <i32>::from_bytes(&buf[84..88]),
            use_: <i32>::from_bytes(&buf[88..92]),
            where_: <i32>::from_bytes(&buf[92..96]),
            abstract_: <i32>::from_bytes(&buf[96..100]),
            become_: <i32>::from_bytes(&buf[100..104]),
            box_: <i32>::from_bytes(&buf[104..108]),
            final_: <i32>::from_bytes(&buf[108..112]),
            gen_: <i32>::from_bytes(&buf[112..116]),
            macro_: <i32>::from_bytes(&buf[116..120]),
            override_: <i32>::from_bytes(&buf[120..124]),
            priv_: <i32>::from_bytes(&buf[124..128]),
            unsized_: <i32>::from_bytes(&buf[128..132]),
            yield_: <i32>::from_bytes(&buf[132..136]),
            macro_rules_: <i32>::from_bytes(&buf[136..140]),
            raw_: <i32>::from_bytes(&buf[140..144]),
            safe_: <i32>::from_bytes(&buf[144..148]),
            vec_: <i32>::from_bytes(&buf[148..152]),
        }
    }
}
pub fn foo_0(
    as_: i32,
    async_: i32,
    await_: i32,
    crate_: i32,
    dyn_: i32,
    fn_: i32,
    impl_: i32,
    in_: i32,
    let_: i32,
    loop_: i32,
    match_: i32,
    mod_: i32,
    move_: i32,
    mut_: i32,
    pub_: i32,
    ref_: i32,
    self_: i32,
    Self_: i32,
    super_: i32,
    trait_: i32,
    type_: i32,
    unsafe_: i32,
    use_: i32,
    where_: i32,
    abstract_: i32,
    become_: i32,
    box_: i32,
    final_: i32,
    gen_: i32,
    macro_: i32,
    override_: i32,
    priv_: i32,
    unsized_: i32,
    yield_: i32,
    macro_rules_: i32,
    raw_: i32,
    safe_: i32,
    vec_: i32,
    dummy: i32,
) -> i32 {
    let as_: Value<i32> = Rc::new(RefCell::new(as_));
    let async_: Value<i32> = Rc::new(RefCell::new(async_));
    let await_: Value<i32> = Rc::new(RefCell::new(await_));
    let crate_: Value<i32> = Rc::new(RefCell::new(crate_));
    let dyn_: Value<i32> = Rc::new(RefCell::new(dyn_));
    let fn_: Value<i32> = Rc::new(RefCell::new(fn_));
    let impl_: Value<i32> = Rc::new(RefCell::new(impl_));
    let in_: Value<i32> = Rc::new(RefCell::new(in_));
    let let_: Value<i32> = Rc::new(RefCell::new(let_));
    let loop_: Value<i32> = Rc::new(RefCell::new(loop_));
    let match_: Value<i32> = Rc::new(RefCell::new(match_));
    let mod_: Value<i32> = Rc::new(RefCell::new(mod_));
    let move_: Value<i32> = Rc::new(RefCell::new(move_));
    let mut_: Value<i32> = Rc::new(RefCell::new(mut_));
    let pub_: Value<i32> = Rc::new(RefCell::new(pub_));
    let ref_: Value<i32> = Rc::new(RefCell::new(ref_));
    let self_: Value<i32> = Rc::new(RefCell::new(self_));
    let Self_: Value<i32> = Rc::new(RefCell::new(Self_));
    let super_: Value<i32> = Rc::new(RefCell::new(super_));
    let trait_: Value<i32> = Rc::new(RefCell::new(trait_));
    let type_: Value<i32> = Rc::new(RefCell::new(type_));
    let unsafe_: Value<i32> = Rc::new(RefCell::new(unsafe_));
    let use_: Value<i32> = Rc::new(RefCell::new(use_));
    let where_: Value<i32> = Rc::new(RefCell::new(where_));
    let abstract_: Value<i32> = Rc::new(RefCell::new(abstract_));
    let become_: Value<i32> = Rc::new(RefCell::new(become_));
    let box_: Value<i32> = Rc::new(RefCell::new(box_));
    let final_: Value<i32> = Rc::new(RefCell::new(final_));
    let gen_: Value<i32> = Rc::new(RefCell::new(gen_));
    let macro_: Value<i32> = Rc::new(RefCell::new(macro_));
    let override_: Value<i32> = Rc::new(RefCell::new(override_));
    let priv_: Value<i32> = Rc::new(RefCell::new(priv_));
    let unsized_: Value<i32> = Rc::new(RefCell::new(unsized_));
    let yield_: Value<i32> = Rc::new(RefCell::new(yield_));
    let macro_rules_: Value<i32> = Rc::new(RefCell::new(macro_rules_));
    let raw_: Value<i32> = Rc::new(RefCell::new(raw_));
    let safe_: Value<i32> = Rc::new(RefCell::new(safe_));
    let vec_: Value<i32> = Rc::new(RefCell::new(vec_));
    let dummy: Value<i32> = Rc::new(RefCell::new(dummy));
    return 0;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let s: Value<S> = Rc::new(RefCell::new(S {
        as_: 0,
        async_: 0,
        await_: 0,
        crate_: 0,
        dyn_: 0,
        fn_: 0,
        impl_: 0,
        in_: 0,
        let_: 0,
        loop_: 0,
        match_: 0,
        mod_: 0,
        move_: 0,
        mut_: 0,
        pub_: 0,
        ref_: 0,
        self_: 0,
        Self_: 0,
        super_: 0,
        trait_: 0,
        type_: 0,
        unsafe_: 0,
        use_: 0,
        where_: 0,
        abstract_: 0,
        become_: 0,
        box_: 0,
        final_: 0,
        gen_: 0,
        macro_: 0,
        override_: 0,
        priv_: 0,
        unsized_: 0,
        yield_: 0,
        macro_rules_: 0,
        raw_: 0,
        safe_: 0,
        vec_: 0,
    }));
    let as_: Value<i32> = Rc::new(RefCell::new(0));
    let async_: Value<i32> = Rc::new(RefCell::new(0));
    let await_: Value<i32> = Rc::new(RefCell::new(0));
    let crate_: Value<i32> = Rc::new(RefCell::new(0));
    let dyn_: Value<i32> = Rc::new(RefCell::new(0));
    let fn_: Value<i32> = Rc::new(RefCell::new(0));
    let impl_: Value<i32> = Rc::new(RefCell::new(0));
    let in_: Value<i32> = Rc::new(RefCell::new(0));
    let let_: Value<i32> = Rc::new(RefCell::new(0));
    let loop_: Value<i32> = Rc::new(RefCell::new(0));
    let match_: Value<i32> = Rc::new(RefCell::new(0));
    let mod_: Value<i32> = Rc::new(RefCell::new(0));
    let move_: Value<i32> = Rc::new(RefCell::new(0));
    let mut_: Value<i32> = Rc::new(RefCell::new(0));
    let pub_: Value<i32> = Rc::new(RefCell::new(0));
    let ref_: Value<i32> = Rc::new(RefCell::new(0));
    let self_: Value<i32> = Rc::new(RefCell::new(0));
    let Self_: Value<i32> = Rc::new(RefCell::new(0));
    let super_: Value<i32> = Rc::new(RefCell::new(0));
    let trait_: Value<i32> = Rc::new(RefCell::new(0));
    let type_: Value<i32> = Rc::new(RefCell::new(0));
    let unsafe_: Value<i32> = Rc::new(RefCell::new(0));
    let use_: Value<i32> = Rc::new(RefCell::new(0));
    let where_: Value<i32> = Rc::new(RefCell::new(0));
    let abstract_: Value<i32> = Rc::new(RefCell::new(0));
    let become_: Value<i32> = Rc::new(RefCell::new(0));
    let box_: Value<i32> = Rc::new(RefCell::new(0));
    let final_: Value<i32> = Rc::new(RefCell::new(0));
    let gen_: Value<i32> = Rc::new(RefCell::new(0));
    let macro_: Value<i32> = Rc::new(RefCell::new(0));
    let override_: Value<i32> = Rc::new(RefCell::new(0));
    let priv_: Value<i32> = Rc::new(RefCell::new(0));
    let unsized_: Value<i32> = Rc::new(RefCell::new(0));
    let yield_: Value<i32> = Rc::new(RefCell::new(0));
    let macro_rules_: Value<i32> = Rc::new(RefCell::new(0));
    let raw_: Value<i32> = Rc::new(RefCell::new(0));
    let safe_: Value<i32> = Rc::new(RefCell::new(0));
    let vec_: Value<i32> = Rc::new(RefCell::new(0));
    return ({
        foo_0(
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        )
    });
}
pub fn __cpp2rust_init_globals() {}
