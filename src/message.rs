//! RPMI 1.0 message headers and common payloads.
//!
//! Specification: <https://github.com/riscv-non-isa/riscv-rpmi/blob/v1.0/src/message-protocol.adoc>.

/// Message type in `Header::flags[2:0]`.
pub const TYPE_MASK: u8 = 0x07;
/// Message type in `Header::flags[2:0]`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum MessageType {
    NormalRequest = 0,
    PostedRequest = 1,
    Acknowledgement = 2,
    Notification = 3,
}
/// Shared-memory transport: request a doorbell for the response.
pub const DOORBELL: u8 = 1 << 3;
/// Notifications use service ID zero.
pub const NOTIFICATION_SERVICE_ID: u8 = 0;
/// Requested operation in `EnableNotificationRequest::req_state`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum NotificationRequestState {
    Disable = 0,
    Enable = 1,
    Query = 2,
}

/// Eight-byte message header; `data_len` is a multiple of four bytes.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct Header {
    pub service_group_id: u16,
    pub service_id: u8,
    pub flags: u8,
    pub data_len: u16,
    pub token: u16,
}

/// Message header followed by raw payload words.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct Message<const WORDS: usize> {
    pub header: Header,
    pub data: [u32; WORDS],
}

/// Four-byte event header; `data_len` is a multiple of four bytes.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct EventHeader {
    pub data_len: u16,
    pub event_id: u8,
    pub reserved: u8,
}

/// One notification event followed by raw event data words.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct Event<const WORDS: usize> {
    pub header: EventHeader,
    pub data: [u32; WORDS],
}

/// Status-only acknowledgement payload.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct StatusResponse {
    pub status: i32,
}

/// Common `ENABLE_NOTIFICATION` request payload.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct EnableNotificationRequest {
    pub event_id: u32,
    pub req_state: u32,
}

/// Common `ENABLE_NOTIFICATION` response payload.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct EnableNotificationResponse {
    pub status: i32,
    pub current_state: u32,
}
