/*!
 * Example queue inspired by a Quviq demo
 *
 *  - http://www.quviq.com/demos/
 *  - http://doc.rust-lang.org/src/collections/home/rustbuild/src/rust-buildbot/slave/nightly-linux/build/src/libcollections/vec.rs.html#105-109
 */
#![feature(phase)]

extern crate alloc;
extern crate core;
extern crate quickcheck;

#[phase(plugin)]
extern crate quickcheck_macros;

use alloc::heap::{allocate};
use core::mem;
use core::ptr;

// This could all reside in its own module 'queue.rs'
#[deriving(Clone,Show)]
pub struct Queue<T> {
    inp: uint,
    outp: uint,
    size: uint,
    buf: *mut T
}

impl<T> Queue<T> {
    pub fn new(size: uint) -> Queue<T> {
        assert!(size > 0);
        let sz = (size + 1).checked_mul(&mem::size_of::<T>())
                           .expect("capacity overflow");
        let buf = unsafe { allocate(sz, mem::min_align_of::<T>()) };
        Queue{inp: 0, outp: 0, size: size + 1, buf: buf as *mut T}
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
    use super::Queue;

    use std::rand::{Rand,Rng};

    use quickcheck::TestResult;
    use quickcheck::{Gen,Arbitrary};

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
    
    impl<T: Send + Clone + 'static> Arbitrary for Queue<T> {
        fn arbitrary<G: Gen>(g: &mut G) -> Queue<T> {
            let s = g.gen_range::<uint>(1,50);
            Queue::<T>::new(s)
        }
    }

    #[quickcheck]
    fn prop_put_one(mut q: Queue<uint>) -> TestResult {
        q.put(1u);
        if q.size() == 1 {
            TestResult::passed()
        } else {
            TestResult::error(format!("Expected '1', but got '{}'", q.size()).as_slice())
        }
    }

    #[quickcheck]
    fn prop_zero_size(sz: uint) -> TestResult {
        if sz == 0 {
            TestResult::discard()
        } else {
            TestResult::from_bool(Queue::<uint>::new(sz).size() == 0)
        }
    }

    /*
     * We want to be able to generate a random, but valid sequence of Action's.
     *
     *  - Put can only be generated if the Queue is not full
     *  - Get can only be generated if the Queue is not empty
     *  - Size can _always_ be generated
     */
    #[deriving(Clone, Show)]
    enum Action { Put, Get, Size }

    impl Rand for Action {
        fn rand<R: Rng>(rng: &mut R) -> Action {
            *rng.choose([Put, Get, Size]).unwrap()
        }
    }

    #[deriving(Clone, Show)]
    struct QueueTestConfig<T> {
        q: Queue<T>,
        actions: Vec<Action>
    }

    impl<T> Arbitrary for QueueTestConfig<T> where T: Send + Clone + 'static {
        fn arbitrary<G: Gen>(g: &mut G) -> QueueTestConfig<T> {
            let s = g.gen_range::<uint>(1, 50);
            let n = g.gen_range::<uint>(1, 1000);

            let mut actions = Vec::with_capacity(n);
            let mut sz = 0;
            for act in g.gen_iter::<Action>().take(n) {
                match act {
                    Put if sz < s => { actions.push(Put); sz += 1 },
                    Get if sz > 0 => { actions.push(Get); sz -= 1 },
                    Size => actions.push(Size),
                    Put  => { actions.push(Get); sz -= 1 },
                    Get  => { actions.push(Put); sz += 1 },
                }
            }

            QueueTestConfig{
                q: Queue::new(s),
                actions: actions
            }
        }

    }

    #[quickcheck]
    fn prop_queue_actions(mut qtc: QueueTestConfig<uint>) -> TestResult {
        let mut size = 0;
        for &act in qtc.actions.iter() {
            match act {
                Put => {
                    qtc.q.put(9);
                    size += 1;
                },
                Get => {
                    qtc.q.get();
                    size -= 1;
                },
                Size => if qtc.q.size() != size {
                    return TestResult::error(format!("Expected '{}', but got '{}'", size, qtc.q.size()).as_slice())
                }
            }
        }
        TestResult::passed()
    }
}
