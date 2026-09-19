//! RPMI 1.0 shared-memory queue layout.
//!
//! Specification, section 1.1.1:
//! <https://docs.riscv.org/reference/rpmi/v1.0/transport.html>.

/// Memory layout of one RPMI shared-memory queue.
///
/// Slot size is `4 * SLOT_WORDS` bytes; `SLOT_WORDS` must be a power of two
/// and at least 16. `MESSAGE_SLOTS >= 2` gives a usable ring with one slot empty.
#[repr(C)]
pub struct Queue<const SLOT_WORDS: usize, const MESSAGE_SLOTS: usize> {
    /// Next message to dequeue, in word 0; remaining words are unused.
    pub head: [u32; SLOT_WORDS],
    /// Next message to enqueue, in word 0; remaining words are unused.
    pub tail: [u32; SLOT_WORDS],
    /// Message slots, indexed by head and tail starting at zero.
    pub messages: [[u32; SLOT_WORDS]; MESSAGE_SLOTS],
}

#[cfg(test)]
mod tests {
    use super::Queue;
    use core::mem::{offset_of, size_of};

    #[test]
    fn queue_offsets() {
        type Queue64 = Queue<16, 14>;
        assert_eq!(offset_of!(Queue64, head), 0);
        assert_eq!(offset_of!(Queue64, tail), 64);
        assert_eq!(offset_of!(Queue64, messages), 128);
        assert_eq!(size_of::<Queue64>(), 1024);

        type Queue256 = Queue<64, 14>;
        assert_eq!(offset_of!(Queue256, head), 0);
        assert_eq!(offset_of!(Queue256, tail), 256);
        assert_eq!(offset_of!(Queue256, messages), 512);
        assert_eq!(size_of::<Queue256>(), 4096);
    }
}
