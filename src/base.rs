//! RPMI 1.0 BASE service group.
//!
//! Specification: <https://github.com/riscv-non-isa/riscv-rpmi/blob/v1.0/src/srvgrp-base.adoc>.

/// Service group ID.
pub const SERVICE_GROUP_ID: u16 = 0x0001;

/// `BASE_ENABLE_NOTIFICATION`.
pub const ENABLE_NOTIFICATION: u8 = 0x01;

/// Payload for `BASE_ENABLE_NOTIFICATION` (request).
pub type EnableNotificationRequest = crate::message::EnableNotificationRequest;

/// Payload for `BASE_ENABLE_NOTIFICATION` (response).
pub type EnableNotificationResponse = crate::message::EnableNotificationResponse;

/// `BASE_GET_IMPLEMENTATION_VERSION`.
pub const GET_IMPLEMENTATION_VERSION: u8 = 0x02;

/// Payload for `BASE_GET_IMPLEMENTATION_VERSION` (response).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct GetImplementationVersionResponse {
    pub status: i32,
    pub impl_version: u32,
}

/// `BASE_GET_IMPLEMENTATION_ID`.
pub const GET_IMPLEMENTATION_ID: u8 = 0x03;

/// Payload for `BASE_GET_IMPLEMENTATION_ID` (response).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct GetImplementationIdResponse {
    pub status: i32,
    pub impl_id: u32,
}

/// `BASE_GET_SPEC_VERSION`.
pub const GET_SPEC_VERSION: u8 = 0x04;

/// Payload for `BASE_GET_SPEC_VERSION` (response).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct GetSpecVersionResponse {
    pub status: i32,
    pub spec_version: u32,
}

/// `BASE_GET_PLATFORM_INFO`.
pub const GET_PLATFORM_INFO: u8 = 0x05;

/// Payload for `BASE_GET_PLATFORM_INFO` (response).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct GetPlatformInfoResponse {
    pub status: i32,
    pub platform_id_len: u32,
    pub platform_id: [u8; 0],
}

/// `BASE_PROBE_SERVICE_GROUP`.
pub const PROBE_SERVICE_GROUP: u8 = 0x06;

/// Payload for `BASE_PROBE_SERVICE_GROUP` (request).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct ProbeServiceGroupRequest {
    pub servicegroup_id: u32,
}

/// Payload for `BASE_PROBE_SERVICE_GROUP` (response).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct ProbeServiceGroupResponse {
    pub status: i32,
    pub service_group_version: u32,
}

/// `BASE_GET_ATTRIBUTES`.
pub const GET_ATTRIBUTES: u8 = 0x07;

/// Payload for `BASE_GET_ATTRIBUTES` (response).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct GetAttributesResponse {
    pub status: i32,
    pub flags0: u32,
    pub flags1: u32,
    pub flags2: u32,
    pub flags3: u32,
}

/// BASE notification event IDs.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum EventId {
    RequestHandleError = 1,
}
