//! RPMI 1.0 HSM service group.
//!
//! Specification: <https://github.com/riscv-non-isa/riscv-rpmi/blob/v1.0/src/srvgrp-hart-state-management.adoc>.

/// Service group ID.
pub const SERVICE_GROUP_ID: u16 = 0x0005;

/// `HSM_ENABLE_NOTIFICATION`.
pub const ENABLE_NOTIFICATION: u8 = 0x01;

/// Payload for `HSM_ENABLE_NOTIFICATION` (request).
pub type EnableNotificationRequest = crate::message::EnableNotificationRequest;

/// Payload for `HSM_ENABLE_NOTIFICATION` (response).
pub type EnableNotificationResponse = crate::message::EnableNotificationResponse;

/// `HSM_GET_HART_STATUS`.
pub const GET_HART_STATUS: u8 = 0x02;

/// Payload for `HSM_GET_HART_STATUS` (request).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct GetHartStatusRequest {
    pub hart_id: u32,
}

/// Payload for `HSM_GET_HART_STATUS` (response).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct GetHartStatusResponse {
    pub status: i32,
    pub hart_state: u32,
}

/// `HSM_GET_HART_LIST`.
pub const GET_HART_LIST: u8 = 0x03;

/// Payload for `HSM_GET_HART_LIST` (request).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct GetHartListRequest {
    pub start_index: u32,
}

/// Payload for `HSM_GET_HART_LIST` (response).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct GetHartListResponse {
    pub status: i32,
    pub remaining: u32,
    pub returned: u32,
    pub hart_id: [u32; 0],
}

/// `HSM_GET_SUSPEND_TYPES`.
pub const GET_SUSPEND_TYPES: u8 = 0x04;

/// Payload for `HSM_GET_SUSPEND_TYPES` (request).
pub type GetSuspendTypesRequest = GetHartListRequest;

/// Payload for `HSM_GET_SUSPEND_TYPES` (response).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct GetSuspendTypesResponse {
    pub status: i32,
    pub remaining: u32,
    pub returned: u32,
    pub suspend_type: [u32; 0],
}

/// `HSM_GET_SUSPEND_INFO`.
pub const GET_SUSPEND_INFO: u8 = 0x05;

/// Payload for `HSM_GET_SUSPEND_INFO` (request).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct GetSuspendInfoRequest {
    pub suspend_type: u32,
}

/// Payload for `HSM_GET_SUSPEND_INFO` (response).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct GetSuspendInfoResponse {
    pub status: i32,
    pub flags: u32,
    pub entry_latency: u32,
    pub exit_latency: u32,
    pub wakeup_latency: u32,
    pub min_residency: u32,
}

/// `HSM_HART_START`.
pub const HART_START: u8 = 0x06;

/// Payload for `HSM_HART_START` (request).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct HartStartRequest {
    pub hart_id: u32,
    pub start_addr_low: u32,
    pub start_addr_high: u32,
}

/// Payload for `HSM_HART_START` (response).
pub type HartStartResponse = crate::message::StatusResponse;

/// `HSM_HART_STOP`.
pub const HART_STOP: u8 = 0x07;

/// Payload for `HSM_HART_STOP` (request).
pub type HartStopRequest = GetHartStatusRequest;

/// Payload for `HSM_HART_STOP` (response).
pub type HartStopResponse = crate::message::StatusResponse;

/// `HSM_HART_SUSPEND`.
pub const HART_SUSPEND: u8 = 0x08;

/// Payload for `HSM_HART_SUSPEND` (request).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct HartSuspendRequest {
    pub hart_id: u32,
    pub suspend_type: u32,
    pub resume_addr_low: u32,
    pub resume_addr_high: u32,
}

/// Payload for `HSM_HART_SUSPEND` (response).
pub type HartSuspendResponse = crate::message::StatusResponse;
