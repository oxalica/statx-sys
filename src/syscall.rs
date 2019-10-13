#![allow(non_upper_case_globals)]
use libc::c_long;

#[cfg(target_arch = "aarch64")]
pub const SYS_statx: c_long = 291;

#[cfg(target_arch = "arm")]
pub const SYS_statx: c_long = 397;

#[cfg(target_arch = "x86")]
pub const SYS_statx: c_long = 383;

#[cfg(target_arch = "mips")]
pub const SYS_statx: c_long = 4366;

#[cfg(target_arch = "mips64")]
pub const SYS_statx: c_long = 5326;

#[cfg(target_arch = "mipsn32")]
pub const SYS_statx: c_long = 6330;

#[cfg(target_arch = "or1k")]
pub const SYS_statx: c_long = 291;

#[cfg(target_arch = "powerpc")]
pub const SYS_statx: c_long = 383;

#[cfg(target_arch = "powerpc64")]
pub const SYS_statx: c_long = 383;

#[cfg(target_arch = "s390x")]
pub const SYS_statx: c_long = 379;

#[cfg(target_arch = "x32")]
pub const SYS_statx: c_long = 0x40000000 + 332;

#[cfg(target_arch = "x86_64")]
pub const SYS_statx: c_long = 332;
