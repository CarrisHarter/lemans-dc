//! 0x8c0b7be8 — initialise a 16-byte struct at `r4`; returns r4.
//!
//! ```text
//! mov.l 0x8c0b7c08,r0 ; mov #-1,r1 ; mov.l r0,@r4      ; *p = lit1
//! mov.l 0x8c0b7c0c,r0 ; mov.l r0,@r4                   ; *p = lit2 (overwrites)
//! mov.l 0x8c0b7c10,r0 ; mov.l r0,@r4                   ; *p = lit3 (overwrites)
//! mov #1,r0 ; mov.b r0,@(4,r4)                         ; p[4] = 1 (byte)
//! mov.l 0x8c0b7c14,r0 ; mov.l r0,@r4                   ; *p = lit4 (final)
//! mov.l r1,@(8,r4)                                    ; p[8] = -1
//! mov r4,r0 ; rts
//! mov.l r1,@(12,r4)                                   ; delay slot: p[12] = -1
//! ```
//! Hidden side effects: all four `*p = lit` stores hit the write log in
//! order (only the last survives in memory, but the log is compared), the
//! byte store at p+4, both -1 stores, r1 = -1, and r0 = r4.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[1] = 0xFFFF_FFFF; // mov #-1,r1
    let mut r0 = mem.read_u32(0x8C0B7C08).unwrap();
    let _ = mem.write_u32(cpu.r[4], r0);
    r0 = mem.read_u32(0x8C0B7C0C).unwrap();
    let _ = mem.write_u32(cpu.r[4], r0);
    r0 = mem.read_u32(0x8C0B7C10).unwrap();
    let _ = mem.write_u32(cpu.r[4], r0);
    cpu.r[0] = 1; // mov #1,r0
    let _ = mem.write_u8(cpu.r[4].wrapping_add(4), cpu.r[0] as u8); // mov.b r0,@(4,r4)
    r0 = mem.read_u32(0x8C0B7C14).unwrap();
    let _ = mem.write_u32(cpu.r[4], r0);
    let _ = mem.write_u32(cpu.r[4].wrapping_add(8), cpu.r[1]); // mov.l r1,@(8,r4)
    cpu.r[0] = cpu.r[4]; // mov r4,r0
    let _ = mem.write_u32(cpu.r[4].wrapping_add(12), cpu.r[1]); // rts delay slot
}
