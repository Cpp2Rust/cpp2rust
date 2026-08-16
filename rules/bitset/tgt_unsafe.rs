// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

fn t1() -> Vec<bool> {
    Vec::new()
}

fn f1() -> Vec<bool> {
    Vec::new()
}

fn f2(a0: u64) -> Vec<bool> {
    (0..u64::BITS).map(|i| (a0 >> i) & 1 == 1).collect()
}

fn f3(a0: Vec<bool>, a1: usize) -> Vec<bool> {
    let mut bits = vec![false; a1];
    bits.extend(a0.iter());
    bits
}

fn f4(a0: Vec<bool>, a1: usize) -> Vec<bool> {
    a0.iter().skip(a1).copied().collect()
}

fn f5(a0: Vec<bool>) -> u64 {
    a0.iter().rev().fold(0u64, |acc, &b| (acc << 1) | b as u64)
}
