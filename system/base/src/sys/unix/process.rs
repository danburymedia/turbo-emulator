// Copyright 2022 The ChromiumOS Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

//! Provides [fork_process] to fork a process.

#![deny(missing_docs)]

use std::ffi::CString;
use std::mem::ManuallyDrop;
use std::os::unix::process::ExitStatusExt;
use std::process;

use log::warn;

use crate::error;
use crate::unix::wait_for_pid;
use crate::unix::Pid;
use crate::RawDescriptor;

/// Child represents the forked process.
pub struct Child {
    /// The pid of the child process.
    pub pid: Pid,
}

impl Child {
    /// wait for the child process exit using `waitpid(2)`.
    pub fn wait(self) -> crate::Result<u8> {
        let (_, status) = wait_for_pid(self.pid, 0)?;
        // suppress warning from the drop().
        let _ = ManuallyDrop::new(self);
        if let Some(exit_code) = status.code() {
            Ok(exit_code as u8)
        } else if let Some(signal) = status.signal() {
            let exit_code = if signal as i32 >= 128 {
                warn!("wait for child: unexpected signal({:?})", signal);
                255
            } else {
                128 + signal as u8
            };
            Ok(exit_code)
        } else {
            unreachable!("waitpid with option 0 only waits for exited and signaled status");
        }
    }
}

impl Drop for Child {
    fn drop(&mut self) {
        warn!("the child process have not been waited.");
    }
}