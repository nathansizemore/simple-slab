# simple-slab [<img src="https://travis-ci.org/nathansizemore/simple-slab.png?branch=master">][q]

simple-slab is a simple slab-allocator in Rust. It aims to be as minimal and fast as possible.

[Documentation][w]

## Usage

~~~rust
extern crate simple_slab;

use simple_slab::Slab;

fn main() {
    const MAX_ELEMS: usize = 100000;

    let mut slab = Slab::<u32>::new(MAX_ELEMS);

    // Insertion
    for num in 0..MAX_ELEMS {
        slab.insert(num);
    }

    // Traversal
    for offset in 0..slab.len() {
        match slab[offset] {
            Some(num) => {
                // Stuff...
            }
            None => {
                // Stuff...
            }
        }
    }

    // Iteration
    for num in slab.iter() {
        // Stuff..
    }

    // Removal
    for offset in 0..slab.len() {
        let num = slab.remove(offset).unwrap();
    }
}
~~~

## Author

Nathan Sizemore, nathanrsizemore@gmail.com

## License

simple-slab is available under the MPL-2.0 license. See the LICENSE file for more info.


[q]: https://travis-ci.org/nathansizemore/simple-slab
[w]: https://nathansizemore.github.io/simple-slab/simple_slab/index.html
