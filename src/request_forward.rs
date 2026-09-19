//! RPMI 1.0 REQUEST_FORWARD service group.
//!
//! Specification: <https://github.com/riscv-non-isa/riscv-rpmi/blob/v1.0/src/srvgrp-request-forward.adoc>.

/// Service group ID.
pub const SERVICE_GROUP_ID: u16 = 0x000d;

/// `REQFWD_ENABLE_NOTIFICATION`.
pub const ENABLE_NOTIFICATION: u8 = 0x01;

/// Payload for `REQFWD_ENABLE_NOTIFICATION` (request).
pub type EnableNotificationRequest = crate::message::EnableNotificationRequest;

/// Payload for `REQFWD_ENABLE_NOTIFICATION` (response).
pub type EnableNotificationResponse = crate::message::EnableNotificationResponse;

/// `REQFWD_RETRIEVE_CURRENT_MESSAGE`.
pub const RETRIEVE_CURRENT_MESSAGE: u8 = 0x02;

/// Payload for `REQFWD_RETRIEVE_CURRENT_MESSAGE` (request).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct RetrieveCurrentMessageRequest {
    pub start_index: u32,
}

/// Payload for `REQFWD_RETRIEVE_CURRENT_MESSAGE` (response).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct RetrieveCurrentMessageResponse {
    pub status: i32,
    pub remaining: u32,
    pub returned: u32,
    pub request_message: [u8; 0],
}

/// `REQFWD_COMPLETE_CURRENT_MESSAGE`.
pub const COMPLETE_CURRENT_MESSAGE: u8 = 0x03;

/// Payload for `REQFWD_COMPLETE_CURRENT_MESSAGE` (request).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct CompleteCurrentMessageRequest {
    pub response_data: [u8; 0],
}

/// Payload for `REQFWD_COMPLETE_CURRENT_MESSAGE` (response).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct CompleteCurrentMessageResponse {
    pub status: i32,
    pub num_messages: u32,
}

/// REQUEST_FORWARD notification event IDs.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum EventId {
    NewMessage = 1,
}
