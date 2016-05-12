// Copyright 2016 Nathan Sizemore <nathanrsizemore@gmail.com>
//
// This Source Code Form is subject to the terms of the
// Mozilla Public License, v. 2.0. If a copy of the MPL was not
// distributed with this file, You can obtain one at
// http://mozilla.org/MPL/2.0/.


extern crate time;
extern crate rand;
extern crate slab;
extern crate simple_slab;


use rand::Rng;


const NUM_ELEMS: usize = 10000000;


fn main() {
    bench_inserts();
    bench_removals();
    bench_traversals();
}

fn bench_inserts() {
    vec_insert();
    slab_insert();
    simple_slab_insert();
}

fn bench_removals() {
    vec_remove();
    slab_remove();
    simple_slab_remove();
}

fn bench_traversals() {
    vec_traverse();
    slab_traverse();    
    simple_slab_traverse();
}

fn vec_insert() {
    let mut buf = Vec::<u32>::with_capacity(NUM_ELEMS);

    let start_time = time::get_time();
    for x in 0..NUM_ELEMS as u32 {
        buf.push(x);
    }
    let end_time = time::get_time();

    let duration = end_time - start_time;
    println!("std::vec::Vec        insert       {}", duration);
}

fn vec_remove() {
    let mut buf = Vec::<u32>::with_capacity(NUM_ELEMS);
    for x in 0..NUM_ELEMS as u32 {
        buf.push(x);
    }
    
    let start_time = time::get_time();
    for _ in 0..(NUM_ELEMS / 10) {
        let offset = rand::thread_rng().gen_range::<usize>(0, buf.len());
        buf.swap_remove(offset);
    }
    let end_time = time::get_time();

    let duration = end_time - start_time;
    println!("std::vec::Vec        removal      {}", duration);
}

fn vec_traverse() {
    let mut buf = Vec::<u32>::with_capacity(NUM_ELEMS);
    for x in 0..NUM_ELEMS as u32 {
        buf.push(x);
    }
    
    let start_time = time::get_time();
    for x in 0..buf.len() {
        if buf[x] == buf[buf.len() - 1] {
            let end_time = time::get_time();
            let duration = end_time - start_time;
            println!("std::vec::Vec        traversal    {}", duration);
        }
    }
}

fn slab_insert() {
    let mut buf = slab::Slab::<u32, usize>::new(NUM_ELEMS);

    let start_time = time::get_time();
    for x in 0..NUM_ELEMS as u32 {
        let _ = buf.insert(x);
    }
    let end_time = time::get_time();

    let duration = end_time - start_time;
    println!("slab::Slab           insert       {}", duration);
}

fn slab_remove() {
    let mut buf = slab::Slab::<u32, usize>::new(NUM_ELEMS);
    for x in 0..NUM_ELEMS as u32 {
        let _ = buf.insert(x);
    }
    
    let start_time = time::get_time();
    for _ in 0..(NUM_ELEMS / 10) {
        let offset = rand::thread_rng().gen_range::<usize>(0, buf.count());
        let _ = buf.remove(offset);
    }
    let end_time = time::get_time();

    let duration = end_time - start_time;
    println!("slab::Slab           removal      {}", duration);
}

fn slab_traverse() {
    let mut buf = slab::Slab::<u32, usize>::new(NUM_ELEMS);
    for x in 0..NUM_ELEMS as u32 {
        let _ = buf.insert(x);
    }
    
    let start_time = time::get_time();
    for x in 0..buf.count() {
        if buf[x] == buf[buf.count() - 1] {
            let end_time = time::get_time();
            let duration = end_time - start_time;
            println!("slab::Slab           traversal    {}", duration);
        }
    }
}

fn simple_slab_insert() {
    let mut buf = simple_slab::Slab::<u32>::with_capacity(NUM_ELEMS);

    let start_time = time::get_time();
    for x in 0..NUM_ELEMS as u32 {
        buf.insert(x);
    }
    let end_time = time::get_time();

    let duration = end_time - start_time;
    println!("simple_slab::Slab    insert       {}", duration);
}

fn simple_slab_remove() {
    let mut buf = simple_slab::Slab::<u32>::with_capacity(NUM_ELEMS);
    for x in 0..NUM_ELEMS as u32 {
        buf.insert(x);
    }
    
    let start_time = time::get_time();
    for _ in 0..(NUM_ELEMS / 10) {
        let offset = rand::thread_rng().gen_range::<usize>(0, buf.len());
        let _ = buf.remove(offset);
    }
    let end_time = time::get_time();

    let duration = end_time - start_time;
    println!("simple_slab::Slab    removal      {}", duration);
}

fn simple_slab_traverse() {
    let mut buf = simple_slab::Slab::<u32>::with_capacity(NUM_ELEMS);
    for x in 0..NUM_ELEMS as u32 {
        buf.insert(x);
    }
    
    let start_time = time::get_time();
    for x in 0..buf.len() {
        if buf[x] == buf[buf.len() - 1] {
            let end_time = time::get_time();
            let duration = end_time - start_time;
            println!("simple_slab::Slab    traversal    {}", duration);
        }
    }
}

