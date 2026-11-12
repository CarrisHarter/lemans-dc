//! 0x8c219fb6 - tail-dispatch on r4 to one of ten field accessors.
//!
//! `1,2,4,3,6,5,0,7,8,9` jump to 0x8c21b0ac, 0x8c21b07a, 0x8c21b100,
//! 0x8c21b0c0, 0x8c21b0ec, 0x8c21b098, 0x8c21b11a, 0x8c21b138, 0x8c21b14c,
//! 0x8c21b186. Anything else returns -1. `jmp` does not touch pr, so the
//! callee returns to our caller. T on a hit is whatever the callee left
//! (the selecting `cmp/eq` set it to 1, and these loads do not clear it).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = cpu.r[4]; // mov r4,r0
    let sel = cpu.r[0];
    let table: [(u32, u32, fn(&mut Cpu, &mut Memory)); 10] = [
        (1, 0x8c21a084, crate::registry::p_8c21b0ac::run),
        (2, 0x8c21a088, crate::registry::p_8c21b07a::run),
        (4, 0x8c21a08c, crate::registry::p_8c21b100::run),
        (3, 0x8c21a090, crate::registry::p_8c21b0c0::run),
        (6, 0x8c21a094, crate::registry::p_8c21b0ec::run),
        (5, 0x8c21a098, crate::registry::p_8c21b098::run),
        (0, 0x8c21a09c, crate::registry::p_8c21b11a::run),
        (7, 0x8c21a0a0, crate::registry::p_8c21b138::run),
        (8, 0x8c21a0a4, crate::registry::p_8c21b14c::run),
        (9, 0x8c21a0a8, crate::registry::p_8c21b186::run),
    ];
    for (imm, pool, callee) in table {
        cpu.set_t(sel == imm); // cmp/eq #imm,r0
        if cpu.t() {
            let reg_is_r3 = imm != 2 && imm != 3 && imm != 5 && imm != 7 && imm != 9;
            let target = mem.read_u32(pool).unwrap_or(0);
            if reg_is_r3 {
                cpu.r[3] = target;
            } else {
                cpu.r[2] = target;
            }
            callee(cpu, mem);
            return;
        }
    }
    cpu.r[0] = 0xffff_ffff; // rts delay: mov #-1,r0
}
