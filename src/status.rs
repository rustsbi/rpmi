//! RPMI 1.0 status codes (the first signed word of an acknowledgement).
//!
//! Specification: <https://github.com/riscv-non-isa/riscv-rpmi/blob/v1.0/src/message-protocol.adoc#possible-error-codes>.

/// Standard RPMI acknowledgement status codes.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(i32)]
pub enum Status {
    Success = 0,
    Failed = -1,
    NotSupported = -2,
    InvalidParam = -3,
    Denied = -4,
    InvalidAddr = -5,
    Already = -6,
    Extension = -7,
    HwFault = -8,
    Busy = -9,
    InvalidState = -10,
    BadRange = -11,
    Timeout = -12,
    Io = -13,
    NoData = -14,
}
