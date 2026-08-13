# Overview

Translation rules describe how C++ library APIs are mapped to Rust.
Each rule module lives in the `rules/` directory and pairs a C++ source file
(`src.cpp`) with its Rust translation for each model (`tgt_refcount.rs` and
`tgt_unsafe.rs`).

This part of the book explains how rules work and how to write new ones.
