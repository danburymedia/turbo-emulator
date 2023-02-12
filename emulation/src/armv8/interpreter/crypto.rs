use crate::armv8::decodedefs::ArmInstr;
use crate::armv8::interpreter::main::Arm64Cpu;
use crate::common::arm_crypto::*;

pub fn crypto_aes(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    let opcode =  ((arg.insn >> 12) & 0x1f);
    match opcode {
        0b00100 | 0b00101 => {
            // aese, aesd
            let op1 = ai.vreg[arg.get_rd() as usize].vect.to_ne_bytes();
            let op2 = ai.vreg[arg.get_rn() as usize].vect.to_ne_bytes();
            let mut res: [u8; 16] = [0; 16];
            if opcode & 1 != 0 {
                aes_decrypt(&mut res, &op1, &op2);
            } else {
                aes_encrypt(&mut res, &op1, &op2);
            }
            ai.vreg[arg.get_rd() as usize].vect = u128::from_ne_bytes(res);
        },
        0b00110 | 0b00111 => {
            let op = ai.vreg[arg.get_rn() as usize].vect.to_ne_bytes();
            let mut res: [u8; 16] = [0; 16];
            if opcode & 1 != 0 {
                aes_inv_mix_columns(&mut res, &op);
            } else {
                aes_mix_columns(&mut res, &op);
            }
            ai.vreg[arg.get_rd() as usize].vect = u128::from_ne_bytes(res);
        }
        _ => panic!()
    }
}
pub fn crypto_sha_register_3(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    let opcode =  ((arg.insn >> 12) & 0b111);
    let mut x = ai.vreg[arg.get_rd() as usize].vect.to_ne_bytes();
    let y = ai.vreg[arg.get_rn() as usize].vect.to_ne_bytes();
    let w = ai.vreg[arg.get_rm() as usize].vect.to_ne_bytes();
    let mut result: u128;
    match opcode {
        0 => {
            sha1c(&mut x, &y, &w);
        },
        1 => {
            sha1p(&mut x, &y, &w);
        }
        2 => {
            sha1m(&mut x, &y, &w);
        }
        3 => {
            sha1su0(&mut x, &y, &w);
        },
        4 => {
            sha256h(&mut x, &y, &w);
        }
        5 => {
            sha256h2(&mut x, &y, &w);
        }
        6 => {
            sha256su1(&mut x, &y, &w);
        },
        _ => panic!()
    }
    result = u128::from_ne_bytes(x);
    ai.vreg[arg.get_rd() as usize].vect = result;
}
pub fn crypto_sha_register_2(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    let opcode =  ((arg.insn >> 12) & 0b11111);
    let mut x = ai.vreg[arg.get_rd() as usize].vect.to_ne_bytes();
    let y = ai.vreg[arg.get_rn() as usize].vect.to_ne_bytes();
    let mut result: u128;
    match opcode {
        0 => {
            sha1h(&mut x, &y);
        },
        1 => {
            sha1su1(&mut x, &y);
        }
        2 => {
            sha256su0(&mut x, &y);
        }
        _ => panic!()
    }
    result = u128::from_ne_bytes(x);
    ai.vreg[arg.get_rd() as usize].vect = result;
}