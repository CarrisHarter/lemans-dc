//! 0x8c0273a0 — copy a 12-byte triple into obj+0x68.
//!
//! `mov.l @r5,r1 ; add #104,r4 ; mov.l @(4,r5),r0 ; mov.l r1,@r4 ;
//!  mov.l r0,@(4,r4) ; mov.l @(8,r5),r0 ; rts ; mov.l r0,@(8,r4)`
//! The C hides that r4 itself ends as `base+0x68`, and that r0/r1 keep
//! the loaded fields; the third store is the `rts` delay slot.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let base = cpu.r[4];
    let src = cpu.r[5];
    cpu.r[1] = mem.read_u32(src).unwrap();
    cpu.r[4] = base.wrapping_add(0x68); // add #104,r4
    let v1 = mem.read_u32(src + 4).unwrap();
    let _ = mem.write_u32(cpu.r[4], cpu.r[1]);
    let _ = mem.write_u32(cpu.r[4] + 4, v1);
    cpu.r[0] = mem.read_u32(src + 8).unwrap();
    let _ = mem.write_u32(cpu.r[4] + 8, cpu.r[0]); // delay slot of `rts`
}
