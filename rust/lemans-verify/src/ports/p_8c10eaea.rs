//! 0x8c10eaea — byte-wise strcmp loop entered at its body.
//!
//! Each iteration loads `r3 = *r4++` and `r1 = *r5++` (both zero-extended after
//! a sign-extended `mov.b`), compares them, and steps r5 in the `bt.s` delay
//! slot. A mismatch returns `r0 - r1` in the `rts` delay slot; a match jumps to
//! 0x8c10eae0, which returns 0 when r3 is NUL and otherwise loops.

use sh4_oracle::{Cpu, Memory};

fn byte(mem: &Memory, addr: u32) -> u32 {
    mem.read_u8(addr).unwrap_or(0) as u32 & 0xff
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    loop {
        let dst = byte(mem, cpu.r[4]);
        let src = byte(mem, cpu.r[5]);
        cpu.r[4] = cpu.r[4].wrapping_add(1);
        cpu.r[3] = dst;
        cpu.r[0] = dst;
        cpu.r[1] = src;
        cpu.set_t(cpu.r[1] == cpu.r[0]);
        cpu.r[5] = cpu.r[5].wrapping_add(1);
        if cpu.r[1] != cpu.r[0] {
            cpu.r[0] = cpu.r[0].wrapping_sub(cpu.r[1]);
            return;
        }
        cpu.r[0] = cpu.r[3] & 0xff;
        cpu.set_t(cpu.r[0] == 0);
        if cpu.r[0] == 0 {
            cpu.r[0] = 0;
            return;
        }
    }
}
