//! 0x8c1506a4 — init a 68-byte struct at r4: store the two global pointers
//! from the literal pool to `*r4` (second wins), then copy 16 words (64
//! bytes) from 0x8c1d3e88 to r4+4..r4+64. The last store (`mov.l r1,@(60,r3)`)
//! is the rts delay slot.
//!
//! Register residue: r0=r4, r3=r4+4, r6=mem[src], r5=mem[src+48],
//! r4=mem[src+56], r1=mem[src+60].

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r4 = cpu.r[4];
    let g1 = mem.read_u32(0x8c1506f8).unwrap_or(0); // 0x8c1ef3a8
    let g2 = mem.read_u32(0x8c1506fc).unwrap_or(0); // 0x8c1ef3b4
    let src = mem.read_u32(0x8c150700).unwrap_or(0); // 0x8c1d3e88
    let _ = mem.write_u32(r4, g1);
    let _ = mem.write_u32(r4, g2);
    let mut w = [0u32; 16];
    for (i, slot) in w.iter_mut().enumerate() {
        *slot = mem.read_u32(src + 4 * i as u32).unwrap_or(0);
    }
    for (i, v) in w.iter().enumerate() {
        let _ = mem.write_u32(r4 + 4 + 4 * i as u32, *v);
    }
    cpu.r[0] = r4;
    cpu.r[1] = w[15];
    cpu.r[3] = r4 + 4;
    cpu.r[4] = w[14]; // last r4 load: mov.l @(56,r1),r4 at 0x8c1506ec
    cpu.r[5] = w[12]; // last r5 load: mov.l @(48,r1),r5 at 0x8c1506e4
    cpu.r[6] = w[0];
}
