#[cfg(target_os = "windows")]
pub(crate) mod windows;

#[cfg(target_os = "linux")]
pub(crate) mod linux;