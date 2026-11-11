//! 0x8c1bc370 — initialise a 29-byte record at `r4` and register it globally.
//!
//! Original:
//! ```text
//! r0 = 0x8c2b82b8 ; r5 = r4 ; r1 = r4 ; *r0 = r4          (global head = r4)
//! r6 = 0 ; (r4+8) = 0 ; r5 += 16 ; (r4+12) = 0 ; r1 += 20
//! r3 = *0x8c23dbf8 + 0xf0 ; fmov @r3,fr0                  (seed float)
//! r0 = r4+4 ; fmov fr0,@r5 (r4+16) ; fldi0 fr0
//! fmov fr0,@r1 (r4+20) ; fmov fr0,@r0 (r4+4) ; (r4) = 0
//! r0 = 24 ; (r4+28) = 0 ; mov.b r6,@(r0,r4) (byte at r4+24)
//! rts ; mov r4,r0   (delay)
//! ```
//! No T-writing instruction, so T is unchanged. `fr0` ends 0.0 (fldi0).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r4 = cpu.r[4];
    let global = mem.read_u32(0x8c1bc3a4).unwrap_or(0); // pool: 0x8c2b82b8
    cpu.r[5] = r4;
    cpu.r[1] = r4;
    mem.write_u32(global, r4).ok(); // mov.l r4,@r0
    cpu.r[6] = 0;
    mem.write_u32(r4 + 8, 0).ok();
    cpu.r[5] = r4.wrapping_add(16);
    mem.write_u32(r4 + 12, 0).ok();
    cpu.r[1] = r4.wrapping_add(20);
    let table = mem.read_u32(0x8c1bc3a8).unwrap_or(0); // pool: 0x8c23dbf8
    let base = mem.read_u32(table).unwrap_or(0);
    let off = mem.read_u32(0x8c1bc3ac).unwrap_or(0); // pool: 0xf0
    cpu.fr[0] = mem.read_f32(base.wrapping_add(off)).unwrap_or(0.0);
    cpu.r[3] = base.wrapping_add(off);
    cpu.r[0] = r4.wrapping_add(4);
    mem.write_f32(r4 + 16, cpu.fr[0]).ok(); // fmov fr0,@r5
    cpu.fr[0] = 0.0; // fldi0
    mem.write_f32(r4 + 20, 0.0).ok(); // fmov fr0,@r1
    mem.write_f32(r4 + 4, 0.0).ok(); // fmov fr0,@r0
    mem.write_u32(r4, 0).ok(); // mov.l r6,@r4
    cpu.r[0] = 24;
    mem.write_u32(r4 + 28, 0).ok();
    mem.write_u8(r4 + 24, 0).ok(); // mov.b r6,@(r0,r4)
    // rts delay slot
    cpu.r[0] = r4;
}
