//! RPMI 1.0 SYSTEM_RESET service group.
//!
//! Specification: <https://github.com/riscv-non-isa/riscv-rpmi/blob/v1.0/src/srvgrp-system-reset.adoc>.

/// Service group ID.
pub const SERVICE_GROUP_ID: u16 = 0x0003;

/// `SYSRST_ENABLE_NOTIFICATION`.
pub const ENABLE_NOTIFICATION: u8 = 0x01;

/// Payload for `SYSRST_ENABLE_NOTIFICATION` (request).
pub type EnableNotificationRequest = crate::message::EnableNotificationRequest;

/// Payload for `SYSRST_ENABLE_NOTIFICATION` (response).
pub type EnableNotificationResponse = crate::message::EnableNotificationResponse;

/// `SYSRST_GET_ATTRIBUTES`.
pub const GET_ATTRIBUTES: u8 = 0x02;

/// Payload for `SYSRST_GET_ATTRIBUTES` (request).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct GetAttributesRequest {
    pub reset_type: u32,
}

/// Payload for `SYSRST_GET_ATTRIBUTES` (response).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct GetAttributesResponse {
    pub status: i32,
    pub flags: u32,
}

/// `SYSRST_RESET` (posted request; no response).
pub const RESET: u8 = 0x03;

/// Payload for `SYSRST_RESET` (request).
pub type ResetRequest = GetAttributesRequest;
