//! RISC-V Platform Management Interface constants and runtime library.

#![no_std]

pub mod base;
pub mod clock;
pub mod cppc;
pub mod device_power;
pub mod hsm;
pub mod management_mode;
pub mod message;
pub mod performance;
pub mod ras_agent;
pub mod request_forward;
pub mod shmem;
pub mod status;
pub mod system_msi;
pub mod system_reset;
pub mod system_suspend;
pub mod voltage;
