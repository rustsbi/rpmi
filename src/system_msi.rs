//! RPMI 1.0 SYSTEM_MSI service group.
//!
//! Specification: <https://github.com/riscv-non-isa/riscv-rpmi/blob/v1.0/src/srvgrp-system-msi.adoc>.

/// Service group ID.
pub const SERVICE_GROUP_ID: u16 = 0x0002;

/// `SYSMSI_ENABLE_NOTIFICATION`.
pub const ENABLE_NOTIFICATION: u8 = 0x01;

/// Payload for `SYSMSI_ENABLE_NOTIFICATION` (request).
pub type EnableNotificationRequest = crate::message::EnableNotificationRequest;

/// Payload for `SYSMSI_ENABLE_NOTIFICATION` (response).
pub type EnableNotificationResponse = crate::message::EnableNotificationResponse;

/// `SYSMSI_GET_ATTRIBUTES`.
pub const GET_ATTRIBUTES: u8 = 0x02;

/// Payload for `SYSMSI_GET_ATTRIBUTES` (response).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct GetAttributesResponse {
    pub status: i32,
    pub sys_num_msi: u32,
    pub flags0: u32,
    pub flags1: u32,
}

/// `SYSMSI_GET_MSI_ATTRIBUTES`.
pub const GET_MSI_ATTRIBUTES: u8 = 0x03;

/// Payload for `SYSMSI_GET_MSI_ATTRIBUTES` (request).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct GetMsiAttributesRequest {
    pub sys_msi_index: u32,
}

/// Payload for `SYSMSI_GET_MSI_ATTRIBUTES` (response).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct GetMsiAttributesResponse {
    pub status: i32,
    pub flags0: u32,
    pub flags1: u32,
    pub sys_msi_name: [u8; 16],
}

/// `SYSMSI_SET_MSI_STATE`.
pub const SET_MSI_STATE: u8 = 0x04;

/// Payload for `SYSMSI_SET_MSI_STATE` (request).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct SetMsiStateRequest {
    pub sys_msi_index: u32,
    pub sys_msi_state: u32,
}

/// Payload for `SYSMSI_SET_MSI_STATE` (response).
pub type SetMsiStateResponse = crate::message::StatusResponse;

/// `SYSMSI_GET_MSI_STATE`.
pub const GET_MSI_STATE: u8 = 0x05;

/// Payload for `SYSMSI_GET_MSI_STATE` (request).
pub type GetMsiStateRequest = GetMsiAttributesRequest;

/// Payload for `SYSMSI_GET_MSI_STATE` (response).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct GetMsiStateResponse {
    pub status: i32,
    pub sys_msi_state: u32,
}

/// `SYSMSI_SET_MSI_TARGET`.
pub const SET_MSI_TARGET: u8 = 0x06;

/// Payload for `SYSMSI_SET_MSI_TARGET` (request).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct SetMsiTargetRequest {
    pub sys_msi_index: u32,
    pub sys_msi_address_low: u32,
    pub sys_msi_address_high: u32,
    pub sys_msi_data: u32,
}

/// Payload for `SYSMSI_SET_MSI_TARGET` (response).
pub type SetMsiTargetResponse = crate::message::StatusResponse;

/// `SYSMSI_GET_MSI_TARGET`.
pub const GET_MSI_TARGET: u8 = 0x07;

/// Payload for `SYSMSI_GET_MSI_TARGET` (request).
pub type GetMsiTargetRequest = GetMsiAttributesRequest;

/// Payload for `SYSMSI_GET_MSI_TARGET` (response).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct GetMsiTargetResponse {
    pub status: i32,
    pub sys_msi_address_low: u32,
    pub sys_msi_address_high: u32,
    pub sys_msi_data: u32,
}
