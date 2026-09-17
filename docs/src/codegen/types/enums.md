# Enums

An enum becomes a Rust `enum` with one variant per enumerator and explicit
discriminants, followed by the impls that give it C semantics. Given

```cpp
enum Color { RED, GREEN, BLUE };
```

both models produce

```rust
#[derive(Clone, Copy, PartialEq, Debug, Default)]
enum Color {
    #[default]
    RED = 0,
    GREEN = 1,
    BLUE = 2,
}
impl From<i32> for Color {
    fn from(n: i32) -> Color {
        match n {
            0 => Color::RED,
            1 => Color::GREEN,
            2 => Color::BLUE,
            _ => panic!("invalid Color value: {}", n),
        }
    }
}
libcc2rs::impl_enum_inc_dec!(Color);
```

and the refcount model adds an `impl ByteRepr for Color` that converts through
the `i32` value.

The first enumerator is the `#[default]`, which is what a zero-initialized or
default-constructed enum variable holds. `From<i32>` is the integer-to-enum
cast; a value that matches no enumerator panics, where C would silently keep the
integer. [`impl_enum_inc_dec!`](../../runtime/inc-dec.md) implements the four
`++`/`--` forms by stepping through the enumerators. Enum-to-integer casts need
no impl and become `as i32`.

`enum class` is translated the same way; enumerators are always spelled
`Color::RED` on the Rust side, scoped or not. In C, an anonymous enum named only
through a typedef (`typedef enum { ... } Tag;`) is emitted as `Tag_enum`, and an
anonymous enum with no name at all as `anon_N` (see [Naming](./naming.md)).
