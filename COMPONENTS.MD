# Components
<b>emulation</b> - Most of the relevant code is located here. contains the foundation for the Aarch64 and RISC-V frontends, along with the code supporting the user-mode emulation capability.


<b>system, vm-memory, misc</b> - Safe and platform-generic wrappers for system operations (mmap, event handling, etc..), along with definitions for some relevant data structures. Almost all the code in these files are from the rust-vmm project (which shares the same license as this project), but they are included in the repo instead of imported via Cargo because many of the crates required modifications and improvements. Much of the code in here is not actually used, but is still a dependency in the code that is used, so they are included anyway. 

<b>jit</b> - Where the future JIT and the various backends will be placed. It's basically empty now because it is currently unstable and being worked upon in my private branch, but the extract functions are still used by the emulation subcrate (hence the folder's presence).

<b>src</b> - the argument parsing and startup code is located here.