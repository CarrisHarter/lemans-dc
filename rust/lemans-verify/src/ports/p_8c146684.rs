//! 0x8c146684 — call 0x8c146d78 on every non-null entry of a 40-word table.
//!
//! `r12` is the table base, `r13` the byte offset (0, 4, … 156) and `r14` the
//! iteration counter. Each non-null word is passed to 0x8c146d78 through
//! `jsr @r0`; the `bt` is immediate (no delay slot) and the loop back-edge is
//! `bt.s` whose delay slot `add #4,r13` also runs on the final, not-taken
//! iteration. The last flag-setting instruction is `cmp/gt r14,r0` with
//! `r0 = 40`, so `T` is 0 at both exits. `r4` ends holding the last entry read.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // mov.l r14/@-r15 ; mov.l r13/@-r15 ; mov.l r12/@-r15 ; sts.l pr/@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[12]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);

    cpu.r[12] = cpu.r[4]; // mov r4,r12
    cpu.r[14] = 0; // mov #0,r14
    cpu.r[13] = cpu.r[14]; // mov r14,r13

    loop {
        // 0x8c146692: mov r13,r0 ; mov.l @(r0,r12),r4
        cpu.r[0] = cpu.r[13];
        cpu.r[4] = mem
            .read_u32(cpu.r[0].wrapping_add(cpu.r[12]))
            .unwrap_or(0);
        let t = cpu.r[4] == 0; // tst r4,r4
        cpu.set_t(t);
        if !t {
            // bt not taken: mov.l 0x8c1466b4,r0 ; jsr @r0 ; nop
            cpu.r[0] = mem.read_u32(0x8c1466b4).unwrap_or(0);
            cpu.pr = 0x8c1466a0;
            crate::registry::p_8c146d78::run(cpu, mem);
        }
        // 0x8c1466a0: add #1,r14 ; mov #40,r0 ; cmp/gt r14,r0
        cpu.r[14] = cpu.r[14].wrapping_add(1);
        cpu.r[0] = 40;
        let t = (cpu.r[0] as i32) > (cpu.r[14] as i32);
        cpu.set_t(t);
        cpu.r[13] = cpu.r[13].wrapping_add(4); // bt.s delay slot
        if !t {
            break;
        }
    }

    // lds.l @r15+,pr ; mov.l @r15+,r12 ; mov.l @r15+,r13 ; rts ; mov.l @r15+,r14
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[12] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
