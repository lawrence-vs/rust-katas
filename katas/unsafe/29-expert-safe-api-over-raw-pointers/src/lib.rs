/// A fixed-size byte buffer.
/// CLUE: A caller can read `ptr` and `len` and do anything with them. Which invariants can this type still promise?
/// CLUE: Who frees the allocation? Run `cargo +nightly miri test` after it compiles.
pub struct Buffer {
    pub ptr: *mut u8,
    pub len: usize,
}

impl Buffer {
    pub fn new(len: usize) -> Self {
        let boxed = vec![0u8; len].into_boxed_slice();
        Buffer {
            ptr: Box::into_raw(boxed).cast::<u8>(),
            len,
        }
    }

    /// # Safety
    /// `i` must be less than `len`.
    pub unsafe fn get(&self, i: usize) -> Option<u8> {
        Some(unsafe { *self.ptr.add(i) })
    }

    /// # Safety
    /// `i` must be less than `len`.
    pub unsafe fn set(&mut self, i: usize, value: u8) -> bool {
        unsafe { *self.ptr.add(i) = value };
        true
    }
}
