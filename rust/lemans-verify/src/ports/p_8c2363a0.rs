//! 0x8c2363a0 - store 1 or 0 at 0x8c2c9db4, or return 0xeeee.
//!
//! 0x8c237600 returning nonzero takes the 0xeeee path. Otherwise the stored
//! value is 1 when the entry r4 is nonzero. The 0 is the `rts` delay slot.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    cpu.r[3] = mem.read_u32(0x8c2363e0).unwrap_or(0x8c237600);
    let _ = mem.write_u32(cpu.r[15], cpu.r[4]); // jsr delay
    cpu.pr = 0x8c2363aa;
    crate::registry::p_8c237600::run(cpu, mem);
    cpu.set_t(cpu.r[0] == 0); // tst r0,r0
    if !cpu.t() {
        cpu.r[0] = mem.read_u32(0x8c2363e8).unwrap_or(0xeeee);
        cpu.r[15] = cpu.r[15].wrapping_add(4);
        cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
        cpu.r[15] = cpu.r[15].wrapping_add(4);
        return;
    }
    cpu.r[2] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.set_t(cpu.r[2] == 0); // tst r2,r2
    if !cpu.t() {
        cpu.r[3] = 1; // bra delay
    } else {
        cpu.r[3] = 0;
    }
    cpu.r[1] = mem.read_u32(0x8c2363e4).unwrap_or(0x8c2c9db4);
    let _ = mem.write_u32(cpu.r[1], cpu.r[3]);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[0] = 0; // rts delay
}
