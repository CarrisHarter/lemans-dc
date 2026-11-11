//! 0x8c06be1c — two-phase indirect store through r4's struct.
//!
//! Phase 1: if *(r4+72) != 0, store *(r4+68) at absolute (68 + *(r4+72));
//! else store *(r4+68) at *r5. Phase 2: if *(r4+68) != 0, r4 += 72 and store
//! *(r4+72) at absolute (72 + *(r4+68)). T = last tst.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = cpu.r[4].wrapping_add(72);
    cpu.r[3] = mem.read_u32(cpu.r[0]).unwrap_or(0);
    cpu.set_t(cpu.r[3] == 0); // tst r3,r3
    if cpu.r[3] != 0 {
        cpu.r[0] = cpu.r[4].wrapping_add(68);
        cpu.r[1] = mem.read_u32(cpu.r[0]).unwrap_or(0);
        cpu.r[0] = 68;
        let _ = mem.write_u32(cpu.r[3].wrapping_add(cpu.r[0]), cpu.r[1]); // bra delay: mov.l r1,@(r0,r3)
    } else {
        cpu.r[0] = cpu.r[4].wrapping_add(68);
        cpu.r[0] = mem.read_u32(cpu.r[0]).unwrap_or(0);
        let _ = mem.write_u32(cpu.r[5], cpu.r[0]); // mov.l r0,@r5
    }
    cpu.r[0] = cpu.r[4].wrapping_add(68);
    cpu.r[3] = mem.read_u32(cpu.r[0]).unwrap_or(0);
    cpu.set_t(cpu.r[3] == 0);
    if cpu.r[3] != 0 {
        cpu.r[4] = cpu.r[4].wrapping_add(72);
        cpu.r[1] = mem.read_u32(cpu.r[4]).unwrap_or(0);
        cpu.r[0] = 72;
        let _ = mem.write_u32(cpu.r[3].wrapping_add(cpu.r[0]), cpu.r[1]); // mov.l r1,@(r0,r3)
    }
}
