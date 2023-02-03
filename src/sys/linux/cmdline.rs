// Copyright 2017 The Chromium OS Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use argh::FromArgs;
use crate::config::from_key_values;

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "runuser")]
/// Load and run an ELF executable
pub struct RunUserCommand {
    #[argh(positional)]
    /// the absolute path of an executable file to load and run
    pub exec_path: String,

    #[argh(positional, greedy)]
    /// arguments for the executable file
    pub args: Vec<String>,
}

#[derive(FromArgs)]
#[argh(subcommand)]
/// Unix Commands
pub enum Commands {
    #[cfg(target_os = "linux")]
    RunUser(RunUserCommand)

}
