//! 0x8c0b4c10 — write an identity-ish 4x4 matrix (byte 1 at [r4], then 21
//! words: 1.0 at +4,+32,+36,+52,+68, zero elsewhere) at `[r4]`.
//! r9..r14 are pushed and popped (r14 in the rts delay slot), so all survive.
//!
//! Leftover registers: r0=r4 (set just before rts), r1=r4+76, r3=r4+72,
//! r5=r4+68, r6=r4+64, r7=r4+60; fr0=0.0, fr1=1.0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let dst = cpu.r[4];
    let w = |mem: &mut Memory, a: u32, v: f32| {
        let _ = mem.write_f32(a, v);
    };

    let _ = mem.write_u8(dst, 1);
    w(mem, dst + 4, 1.0);
    w(mem, dst + 8, 0.0);
    w(mem, dst + 12, 0.0);
    w(mem, dst + 16, 0.0);
    w(mem, dst + 20, 0.0);
    w(mem, dst + 24, 0.0);
    w(mem, dst + 28, 0.0);
    w(mem, dst + 32, 1.0);
    w(mem, dst + 36, 1.0);
    w(mem, dst + 40, 0.0);
    w(mem, dst + 44, 0.0);
    w(mem, dst + 48, 0.0);
    w(mem, dst + 52, 1.0);
    w(mem, dst + 56, 0.0);
    w(mem, dst + 60, 0.0);
    w(mem, dst + 64, 0.0);
    w(mem, dst + 68, 1.0);
    w(mem, dst + 72, 0.0);
    w(mem, dst + 76, 0.0);
    w(mem, dst + 80, 0.0);

    cpu.fr[0] = 0.0;
    cpu.fr[1] = 1.0;
    cpu.r[0] = dst;
    cpu.r[1] = dst + 76;
    cpu.r[3] = dst + 72;
    cpu.r[5] = dst + 68;
    cpu.r[6] = dst + 64;
    cpu.r[7] = dst + 60;
}
