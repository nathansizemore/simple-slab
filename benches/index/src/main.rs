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
    let find_list = get_find_list();
    vec_index(&find_list, &mut results);
    vec_get_unchecked(&find_list, &mut results);
    slab_get(&find_list, &mut results);
    simple_slab_index(&find_list, &mut results);

    print_results(results);
}

fn vec_index(find_list: &Vec<usize>, results: &mut Vec<BenchResult>) {
    let mut buf = Vec::<usize>::new();
    for x in 0..NUM_ELEMS {
        buf.push(x);
    }

    let mut temp_var = 0;
    let start = Instant::now();
    for find in find_list {
        if buf[*find] == *find {
            temp_var += 1;
        }
    }
    let duration = start.elapsed();
    if temp_var != find_list.len() { panic!(""); }

    results.push(BenchResult {
        _type: "Vec".to_owned(),
        method: "[index]".to_owned(),
        secs: duration.as_secs() as u32,
        nanos: duration.subsec_nanos()
    });
}

fn vec_get_unchecked(find_list: &Vec<usize>, results: &mut Vec<BenchResult>) {
    let mut buf = Vec::<usize>::new();
    for x in 0..NUM_ELEMS {
        buf.push(x);
    }

    let mut temp_var = 0;
    let start = Instant::now();
    for find in find_list {
        if unsafe { *buf.get_unchecked(*find) } == *find {
            temp_var += 1;
        }
    }
    let duration = start.elapsed();
    if temp_var != find_list.len() { panic!(""); }

    results.push(BenchResult {
        _type: "Vec".to_owned(),
        method: "get_unchecked".to_owned(),
        secs: duration.as_secs() as u32,
        nanos: duration.subsec_nanos()
    });
}

fn slab_get(find_list: &Vec<usize>, results: &mut Vec<BenchResult>) {
    let mut buf = slab::Slab::<usize>::with_capacity(NUM_ELEMS);
    for x in 0..NUM_ELEMS {
        let _ = buf.insert(x);
    }

    let mut temp_var = 0;
    let start = Instant::now();
    for find in find_list {
        if *buf.get(*find).unwrap() == *find { temp_var += 1; }
    }
    let duration = start.elapsed();
    if temp_var != find_list.len() { panic!(""); }

    results.push(BenchResult {
        _type: "slab::Slab".to_owned(),
        method: "get".to_owned(),
        secs: duration.as_secs() as u32,
        nanos: duration.subsec_nanos()
    });
}

fn simple_slab_index(find_list: &Vec<usize>, results: &mut Vec<BenchResult>) {
    let mut buf = simple_slab::Slab::<usize>::with_capacity(NUM_ELEMS);
    for x in 0..NUM_ELEMS {
        buf.insert(x);
    }

    let mut temp_var = 0;
    let start = Instant::now();
    for find in find_list {
        if buf[*find] == *find {
            temp_var += 1;
        }
    }
    let duration = start.elapsed();
    if temp_var != find_list.len() { panic!(""); }

    results.push(BenchResult {
        _type: "simple_slab::Slab".to_owned(),
        method: "[index]".to_owned(),
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

fn get_find_list() -> Vec<usize> {
    let mut buf = Vec::<usize>::new();
    for _ in 0..NUM_ELEMS {
        let find = rand::thread_rng().gen_range::<usize>(0, NUM_ELEMS);
        buf.push(find);
    }

    return buf;
}
