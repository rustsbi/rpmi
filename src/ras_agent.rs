//! RPMI 1.0 RAS_AGENT service group.
//!
//! Specification: <https://github.com/riscv-non-isa/riscv-rpmi/blob/v1.0/src/srvgrp-ras-agent.adoc>.

/// Service group ID.
pub const SERVICE_GROUP_ID: u16 = 0x000c;

/// `RAS_ENABLE_NOTIFICATION`.
pub const ENABLE_NOTIFICATION: u8 = 0x01;

/// Payload for `RAS_ENABLE_NOTIFICATION` (request).
pub type EnableNotificationRequest = crate::message::EnableNotificationRequest;

/// Payload for `RAS_ENABLE_NOTIFICATION` (response).
pub type EnableNotificationResponse = crate::message::EnableNotificationResponse;

/// `RAS_GET_NUM_ERR_SRCS`.
pub const GET_NUM_ERR_SRCS: u8 = 0x02;

/// Payload for `RAS_GET_NUM_ERR_SRCS` (response).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct GetNumErrSrcsResponse {
    pub status: i32,
    pub num_err_srcs: u32,
}

/// `RAS_GET_ERR_SRCS_ID_LIST`.
pub const GET_ERR_SRCS_ID_LIST: u8 = 0x03;

/// Payload for `RAS_GET_ERR_SRCS_ID_LIST` (request).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct GetErrSrcsIdListRequest {
    pub start_index: u32,
}

/// Payload for `RAS_GET_ERR_SRCS_ID_LIST` (response).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct GetErrSrcsIdListResponse {
    pub status: i32,
    pub flags: u32,
    pub remaining: u32,
    pub returned: u32,
    pub ras_err_src_id: [u32; 0],
}

/// `RAS_GET_ERR_SRC_DESC`.
pub const GET_ERR_SRC_DESC: u8 = 0x04;

/// Payload for `RAS_GET_ERR_SRC_DESC` (request).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct GetErrSrcDescRequest {
    pub ras_err_src_id: u32,
    pub byte_offset: u32,
}

/// Payload for `RAS_GET_ERR_SRC_DESC` (response).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct GetErrSrcDescResponse {
    pub status: i32,
    pub flags: u32,
    pub remaining: u32,
    pub returned: u32,
    pub err_src_desc: [u8; 0],
}
