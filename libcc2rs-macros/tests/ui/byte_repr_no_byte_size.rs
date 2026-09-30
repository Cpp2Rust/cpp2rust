use libcc2rs_macros::ByteRepr;

#[derive(ByteRepr)]
struct NoByteSize {
    #[offset(0)]
    x: i32,
}

fn main() {}
