cfg_if::cfg_if! {
    if #[cfg(target_os = "linux")] {
        pub mod linux;
        pub use linux as platform;
        // pub(crate) use crate::sys::linux::{run_config, ExitState};
    } else if #[cfg(windows)] {
        pub(crate) mod windows;
        use windows as platform;
        pub(crate) use windows::ExitState;
        pub(crate) use windows::run_config;
    } else {
        compile_error!("Unsupported platform");
    }
}