//! 0x8c218876 - copy three fields from the record 0x8c216fca finds, or -8.
//!
//! The scan is `0x8c216fca(r4, [[0x8c23d428]+16], 0)`. A zero result returns
//! -8. Otherwise record `index-1` of that table (stride 44, starting at +16)
//! contributes `[rec]+0x96`, `[rec+4]`, and byte `[rec+8]` into the struct at
//! the entry r5. Success returns 0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[3] = mem.read_u32(0x8c2188d4).unwrap_or(0x8c23d428);
    cpu.r[6] = 0;
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[14] = cpu.r[5];
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[5] = mem.read_u32(cpu.r[3]).unwrap_or(0);
    cpu.r[2] = mem.read_u32(0x8c2188dc).unwrap_or(0x8c216fca);
    cpu.r[5] = mem.read_u32(cpu.r[5].wrapping_add(16)).unwrap_or(0); // jsr delay
    cpu.pr = 0x8c218888;
    crate::registry::p_8c216fca::run(cpu, mem);
    cpu.r[5] = cpu.r[0];
    cpu.set_t(cpu.r[5] == 0); // tst r5,r5
    if cpu.t() {
        cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
        cpu.r[15] = cpu.r[15].wrapping_add(4);
        cpu.r[0] = (-8i32) as u32;
        cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // rts delay
        cpu.r[15] = cpu.r[15].wrapping_add(4);
        return;
    }
    cpu.r[2] = 44;
    cpu.r[3] = mem.read_u32(0x8c2188d4).unwrap_or(0x8c23d428);
    cpu.r[5] = cpu.r[5].wrapping_add((-1i32) as u32);
    cpu.r[1] = mem.read_u16(0x8c2188cc).unwrap_or(0x0096) as i16 as i32 as u32;
    let product = (cpu.r[2] as i32 as i64) * (cpu.r[5] as i32 as i64);
    cpu.mach = (product >> 32) as u32;
    cpu.macl = product as u32;
    cpu.r[4] = mem.read_u32(cpu.r[3]).unwrap_or(0);
    cpu.r[4] = mem.read_u32(cpu.r[4].wrapping_add(16)).unwrap_or(0);
    cpu.r[5] = cpu.macl;
    cpu.r[4] = cpu.r[4].wrapping_add(16);
    cpu.r[4] = cpu.r[4].wrapping_add(cpu.r[5]);
    cpu.r[0] = mem.read_u32(cpu.r[4]).unwrap_or(0);
    cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[1]);
    let _ = mem.write_u32(cpu.r[14], cpu.r[0]);
    cpu.r[3] = mem.read_u32(cpu.r[4].wrapping_add(4)).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[14].wrapping_add(4), cpu.r[3]);
    cpu.r[0] = mem.read_u8(cpu.r[4].wrapping_add(8)).unwrap_or(0) as i8 as i32 as u32;
    let _ = mem.write_u8(cpu.r[14].wrapping_add(8), cpu.r[0] as u8);
    cpu.r[0] = 0;
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // rts delay
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
