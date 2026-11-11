//! 0x8c02753c — copy 4 u16 from (r4+0x114+r5*8) to [r15] (a pointer popped
//! from the stack slot the caller set up).
//!
//! Hidden side effects the C does not show:
//! - the destination pointer r7 is *loaded from* [r15], not passed in r7
//! - r5 ends scaled by 8, r4 is mutated in place (r4 += 0x11a)
//! - r6 keeps the first loaded u16, r0 the last, r3 = 0x118, r1 = 0x11a
//! - `shll r5` sets T to bit 31 of r5<<2 (bit 29 of the original r5)
//! - the 4th store is the `rts` delay slot
//!
//! NOTE: blocked against the current oracle — same group-8 decode bug as
//! 0x8c027500: the delay-slot `mov.w r0,@(6,r7)` (0x8173) is executed as a
//! store at r1+230 = 0x200 (r1 holds the literal 0x11a), which is unmapped.
//! This port follows the real SH-4 encoding (reg = bits 4-7, disp = bits 0-3).

use sh4_oracle::{Cpu, Memory};

fn sext16(v: u16) -> u32 {
    v as u16 as i16 as i32 as u32
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[7] = mem.read_u32(cpu.r[15]).unwrap(); // mov.l @r15,r7
    let r4 = cpu.r[4];
    cpu.r[3] = mem.read_u32(0x8C027570).unwrap(); // 0x114
    cpu.r[5] = cpu.r[5] << 2; // shll2 r5
    let t = (cpu.r[5] >> 31) != 0;
    cpu.r[5] = cpu.r[5] << 1; // shll r5 — sets T
    cpu.set_t(t);
    cpu.r[1] = r4;
    cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[3]); // r4+0x114
    cpu.r[0] = cpu.r[5];
    cpu.r[6] = sext16(mem.read_u16(cpu.r[0].wrapping_add(cpu.r[1])).unwrap());
    cpu.r[3] = mem.read_u32(0x8C027574).unwrap(); // 0x116
    cpu.r[1] = r4;
    let _ = mem.write_u16(cpu.r[7], cpu.r[6] as u16);
    cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[3]); // r4+0x116
    cpu.r[0] = sext16(mem.read_u16(cpu.r[0].wrapping_add(cpu.r[1])).unwrap());
    cpu.r[3] = mem.read_u32(0x8C027578).unwrap(); // 0x118
    cpu.r[1] = r4;
    let _ = mem.write_u16(cpu.r[7] + 2, cpu.r[0] as u16);
    cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[3]); // r4+0x118
    cpu.r[0] = cpu.r[5];
    cpu.r[0] = sext16(mem.read_u16(cpu.r[0].wrapping_add(cpu.r[1])).unwrap());
    cpu.r[1] = mem.read_u32(0x8C02757C).unwrap(); // 0x11a
    let _ = mem.write_u16(cpu.r[7] + 4, cpu.r[0] as u16);
    cpu.r[4] = r4.wrapping_add(cpu.r[1]); // add r1,r4
    cpu.r[0] = cpu.r[5];
    cpu.r[0] = sext16(mem.read_u16(cpu.r[0].wrapping_add(cpu.r[4])).unwrap());
    // rts delay slot: mov.w r0,@(6,r7)
    let _ = mem.write_u16(cpu.r[7] + 6, cpu.r[0] as u16);
}
