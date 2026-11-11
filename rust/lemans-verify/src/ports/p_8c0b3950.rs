//! 0x8c0b3950 — copy three 12-byte globals (0x8c1d3f30, 0x8c1d3f40,
//! 0x8c1d3f50, loaded through the literal pool at 0x8c0b3984/88/8c) into
//! `[r4]`, `[r4+12]`, `[r4+24]`; the store to `[r4+32]` is the rts delay slot.
//!
//! Leftover registers: r7=g1, r5=g2, r3=g3, r6=r4+12, r4=r4+24, r0=[g3+8],
//! r1=[g3].

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let g1 = mem.read_u32(0x8c0b3984).unwrap_or(0);
    let g2 = mem.read_u32(0x8c0b3988).unwrap_or(0);
    let g3 = mem.read_u32(0x8c0b398c).unwrap_or(0);
    let dst = cpu.r[4];
    let rd = |mem: &Memory, a: u32| mem.read_u32(a).unwrap_or(0);

    let r1 = rd(mem, g1);
    let r0 = rd(mem, g1 + 4);
    let _ = mem.write_u32(dst, r1);
    let _ = mem.write_u32(dst + 4, r0);
    let r0 = rd(mem, g1 + 8);
    let _ = mem.write_u32(dst + 8, r0);

    let r1 = rd(mem, g2);
    let r0 = rd(mem, g2 + 4);
    let _ = mem.write_u32(dst + 12, r1);
    let _ = mem.write_u32(dst + 16, r0);
    let r0 = rd(mem, g2 + 8);
    let _ = mem.write_u32(dst + 20, r0);

    let r1 = rd(mem, g3);
    let r0 = rd(mem, g3 + 4);
    let _ = mem.write_u32(dst + 24, r1);
    let _ = mem.write_u32(dst + 28, r0);
    let r0 = rd(mem, g3 + 8);
    let _ = mem.write_u32(dst + 32, r0); // delay slot of `rts`

    cpu.r[7] = g1;
    cpu.r[6] = dst + 12;
    cpu.r[5] = g2;
    cpu.r[3] = g3;
    cpu.r[4] = dst + 24;
    cpu.r[0] = r0;
    cpu.r[1] = r1;
}
