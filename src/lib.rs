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
//!     let mut slab = Slab::<u32>::with_capacity(MAX_ELEMS);
//!
//!     // Insertion
//!     for num in 0..MAX_ELEMS {
//!         slab.insert(num as u32);
//!     }
//!
//!     // Traversal
//!     for offset in 0..slab.len() {
//!         slab[offset] = 33;
//!     }
//!
//!     // Iteration
//!     for num in slab.iter() {
//!         // Stuff...
//!     }
//!
//!     // Removal
//!     for offset in 0..slab.len() {
//!         let num = slab.remove(offset);
//!     }
//! }
//! ```

extern crate libc;


use std::{mem, ptr};
use std::ops::{Drop, Index};
use std::iter::{Iterator, IntoIterator};


pub struct Slab<T> {
    capacity: usize,
    num_elems: usize,
    mem_ptr: *mut T
}

pub struct SlabIter<'a, T: 'a> {
    slab: &'a Slab<T>,
    current_offset: usize
}

pub struct SlabMutIter<'a, T: 'a> {
    iter: SlabIter<'a, T>
}

impl<T> Slab<T> {
    /// Creates a new, empty Slab
    pub fn new() -> Slab<T> {
        Slab {
            capacity: 0,
            num_elems: 0,
            mem_ptr: ptr::null_mut()
        }
    }

    /// Creates a new, empty Slab with room for `capacity` elems
    ///
    /// # Panics
    ///
    /// Panics if the host system is out of memory
    pub fn with_capacity(capacity: usize) -> Slab<T> {
        unsafe {
            let maybe_ptr = libc::malloc((mem::size_of::<T>() * capacity)) as *mut T;

            // malloc will return NULL if called with zero
            if maybe_ptr.is_null() && capacity != 0 {
                panic!("Unable to allocate requested capacity")
            }

            return Slab {
                capacity: capacity,
                num_elems: 0,
                mem_ptr: maybe_ptr
            }
        }
    }

    /// Inserts a new element into the slab, re-allocating if neccessary.
    ///
    /// # Panics
    /// * If the host system is out of memory.
    #[inline]
    pub fn insert(&mut self, elem: T) {
        if self.num_elems == self.capacity {
            self.reallocate();
        }

        let next_elem_offset = self.num_elems as isize;
        unsafe {
            ptr::write(self.mem_ptr.offset(next_elem_offset), elem);
        }
        self.num_elems += 1;
    }

    /// Removes the element at `offset`.
    ///
    /// # Panics
    ///
    /// * If `offset` is out of bounds.
    #[inline]
    pub fn remove(&mut self, offset: usize) -> T {
        if offset >= self.num_elems {
            panic!("Offset {} out of bounds for slab.len: {}", offset, self.num_elems)
        }

        let last_elem_offset = (self.num_elems - 1) as isize;
        let elem = unsafe {
            let elem_ptr = self.mem_ptr.offset(offset as isize);
            let last_elem_ptr = self.mem_ptr.offset(last_elem_offset);
            mem::replace(&mut (*elem_ptr), ptr::read(last_elem_ptr))
        };
        self.num_elems -= 1;

        return elem;
    }

    /// Returns the number of elements in the slab
    #[inline]
    pub fn len(&self) -> usize {
        self.num_elems
    }

    /// Returns an iterator over the slab
    #[inline]
    pub fn iter(&self) -> SlabIter<T> {
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

    /// Reserves capacity * 2 extra space in this slab
    ///
    /// # Panics
    ///
    /// Panics if the host system is out of memory
    #[inline]
    fn reallocate(&mut self) {
        let new_capacity = if self.capacity != 0 {
            self.capacity * 2
        } else {
            1
        };

        unsafe {
            let maybe_ptr = libc::realloc(self.mem_ptr as *mut libc::c_void,
                                          (mem::size_of::<T>() * new_capacity)) as *mut T;

            if maybe_ptr.is_null() {
                panic!("Unable to allocate new capacity")
            }

            self.capacity = new_capacity;
            self.mem_ptr = maybe_ptr;
        }
    }
}

impl<T> Drop for Slab<T> {
    fn drop(&mut self) {
        unsafe {
            for x in 0..self.len() {
                let elem_ptr = self.mem_ptr.offset(x as isize);
                ptr::drop_in_place(elem_ptr);
            }

            libc::free(self.mem_ptr as *mut _ as *mut libc::c_void);
        }
    }
}

impl<T> Index<usize> for Slab<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe {
            &(*(self.mem_ptr.offset(index as isize)))
        }
    }
}

impl<'a, T> Iterator for SlabIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<&'a T> {
        while self.current_offset < self.slab.len() {
            let offset = self.current_offset;
            self.current_offset += 1;
            unsafe {
                return Some(&(*self.slab.mem_ptr.offset(offset as isize)));
            }
        }

        return  None;
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
