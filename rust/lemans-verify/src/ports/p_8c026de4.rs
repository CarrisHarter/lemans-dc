//! 0x8c026de4 — `obj->field_4c = v; obj->inner->field_34 = v;`
//!
//! `mov #76,r0 ; mov.l r5,@(r0,r4) ; mov.l @(48,r4),r0 ; rts ; mov.l r5,@(52,r0)`
//! The C hides that r0 is first 0x4c and finally the loaded `obj->field_30`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let base = cpu.r[4];
    let val = cpu.r[5];
    cpu.r[0] = 0x4c; // mov #76,r0
    let _ = mem.write_u32(base.wrapping_add(cpu.r[0]), val);
    cpu.r[0] = mem.read_u32(base.wrapping_add(0x30)).unwrap(); // mov.l @(48,r4),r0
    let _ = mem.write_u32(cpu.r[0].wrapping_add(0x34), val); // delay slot of `rts`
}
