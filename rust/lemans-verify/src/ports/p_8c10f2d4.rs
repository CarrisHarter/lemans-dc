//! 0x8c10f2d4 — compare `[r5+4]` and `[r6+4]` through 0x8c10e35c.
//!
//! Two calls to the field-68 loader, then a signed `cmp/gt`: the function
//! returns 1 when `load(*(r6+4)) > load(*(r5+4))`. The second argument is
//! loaded into `r14` (which the prologue saves) while the first is passed in
//! the `jsr` delay slot.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 8c10f2d4: mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // 8c10f2d6: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 8c10f2d8: mov r6,r14
    cpu.r[14] = cpu.r[6];
    // 8c10f2da: mov.l 0x8c10f2f4,r0 — 0x8c10e35c
    cpu.r[0] = mem.read_u32(0x8c10f2f4).unwrap_or(0);
    // 8c10f2dc: jsr @r0 ; 8c10f2de: mov.l @(4,r5),r4
    cpu.pr = 0x8c10f2e0;
    cpu.r[4] = mem.read_u32(cpu.r[5].wrapping_add(4)).unwrap_or(0);
    crate::registry::p_8c10e35c::run(cpu, mem);
    // 8c10f2e0: mov.l @(4,r14),r4
    cpu.r[4] = mem.read_u32(cpu.r[14].wrapping_add(4)).unwrap_or(0);
    // 8c10f2e2: mov r0,r14
    cpu.r[14] = cpu.r[0];
    // 8c10f2e4: mov.l 0x8c10f2f4,r0 — 0x8c10e35c
    cpu.r[0] = mem.read_u32(0x8c10f2f4).unwrap_or(0);
    // 8c10f2e6: jsr @r0 ; 8c10f2e8: nop
    cpu.pr = 0x8c10f2ea;
    crate::registry::p_8c10e35c::run(cpu, mem);
    // 8c10f2ea: cmp/gt r14,r0 — T = r0 > r14 (signed)
    cpu.set_t((cpu.r[0] as i32) > (cpu.r[14] as i32));
    // 8c10f2ec: movt r0
    cpu.r[0] = cpu.t() as u32;
    // 8c10f2ee: lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 8c10f2f0: rts ; 8c10f2f2: mov.l @r15+,r14
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
