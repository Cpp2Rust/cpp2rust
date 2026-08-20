# Usage

## Translate a single file

```bash
./build/cpp2rust/cpp2rust --file=<file>.cpp -o=<file>.rs
```

By default, the reference counting model is used (fully safe output). To
generate unsafe Rust instead:

```bash
./build/cpp2rust/cpp2rust --file=<file>.cpp -o=<file>.rs --model=unsafe
```

**Minimal example.** Given `hello.cpp`:

```cpp
#include <cstdio>
int main() {
  printf("hello world\n");
  return 0;
}
```

Running `./build/cpp2rust/cpp2rust --file=hello.cpp -o=hello.rs` produces:

```rust
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    println!("hello world");
    return 0;
}
```

Compile and run with:

```bash
rustc hello.rs -L ../libcc2rs/target/debug
./hello
```

## Translate a whole program

First generate a
[`compile_commands.json`](https://clang.llvm.org/docs/JSONCompilationDatabase.html)
for your project. With CMake this is one extra flag:

```bash
cmake -DCMAKE_EXPORT_COMPILE_COMMANDS=ON ..
```

Then run:

```bash
./build/cpp2rust/cpp2rust --dir=<dir> -o <output>.rs
```

`<dir>` must be the directory that contains `compile_commands.json`.
