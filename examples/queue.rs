/*!
 * Example queue inspired by a Quviq demo
 *
 *  - http://www.quviq.com/demos/
 *  - http://doc.rust-lang.org/src/collections/home/rustbuild/src/rust-buildbot/slave/nightly-linux/build/src/libcollections/vec.rs.html#105-109
 */

// This could all reside in its own module 'queue.rs'
pub struct Queue<T> {
    inp: uint,
    outp: uint,
    buf: Vec<T> // change to *mut T
}

impl<T> Queue<T> {
    pub fn new(size: uint) -> Queue<T> {
        Queue{inp:0, outp: 0, buf: Vec::with_capacity(size)}
    }

    pub fn size(&self) -> uint {
        self.buf.capacity()
    }

    pub fn put(&mut self, v: T) {
        self.buf.insert(self.inp] = v;
        self.inp = (self.inp + 1) % self.buf.capacity();
    }

    pub fn get(&mut self) -> Option<T> {
        let v = self.buf.remove(self.outp);
        self.outp = (self.outp + 1) % self.buf.capacity();
        v
    }
}

#[cfg(test)]
mod test {
    use super::Queue;
    #[test]
    fn queue_size() {
        assert_eq!(10, Queue::<String>::new(10).size());
    }

    #[test]
    fn put_put_get() {
        let mut q = Queue::new(10u);
        q.put(1);
        q.put(2);
        assert_eq!(Some(1u), q.get());
    }
}
