// Copyright 2017 Nathan Sizemore <nathanrsizemore@gmail.com>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.


extern crate rand;
extern crate slab;
extern crate simple_slab;


use std::time::Instant;

use rand::Rng;


struct BenchResult {
    _type: String,
    method: String,
    secs: u32,
    nanos: u32
}

const NUM_ELEMS: usize = 1000000;


fn main() {
    let mut results = Vec::<BenchResult>::new();
    vec_swap_remove(&mut results);
    slab_remove(&mut results);
    simple_slab_remove(&mut results);

    print_results(results);
}

fn vec_swap_remove(results: &mut Vec<BenchResult>) {
    let mut buf = Vec::<u64>::new();
    for x in 0..NUM_ELEMS as u64 {
        buf.push(x);
    }

    let start = Instant::now();
    for _ in 0..NUM_ELEMS {
        let offset = rand::thread_rng().gen_range::<usize>(0, buf.len());
        buf.swap_remove(offset);
    }
    let duration = start.elapsed();

    results.push(BenchResult {
        _type: "Vec".to_owned(),
        method: "swap_remove".to_owned(),
        secs: duration.as_secs() as u32,
        nanos: duration.subsec_nanos()
    });
}

fn slab_remove(results: &mut Vec<BenchResult>) {
    let mut offsets = Vec::<usize>::new();
    let mut buf = slab::Slab::<u64, usize>::with_capacity(NUM_ELEMS);
    for x in 0..NUM_ELEMS as u64 {
        let offset = buf.insert(x).unwrap();
        offsets.push(offset);
    }

    let start = Instant::now();
    for _ in 0..NUM_ELEMS {
        let offset_index = rand::thread_rng()
            .gen_range::<usize>(0, offsets.len());
        let offset = offsets.swap_remove(offset_index);
        buf.remove(offset).unwrap();
    }
    let duration = start.elapsed();

    results.push(BenchResult {
        _type: "slab::Slab".to_owned(),
        method: "remove".to_owned(),
        secs: duration.as_secs() as u32,
        nanos: duration.subsec_nanos()
    });
}

fn simple_slab_remove(results: &mut Vec<BenchResult>) {
    let mut buf = simple_slab::Slab::<u64>::with_capacity(NUM_ELEMS);
    for x in 0..NUM_ELEMS as u64 {
        buf.insert(x);
    }

    let start = Instant::now();
    for _ in 0..NUM_ELEMS {
        let offset = rand::thread_rng().gen_range::<usize>(0, buf.len());
        buf.remove(offset);
    }
    let duration = start.elapsed();

    results.push(BenchResult {
        _type: "simple_slab::Slab".to_owned(),
        method: "remove".to_owned(),
        secs: duration.as_secs() as u32,
        nanos: duration.subsec_nanos()
    });
}

fn print_results(results: Vec<BenchResult>) {
    let (type_len, method_len) = get_lens(&results);

    for result in results {
        let mut line = String::new();
        line.push_str(&format!("{:1$}    ", result._type, type_len));
        line.push_str(&format!("{:1$}    ", result.method, method_len));
        line.push_str(&format!("{:02}.", result.secs));
        line.push_str(&format!("{:010}", result.nanos));
        println!("{}", line);
    }
}

fn get_lens(results: &Vec<BenchResult>) -> (usize, usize) {
    let mut type_len = 0;
    let mut method_len = 0;

    for result in results {
        if result._type.len() > type_len {
            type_len = result._type.len();
        }

        if result.method.len() > method_len {
            method_len = result.method.len();
        }
    }

    (type_len, method_len)
}
