# rpmi

[RISC-V RPMI 1.0](https://github.com/riscv-non-isa/riscv-rpmi/tree/v1.0)
constants and `#[repr(C)]` data layouts. `no_std`, no dependencies.

| Modules | Purpose |
| --- | --- |
| `Queue`, `message`, `status`, `base` | Transport layout, messages, discovery |
| `hsm`, `system_reset`, `system_suspend`, `system_msi` | Hart/system control, interrupts |
| `clock`, `voltage`, `device_power`, `cppc`, `performance` | Power and performance |
| `management_mode`, `ras_agent`, `request_forward` | Management, errors, forwarding |

Only standard definitions are included. Probe service-group support on the target.
Multi-byte fields require little-endian encoding. Zero-length arrays mark variable
tails; their storage is not included in `size_of`. These types perform no I/O.

`tests/standard.rs` checks service IDs and layouts against the pinned specification.

Contributions are welcomed!
