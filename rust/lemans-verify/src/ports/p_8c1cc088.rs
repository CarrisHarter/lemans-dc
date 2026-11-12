//! 0x8c1cc088 - free nine sub-pointers, clear a global, then free the container.
//!
//! `[0x8c2b8720] = 0` runs first, then the free thunk 0x8c0102cc is called on
//! `[r13+4]`, `+8`, `+12`, `+16`, `+20`, `+32`, `+44`, `+36`, `+48` — note the
//! `+44` before `+36`. A positive `exts.w(r14)` then frees r13. Returns r13.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[14] = cpu.r[5]; // mov r5,r14
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[13] = cpu.r[4]; // mov r4,r13
    cpu.set_t(cpu.r[13] == 0); // tst r13,r13
    if cpu.r[13] != 0 {
        cpu.r[1] = mem.read_u32(0x8c1cc0e8).unwrap_or(0x8c2b8720); // mov.l 0x8c1cc0e8,r1
        cpu.r[0] = 0; // mov #0,r0
        let _ = mem.write_u32(cpu.r[1], cpu.r[0]); // mov.l r0,@r1
        for (off, ret) in [
            (4u32, 0x8c1cc0a2u32),
            (8, 0x8c1cc0a8),
            (12, 0x8c1cc0ae),
            (16, 0x8c1cc0b4),
            (20, 0x8c1cc0ba),
            (32, 0x8c1cc0c0),
            (44, 0x8c1cc0c6),
            (36, 0x8c1cc0cc),
            (48, 0x8c1cc0d2),
        ] {
            cpu.r[0] = mem.read_u32(0x8c1cc0ec).unwrap_or(0x8c0102cc); // mov.l 0x8c1cc0ec,r0
            cpu.r[4] = mem.read_u32(cpu.r[13].wrapping_add(off)).unwrap_or(0); // jsr delay: mov.l @(off,r13),r4
            cpu.pr = ret;
            crate::registry::p_8c0102cc::run(cpu, mem);
        }
        cpu.r[0] = cpu.r[14] as i16 as i32 as u32; // exts.w r14,r0
        cpu.set_t((cpu.r[0] as i32) > 0); // cmp/pl r0
        if cpu.t() {
            cpu.r[0] = mem.read_u32(0x8c1cc0f0).unwrap_or(0x8c01032c); // mov.l 0x8c1cc0f0,r0
            cpu.r[4] = cpu.r[13]; // jsr delay: mov r13,r4
            cpu.pr = 0x8c1cc0de;
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }
    cpu.r[0] = cpu.r[13]; // mov r13,r0
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
