use std::sync::Mutex;

pub struct MemPool<T> {
    bufs: Mutex<Vec<Vec<T>>>,
}

impl<T> MemPool<T> {
    pub const fn new() -> Self {
        Self {
            bufs: Mutex::new(Vec::new()),
        }
    }

    pub fn pop(&self, size: usize) -> Vec<T> {
        if let Some(mut buf) = self.bufs.lock().unwrap().pop() {
            if size > buf.capacity() {
                // TODO fallible allocation
                buf.reserve(size - buf.len());
            }
            return buf;
        }
        // TODO fallible allocation
        Vec::with_capacity(size)
    }

    pub fn push(&self, buf: Vec<T>) {
        self.bufs.lock().unwrap().push(buf);
    }
}

impl<T> Default for MemPool<T> {
    fn default() -> Self {
        Self::new()
    }
}
