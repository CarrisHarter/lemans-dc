//! 0x8c090cf0 — memmove(dst = r4+0x208, src = r4+0xc0, n = 0xb8) via a tail jump.
//!
//! All three literals (0x208, 0xc0, 0xb8) and the callee address are read from
//! the pool at 0x8c090d00..0x8c090d0c. `r5` ends as the *source* (the delay
//! slot adds 0xc0 to `r4`'s entry value), `r4` as the destination, and the call
//! runs through `jmp @r0`, so `pr` is untouched and the callee's `rts` returns
//! to the caller.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let pool = |addr: u32| mem.read_u32(addr).unwrap_or(0);
    cpu.r[0] = pool(0x8c090d00); // mov.l 0x8c090d00,r0  ! 0x208
    cpu.r[5] = cpu.r[4]; // mov r4,r5
    cpu.r[1] = pool(0x8c090d04); // mov.l 0x8c090d04,r1  ! 0xc0
    cpu.r[4] = cpu.r[4].wrapping_add(cpu.r[0]); // add r0,r4
    cpu.r[6] = pool(0x8c090d08); // mov.l 0x8c090d08,r6  ! 0xb8
    cpu.r[0] = pool(0x8c090d0c); // mov.l 0x8c090d0c,r0  ! 0x8c010e84
    cpu.r[5] = cpu.r[5].wrapping_add(cpu.r[1]); // delay slot: add r1,r5
    crate::registry::p_8c010e84::run(cpu, mem); // jmp @r0
}
