
# Turbo Emulator
Turbo Emulator is a processor emulator written in the Rust programming language. It currently supports user-mode emulation of RISC-V and AArch64 Linux programs.

## Building
Simply use "cargo build --release".

## Running

There is one binary for all the supported architectures. To run an aarch64 or riscv binary, simply run "turbo --usermode-directory <i>sysroot</i> runuser -- <i>executable name</i>", where the "sysroot" is the guest architecture sysroot directory (needed for dynamically linked executables) and "executable name" is the directory path of the program you'd like to run.

Do not use "cargo run", it messes up the way arguments are processed. Instead, run it directly from the "target" directory.

## Reporting a bug
To report a user mode emulation bug, run the emulator with the "--log-level debug" argument. It can be placed anywhere after the executable name but before the "runuser" part of it. Then paste the resulting logs, along with your issue, in a Github issue report.

It is not required but highly recommended that we know what the program is, to aid in the fixing of the bugs.  If the program is not already publicly available on the internet, then a compiled version (preferably with debugging symbols/info included) can be uploaded to a file host of your choice for us to look at.

In general, the more info we are able to use to reproduce the bug, the faster we are able to fix it.