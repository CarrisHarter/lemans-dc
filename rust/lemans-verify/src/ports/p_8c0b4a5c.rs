//! 0x8c0b4a5c — normalise a transform record in place.
//!
//! `r4` points at a 36-byte record: a tag byte at +0, four matrix floats at
//! +4..+20, a 3-float vector at +20..+32 and a divisor at +32. The routine
//! negates [r4+4], replaces [r4+32] with its reciprocal unless it is exactly
//! 1.0, overwrites the vector with `-(M * v) / [r4+32]`, then clears the tag.
//!
//! The vector is produced on the stack by two callees: the product
//! 0x8c0b3d84 (r4=matrix at +4, r5=vector at +20, destination pushed) and
//! then the scale 0x8c0b4e30 by `-1/[r4+32]`. When the divisor is already 1.0
//! the reciprocal store is skipped and the negation 0x8c0b4db4 is used
//! instead. Both arms end with the same 3-word copy into +20..+32.
//!
//! ```text
//! push r14 ; push pr ; add #-48,r15 ; r14=r4 ; r4+=4
//! fr0=[r4] ; r0=r14+32 ; fr0=-fr0 ; [r4]=fr0 ; fr1=[r0] ; fldi1 fr0
//! fcmp/eq fr0,fr1 ; bt 0x8c0b4aba
//!   fdiv fr1,fr0 ; [r0]=fr0
//!   push r15+24 ; r0=0x8c0b3d84 ; r5=r14+20 ; jsr @r0
//!   pop ; push r15+36 ; r0=r14+32 ; fr4=[r0] ; r4=r15+28 ; r0=0x8c0b4e30
//!   jsr @r0 (delay: fneg fr4) ; pop
//!   r3=r15+36 ; r4=r14+20 ; copy [r3..+8] -> [r4..+8] ; bra 0x8c0b4aec
//! bt:
//!   push r15 ; r0=0x8c0b3d84 ; r5=r14+20 ; jsr @r0
//!   pop ; push r15+12 ; r4=r15 ; r0=0x8c0b4db4 ; jsr @r0 (delay: add #4,r4)
//!   pop ; r3=r15+12 ; r4=r14+20 ; copy [r3..+8] -> [r4..+8]
//! out: r0=0 ; mov.b r0,@r14 ; r15+=48 ; lds.l pr ; rts ; mov.l @r15+,r14
//! ```

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c0b4a5c: mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // 0x8c0b4a5e: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 0x8c0b4a60: add #-48,r15
    cpu.r[15] = cpu.r[15].wrapping_sub(48);
    // 0x8c0b4a62: mov r4,r14
    cpu.r[14] = cpu.r[4];
    // 0x8c0b4a64: add #4,r4
    cpu.r[4] = cpu.r[4].wrapping_add(4);
    // 0x8c0b4a66: fmov @r4,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[4]).unwrap_or(0.0);
    // 0x8c0b4a68: mov r14,r0
    cpu.r[0] = cpu.r[14];
    // 0x8c0b4a6a: add #32,r0
    cpu.r[0] = cpu.r[0].wrapping_add(32);
    // 0x8c0b4a6c: fneg fr0
    cpu.fr[0] = -cpu.fr[0];
    // 0x8c0b4a6e: fmov fr0,@r4
    let _ = mem.write_f32(cpu.r[4], cpu.fr[0]);
    // 0x8c0b4a70: fmov @r0,fr1
    cpu.fr[1] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
    // 0x8c0b4a72: fldi1 fr0
    cpu.fr[0] = 1.0;
    // 0x8c0b4a74: fcmp/eq fr0,fr1
    cpu.set_t(cpu.fr[1] == cpu.fr[0]);
    // 0x8c0b4a76: bt 0x8c0b4aba
    if !cpu.t() {
        // 0x8c0b4a78: fdiv fr1,fr0
        cpu.fr[0] = cpu.fr[0] / cpu.fr[1];
        // 0x8c0b4a7a: fmov fr0,@r0
        let _ = mem.write_f32(cpu.r[0], cpu.fr[0]);
        // 0x8c0b4a7c: mov r15,r0
        cpu.r[0] = cpu.r[15];
        // 0x8c0b4a7e: add #24,r0
        cpu.r[0] = cpu.r[0].wrapping_add(24);
        // 0x8c0b4a80: mov.l r0,@-r15 (stack argument: destination)
        cpu.r[15] = cpu.r[15].wrapping_sub(4);
        let _ = mem.write_u32(cpu.r[15], cpu.r[0]);
        // 0x8c0b4a82: mov.l 0x8c0b4af8,r0
        cpu.r[0] = mem.read_u32(0x8c0b4af8).unwrap_or(0); // 0x8c0b3d84
        // 0x8c0b4a84: mov r14,r5
        cpu.r[5] = cpu.r[14];
        // 0x8c0b4a86: jsr @r0 ; 0x8c0b4a88: add #20,r5 (delay slot)
        cpu.pr = 0x8c0b4a8a;
        cpu.r[5] = cpu.r[5].wrapping_add(20);
        crate::registry::p_8c0b3d84::run(cpu, mem);
        // 0x8c0b4a8a: add #4,r15
        cpu.r[15] = cpu.r[15].wrapping_add(4);
        // 0x8c0b4a8c: mov r15,r0
        cpu.r[0] = cpu.r[15];
        // 0x8c0b4a8e: add #36,r0
        cpu.r[0] = cpu.r[0].wrapping_add(36);
        // 0x8c0b4a90: mov.l r0,@-r15 (stack argument: destination)
        cpu.r[15] = cpu.r[15].wrapping_sub(4);
        let _ = mem.write_u32(cpu.r[15], cpu.r[0]);
        // 0x8c0b4a92: mov r14,r0
        cpu.r[0] = cpu.r[14];
        // 0x8c0b4a94: add #32,r0
        cpu.r[0] = cpu.r[0].wrapping_add(32);
        // 0x8c0b4a96: fmov @r0,fr4
        cpu.fr[4] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
        // 0x8c0b4a98: mov r15,r4
        cpu.r[4] = cpu.r[15];
        // 0x8c0b4a9a: add #28,r4
        cpu.r[4] = cpu.r[4].wrapping_add(28);
        // 0x8c0b4a9c: mov.l 0x8c0b4afc,r0
        cpu.r[0] = mem.read_u32(0x8c0b4afc).unwrap_or(0); // 0x8c0b4e30
        // 0x8c0b4a9e: jsr @r0 ; 0x8c0b4aa0: fneg fr4 (delay slot)
        cpu.pr = 0x8c0b4aa2;
        cpu.fr[4] = -cpu.fr[4];
        crate::registry::p_8c0b4e30::run(cpu, mem);
        // 0x8c0b4aa2: add #4,r15
        cpu.r[15] = cpu.r[15].wrapping_add(4);
        // 0x8c0b4aa4: mov r15,r3
        cpu.r[3] = cpu.r[15];
        // 0x8c0b4aa6: add #36,r3
        cpu.r[3] = cpu.r[3].wrapping_add(36);
        // 0x8c0b4aa8: mov r14,r4
        cpu.r[4] = cpu.r[14];
        // 0x8c0b4aaa: mov.l @r3,r1
        cpu.r[1] = mem.read_u32(cpu.r[3]).unwrap_or(0);
        // 0x8c0b4aac: add #20,r4
        cpu.r[4] = cpu.r[4].wrapping_add(20);
        // 0x8c0b4aae: mov.l @(4,r3),r0
        cpu.r[0] = mem.read_u32(cpu.r[3].wrapping_add(4)).unwrap_or(0);
        // 0x8c0b4ab0: mov.l r1,@r4
        let _ = mem.write_u32(cpu.r[4], cpu.r[1]);
        // 0x8c0b4ab2: mov.l r0,@(4,r4)
        let _ = mem.write_u32(cpu.r[4].wrapping_add(4), cpu.r[0]);
        // 0x8c0b4ab4: mov.l @(8,r3),r0
        cpu.r[0] = mem.read_u32(cpu.r[3].wrapping_add(8)).unwrap_or(0);
        // 0x8c0b4ab6: bra 0x8c0b4aec ; 0x8c0b4ab8: mov.l r0,@(8,r4) (delay slot)
        let _ = mem.write_u32(cpu.r[4].wrapping_add(8), cpu.r[0]);
    } else {
        // 0x8c0b4aba: mov r15,r0
        cpu.r[0] = cpu.r[15];
        // 0x8c0b4abc: mov.l r0,@-r15 (stack argument: destination)
        cpu.r[15] = cpu.r[15].wrapping_sub(4);
        let _ = mem.write_u32(cpu.r[15], cpu.r[0]);
        // 0x8c0b4abe: mov.l 0x8c0b4af8,r0
        cpu.r[0] = mem.read_u32(0x8c0b4af8).unwrap_or(0); // 0x8c0b3d84
        // 0x8c0b4ac0: mov r14,r5
        cpu.r[5] = cpu.r[14];
        // 0x8c0b4ac2: jsr @r0 ; 0x8c0b4ac4: add #20,r5 (delay slot)
        cpu.pr = 0x8c0b4ac6;
        cpu.r[5] = cpu.r[5].wrapping_add(20);
        crate::registry::p_8c0b3d84::run(cpu, mem);
        // 0x8c0b4ac6: add #4,r15
        cpu.r[15] = cpu.r[15].wrapping_add(4);
        // 0x8c0b4ac8: mov r15,r0
        cpu.r[0] = cpu.r[15];
        // 0x8c0b4aca: add #12,r0
        cpu.r[0] = cpu.r[0].wrapping_add(12);
        // 0x8c0b4acc: mov.l r0,@-r15 (stack argument: destination)
        cpu.r[15] = cpu.r[15].wrapping_sub(4);
        let _ = mem.write_u32(cpu.r[15], cpu.r[0]);
        // 0x8c0b4ace: mov r15,r4
        cpu.r[4] = cpu.r[15];
        // 0x8c0b4ad0: mov.l 0x8c0b4b00,r0
        cpu.r[0] = mem.read_u32(0x8c0b4b00).unwrap_or(0); // 0x8c0b4db4
        // 0x8c0b4ad2: jsr @r0 ; 0x8c0b4ad4: add #4,r4 (delay slot)
        cpu.pr = 0x8c0b4ad6;
        cpu.r[4] = cpu.r[4].wrapping_add(4);
        crate::registry::p_8c0b4db4::run(cpu, mem);
        // 0x8c0b4ad6: add #4,r15
        cpu.r[15] = cpu.r[15].wrapping_add(4);
        // 0x8c0b4ad8: mov r15,r3
        cpu.r[3] = cpu.r[15];
        // 0x8c0b4ada: add #12,r3
        cpu.r[3] = cpu.r[3].wrapping_add(12);
        // 0x8c0b4adc: mov r14,r4
        cpu.r[4] = cpu.r[14];
        // 0x8c0b4ade: mov.l @r3,r1
        cpu.r[1] = mem.read_u32(cpu.r[3]).unwrap_or(0);
        // 0x8c0b4ae0: add #20,r4
        cpu.r[4] = cpu.r[4].wrapping_add(20);
        // 0x8c0b4ae2: mov.l @(4,r3),r0
        cpu.r[0] = mem.read_u32(cpu.r[3].wrapping_add(4)).unwrap_or(0);
        // 0x8c0b4ae4: mov.l r1,@r4
        let _ = mem.write_u32(cpu.r[4], cpu.r[1]);
        // 0x8c0b4ae6: mov.l r0,@(4,r4)
        let _ = mem.write_u32(cpu.r[4].wrapping_add(4), cpu.r[0]);
        // 0x8c0b4ae8: mov.l @(8,r3),r0
        cpu.r[0] = mem.read_u32(cpu.r[3].wrapping_add(8)).unwrap_or(0);
        // 0x8c0b4aea: mov.l r0,@(8,r4)
        let _ = mem.write_u32(cpu.r[4].wrapping_add(8), cpu.r[0]);
    }
    // 0x8c0b4aec: mov #0,r0
    cpu.r[0] = 0;
    // 0x8c0b4aee: mov.b r0,@r14
    let _ = mem.write_u8(cpu.r[14], 0);
    // 0x8c0b4af0: add #48,r15
    cpu.r[15] = cpu.r[15].wrapping_add(48);
    // 0x8c0b4af2: lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c0b4af4: rts ; 0x8c0b4af6: mov.l @r15+,r14 (delay slot)
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
