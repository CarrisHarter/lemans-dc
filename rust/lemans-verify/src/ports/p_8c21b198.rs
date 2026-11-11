//! 0x8c21b198 — byteswap_words.
//!
//! Byte-swaps `r5` words at `r4` using `swap.b`/`swap.w`/`swap.b`; `dt` drives
//! the loop and the pointer advances in the `bf.s` delay slot. Returns 0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let mut r4 = cpu.r[4];
    loop {
        cpu.r[0] = mem.read_u32(r4).unwrap_or(0);
        let v = cpu.r[0];
        // swap.b swaps only the low 16 bits; the high half is unchanged.
        cpu.r[1] = (v & 0xFFFF_0000) | ((v & 0xFF) << 8) | ((v >> 8) & 0xFF);
        let w = cpu.r[1];
        cpu.r[0] = (w << 16) | (w >> 16); // swap.w r1,r0
        let x = cpu.r[0];
        cpu.r[1] = (x & 0xFFFF_0000) | ((x & 0xFF) << 8) | ((x >> 8) & 0xFF);
        cpu.r[5] = cpu.r[5].wrapping_sub(1);
        cpu.set_t(cpu.r[5] == 0);
        let _ = mem.write_u32(r4, cpu.r[1]);
        r4 = r4.wrapping_add(4); // delay slot of `bf.s`
        if cpu.r[5] == 0 {
            break;
        }
    }
    cpu.r[4] = r4;
    cpu.r[0] = 0; // delay slot of `rts`
}
