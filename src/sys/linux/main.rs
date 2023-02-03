// Copyright 2022 The Chromium OS Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::fs::OpenOptions;
use std::thread::sleep;
use std::time::Duration;

use anyhow::anyhow;
use anyhow::Context;
use base::kill_process_group;
use base::reap_child;
use base::syslog;
use base::syslog::LogConfig;
use base::warn;
use crate::CommandStatus;
use crate::Config;
use crate::sys::platform::cmdline::Commands;

// Wait for all children to exit. Return true if they have all exited, false
// otherwise.
fn wait_all_children() -> bool {
    const CHILD_WAIT_MAX_ITER: isize = 100;
    const CHILD_WAIT_MS: u64 = 10;
    for _ in 0..CHILD_WAIT_MAX_ITER {
        loop {
            match reap_child() {
                Ok(0) => break,
                // We expect ECHILD which indicates that there were no children left.
                Err(e) if e.errno() == libc::ECHILD => return true,
                Err(e) => {
                    warn!("error while waiting for children: {}", e);
                    return false;
                }
                // We reaped one child, so continue reaping.
                _ => {}
            }
        }
        // There's no timeout option for waitpid which reap_child calls internally, so our only
        // recourse is to sleep while waiting for the children to exit.
        sleep(Duration::from_millis(CHILD_WAIT_MS));
    }

    // If we've made it to this point, not all of the children have exited.
    false
}

pub(crate) fn cleanup() {
    // Reap exit status from any child device processes. At this point, all devices should have been
    // dropped in the main process and told to shutdown. Try over a period of 100ms, since it may
    // take some time for the processes to shut down.
    if !wait_all_children() {
        // We gave them a chance, and it's too late.
        warn!("not all child processes have exited; sending SIGKILL");
        if let Err(e) = kill_process_group() {
            // We're now at the mercy of the OS to clean up after us.
            warn!("unable to kill all child processes: {}", e);
        }
    }
}

pub fn get_library_watcher() -> std::io::Result<()> {
    Ok(())
}

pub(crate) fn run_command(command: Commands) -> anyhow::Result<()> {
    match command {
        _ => Ok(()),
        // Commands::Devices(cmd) => start_devices(cmd).context("start_devices subcommand failed"), todo comment
    }
}

pub(crate) fn init_log<F: 'static>(log_config: LogConfig<F>, cfg: &Config) -> anyhow::Result<()>
    where
        F: Fn(&mut syslog::fmt::Formatter, &log::Record<'_>) -> std::io::Result<()> + Sync + Send,
{
    if let Err(e) = syslog::init_with(LogConfig {
        proc_name: if let Some(ref tag) = cfg.syslog_tag {
            tag.to_string()
        } else {
            String::from("turbo-emulator")
        },
        pipe: if let Some(log_file_path) = &cfg.log_file {
            let file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(log_file_path)
                .unwrap();
            Some(Box::new(file))
        } else {
            log_config.pipe
        },
        ..log_config
    }) {
        eprintln!("failed to initialize syslog: {}", e);
        return Err(anyhow!("failed to initialize syslog: {}", e));
    }
    Ok(())
}
pub(crate) fn init_log_nocfg<F: 'static>(log_config: LogConfig<F>, syslog_tag: Option<String>, log_file: Option<String>) -> anyhow::Result<()>
    where
        F: Fn(&mut syslog::fmt::Formatter, &log::Record<'_>) -> std::io::Result<()> + Sync + Send,
{
    if let Err(e) = syslog::init_with(LogConfig {
        proc_name: if let Some(ref tag) = syslog_tag {
            tag.to_string()
        } else {
            String::from("turbo-emulator")
        },
        pipe: if let Some(log_file_path) = log_file {
            let file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(log_file_path)
                .unwrap();
            Some(Box::new(file))
        } else {
            log_config.pipe
        },
        ..log_config
    }) {
        eprintln!("failed to initialize syslog: {}", e);
        return Err(anyhow!("failed to initialize syslog: {}", e));
    }
    Ok(())
}

pub(crate) fn error_to_exit_code(_res: &std::result::Result<CommandStatus, anyhow::Error>) -> i32 {
    1
}
