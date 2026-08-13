# Building

## Requirements

On Ubuntu, install the required dependencies with:

```bash
sudo apt install libclang-22-dev clang++-22 ninja-build cmake
pip install ruff==0.15.22
```

## Build

```bash
mkdir build
cd build
cmake -GNinja ..
ninja
ninja check
```
