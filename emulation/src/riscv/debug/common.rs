use crate::riscv::interpreter::main::RiscvInt;
cfg_if::cfg_if! {
    if #[cfg(feature = "linux-usermode")] {
        use crate::riscv::ume::signals::setup_rt_frame;
        use crate::linux_usermode::signals::{SIGNAL_AVAIL, SINFO};
        use crate::riscv::common::Exception::EnvironmentCallFromMMode;
    }
}
impl RiscvInt {
    pub fn debug_step(&mut self, bpoints: Vec<u64>) {
        loop {
            self.step_one_instr();
            if self.stop_exec || bpoints.contains(&self.pc) { // todo: use pc function
                break;
            }
        }
        if self.trap.is_some() {
            if self.usermode {
                #[cfg(feature = "linux-usermode")]
                {
                    let trp = self.trap.unwrap();
                    if trp.ttype == EnvironmentCallFromMMode {
                        self.handle_syscall();
                        self.stop_exec = false;
                        self.trap = None;

                    } else {
                        panic!("Protection error  - Suffered RISCV trap in user mode: {:?}", self.trap.unwrap())
                    }
                }
                #[cfg(not(feature = "linux-usermode"))]
                {
                    unreachable!("usermode functionality not included but CPU has usermode variable set")
                }
            } else {
                self.handle_trap(self.trap.unwrap(), self.trap_pc);
                self.trap_pc = 0;
                self.trap = None;
                self.want_pc = None;
                self.wfi = false;
                self.stop_exec = false;
                return;;
            }

        }
        #[cfg(feature = "linux-usermode")]
        {
            if self.usermode {
                SIGNAL_AVAIL.with(|z| {
                    let mut zz = z.borrow_mut();
                    if *zz == true {
                        // signal
                        SINFO.with(|a| {
                            let mut aa = a.borrow_mut();
                            let signum = aa.use_idx.unwrap();
                            setup_rt_frame(self, signum as i32, &mut aa);
                        });
                        *zz = false; // we will unblock signals later
                    }
                });
            }

        }
        if let Some(f) = self.want_pc {
            // todo: any checks?
            self.pc = f;
            self.want_pc = None;
        }
        if self.wfi {
            unimplemented!();
        }
        self.stop_exec = false;
    }
}