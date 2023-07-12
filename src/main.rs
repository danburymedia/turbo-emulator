pub mod sys;
pub mod config;
pub mod cmdline;
use anyhow::Result;
use base::syslog::{Log, LogConfig, Metadata};
use argh::FromArgs;
#[cfg(feature = "linux-usermode")]
use emulation::elf::init_user_mode_emulation;
use log::{info, Record};
use crate::config::*;
use crate::sys::platform::cmdline::Commands;
use crate::cmdline::{Command, CrossPlatformCommands, GeneralCmdlineArgs};
use crate::sys::platform::main::init_log_nocfg;

fn main() {
    gen_main().unwrap();
    println!("Emulator is done running.");
}

pub enum CommandStatus {
    Success,
    InvalidArgs,
}
fn linux_sys_cmd(c: crate::sys::platform::cmdline::Commands, usermode: Option<String>) -> Result<CommandStatus> {
    match c {
        #[cfg(feature = "linux-usermode")]
        Commands::RunUser(userm) => {
            init_user_mode_emulation(userm.exec_path, userm.args,
                                     usermode.unwrap_or(String::from(""))).unwrap();
            // probably will not return after this

        }
        Commands::Nothing(_) => {
            println!("This does nothing.");
        }

    }
    Ok(CommandStatus::Success)
}
fn gen_main() -> Result<CommandStatus> {
    let args = prepare_argh_args(std::env::args());
    let args = args.iter().map(|s| s.as_str()).collect::<Vec<_>>();
    let args: GeneralCmdlineArgs = match crate::cmdline::GeneralCmdlineArgs::from_args(&args[..1], &args[1..]) {
        Ok(args) => args,
        Err(e) => {
            eprintln!("arg parsing failed: {}", e.output);
            return Ok(CommandStatus::InvalidArgs);
        }
    };
    let extended_status = args.extended_status;
    info!("CLI arguments parsed.");
    let mut log_config = LogConfig {
        filter: &args.log_level,
        proc_name: args.syslog_tag.unwrap_or("turbo_emulator".to_string()),
        syslog: !args.no_syslog,
        ..Default::default()
    };
    let ret = match args.command {
        Command::Sys(z) => {
            // when we do other oses take care of this
            init_log_nocfg(log_config, None, None).unwrap();
            linux_sys_cmd(z, args.usermode_directory)
        }
    };
    // Code below this point may not run
    ret.map(|s| {
        if extended_status {
            s
        } else {
            CommandStatus::Success
        }
    })
}
struct SimpleLogger;

impl Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        println!("Slog - {} - {}", record.level(), record.args());
    }

    fn flush(&self) {}
}
static LOGGER: SimpleLogger = SimpleLogger;
// Returns true if the argument is a flag (e.g. `-s` or `--long`).
//
// As a special case, `-` is not treated as a flag, since it is typically used to represent
// `stdin`/`stdout`.
fn is_flag(arg: &str) -> bool {
    arg.len() > 1 && arg.starts_with('-')
}
fn prepare_argh_args<I: IntoIterator<Item = String>>(args_iter: I) -> Vec<String> {
    let mut args: Vec<String> = Vec::default();
    for arg in args_iter {
        match arg.as_str() {
            "-h" => args.push("--help".to_string()),
            arg if is_flag(arg) => {
                // Split `--arg=val` into `--arg val`, since argh doesn't support the former.
                if let Some((key, value)) = arg.split_once("=") {
                    args.push(key.to_string());
                    args.push(value.to_string());
                } else {
                    args.push(arg.to_string());
                }
            }
            arg => args.push(arg.to_string()),
        }
    }

    args
}