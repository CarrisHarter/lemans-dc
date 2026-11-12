//! 0x8c222924 — zero 0xa40 bytes at 0x8c2c02b8, then call 0x8c221f24.
//!
//! `r3 = memset`, `r4 = 0x8c2c02b8`, `r6 = 0xa40`, `r5 = 0` in the delay slot;
//! then `r2 = [0x8c222960]` and `jsr @r2` (nop delay). The `rts` delay slot
//! reloads `r0 = 0`, so both callees' return values are discarded.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[3] = mem.read_u32(0x8c22295c).unwrap_or(0x8c010f88); // mov.l 0x8c22295c,r3
    cpu.r[4] = mem.read_u32(0x8c22294c).unwrap_or(0x8c2c02b8); // mov.l 0x8c22294c,r4
    cpu.r[6] = mem.read_u16(0x8c222944).unwrap_or(0x0a40) as i16 as i32 as u32; // mov.w
    cpu.r[5] = 0; // delay slot: mov #0,r5
    cpu.pr = 0x8c222930;
    crate::registry::p_8c010f88::run(cpu, mem); // jsr @r3
    cpu.r[2] = mem.read_u32(0x8c222960).unwrap_or(0x8c221f24); // mov.l 0x8c222960,r2
    cpu.pr = 0x8c222936;
    crate::registry::p_8c221f24::run(cpu, mem); // jsr @r2 ; nop
    cpu.r[0] = 0; // rts delay: mov #0,r0
}
