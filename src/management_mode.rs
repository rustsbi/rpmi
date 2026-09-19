//! RPMI 1.0 MANAGEMENT_MODE service group.
//!
//! Specification: <https://github.com/riscv-non-isa/riscv-rpmi/blob/v1.0/src/srvgrp-management.adoc>.

/// Service group ID.
pub const SERVICE_GROUP_ID: u16 = 0x000b;

/// `MM_ENABLE_NOTIFICATION`.
pub const ENABLE_NOTIFICATION: u8 = 0x01;

/// Payload for `MM_ENABLE_NOTIFICATION` (request).
pub type EnableNotificationRequest = crate::message::EnableNotificationRequest;

/// Payload for `MM_ENABLE_NOTIFICATION` (response).
pub type EnableNotificationResponse = crate::message::EnableNotificationResponse;

/// `MM_GET_ATTRIBUTES`.
pub const GET_ATTRIBUTES: u8 = 0x02;

/// Payload for `MM_GET_ATTRIBUTES` (response).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct GetAttributesResponse {
    pub status: i32,
    pub mm_version: u32,
    pub mm_shmem_addr_low: u32,
    pub mm_shmem_addr_high: u32,
    pub mm_shmem_size: u32,
}

/// `MM_COMMUNICATE`.
pub const COMMUNICATE: u8 = 0x03;

/// Payload for `MM_COMMUNICATE` (request).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct CommunicateRequest {
    pub mm_comm_input_data_offset: u32,
    pub mm_comm_input_data_size: u32,
    pub mm_comm_output_data_offset: u32,
    pub mm_comm_output_data_size: u32,
}

/// Payload for `MM_COMMUNICATE` (response).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct CommunicateResponse {
    pub status: i32,
    pub mm_comm_return_data_size: u32,
}
