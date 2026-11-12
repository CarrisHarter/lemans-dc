//! 0x8c059954 — call 0x8c0a3b70, then `*(char *)(*(int *)(param_1 + 4) + 4) = 1;`
//!
//! `mov.l r14,@-r15 ; sts.l pr,@-r15 ; mov r4,r14 ; mov.l @(PC),r0 ;
//! jsr @r0 ; nop ; mov.l @(4,r14),r1 ; mov #1,r0 ; mov.b r0,@(4,r1) ;
//! lds.l @r15+,pr ; rts ; mov.l @r15+,r14` — the `jsr` delay slot is a nop so
//! r4 survives into r14, the callee is an empty stub (returns without touching
//! anything), and r14 is restored in the `rts` delay slot.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]); // mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr); // sts.l pr,@-r15
    cpu.r[14] = cpu.r[4]; // mov r4,r14
    cpu.r[0] = mem.read_u32(0x8c05996c).unwrap(); // mov.l @(0x8c05996c),r0 = 0x8c0a3b70
    cpu.pr = 0x8c059960; // jsr @r0 (delay slot: nop)
    crate::registry::p_8c0a3b70::run(cpu, mem);
    cpu.r[1] = mem.read_u32(cpu.r[14].wrapping_add(4)).unwrap(); // mov.l @(4,r14),r1
    cpu.r[0] = 1; // mov #1,r0
    let _ = mem.write_u8(cpu.r[1].wrapping_add(4), cpu.r[0] as u8); // mov.b r0,@(4,r1)
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap(); // lds.l @r15+,pr
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap(); // delay slot of `rts`: mov.l @r15+,r14
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
