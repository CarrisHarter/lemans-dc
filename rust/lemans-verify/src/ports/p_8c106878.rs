//! 0x8c106878 — byte-wise strcmp tail loop; the entry expects r3 to hold the
//! previously loaded byte (0 on a normal call).
//!
//! `r0 = extu.b(r3)`; a zero r3 returns 0 (`rts` delay reloads r0 = 0), else
//! the loop loads `r3 = *r4++` and `r1 = *r5++` (both zero-extended), compares
//! them, and on equality jumps back to the top with T = (r3 == 0). A mismatch
//! returns `r0 - r1` (both 0..255, wrapping) in the `rts` delay slot. r4/r5
//! always advance; r3 ends as the last byte loaded from r4.

use sh4_oracle::{Cpu, Memory};

fn byte(mem: &Memory, addr: u32) -> u32 {
    mem.read_u8(addr).unwrap_or(0) as u32 & 0xff
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    loop {
        // 0x8c106878: extu.b r3,r0 ; tst r0,r0 ; bf 0x8c106882
        cpu.r[0] = cpu.r[3] & 0xff; // extu.b r3,r0
        cpu.set_t(cpu.r[0] == 0); // tst r0,r0
        if cpu.r[0] == 0 {
            cpu.r[0] = 0; // rts delay: mov #0,r0
            return;
        }
        // 0x8c106882
        cpu.r[3] = byte(mem, cpu.r[4]); // mov.b @r4,r3 (then extu.b below)
        cpu.r[1] = byte(mem, cpu.r[5]); // mov.b @r5,r1
        cpu.r[4] = cpu.r[4].wrapping_add(1);
        cpu.r[0] = cpu.r[3]; // extu.b r3,r0
        cpu.r[5] = cpu.r[5].wrapping_add(1); // bt.s delay slot
        cpu.set_t(cpu.r[0] == cpu.r[1]); // cmp/eq r1,r0
        if cpu.r[0] != cpu.r[1] {
            cpu.r[0] = cpu.r[0].wrapping_sub(cpu.r[1]); // rts delay: sub r1,r0
            return;
        }
        // bt.s 0x8c106878 taken
    }
}
