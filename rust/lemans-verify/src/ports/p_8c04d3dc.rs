//! 0x8c04d3dc — sum two vectors reached through a tree lookup, scale by 0.5,
//! and copy the three floats to a caller-supplied pointer.
//!
//! ```text
//! push r14, r13, r12, pr ; add #-24,r15
//! r14 = [entry_r15]              ; 5th argument: output pointer
//! r12 = r4
//! w   = 0x8c0bfb4c(r4 + 0x60, 1) ; tree lookup, key 1, twice
//! r13 = 0x8c0c010c(w)            ; r13 = [w+4] + 8
//! r0  = 0x8c0c0100(w)            ; r0  = [w+4] + 56
//! push r15 ; r4 = r13 ; r5 = r0  ; dst = frame base (stack argument)
//! 0x8c0b4f94                     ; frame[0..2] = r13[0..2] + r0[0..2]
//! frame[3..5] = frame[0..2]      ; via fr0/fr1
//! 0x8c0b4ef0(frame+12, fr4=0.5)  ; frame[3..5] *= 0.5 in place
//! [r14] = frame[3] ; [r14+4] = frame[4] ; [r14+8] = frame[5]
//! ```
//!
//! Exit state: r0 = out+4, r1/r3/r6/r7 hold the 0x8c0b4f94 residue, r2 =
//! frame+20, r4 = frame+20 (the 0x8c0b4ef0 residue), r12 = r4 entry + 0x60,
//! r13 = [w+4]+8, r14/r15/pr restored to their entry values. No flag-setting
//! instruction of our own; T is whatever the last callee left.
//!
//! The two `0x8c0bfb4c` calls are identical (same argument, same key), so the
//! tree walk, its insert-pointer store and both thunks run twice per call.

use sh4_oracle::{Cpu, Memory};

fn call(cpu: &mut Cpu, mem: &mut Memory, target: u32, from: u32) {
    match crate::registry::lookup(target) {
        Some(port) => port(cpu, mem),
        None => panic!("0x{from:08x}: call to unported target 0x{target:08x}"),
    }
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 8c04d3dc: mov.l r14,@-r15 ; 8c04d3de: mov.l r13,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    // 8c04d3e0: mov.l r12,@-r15 ; 8c04d3e2: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[12]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 8c04d3e4: add #-24,r15
    cpu.r[15] = cpu.r[15].wrapping_sub(24);
    // 8c04d3e6: mov.l @(40,r15),r14 — the caller's 5th argument (out pointer)
    cpu.r[14] = mem.read_u32(cpu.r[15].wrapping_add(40)).unwrap_or(0);
    // 8c04d3e8: mov r4,r12
    cpu.r[12] = cpu.r[4];
    // 8c04d3ea: mov.l 0x8c04d474,r0 (0x8c0bfb4c) ; 8c04d3ec: add #96,r4
    cpu.r[0] = mem.read_u32(0x8c04d474).unwrap_or(0);
    cpu.r[4] = cpu.r[4].wrapping_add(96);
    // 8c04d3ee: jsr @r0 ; delay 8c04d3f0: mov #1,r5
    let target = cpu.r[0];
    cpu.pr = 0x8c04d3f2;
    cpu.r[5] = 1;
    call(cpu, mem, target, 0x8c04d3ee);
    // 8c04d3f2: mov r0,r4 ; 8c04d3f4: mov.l 0x8c04d478,r0 (0x8c0c010c)
    cpu.r[4] = cpu.r[0];
    cpu.r[0] = mem.read_u32(0x8c04d478).unwrap_or(0);
    // 8c04d3f6: jsr @r0 ; delay 8c04d3f8: nop
    let target = cpu.r[0];
    cpu.pr = 0x8c04d3fa;
    call(cpu, mem, target, 0x8c04d3f6);
    // 8c04d3fa: mov r0,r13 ; 8c04d3fc: mov.l 0x8c04d474,r0 (0x8c0bfb4c)
    cpu.r[13] = cpu.r[0];
    cpu.r[0] = mem.read_u32(0x8c04d474).unwrap_or(0);
    // 8c04d3fe: add #96,r12 ; 8c04d400: mov r12,r4
    cpu.r[12] = cpu.r[12].wrapping_add(96);
    cpu.r[4] = cpu.r[12];
    // 8c04d402: jsr @r0 ; delay 8c04d404: mov #1,r5
    let target = cpu.r[0];
    cpu.pr = 0x8c04d406;
    cpu.r[5] = 1;
    call(cpu, mem, target, 0x8c04d402);
    // 8c04d406: mov r0,r4 ; 8c04d408: mov.l 0x8c04d47c,r0 (0x8c0c0100)
    cpu.r[4] = cpu.r[0];
    cpu.r[0] = mem.read_u32(0x8c04d47c).unwrap_or(0);
    // 8c04d40a: jsr @r0 ; delay 8c04d40c: nop
    let target = cpu.r[0];
    cpu.pr = 0x8c04d40e;
    call(cpu, mem, target, 0x8c04d40a);
    // 8c04d40e: mov r15,r1 ; 8c04d410: mov.l r1,@-r15
    cpu.r[1] = cpu.r[15];
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[1]);
    // 8c04d412: mov r0,r5 ; 8c04d414: mov.l 0x8c04d480,r0 (0x8c0b4f94)
    cpu.r[5] = cpu.r[0];
    cpu.r[0] = mem.read_u32(0x8c04d480).unwrap_or(0);
    // 8c04d416: jsr @r0 ; delay 8c04d418: mov r13,r4
    let target = cpu.r[0];
    cpu.pr = 0x8c04d41a;
    cpu.r[4] = cpu.r[13];
    call(cpu, mem, target, 0x8c04d416);
    // 8c04d41a: add #4,r15
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 8c04d41c: mov #4,r2 ; 8c04d41e: add r15,r2 ; 8c04d420: fmov @r2,fr1
    cpu.r[2] = 4;
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[15]);
    cpu.fr[1] = mem.read_f32(cpu.r[2]).unwrap_or(0.0);
    // 8c04d422: mov #0,r0 ; 8c04d424: fmov @(r0,r15),fr0
    cpu.r[0] = 0;
    cpu.fr[0] = mem.read_f32(cpu.r[0].wrapping_add(cpu.r[15])).unwrap_or(0.0);
    // 8c04d426: mov #12,r2 ; 8c04d428: add r15,r2 ; 8c04d42a: fmov fr0,@r2
    cpu.r[2] = 12;
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[15]);
    let _ = mem.write_f32(cpu.r[2], cpu.fr[0]);
    // 8c04d42c: mov r15,r4 ; 8c04d42e: mov #8,r2 ; 8c04d430: add r15,r2
    cpu.r[4] = cpu.r[15];
    cpu.r[2] = 8;
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[15]);
    // 8c04d432: fmov @r2,fr0 ; 8c04d434: mova 0x8c04d484,r0 ; 8c04d436: add #12,r4
    cpu.fr[0] = mem.read_f32(cpu.r[2]).unwrap_or(0.0);
    cpu.r[0] = 0x8c04d484;
    cpu.r[4] = cpu.r[4].wrapping_add(12);
    // 8c04d438: mov #16,r2 ; 8c04d43a: add r15,r2 ; 8c04d43c: fmov fr1,@r2
    cpu.r[2] = 16;
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[15]);
    let _ = mem.write_f32(cpu.r[2], cpu.fr[1]);
    // 8c04d43e: mov #20,r2 ; 8c04d440: mov.l 0x8c04d488,r1 (0x8c0b4ef0)
    cpu.r[2] = 20;
    cpu.r[1] = mem.read_u32(0x8c04d488).unwrap_or(0);
    // 8c04d442: add r15,r2 ; 8c04d444: fmov fr0,@r2
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[15]);
    let _ = mem.write_f32(cpu.r[2], cpu.fr[0]);
    // 8c04d446: jsr @r1 ; delay 8c04d448: fmov @r0,fr4
    let target = cpu.r[1];
    cpu.pr = 0x8c04d44a;
    cpu.fr[4] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
    call(cpu, mem, target, 0x8c04d446);
    // 8c04d44a: mov #12,r0 ; 8c04d44c: fmov @(r0,r15),fr0
    cpu.r[0] = 12;
    cpu.fr[0] = mem.read_f32(cpu.r[0].wrapping_add(cpu.r[15])).unwrap_or(0.0);
    // 8c04d44e: mov #16,r2 ; 8c04d450: add r15,r2 ; 8c04d452: fmov fr0,@r14
    cpu.r[2] = 16;
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[15]);
    let _ = mem.write_f32(cpu.r[14], cpu.fr[0]);
    // 8c04d454: mov r14,r0 ; 8c04d456: fmov @r2,fr0 ; 8c04d458: add #8,r14
    cpu.r[0] = cpu.r[14];
    cpu.fr[0] = mem.read_f32(cpu.r[2]).unwrap_or(0.0);
    cpu.r[14] = cpu.r[14].wrapping_add(8);
    // 8c04d45a: add #4,r0 ; 8c04d45c: mov #20,r2 ; 8c04d45e: fmov fr0,@r0
    cpu.r[0] = cpu.r[0].wrapping_add(4);
    cpu.r[2] = 20;
    let _ = mem.write_f32(cpu.r[0], cpu.fr[0]);
    // 8c04d460: add r15,r2 ; 8c04d462: fmov @r2,fr0 ; 8c04d464: fmov fr0,@r14
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[15]);
    cpu.fr[0] = mem.read_f32(cpu.r[2]).unwrap_or(0.0);
    let _ = mem.write_f32(cpu.r[14], cpu.fr[0]);
    // 8c04d466: add #24,r15 ; 8c04d468: lds.l @r15+,pr
    cpu.r[15] = cpu.r[15].wrapping_add(24);
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 8c04d46a: mov.l @r15+,r12 ; 8c04d46c: mov.l @r15+,r13
    cpu.r[12] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 8c04d46e: rts ; delay 8c04d470: mov.l @r15+,r14
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
