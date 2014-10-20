/*!
 * Example queue inspired by a Quviq demo
 *
 *  - http://www.quviq.com/demos/
 *  - http://doc.rust-lang.org/src/collections/home/rustbuild/src/rust-buildbot/slave/nightly-linux/build/src/libcollections/vec.rs.html#105-109
 */

extern crate alloc;
extern crate core;
extern crate quickcheck;

use alloc::heap::{allocate};
use core::mem;
use core::ptr;

// This could all reside in its own module 'queue.rs'
pub struct Queue<T> {
    inp: uint,
    outp: uint,
    size: uint,
    buf: *mut T
}

impl<T> Queue<T> {
    pub fn new(size: uint) -> Queue<T> {
        assert!(size > 0);
        let sz = size.checked_mul(&mem::size_of::<T>())
                     .expect("capacity overflow");
        let buf = unsafe { allocate(sz, mem::min_align_of::<T>()) };
        Queue{inp: 0, outp: 0, size: size, buf: buf as *mut T}
    }

    pub fn put(&mut self, v: T) {
        unsafe {
            let p = self.buf.offset(self.inp as int);
            ptr::write(&mut *p, v)
        }
        self.inp = (self.inp + 1) % self.size;
    }

    pub fn get(&mut self) -> T {
        let r: T;
        unsafe {
            let p = self.buf.offset(self.outp as int);
            r = ptr::read(p as *const T);
        }
        self.outp = (self.outp + 1) % self.size;
        r
    }

    pub fn size(&self) -> uint {
        (self.inp - self.outp) % self.size
    }
}

#[cfg(test)]
mod test {

    use quickcheck::quickcheck;
    use super::Queue;

    #[test]
    fn queue_size() {
        assert_eq!(0, Queue::<String>::new(10).size());
    }

    #[test]
    #[should_fail]
    fn size0() {
        Queue::<uint>::new(0);
    }

    #[test]
    fn put_put_get() {
        let mut q = Queue::new(10u);
        q.put(1);
        q.put(2);
        assert_eq!(1u, q.get());
        assert_eq!(2u, q.get());
    }

    #[test]
    fn qc_prop_zero_size() {
        fn prop_zero_size(sz: uint) -> bool {
            Queue::<uint>::new(sz).size() == 0
        }
        quickcheck(prop_zero_size);
    }
}
