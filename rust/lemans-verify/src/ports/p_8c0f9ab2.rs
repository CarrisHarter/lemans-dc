//! 0x8c0f9ab2 — byte-wise strcmp loop entered at its body.
//!
//! Each iteration loads `r3 = *r4++` and `r1 = *r5++` (both zero-extended after
//! a sign-extended `mov.b`), compares them, and steps r5 in the `bt.s` delay
//! slot. A mismatch returns `r0 - r1` (both 0..255, wrapping) in the `rts`
//! delay slot; a match jumps to 0x8c0f9aa8, which returns 0 when r3 is the NUL
//! and otherwise loops. r3 holds the last byte loaded from r4.

use sh4_oracle::{Cpu, Memory};

fn byte(mem: &Memory, addr: u32) -> u32 {
    mem.read_u8(addr).unwrap_or(0) as u32 & 0xff
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    loop {
        // 0x8c0f9ab2
        let dst = byte(mem, cpu.r[4]); // mov.b @r4,r3
        let src = byte(mem, cpu.r[5]); // mov.b @r5,r1
        cpu.r[4] = cpu.r[4].wrapping_add(1); // add #1,r4
        cpu.r[3] = dst; // extu.b r3,r3
        cpu.r[0] = dst; // extu.b r3,r0
        cpu.r[1] = src; // extu.b twice
        cpu.set_t(cpu.r[1] == cpu.r[0]); // cmp/eq r1,r0
        cpu.r[5] = cpu.r[5].wrapping_add(1); // bt.s delay slot
        if cpu.r[1] != cpu.r[0] {
            cpu.r[0] = cpu.r[0].wrapping_sub(cpu.r[1]); // rts delay: sub r1,r0
            return;
        }
        // 0x8c0f9aa8: extu.b r3,r0 ; tst r0,r0 ; bf 0x8c0f9ab2
        cpu.r[0] = cpu.r[3] & 0xff;
        cpu.set_t(cpu.r[0] == 0);
        if cpu.r[0] == 0 {
            cpu.r[0] = 0; // rts delay: mov #0,r0
            return;
        }
    }
}
