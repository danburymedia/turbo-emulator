use crate::threaded::x64::ThreadJit;


#[cfg(test)]
mod ThreadTest {
    use crate::base::{DataDesc, ExtVarDesc, ScribuntoBackend, ScribuntoOps};
    use crate::main::{DataDesc, ExtVarDesc};

    use crate::threaded::x64::{global_thread_ptr, ThreadJit};
    fn create_regs() -> Vec<ExtVarDesc> {
        let mut v: Vec<ExtVarDesc> = Vec::new();
        v.push(ExtVarDesc::create_new("r0".to_string()));
        v.push(ExtVarDesc::create_new("r1".to_string()));
        v.push(ExtVarDesc::create_new("r2".to_string()));
        v.push(ExtVarDesc::create_new("r3".to_string()));
        v.push(ExtVarDesc::create_new("r4".to_string()));
        v.push(ExtVarDesc::create_new("r5".to_string()));
        v.push(ExtVarDesc::create_new("r6".to_string()));
        v.push(ExtVarDesc::create_new("r7".to_string()));
        v.push(ExtVarDesc::create_new("r8".to_string()));
        v.push(ExtVarDesc::create_new("r9".to_string()));
        v
    }
    pub struct ThreadTest {
        pub tj: ThreadJit
    }
    impl ThreadTest {
        pub fn init_tt() -> ThreadTest {
            let l = ThreadJit::init();
            let mut tt = ThreadTest {
                tj: l
            };
            unsafe {
                global_thread_ptr = &mut tt.tj;
            }
           // tt.tj.init_guest_registers(create_regs()).unwrap(); for now
            tt
        }
    }
    #[test]
    fn test_reg_arith() {
        let mut tt = ThreadTest::init_tt();
        tt.tj.new_block(0).unwrap();
        let imm1 = DataDesc::new_imm(1);
        let imm2 = DataDesc::new_imm(1);
        tt.tj.add(DataDesc::new_register(0), imm1, imm2);
        tt.tj.end_block().unwrap();
        tt.tj.exec_block(0).unwrap();
        assert_eq!(tt.tj.regs[0], 2);
    }
}