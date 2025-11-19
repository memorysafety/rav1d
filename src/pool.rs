use parking_lot::Mutex;

pub struct MemPool<T> {
    bufs: Mutex<Vec<Vec<T>>>,
}

impl<T> MemPool<T> {
    pub const fn new() -> Self {
        Self {
            bufs: Mutex::new(Vec::new()),
        }
    }

    pub fn _pop(&self, size: usize) -> Vec<T> {
        if let Some(mut buf) = self.bufs.lock().pop() {
            if size > buf.capacity() {
                // TODO fallible allocation
                buf.reserve(size - buf.len());
            }
            return buf;
        }
        // TODO fallible allocation
        Vec::with_capacity(size)
    }

    /// A version of [`Self::pop`] that initializes the [`Vec`].
    /// This allows it to use [`vec!`], which, if used with `0`,
    /// calls [`alloc_zeroed`], and thus can leave zero initialization to the OS.
    ///
    /// [`alloc_zeroed`]: std::alloc::alloc_zeroed
    pub fn pop_init(&self, size: usize, init_value: T) -> Vec<T>
    where
        T: Copy,
    {
        if let Some(buf) = self.bufs.lock().pop() {
            if size <= buf.len() {
                return buf;
            }
        }
        // TODO fallible allocation
        vec![init_value; size]
    }

    pub fn push(&self, buf: Vec<T>) {
        self.bufs.lock().push(buf);
    }
}

impl<T> Default for MemPool<T> {
    fn default() -> Self {
        Self::new()
    }
}
