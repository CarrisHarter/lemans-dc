//! 0x8c086b14 — zero 28 bytes at r4 via a 24-byte stack scratch buffer.
//!
//! The C's `in_fr1`/FPSCR_SZ branches are fiction: the code unconditionally
//! does `fldi0 fr0` and spills it. Side effects the C does not show:
//! r15 is decremented 24 and restored only in the `rts` delay slot, so the
//! live values are r2 = r15-16, r5 = r15-24, r7 = r15-12 (r15 = entry value);
//! r0 = original r4, r1 = r4+12, r3 = 0, r4 = 0, r6 = r4+16, fr0 = 0.0.
//! Non-stack stores, in order: u32 0 at r4+0,+4,+8,+16,+20,+24 and f32 0 at r4+12.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let sp = cpu.r[15].wrapping_sub(24); // add #-24,r15
    cpu.r[15] = sp;
    cpu.fr[0] = 0.0; // fldi0 fr0
    let z = 0.0f32;
    let _ = mem.write_f32(sp + 12, z); // fmov fr0,@r2 (r2 = r15+12)
    cpu.r[7] = sp; // mov r15,r7
    let _ = mem.write_f32(sp + 16, z);
    cpu.r[6] = cpu.r[4]; // mov r4,r6
    let _ = mem.write_f32(sp + 20, z);
    cpu.r[7] = sp.wrapping_add(12); // add #12,r7
    let r0 = mem.read_u32(cpu.r[7]).unwrap_or(0); // mov.l @r7,r0
    let r2 = sp; // mov #0,r2 ; add r15,r2
    let r3 = mem.read_u32(cpu.r[7] + 4).unwrap_or(0); // mov.l @(4,r7),r3
    cpu.r[1] = cpu.r[4]; // mov r4,r1
    let _ = mem.write_u32(cpu.r[4], r0); // mov.l r0,@r4
    cpu.r[5] = sp; // mov r15,r5
    let _ = mem.write_u32(cpu.r[4] + 4, r3); // mov.l r3,@(4,r4)
    cpu.r[0] = cpu.r[4]; // mov r4,r0
    let r3 = mem.read_u32(cpu.r[7] + 8).unwrap_or(0); // mov.l @(8,r7),r3
    cpu.r[6] = cpu.r[6].wrapping_add(16); // add #16,r6
    cpu.r[1] = cpu.r[1].wrapping_add(12); // add #12,r1
    let _ = mem.write_u32(cpu.r[4] + 8, r3); // mov.l r3,@(8,r4)
    let _ = mem.write_f32(r2, z); // fmov fr0,@r2
    let _ = mem.write_f32(sp + 4, z);
    let _ = mem.write_f32(sp + 8, z);
    cpu.r[4] = mem.read_u32(cpu.r[5]).unwrap_or(0); // mov.l @r5,r4
    let r3 = mem.read_u32(cpu.r[5] + 4).unwrap_or(0); // mov.l @(4,r5),r3
    let _ = mem.write_u32(cpu.r[6], cpu.r[4]); // mov.l r4,@r6
    let _ = mem.write_u32(cpu.r[6] + 4, r3); // mov.l r3,@(4,r6)
    let r3 = mem.read_u32(cpu.r[5] + 8).unwrap_or(0); // mov.l @(8,r5),r3
    let _ = mem.write_u32(cpu.r[6] + 8, r3); // mov.l r3,@(8,r6)
    let _ = mem.write_f32(cpu.r[1], z); // fmov fr0,@r1
    cpu.r[2] = sp.wrapping_add(8); // last: mov #8,r2 ; add r15,r2
    cpu.r[15] = cpu.r[15].wrapping_add(24); // rts delay slot: add #24,r15
}
