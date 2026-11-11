//! 0x8c0b4bb8 — write a 33-byte header at `[r4]`: byte 0, then [r5] (4 words),
//! then [r6] (3 words), then fr4's bits at [r4+32]. The `mov r4,r0` after
//! `rts` is the delay slot, so r0=r4 on exit.
//!
//! Leftover registers: r0=r4, r1=r4+28, r5=r5+12, r6=r6+8; fr0=[r6+8].

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let dst = cpu.r[4];
    let s1 = cpu.r[5];
    let s2 = cpu.r[6];
    let rd = |mem: &Memory, a: u32| mem.read_u32(a).unwrap_or(0);

    let _ = mem.write_u8(dst, 0);
    let _ = mem.write_u32(dst + 4, rd(mem, s1));
    let _ = mem.write_u32(dst + 8, rd(mem, s1 + 4));
    let _ = mem.write_u32(dst + 12, rd(mem, s1 + 8));
    let _ = mem.write_u32(dst + 16, rd(mem, s1 + 12));
    let _ = mem.write_u32(dst + 20, rd(mem, s2));
    let _ = mem.write_u32(dst + 24, rd(mem, s2 + 4));
    let _ = mem.write_u32(dst + 28, rd(mem, s2 + 8));
    let _ = mem.write_f32(dst + 32, cpu.fr[4]);

    cpu.fr[0] = f32::from_bits(rd(mem, s2 + 8));
    cpu.r[0] = dst;
    cpu.r[1] = dst + 28;
    cpu.r[5] = s1 + 12;
    cpu.r[6] = s2 + 8;
}
