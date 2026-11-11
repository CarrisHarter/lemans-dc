//! 0x8c0b3990 — copy a 4x4 matrix `[r5]` -> `[r4]` (64 bytes), zeroing the
//! [12], [28], [44] words and writing 1.0 to [60]. r14/r13 are pushed and
//! popped (r13 before `rts`, r14 in its delay slot), so both survive.
//!
//! Leftover registers: r0=r4, r1=r4+60, r3=r4+56, r5=r5+44, r6=r4+52,
//! r7=r5+36; fr0=1.0, fr1=[r5+44].

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let src = cpu.r[5];
    let dst = cpu.r[4];
    let rd = |mem: &Memory, a: u32| mem.read_u32(a).unwrap_or(0);
    let w = |mem: &mut Memory, a: u32, v: u32| {
        let _ = mem.write_u32(a, v);
    };

    w(mem, dst, rd(mem, src));
    w(mem, dst + 4, rd(mem, src + 4));
    w(mem, dst + 8, rd(mem, src + 8));
    w(mem, dst + 12, 0);
    w(mem, dst + 16, rd(mem, src + 12));
    w(mem, dst + 20, rd(mem, src + 16));
    w(mem, dst + 24, rd(mem, src + 20));
    w(mem, dst + 28, 0);
    w(mem, dst + 32, rd(mem, src + 24));
    w(mem, dst + 36, rd(mem, src + 28));
    w(mem, dst + 40, rd(mem, src + 32));
    w(mem, dst + 44, 0);
    w(mem, dst + 48, rd(mem, src + 36));
    w(mem, dst + 52, rd(mem, src + 40));
    let s44 = rd(mem, src + 44);
    w(mem, dst + 56, s44);
    w(mem, dst + 60, 0x3f80_0000); // fldi1

    cpu.fr[0] = 1.0;
    cpu.fr[1] = f32::from_bits(s44);
    cpu.r[0] = dst;
    cpu.r[1] = dst + 60;
    cpu.r[3] = dst + 56;
    cpu.r[5] = src.wrapping_add(44);
    cpu.r[6] = dst + 52;
    cpu.r[7] = src.wrapping_add(36);
}
