//! 0x8c0846d0 — bounded memchr: scan [base+idx, base+count) for byte r5.
//!
//! Returns offset from base, or -1. Original side effects: r4 = count then
//! last byte (exts.b), r7 = base, r3 = exts.b(r5), r0 = cursor then result,
//! r1 = end, T = last cmp; r0 = -1 via rts delay on both exits.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(cpu.r[4]).unwrap_or(0);
    cpu.r[4] = mem.read_u32(cpu.r[0]).unwrap_or(0); // count
    cpu.set_t(cpu.r[6] >= cpu.r[4]); // cmp/hs r4,r6: T=(r6>=r4)
    if cpu.r[6] >= cpu.r[4] {
        cpu.r[0] = (-1i32) as u32; // rts delay slot
        return;
    }
    cpu.r[7] = mem.read_u32(cpu.r[0] + 12).unwrap_or(0); // base
    cpu.r[3] = (cpu.r[5] as u8 as i8) as u32; // exts.b r5,r3
    cpu.r[0] = cpu.r[7].wrapping_add(cpu.r[6]);
    cpu.r[1] = cpu.r[7].wrapping_add(cpu.r[4]); // bra delay slot
    loop {
        cpu.set_t(cpu.r[0] == cpu.r[1]); // cmp/eq r0,r1 (loop head 0x8c0846f8)
        if cpu.r[0] == cpu.r[1] {
            cpu.r[0] = (-1i32) as u32; // rts delay slot
            return;
        }
        let b = mem.read_u8(cpu.r[0]).unwrap_or(0);
        cpu.r[4] = (b as i8 as i32) as u32; // mov.b + exts.b
        cpu.set_t(cpu.r[4] == cpu.r[3]); // cmp/eq r3,r4
        if cpu.r[4] == cpu.r[3] {
            cpu.r[0] = cpu.r[0].wrapping_sub(cpu.r[7]); // rts delay slot
            return;
        }
        cpu.r[0] = cpu.r[0].wrapping_add(1);
    }
}
