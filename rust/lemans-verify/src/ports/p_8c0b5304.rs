//! 0x8c0b5304 — clear/initialise `count` (=[r4+4]) records of 44 bytes at
//! base=[r4]: each record gets f32 0 at +0,+8,+12,+16,+24,+32,+40, f32
//! [0x8c1cd000] at +4, and u32 r1(=0) at +20,+36. r13/r14 are pushed and
//! popped, so they end restored; the loop counter lives in r14 only while
//! running. `fldi0 fr1` is the `bra` delay slot and runs exactly once.
//!
//! Residue: r0 = count (last loop-head load), r1 = 0, r3/r5/r7 = last record
//! base +16/+12/+40, r6 = 0x8c1cd000, fr0 = [0x8c1cd000], fr1 = 0.0, T = 0
//! (final `cmp/gt r14,r0` with r14 == count).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let entry_r13 = cpu.r[13];
    let entry_r14 = cpu.r[14];
    cpu.r[6] = mem.read_u32(0x8c0b5358).unwrap_or(0); // 0x8c1cd000
    let mut r14: u32 = 0; // mov #0,r14
    let mut r13: u32 = 0; // mov r14,r13
    cpu.r[1] = 0; // mov r14,r1
    let r1 = cpu.r[1];
    cpu.fr[1] = 0.0; // fldi0 fr1 (bra delay slot)
    let r4 = cpu.r[4];
    loop {
        // 0x8c0b534c: mov.l @(4,r4),r0 ; cmp/gt r14,r0 ; bt 0x8c0b5314
        let r0 = mem.read_u32(r4.wrapping_add(4)).unwrap_or(0);
        cpu.r[0] = r0;
        let t = (r0 as i32) > (r14 as i32);
        cpu.set_t(t);
        if !t {
            break;
        }
        // body at 0x8c0b5314
        let mut r7 = mem.read_u32(r4).unwrap_or(0);
        r14 = r14.wrapping_add(1); // add #1,r14
        r7 = r7.wrapping_add(r13); // add r13,r7
        let _ = mem.write_f32(r7, cpu.fr[1]); // fmov fr1,@r7        +0
        let mut r3 = r7;
        cpu.fr[0] = mem.read_f32(cpu.r[6]).unwrap_or(0.0); // fmov @r6,fr0
        r3 = r3.wrapping_add(4);
        let mut r0 = r7;
        let _ = mem.write_f32(r3, cpu.fr[0]); // fmov fr0,@r3        +4
        r0 = r0.wrapping_add(8);
        let mut r5 = r7;
        let _ = mem.write_f32(r0, cpu.fr[1]); // fmov fr1,@r0        +8
        r5 = r5.wrapping_add(12);
        r3 = r7;
        r0 = r7;
        let _ = mem.write_f32(r5, cpu.fr[1]); // fmov fr1,@r5        +12
        r3 = r3.wrapping_add(16);
        let _ = mem.write_f32(r3, cpu.fr[1]); // fmov fr1,@r3        +16
        r0 = r0.wrapping_add(24);
        let _ = mem.write_f32(r0, cpu.fr[1]); // fmov fr1,@r0        +24
        r13 = r13.wrapping_add(44); // add #44,r13
        r0 = r7;
        let _ = mem.write_u32(r7.wrapping_add(20), r1); // mov.l r1,@(20,r7)
        r0 = r0.wrapping_add(32);
        let _ = mem.write_f32(r0, cpu.fr[1]); // fmov fr1,@r0        +32
        let _ = mem.write_u32(r7.wrapping_add(36), r1); // mov.l r1,@(36,r7)
        r7 = r7.wrapping_add(40);
        let _ = mem.write_f32(r7, cpu.fr[1]); // fmov fr1,@r7        +40
        cpu.r[0] = r0; // r7_old + 32 (reloaded at the next loop head)
        cpu.r[3] = r3;
        cpu.r[5] = r5;
        cpu.r[7] = r7;
    }
    // mov.l @r15+,r13 ; rts ; mov.l @r15+,r14 — restores entry r13/r14
    cpu.r[13] = entry_r13;
    cpu.r[14] = entry_r14;
}
