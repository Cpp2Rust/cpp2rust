# Control Flow Macros

Rust has no `goto`, and a `match` arm never falls into the next one. The
`libcc2rs-macros` crate provides two procedural macros, re-exported by
`libcc2rs`, that express these C constructs as a state machine. Both models use
them.

## goto_block

`goto_block!` takes a sequence of labeled blocks. Execution starts in the first
block and falls through from each block into the next; `goto!('label)` jumps to
the block with that label, forwards or backwards:

```c
int retry(int n) {
  int count = 0;
  int acc = 0;
again:
  count += 1;
  acc += n;
  if (count < 3)
    goto again;
  return acc;
}
```

```rust
pub fn retry_0(n: i32) -> i32 {
    let n: Value<i32> = Rc::new(RefCell::new(n));
    let count: Value<i32> = <Value<i32>>::default();
    let acc: Value<i32> = <Value<i32>>::default();
    goto_block!({
        'entry: {
            *count.borrow_mut() = 0;
            *acc.borrow_mut() = 0;
        }
        'again: {
            (*count.borrow_mut()) += 1;
            (*acc.borrow_mut()) += (*n.borrow());
            if *count.borrow() < 3 {
                goto!('again);
            }
            return (*acc.borrow());
        }
    });
    panic!("ub: non-void function does not return a value")
}
```

The code generator puts the statements that precede the first C label in an
`'entry` block. The `panic!` after the block is there for the Rust compiler: the
function returns from inside the state machine, but the compiler cannot see that
every path does, so without a final diverging statement it rejects the function
for not returning a value.

The macro expands to a `loop` over a `match` on a state variable, one arm per
block. Each arm ends by setting the next state and continuing the loop, and
`goto!('label)` sets the target state instead. In outline, the block above
becomes:

```rust
let mut state: u32 = 0;
'sm: loop {
    match state {
        0 => { /* entry body */ state = 1; continue 'sm; }
        1 => { /* again body, with goto!('again) as */ { state = 1; continue 'sm; } break 'sm; }
        _ => break 'sm,
    }
}
```

`break` and `continue` written inside a block (outside any loop nested in it)
still refer to the loop enclosing the `goto_block!`: the macro records them in a
flag, leaves the state machine loop, and re-issues them after it. `goto!`
outside a `goto_block!` is a compile error.

Supported `goto` patterns:

- labels at the top level of a block: a function body, a loop body, or a
  compound statement;
- a `goto` anywhere inside that block, including in nested `if`s, loops, and
  `switch` cases;
- forward and backward jumps.

Not supported yet:

- a jump to a label that is not at the top level of a block enclosing the
  `goto`, such as from outside a loop to a label in its body;
- a jump to a label inside an `if` branch.

## switch

A `switch` without fallthrough is translated as a plain `match` inside a labeled
block, where `break` becomes a `break` out of that block. When some case falls
into the next, the code generator uses `switch!` instead. It is written like a
`match`, but an arm whose body does not end in `break` continues into the body
of the following arm, as C does:

```c
switch (x) {
case 1:
  r += 10;
case 2:
  r += 20;
  break;
default:
  r = -1;
  break;
}
```

```rust
switch!(match (*x.borrow()) {
    v if v == 1 => {
        (*r.borrow_mut()) += 10;
    }
    v if v == 2 => {
        (*r.borrow_mut()) += 20;
        break;
    }
    _ => {
        (*r.borrow_mut()) = -1;
        break;
    }
});
```

`switch!` desugars to a `goto_block!` whose first block dispatches on the
condition to the block of the matching case; the case bodies follow as
consecutive blocks, so falling off the end of one enters the next, and `break`
leaves the whole `switch!`. `goto` and `switch` mix freely: a `switch!` can be
nested in a `goto_block!`, a `goto!` inside a case can target a label of the
enclosing block, and a label attached to a `case` is supported. Statements
between the `switch` and its first `case` are not supported yet.

## Hoisted declarations

In C a variable declared in one case is visible in the cases after it, because
they all belong to the same block. Each `switch!` arm is a separate Rust block,
so the code generator hoists such declarations above the macro and leaves an
assignment in the case:

```c
switch (x) {
case 1:
  r = 1;
  int y;
  y = 10;
  r += y;
case 2:
  y = 20;
  r = y;
  break;
}
```

```rust
let y: Value<i32> = <Value<i32>>::default();
switch!(match (*x.borrow()) {
    v if v == 1 => {
        (*r.borrow_mut()) = 1;
        *y.borrow_mut() = 10;
        (*r.borrow_mut()) += *y.borrow();
    }
    v if v == 2 => {
        *y.borrow_mut() = 20;
        (*r.borrow_mut()) = *y.borrow();
        break;
    }
    _ => {}
});
```

The same hoisting applies to variables used across the labeled blocks of a
`goto_block!`, as `count` and `acc` above show.
