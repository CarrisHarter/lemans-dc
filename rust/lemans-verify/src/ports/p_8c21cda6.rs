//! 0x8c21cda6 — return the counter table entry for the global counter.
//!
//! ```text
//! sts.l pr,@-r15 ; r3=[0x8c21cdfc] (0x8c21a036) ; jsr @r3 ; nop
//! r4=r0 ; cmp/pl r4 ; bf out
//! r0=[0x8c21ce00] (0x8c23c520) ; r3=r4 ; shll2 r3 ; r4=[r0+r3]
//! out: lds.l @r15+,pr ; rts ; r0=r4 (delay slot)
//! ```
//!
//! The callee returns `*0x8c2b8dcc`. When it is positive, the port returns
//! `*(int *)(0x8c23c520 + n*4)`; otherwise the counter itself. T is `cmp/pl`'s
//! signed test of that counter, and neither `rts` nor the delay-slot `mov`
//! disturbs it.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c21cda6: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 0x8c21cda8: mov.l 0x8c21cdfc,r3
    cpu.r[3] = mem.read_u32(0x8c21cdfc).unwrap_or(0); // 0x8c21a036
    // 0x8c21cdaa: jsr @r3 ; 0x8c21cdac: nop (delay slot)
    cpu.pr = 0x8c21cdae;
    crate::registry::p_8c21a036::run(cpu, mem);
    // 0x8c21cdae: mov r0,r4
    cpu.r[4] = cpu.r[0];
    // 0x8c21cdb0: cmp/pl r4
    let positive = (cpu.r[4] as i32) > 0;
    cpu.set_t(positive);
    // 0x8c21cdb2: bf 0x8c21cdbc
    if positive {
        // 0x8c21cdb4: mov.l 0x8c21ce00,r0
        cpu.r[0] = mem.read_u32(0x8c21ce00).unwrap_or(0); // 0x8c23c520
        // 0x8c21cdb6: mov r4,r3
        cpu.r[3] = cpu.r[4];
        // 0x8c21cdb8: shll2 r3
        cpu.r[3] <<= 2;
        // 0x8c21cdba: mov.l @(r0,r3),r4
        cpu.r[4] = mem
            .read_u32(cpu.r[0].wrapping_add(cpu.r[3]))
            .unwrap_or(0);
    }
    // 0x8c21cdbc: lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c21cdbe: rts ; 0x8c21cdc0: mov r4,r0 (delay slot)
    cpu.r[0] = cpu.r[4];
}
