use argh::FromArgs;
#[derive(FromArgs)]
/// General Turbo Emulator settings
pub struct GeneralCmdlineArgs {
    #[argh(switch)]
    /// use extended exit status
    pub extended_status: bool,
    #[argh(option, default = r#"String::from("info")"#)]
    /// specify log level, eg "off", "error", "debug,disk=off", etc
    pub log_level: String,
    #[argh(option, arg_name = "TAG")]
    /// when logging to syslog, use the provided tag
    pub syslog_tag: Option<String>,
    #[argh(option, arg_name = "PATH")]
    /// usermode emulator sysroot
    pub usermode_directory: Option<String>,
    #[argh(switch)]
    /// disable output to syslog
    pub no_syslog: bool,
    #[argh(subcommand)]
    pub command: Command,
}
#[allow(clippy::large_enum_variant)]
#[derive(FromArgs)]
#[argh(subcommand)]
pub enum CrossPlatformCommands {
}

#[allow(clippy::large_enum_variant)]
#[derive(argh_helpers::FlattenSubcommand)]
pub enum Command {
    // CrossPlatform(CrossPlatformCommands),
    Sys(crate::sys::platform::cmdline::Commands),
}