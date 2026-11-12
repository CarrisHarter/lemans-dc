//! 0x8c017d40 — skip one extra input byte when bit6 of `[r4]` is set, then
//! tail-call the bit unpacker (0x8c017b94) with r5 = stack slot. The bt.s
//! delay slot `add #1,r4` runs on both paths, so r4 advances once when the
//! bit is clear and twice when set.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    cpu.r[14] = cpu.r[15];
    cpu.r[0] = mem.read_u8(cpu.r[4]).unwrap_or(0) as i8 as i32 as u32; // mov.b + exts.b
    cpu.r[0] &= 64;
    cpu.set_t(cpu.r[0] & cpu.r[0] == 0); // tst r0,r0
    cpu.r[4] = cpu.r[4].wrapping_add(1); // bt.s delay slot: always runs
    if !cpu.t() {
        cpu.r[4] = cpu.r[4].wrapping_add(1);
    }
    cpu.r[0] = mem.read_u32(0x8c017d68).unwrap_or(0); // 0x8c017b94
    cpu.r[5] = cpu.r[14]; // jsr delay slot
    cpu.pr = 0x8c017d5c;
    crate::registry::p_8c017b94::run(cpu, mem);
    cpu.r[15] = cpu.r[14];
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // rts delay slot
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
