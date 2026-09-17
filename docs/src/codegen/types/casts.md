# Casts

Casts are of two kinds: scalar casts, which both models spell with Rust's `as`
or with a small expression, and pointer casts, where the models diverge. Most
casts in the input are implicit, inserted by clang, and are translated the same
way as explicit ones.

## Scalar casts

An integer conversion becomes `expr as T` (an integer literal is instead
re-typed in place: `1` cast to `unsigned char` prints as `1_u8`), and is dropped
when source and target map to the same Rust type, so `int` to `long` on a
platform where both are `i32` prints nothing. Floating conversions are `as` as
well. The other scalar casts have their own spellings:

- Integer to `bool`: `x != 0`; a comparison or logical operator that already
  yields `bool` is left alone. Enum to `bool` compares against `<E>::from(0)`.
- Pointer to `bool`: `!p.is_null()`.
- Integer to enum: `<E>::from(x)`, the `From<i32>` impl from the
  [Enums](./enums.md) page. When the operand is itself a constant of that same
  enum, which C++ sees as an integer being converted back to the enum, the cast
  is dropped and the constant is printed directly (`Color::RED`, not
  `<Color>::from(Color::RED as i32)`). Enum to integer is `as`.
- A cast to `void`, used to silence an unused-variable warning, becomes a
  statement that only mentions the operand: `&x;` in the unsafe model,
  `(*x.borrow()).clone();` in the refcount model.

Explicit `static_cast`, C-style, and `reinterpret_cast` between scalars follow
the same rules; a cast to the operand's own type is elided.

### Implicit conversions to `usize` and `isize`

`size_t`, `size_type`, and `ssize_t` are translated as `usize` and `isize`
rather than as the `u64`/`i64` of the `unsigned long`/`long` they are typedefs
of (built-in type rules, looked up on the sugared type before it is desugared).
This keeps rules and output free of `as usize` casts on lengths and indexes, but
it splits one C type in two: clang inserts no conversion between `size_t` and
`unsigned long`, while `usize` and `u64` do not mix in Rust. Given

```cpp
unsigned long take_ulong(unsigned long x);

size_t sz = 20;
unsigned long r = take_ulong(sz);
```

the refcount model produces

```rust
let sz: Value<usize> = Rc::new(RefCell::new(20_usize));
let r: Value<u64> = Rc::new(RefCell::new(take_ulong_0(*sz.borrow() as u64)));
```

`Convert(expr, implicit_convert_to)` is the single place where such a cast is
added: the caller passes the type the context expects, `NeedsImplicitScalarCast`
checks that it is the same C type as the expression's but maps to a different
Rust type, and if so the expression is wrapped in `(...) as <target>`. Callers
that pass a target are assignments and initializations (the variable's type),
call arguments (the parameter type of the callee or rule,
`GetParamImplicitConvertTarget`, as in the example), and binary operators, which
pick one Rust type for both operands (`GetOperandImplicitConversionTarget`).

## Pointer casts

Given

```cpp
uint32_t value = 0x04030201;
uint8_t *bytes = (uint8_t *)&value;
void *any = bytes;
uint8_t *back = (uint8_t *)any;
```

the unsafe model produces

```rust
let mut value: u32 = 67305985_u32;
let mut bytes: *mut u8 = (&mut value as *mut u32) as *mut u8;
let mut any: *mut ::libc::c_void = bytes as *mut ::libc::c_void;
let mut back: *mut u8 = any as *mut u8;
```

and the refcount model produces

```rust
let value: Value<u32> = Rc::new(RefCell::new(67305985_u32));
let bytes: Value<Ptr<u8>> =
    Rc::new(RefCell::new(value.as_pointer().reinterpret_cast::<u8>()));
let any: Value<AnyPtr> = Rc::new(RefCell::new((*bytes.borrow()).to_any()));
let back: Value<Ptr<u8>> =
    Rc::new(RefCell::new((*any.borrow()).reinterpret_cast::<u8>()));
```

### Unsafe model

Every pointer cast, whether written as a C cast, `static_cast`, or
`reinterpret_cast`, is a Rust `as` between raw pointer types. A cast that only
adds or removes `const` changes the Rust type too, since `T *` is `*mut T` and
`const T *` is `*const T`, and becomes `.cast_const()` or `.cast_mut()`. A cast
that changes nothing in Rust, such as a `typedef` to its underlying type, is not
emitted. Casts between pointers and integers are also `as`.

### Refcount model

A `Ptr<T>` is a weak reference to a `RefCell<T>`, so it cannot simply be
relabeled as a `Ptr<U>`: the cell it points to holds a `T`. A cast to another
pointee type therefore produces a different kind of pointer, one that views the
allocation as bytes. Three helpers from the runtime cover the cases:

- `p.reinterpret_cast::<U>()` produces a `Ptr<U>` of the
  [`Reinterpreted` kind](../../runtime/reinterpret.md#views-over-the-original-allocation):
  a byte-level view over the original allocation, with the offset counted in
  bytes. Reads and writes through it go through
  [`ByteRepr`](../../runtime/reinterpret.md#byterepr), which is why every record
  type gets a `ByteRepr` impl.
- `p.to_any()` erases the type into an [`AnyPtr`](../../runtime/void.md), the
  translation of `void *`, remembering the original type.
- `any.reinterpret_cast::<T>()` recovers a `Ptr<T>` from an `AnyPtr`: the
  original pointer if `T` is the type it was erased from, a byte view otherwise
  (see [AnyPtr casts](../../runtime/reinterpret.md#anyptr-casts)).

Two casts do not use these helpers. An array decaying to a pointer is spelled
`arr.as_pointer() as Ptr<T>`, where the `as` only names the pointer type. An
upcast from a derived class to an abstract base becomes
`(p.to_strong() as Value<dyn Base>).as_pointer_dyn()`, which is the ordinary
Rust unsizing coercion applied to the owning cell (see
[Virtual Classes](../../runtime/ptr-dyn.md)). Casts between pointers and
integers use the [integer cast](../../runtime/rc.md#integer-casts) API of `Ptr`.

Constness is dropped in a cast as everywhere else, so `const_cast` is a no-op.
`dynamic_cast` is not supported.

### Function pointers

Casting a function pointer to a different signature, which C code does to call
through a generic type, wraps the function in an adapter closure that converts
the arguments; see [Casts](../../runtime/fn-ptr.md#casts) on the runtime page.
Storing a function pointer in a `void *` uses `to_any()` like any other pointer.
