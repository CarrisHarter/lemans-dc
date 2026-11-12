//! 0x8c1b69ec — submit 11 handles to 0x8c16eaec with one shared FP triple.
//!
//! First loop: 7 calls with r4 = `[r14+28+4i]`; second loop: 4 calls with
//! r4 = `[r14+56+4i]`. Every call passes fr4 = the float at the caller's
//! stack word 0, fr5 = word 1 (also moved from fr14), fr6 = word 2 (fr15);
//! 0x8c16eaec stages those three floats on the handle and flags it. r12, r13,
//! r14, pr, fr14 and fr15 are saved and restored; r0 ends as 4 and T clear.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c1b69ec: fmov fr15,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_f32(cpu.r[15], cpu.fr[15]);
    // 0x8c1b69ee: fmov fr14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_f32(cpu.r[15], cpu.fr[14]);
    // 0x8c1b69f0: mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // 0x8c1b69f2: mov.l r13,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    // 0x8c1b69f4: mov r4,r14
    cpu.r[14] = cpu.r[4];
    // 0x8c1b69f6: mov.l r12,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[12]);
    // 0x8c1b69f8: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 0x8c1b69fa: mov #32,r0 ; 0x8c1b69fc: fmov @(r0,r15),fr15
    cpu.r[0] = 32;
    cpu.fr[15] = mem.read_f32(cpu.r[0].wrapping_add(cpu.r[15])).unwrap_or(0.0);
    // 0x8c1b69fe: mov #0,r13 ; 0x8c1b6a00: mov r13,r12
    cpu.r[13] = 0;
    cpu.r[12] = cpu.r[13];
    // 0x8c1b6a02: mov #28,r0 ; 0x8c1b6a04: fmov @(r0,r15),fr14
    cpu.r[0] = 28;
    cpu.fr[14] = mem.read_f32(cpu.r[0].wrapping_add(cpu.r[15])).unwrap_or(0.0);
    // 0x8c1b6a06: shll2 r12
    cpu.r[12] <<= 2;

    loop {
        // 0x8c1b6a08: mov r14,r1
        cpu.r[1] = cpu.r[14];
        // 0x8c1b6a0a: mov r12,r0
        cpu.r[0] = cpu.r[12];
        // 0x8c1b6a0c: add #28,r1
        cpu.r[1] = cpu.r[1].wrapping_add(28);
        // 0x8c1b6a0e: mov.l @(r0,r1),r4
        cpu.r[4] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[1])).unwrap_or(0);
        // 0x8c1b6a10: mov #24,r2 ; 0x8c1b6a12: add r15,r2
        cpu.r[2] = 24u32.wrapping_add(cpu.r[15]);
        // 0x8c1b6a14: fmov @r2,fr4
        cpu.fr[4] = mem.read_f32(cpu.r[2]).unwrap_or(0.0);
        // 0x8c1b6a16: fmov fr15,fr6
        cpu.fr[6] = cpu.fr[15];
        // 0x8c1b6a18: mov.l 0x8c1b6a5c,r0 (0x8c16eaec)
        cpu.r[0] = mem.read_u32(0x8c1b6a5c).unwrap_or(0);
        // 0x8c1b6a1a: jsr @r0 ; 0x8c1b6a1c: fmov fr14,fr5 (delay slot)
        cpu.pr = 0x8c1b6a1e;
        cpu.fr[5] = cpu.fr[14];
        crate::registry::p_8c16eaec::run(cpu, mem);
        // 0x8c1b6a1e: add #1,r13
        cpu.r[13] = cpu.r[13].wrapping_add(1);
        // 0x8c1b6a20: mov #7,r0 ; 0x8c1b6a22: cmp/gt r13,r0
        cpu.r[0] = 7;
        cpu.set_t((cpu.r[0] as i32) > (cpu.r[13] as i32));
        // 0x8c1b6a24: bt.s 0x8c1b6a08 ; 0x8c1b6a26: add #4,r12 (delay slot)
        let more = cpu.t();
        cpu.r[12] = cpu.r[12].wrapping_add(4);
        if !more {
            break;
        }
    }

    // 0x8c1b6a28: mov #0,r13 ; 0x8c1b6a2a: mov r13,r12
    cpu.r[13] = 0;
    cpu.r[12] = cpu.r[13];
    // 0x8c1b6a2c: shll2 r12
    cpu.r[12] <<= 2;

    loop {
        // 0x8c1b6a2e: mov r14,r1
        cpu.r[1] = cpu.r[14];
        // 0x8c1b6a30: mov r12,r0
        cpu.r[0] = cpu.r[12];
        // 0x8c1b6a32: add #56,r1
        cpu.r[1] = cpu.r[1].wrapping_add(56);
        // 0x8c1b6a34: mov.l @(r0,r1),r4
        cpu.r[4] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[1])).unwrap_or(0);
        // 0x8c1b6a36: mov #24,r2 ; 0x8c1b6a38: add r15,r2
        cpu.r[2] = 24u32.wrapping_add(cpu.r[15]);
        // 0x8c1b6a3a: fmov @r2,fr4
        cpu.fr[4] = mem.read_f32(cpu.r[2]).unwrap_or(0.0);
        // 0x8c1b6a3c: fmov fr15,fr6
        cpu.fr[6] = cpu.fr[15];
        // 0x8c1b6a3e: mov.l 0x8c1b6a5c,r0 (0x8c16eaec)
        cpu.r[0] = mem.read_u32(0x8c1b6a5c).unwrap_or(0);
        // 0x8c1b6a40: jsr @r0 ; 0x8c1b6a42: fmov fr14,fr5 (delay slot)
        cpu.pr = 0x8c1b6a44;
        cpu.fr[5] = cpu.fr[14];
        crate::registry::p_8c16eaec::run(cpu, mem);
        // 0x8c1b6a44: add #1,r13
        cpu.r[13] = cpu.r[13].wrapping_add(1);
        // 0x8c1b6a46: mov #4,r0 ; 0x8c1b6a48: cmp/gt r13,r0
        cpu.r[0] = 4;
        cpu.set_t((cpu.r[0] as i32) > (cpu.r[13] as i32));
        // 0x8c1b6a4a: bt.s 0x8c1b6a2e ; 0x8c1b6a4c: add #4,r12 (delay slot)
        let more = cpu.t();
        cpu.r[12] = cpu.r[12].wrapping_add(4);
        if !more {
            break;
        }
    }

    // 0x8c1b6a4e: lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c1b6a50: mov.l @r15+,r12
    cpu.r[12] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c1b6a52: mov.l @r15+,r13
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c1b6a54: mov.l @r15+,r14
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c1b6a56: fmov @r15+,fr14
    cpu.fr[14] = mem.read_f32(cpu.r[15]).unwrap_or(0.0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c1b6a58: rts ; 0x8c1b6a5a: fmov @r15+,fr15 (delay slot)
    cpu.fr[15] = mem.read_f32(cpu.r[15]).unwrap_or(0.0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
