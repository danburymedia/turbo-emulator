// gdb interface (32-bit)

use gdbstub::arch::Arch;
use gdbstub::common::Signal;
use gdbstub::conn::{Connection, ConnectionExt};
use gdbstub::stub::{BaseStopReason, DisconnectReason, GdbStub, GdbStubError, run_blocking, SingleThreadStopReason};
use gdbstub::stub::run_blocking::{Event, WaitForStopReasonError};
use gdbstub::target;
use gdbstub::target::ext::base::singlethread::{SingleThreadBase, SingleThreadResume, SingleThreadResumeOps, SingleThreadSingleStep, SingleThreadSingleStepOps};
use gdbstub::target::{Target, TargetError, TargetResult};
use gdbstub::target::ext::base::single_register_access::{SingleRegisterAccess, SingleRegisterAccessOps};
use gdbstub::target::ext::breakpoints::{Breakpoints, SwBreakpoint, SwBreakpointOps};
use crate::riscv::interpreter::main::RiscvInt;
use gdbstub_arch;
use gdbstub_arch::riscv::reg::id::RiscvRegId;
use crate::debug::{DebugEvent, DebugExecMode, DebugRunEvent, wait_for_tcp};
use crate::riscv::common::{get_privilege_encoding, get_privilege_mode, Trap};

pub struct Riscv32DebugWrapper {
    pub icpu: RiscvInt,
    pub breakpoints: Vec<u64>,
    pub exec_mode: DebugExecMode,

}
impl Riscv32DebugWrapper {
    fn single_step(&mut self) -> Option<DebugEvent> {
        self.icpu.debug_step(self.breakpoints.clone());
        let pc = self.icpu.get_pc_of_current_instr() as u64;
        if self.breakpoints.contains(&pc) {
            return Some(DebugEvent::Break);
        }
        None

    }
    pub fn run_debug(&mut self) {
        let connection: Box<dyn ConnectionExt<Error = std::io::Error>> = {
            Box::new(wait_for_tcp(9001).unwrap())
        };
        let gdb = GdbStub::new(connection);
        // todo: propper logging
        match gdb.run_blocking::<EmuGdbEventLoop>(self) {
            Ok(disconnect_reason) => match disconnect_reason {
                DisconnectReason::Disconnect => {
                    println!("GDB client has disconnected. Running to completion...");
                    while self.single_step() != Some(DebugEvent::Halted) {}
                }
                DisconnectReason::TargetExited(code) => {
                    println!("Target exited with code {}!", code)
                }
                DisconnectReason::TargetTerminated(sig) => {
                    println!("Target terminated with signal {}!", sig)
                }
                DisconnectReason::Kill => println!("GDB sent a kill command!"),
            },
            Err(GdbStubError::TargetError(e)) => {
                println!("target encountered a fatal error: {}", e)
            }
            Err(e) => {
                println!("gdbstub encountered a fatal error: {}", e)
            }
        }
    }
    fn run_debug_internal(&mut self,
                 mut poll_incoming_data: impl FnMut() -> bool) -> DebugRunEvent {
        match self.exec_mode {
            DebugExecMode::Step => DebugRunEvent::Event(self.single_step().unwrap_or(DebugEvent::DoneStep)),
            DebugExecMode::Continue => {
                let mut cycles = 0;
                loop {
                    if cycles % 1024 == 0 {
                        // poll for incoming data
                        if poll_incoming_data() {
                            break DebugRunEvent::IncomingData;
                        }
                    }
                    cycles += 1;

                    if let Some(event) = self.single_step() {
                        break DebugRunEvent::Event(event);
                    };
                }
            }
            DebugExecMode::RangeStep(start, end) => {
                let mut cycles = 0;
                loop {
                    if cycles % 1024 == 0 {
                        // poll for incoming data
                        if poll_incoming_data() {
                            break DebugRunEvent::IncomingData;
                        }
                    }
                    cycles += 1;

                    if let Some(event) = self.single_step() {
                        break DebugRunEvent::Event(event);
                    };
                    let pc = self.icpu.get_pc_of_current_instr() as u32 as u64;
                    if (start..end).contains(&pc) {
                        break DebugRunEvent::Event(DebugEvent::DoneStep);
                    }
                }
            }
        }
    }
}
enum EmuGdbEventLoop {}
impl run_blocking::BlockingEventLoop for EmuGdbEventLoop {
    type Target = Riscv32DebugWrapper;
    type Connection = Box<dyn ConnectionExt<Error = std::io::Error>>;
    type StopReason = SingleThreadStopReason<u32>;
    fn wait_for_stop_reason(target: &mut Self::Target,
                            conn: &mut Self::Connection) -> Result<
        Event<Self::StopReason>,
        WaitForStopReasonError<<Self::Target as Target>::Error,
            <Self::Connection as Connection>::Error>> {

        let poll_incoming_data = || {
            // gdbstub takes ownership of the underlying connection, so the `borrow_conn`
            // method is used to borrow the underlying connection back from the stub to
            // check for incoming data.
            conn.peek().map(|b| b.is_some()).unwrap_or(true)
        };
        match target.run_debug_internal(poll_incoming_data) {
            DebugRunEvent::IncomingData => {
                let byte = conn
                    .read()
                    .map_err(run_blocking::WaitForStopReasonError::Connection)?;
                Ok(run_blocking::Event::IncomingData(byte))
            }
            DebugRunEvent::Event(event) => {
                use gdbstub::target::ext::breakpoints::WatchKind;
                let stop_reason = match event {
                    DebugEvent::DoneStep => SingleThreadStopReason::DoneStep,
                    DebugEvent::Halted => SingleThreadStopReason::Terminated(Signal::SIGSTOP),
                    DebugEvent::Break => SingleThreadStopReason::SwBreak(()),
                    DebugEvent::WatchWrite(addr) => SingleThreadStopReason::Watch {
                        tid: (),
                        kind: WatchKind::Write,
                        addr: addr as u32,
                    },
                    DebugEvent::WatchRead(addr) => SingleThreadStopReason::Watch {
                        tid: (),
                        kind: WatchKind::Read,
                        addr: addr as u32,
                    },
                };
                Ok(run_blocking::Event::TargetStopped(stop_reason))

            }
        }

    }
    fn on_interrupt(target: &mut Self::Target) -> Result<Option<Self::StopReason>, <Self::Target as Target>::Error> {
        Ok(Some(SingleThreadStopReason::Signal(Signal::SIGINT)))
    }
}
impl Target for Riscv32DebugWrapper {
    type Arch = gdbstub_arch::riscv::Riscv32;
    type Error = &'static str;
    #[inline(always)]
    fn base_ops(&mut self) -> target::ext::base::BaseOps<Self::Arch, Self::Error> {
        target::ext::base::BaseOps::SingleThread(self)
    }
    #[inline(always)]
    fn support_breakpoints(
        &mut self,
    ) -> Option<target::ext::breakpoints::BreakpointsOps<'_, Self>> {
        Some(self)
    }

}
impl SingleThreadBase for Riscv32DebugWrapper {
    fn support_resume(&mut self) -> Option<SingleThreadResumeOps<'_, Self>> {
        Some(self)
    }
    fn support_single_register_access(&mut self) -> Option<SingleRegisterAccessOps<'_, (), Self>> {
        Some(self)
    }
    fn read_registers(&mut self, regs: &mut gdbstub_arch::riscv::reg::RiscvCoreRegs<u32>) -> TargetResult<(), Self> {
        for (idx, v) in self.icpu.regs.iter().enumerate() {
            regs.x[idx] = (*v) as u32;
        }
        regs.pc = self.icpu.pc as u32;
        Ok(())
    }

    fn write_registers(&mut self, regs: &gdbstub_arch::riscv::reg::RiscvCoreRegs<u32>) -> TargetResult<(), Self> {
        for i in 0..self.icpu.regs.len() {
            self.icpu.regs[i] = regs.x[i] as u64;
        }
        self.icpu.pc = regs.pc as u64;
        Ok(())

    }

    fn read_addrs(&mut self, start_addr: u32, data: &mut [u8]) -> TargetResult<(), Self> {
        match self.icpu.readx(start_addr as u64,
                                   data.len() as u64, false, false) {
            Ok(p) => {
                data.copy_from_slice(&p);
                Ok(())
            }
            Err(_) => {
                Err(TargetError::NonFatal)

            }
        }
    }

    fn write_addrs(&mut self, start_addr: u32, data: &[u8]) -> TargetResult<(), Self> {
        match self.icpu.writex(start_addr as u64, data.to_vec(), false) {
            Ok(_) => {
                Ok(())

            }
            Err(_) => {
                Err(TargetError::NonFatal)

            }
        }
    }
}
impl SingleRegisterAccess<()> for Riscv32DebugWrapper {
    fn read_register(&mut self,
                     tid: (),
                     reg_id: gdbstub_arch::riscv::reg::id::RiscvRegId<u32>,
                     buf: &mut [u8]) -> TargetResult<usize, Self> {
        match reg_id {
            RiscvRegId::Gpr(g) => {
                let val = self.icpu.regs[g as usize] as u32;
                buf.copy_from_slice(&val.to_le_bytes());
                Ok(buf.len())
            }
            RiscvRegId::Fpr(f) => {
                let val = self.icpu.fregs[f as usize] as u32;
                buf.copy_from_slice(&val.to_le_bytes());
                Ok(buf.len())
            }
            RiscvRegId::Pc => {
                let val = self.icpu.get_pc_of_current_instr() as u32;
                buf.copy_from_slice(&val.to_le_bytes());
                Ok(buf.len())
            }
            RiscvRegId::Csr(c) => {
                let val = self.icpu.get_csr_raw(c as usize) as u32;
                buf.copy_from_slice(&val.to_le_bytes());
                Ok(buf.len())
            }
            RiscvRegId::Priv => {
                let val = get_privilege_encoding(self.icpu.prvmode) as u32;
                buf.copy_from_slice(&val.to_le_bytes());
                Ok(buf.len())
            }
            RiscvRegId::_Marker(_)  => {
                Err(().into())
            }
            _ => {
                Err(().into())

            }
        }
    }

    fn write_register(&mut self, tid: (),
                      reg_id: <Self::Arch as Arch>::RegId,
                      val: &[u8]) -> TargetResult<(), Self> {
        let val = u32::from_le_bytes(
            val.try_into().unwrap()
        );
        match reg_id {
            RiscvRegId::Gpr(g) => {
                self.icpu.regs[g as usize] = val as u64;
                Ok(())
            }
            RiscvRegId::Fpr(f) => {
                self.icpu.fregs[f as usize] = val as u64;
                Ok(())
            }
            RiscvRegId::Pc => {
                self.icpu.want_pc = Some(val as u64);
                self.icpu.stop_exec = true;
                Ok(())

            }
            RiscvRegId::Csr(t) => {
                self.icpu.csr[t as usize] = val as u64;
                Ok(())
            }
            RiscvRegId::Priv => {
                self.icpu.prvmode = get_privilege_mode(val as u64);
                Ok(())

            }
            RiscvRegId::_Marker(_) | _ => {
                // no - op
                Ok(())
            }
        }
    }
}

impl Breakpoints for Riscv32DebugWrapper {
    fn support_sw_breakpoint(&mut self) -> Option<SwBreakpointOps<'_, Self>> {
        Some(self)
    }

}

impl SwBreakpoint for Riscv32DebugWrapper {
    fn add_sw_breakpoint(&mut self, addr: <Self::Arch as Arch>::Usize, kind: <Self::Arch as Arch>::BreakpointKind) -> TargetResult<bool, Self> {
        self.breakpoints.push(addr as u64);
        Ok(true)
    }

    fn remove_sw_breakpoint(&mut self, addr: <Self::Arch as Arch>::Usize, kind: <Self::Arch as Arch>::BreakpointKind) -> TargetResult<bool, Self> {
        match self.breakpoints.iter().position(|x| *x == (addr as u64)) {
            None => return Ok(false),
            Some(pos) => self.breakpoints.remove(pos),
        };

        Ok(true)
    }
}
impl SingleThreadResume for Riscv32DebugWrapper {
    fn resume(&mut self, signal: Option<Signal>) -> Result<(), Self::Error> {
        self.exec_mode = DebugExecMode::Continue;
        Ok(())
    }
    fn support_single_step(&mut self) -> Option<SingleThreadSingleStepOps<'_, Self>> {
        Some(self)
    }
}
impl SingleThreadSingleStep for Riscv32DebugWrapper {
    fn step(&mut self, signal: Option<Signal>) -> Result<(), Self::Error> {
        if signal.is_some() {
            return Err("no support for stepping with signal");
        }
        self.exec_mode = DebugExecMode::Step;
        Ok(())

    }
}