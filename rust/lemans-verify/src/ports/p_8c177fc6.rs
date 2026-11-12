//! 0x8c177fc6 - clear a table, stamp a handler word, then tail into 0x8c172b3c.
//!
//! memset(0x8c2b65e8, 0, 0x504) via 0x8c010f88, `[0x8c2b65e8] = 0x8c178122`, then
//! `jmp 0x8c172b3c` with `r4 = 0x8c177e9c`. The `jmp` delay slot restores pr, so
//! the callee's `rts` returns straight to our caller.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[3] = mem.read_u32(0x8c17806c).unwrap_or(0x8c010f88); // mov.l 0x8c17806c,r3
    cpu.r[4] = mem.read_u32(0x8c178068).unwrap_or(0x8c2b65e8); // mov.l 0x8c178068,r4
    cpu.r[6] = mem.read_u16(0x8c178046).unwrap_or(0x504) as i16 as i32 as u32; // mov.w (sign-extends)
    cpu.r[5] = 0; // jsr delay: mov #0,r5
    cpu.pr = 0x8c177fd2;
    crate::registry::p_8c010f88::run(cpu, mem);
    cpu.r[3] = mem.read_u32(0x8c178068).unwrap_or(0x8c2b65e8); // mov.l 0x8c178068,r3
    cpu.r[2] = mem.read_u32(0x8c178070).unwrap_or(0x8c178122); // mov.l 0x8c178070,r2
    cpu.r[4] = mem.read_u32(0x8c178074).unwrap_or(0x8c177e9c); // mov.l 0x8c178074,r4
    let _ = mem.write_u32(cpu.r[3], cpu.r[2]); // mov.l r2,@r3
    cpu.r[2] = mem.read_u32(0x8c178078).unwrap_or(0x8c172b3c); // mov.l 0x8c178078,r2
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0); // jmp delay: lds.l @r15+,pr
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    crate::registry::p_8c172b3c::run(cpu, mem);
}
