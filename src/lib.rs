// Copyright 2016 Nathan Sizemore <nathanrsizemore@gmail.com>
//
// This Source Code Form is subject to the terms of the
// Mozilla Public License, v. 2.0. If a copy of the MPL was not
// distributed with this file, You can obtain one at
// http://mozilla.org/MPL/2.0/.


//! simple_slab provides a fast, minimal, slab allocator.
//! ## Usage
//!
//! ```no_run
//! extern crate simple_slab;
//!
//! use simple_slab::Slab;
//!
//! fn main() {
//!     const MAX_ELEMS: usize = 100000;
//!
//!     let mut slab = Slab::<u32>::new(MAX_ELEMS);
//!
//!     // Insertion
//!     for num in 0..MAX_ELEMS {
//!         slab.insert(num as u32);
//!     }
//!
//!     // Traversal
//!     for offset in 0..slab.len() {
//!         match slab[offset] {
//!             Some(num) => {
//!                 // Stuff...
//!             }
//!             None => {
//!                 // Stuff...
//!             }
//!         }
//!     }
//!
//!     // Iteration
//!     for num in slab.iter() {
//!         // Stuff...
//!     }
//!
//!     // Removal
//!     for offset in 0..slab.len() {
//!         let num = slab.remove(offset).unwrap();
//!     }
//! }
//! ```


use std::{mem, ops};
use std::iter::{Iterator, IntoIterator};


#[derive(Clone)]
/// Pre-allocated chunk of memory sized specifically for `T`
pub struct Slab<T> {
    num_elems: usize,
    buf: Vec<Option<T>>
}

impl<T> Slab<T> {
    /// Creates a new Slab with initial room without re-allocation for `capacity` num elements.
    pub fn new(capacity: usize) -> Slab<T> {
        let mut buf = Vec::<Option<T>>::with_capacity(capacity);
        for _ in 0..capacity {
            buf.push(None);
        }

        Slab {
            num_elems: 0,
            buf: buf
        }
    }

    /// Inserts a new element into the slab, re-allocating if neccessary.
    ///
    /// # Panics
    /// Panics if re-allocation overflows `usize`.
    #[inline]
    pub fn insert(&mut self, elem: T) {
        if self.num_elems == self.buf.len() {
            let extra_mem = self.num_elems * 2;
            self.allocate_exact(extra_mem);
        }

        // Add new element
        self.buf[self.num_elems] = Some(elem);

        // Update counts
        self.num_elems += 1;
    }

    /// Remove the element at `offset`.
    ///
    /// # Panics
    /// Panics if `offset` is out of bounds.
    #[inline]
    pub fn remove(&mut self, offset: usize) -> Option<T> {
        self.num_elems -= 1;

        // Removal of only element
        if self.num_elems == 0 {
            return mem::replace(&mut self.buf[0], None);
        }

        // Removal of last element in buf
        if offset == self.num_elems {
            return mem::replace(&mut self.buf[self.num_elems], None);
        }

        // Removal of any other element
        self.buf.swap(offset, self.num_elems);
        mem::replace(&mut self.buf[self.num_elems], None)
    }

    /// Returns the number of elements in the slab.
    #[inline]
    pub fn len(&self) -> usize {
        self.num_elems
    }

    /// Returns an iterator over the slab.
    #[inline]
    pub fn iter(&self) -> SlabIter<T>  {
        SlabIter {
            slab: self,
            current_offset: 0
        }
    }

    /// Returns a mutable iterator over the slab
    #[inline]
    pub fn iter_mut(&mut self) -> SlabMutIter<T> {
        SlabMutIter {
            iter: self.iter()
        }
    }

    #[inline]
    fn allocate_exact(&mut self, size: usize) {
        // Allocate extra space inside buffer
        self.buf.reserve_exact(size);

        // Fill the extra space with tombstones
        for _ in 0..size {
            self.buf.push(None);
        }
    }
}

impl<T> ops::Index<usize> for Slab<T> {
    type Output = Option<T>;
    fn index(&self, index: usize) -> &Self::Output {
        &(self.buf[index])
    }
}

impl<T> ops::IndexMut<usize> for Slab<T> {
    fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut Option<T> {
        &mut (self.buf[index])
    }
}

pub struct SlabIter<'a, T: 'a> {
    current_offset: usize,
    slab: &'a Slab<T>
}

pub struct SlabMutIter<'a, T: 'a> {
    iter: SlabIter<'a, T>
}

impl<'a, T> Iterator for SlabIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<&'a T> {
        while self.current_offset < self.slab.len() {
            match self.slab.buf[self.current_offset] {
                Some(ref elem) => {
                    self.current_offset += 1;
                    return Some(elem);
                }
                None => self.current_offset += 1
            }
        }

        None
    }
}

impl<'a, T> Iterator for SlabMutIter<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<&'a mut T> {
        unsafe { mem::transmute(self.iter.next()) }
    }
}

impl<'a, T> IntoIterator for &'a Slab<T> {
    type Item = &'a T;
    type IntoIter = SlabIter<'a, T>;
    fn into_iter(self) -> SlabIter<'a, T> {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Slab<T> {
    type Item = &'a mut T;
    type IntoIter = SlabMutIter<'a, T>;
    fn into_iter(self) -> SlabMutIter<'a, T> {
        self.iter_mut()
    }
}

unsafe impl<T: Send> Send for Slab<T> {}
unsafe impl<T: Sync> Sync for Slab<T> {}
