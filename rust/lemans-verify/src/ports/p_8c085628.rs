//! 0x8c085628 — 4x16-bit subtraction with borrows (1000 / 60 / 60 cascades).
//!
//! Side effects the C does not show: r1 := param_3 (loaded from @(4,r15) after
//! the r14 push), r14 is restored by the rts delay slot, r0 := r3 at exit,
//! T = last cmp/pz. Only the `bt.s` (0x8D) at 0x8c085650 has a delay slot
//! (`sub r0,r3` runs always); the two plain `bt` (0x89) are UNdelayed, so the
//! `add #60` corrections run only on the fall-through (borrow) path, exactly
//! as the C shows. Borrows: if (s16)(p1[0]-p2[0]) < 0 { r7 -= 1; r6 += 1000 },
//! if (s16)r7 < 0 { r7 += 60; r14 -= 1 }, if (s16)r14 < 0 { r14 += 60; r3 -= 1 }.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4); // mov.l r14,@-r15 (stack window)
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[1] = mem.read_u32(cpu.r[15].wrapping_add(4)).unwrap_or(0); // param_3
    let (p1, p2) = (cpu.r[4], cpu.r[5]);
    let a1 = mem.read_u16(p1.wrapping_add(2)).unwrap_or(0) as u32; // mov.w @(2,r4),r0
    cpu.r[7] = a1; // mov r0,r7
    let a0 = mem.read_u16(p1).unwrap_or(0) as u32; // mov.w @r4,r3
    let b1 = mem.read_u16(p2.wrapping_add(2)).unwrap_or(0) as u32; // mov.w @(2,r5),r0
    cpu.r[6] = a0 as i16 as i32 as u32; // exts.w r3,r6
    let b0 = mem.read_u16(p2).unwrap_or(0); // mov.w @r5,r3
    cpu.r[7] = cpu.r[7].wrapping_sub(b1); // sub r0,r7
    let a2 = mem.read_u16(p1.wrapping_add(4)).unwrap_or(0) as u32; // mov.w @(4,r4),r0
    cpu.r[3] = b0 as i16 as i32 as u32; // exts.w r3,r3
    cpu.r[6] = cpu.r[6].wrapping_sub(cpu.r[3]); // sub r3,r6
    cpu.r[14] = a2; // mov r0,r14
    let b2 = mem.read_u16(p2.wrapping_add(4)).unwrap_or(0) as u32; // mov.w @(4,r5),r0
    let s2 = (cpu.r[6] as i16) as i32; // exts.w r6,r3
    cpu.set_t(s2 >= 0); // cmp/pz r3
    cpu.r[14] = cpu.r[14].wrapping_sub(b2); // sub r0,r14
    let a3 = mem.read_u16(p1.wrapping_add(6)).unwrap_or(0) as u32; // mov.w @(6,r4),r0
    cpu.r[3] = a3; // mov r0,r3
    let b3 = mem.read_u16(p2.wrapping_add(6)).unwrap_or(0) as u32; // mov.w @(6,r5),r0
    cpu.r[3] = cpu.r[3].wrapping_sub(b3); // bt.s delay slot — ALWAYS runs
    if s2 < 0 {
        let k = mem.read_u32(0x8c085680).unwrap_or(0); // mov.l literal -> 1000
        cpu.r[7] = cpu.r[7].wrapping_sub(1); // add #-1,r7
        cpu.r[6] = cpu.r[6].wrapping_add(k); // add r0,r6
    }
    let s3 = (cpu.r[7] as i16) as i32; // exts.w r7,r0
    cpu.set_t(s3 >= 0); // cmp/pz r0
    if s3 < 0 {
        // plain bt (0x89) is UNdelayed: fall-through runs both adds
        cpu.r[7] = cpu.r[7].wrapping_add(60); // add #60,r7
        cpu.r[14] = cpu.r[14].wrapping_sub(1); // add #-1,r14
    }
    let s4 = (cpu.r[14] as i16) as i32; // exts.w r14,r0
    cpu.set_t(s4 >= 0); // cmp/pz r0
    if s4 < 0 {
        cpu.r[14] = cpu.r[14].wrapping_add(60); // add #60,r14
        cpu.r[3] = cpu.r[3].wrapping_sub(1); // add #-1,r3
    }
    let p3 = cpu.r[1];
    let _ = mem.write_u16(p3, cpu.r[6] as u16); // mov.w r6,@r1
    cpu.r[0] = cpu.r[7]; // mov r7,r0
    let _ = mem.write_u16(p3.wrapping_add(2), cpu.r[7] as u16); // mov.w r0,@(2,r1)
    cpu.r[0] = cpu.r[14]; // mov r14,r0
    let _ = mem.write_u16(p3.wrapping_add(4), cpu.r[14] as u16); // mov.w r0,@(4,r1)
    cpu.r[0] = cpu.r[3]; // mov r3,r0
    let _ = mem.write_u16(p3.wrapping_add(6), cpu.r[3] as u16); // mov.w r0,@(6,r1)
    // rts delay slot: mov.l @r15+,r14
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
