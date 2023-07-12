# Components
<b>emulation</b> - Most of the relevant code is located here. contains the foundation for the Aarch64 and RISC-V frontends, along with the code supporting the user-mode emulation capability.

<b>system</b> - Bindings and abstractions for OS libraries, syscalls, and features such as POSIX, KVM, etc... Mostly from the rust-vmm project.

<b>third-party</b> - assorted third party crates which provide useful functionality.

<b>vm-memory</b> - crate that manages guest memory allocation in system mode emulation. From the rust-vmm project, with some changes were made specifically for Turbo Emulator.

<b>jit</b> - Where the future JIT and the various backends will be placed. It's basically empty now because it is currently unstable and being worked upon in my private branch, but the extract functions are still used by the emulation subcrate (hence the folder's presence).

<b>src</b> - the argument parsing and startup code is located here.