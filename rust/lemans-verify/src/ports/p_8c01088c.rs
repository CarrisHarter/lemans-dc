//! 0x8c01088c — call arccos (0x8c014370) on fr4, then classify fr0:
//! NaN (exponent all-set, mantissa nonzero) writes 0x44d to 0x8c23d8b8,
//! Inf (mantissa zero) writes 0x44c, finite writes nothing. Returns fr0.
//!
//! ```text
//! sts.l pr,@-r15 ; add #-4,r15
//! mov.l 0x8c0108cc,r3 (0x8c014370) ; jsr @r3 ; nop
//! r3=r15 ; r4=0x7f800000 ; fmov fr0,@r15 ; r2=[r3] ; and r4,r2
//! cmp/eq r4,r2 ; bf out
//! r2=r15 ; r1=[r2] ; r3=0x007fffff ; tst r3,r1 ; bt inf
//! nan: r2=0x8c23d8b8 ; r0=0x44d ; bra out ; mov.l r0,@r2 (delay)
//! inf: r1=0x8c23d8b8 ; r2=0x44c ; mov.l r2,@r1
//! out: fmov @r15,fr0 ; add #4,r15 ; lds.l @r15+,pr ; rts ; nop
//! ```
//!
//! T ends set only on the Inf arm (tst r3,r1 leaves it; the NaN arm's bra and
//! the finite arm's bf leave it clear). The 0x44c arm cannot be reached through
//! arccos: fr0 = π/2 − asin(fr4), and asin yields a NaN (never ±Inf) for
//! |x|>1, ±Inf and NaN inputs, so fr0 is finite or NaN. It is transcribed
//! anyway so the port covers every arm of the original.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c01088c: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 0x8c01088e: add #-4,r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    // 0x8c010890: mov.l 0x8c0108cc,r3
    cpu.r[3] = mem.read_u32(0x8c0108cc).unwrap_or(0); // 0x8c014370
    // 0x8c010892: jsr @r3 ; 0x8c010894: nop (delay slot)
    cpu.pr = 0x8c010896;
    crate::registry::p_8c014370::run(cpu, mem);
    // 0x8c010896: mov r15,r3
    cpu.r[3] = cpu.r[15];
    // 0x8c010898: mov.l 0x8c0108d0,r4
    cpu.r[4] = mem.read_u32(0x8c0108d0).unwrap_or(0); // 0x7f800000
    // 0x8c01089a: fmov fr0,@r15
    let _ = mem.write_f32(cpu.r[15], cpu.fr[0]);
    // 0x8c01089c: mov.l @r3,r2
    cpu.r[2] = mem.read_u32(cpu.r[3]).unwrap_or(0);
    // 0x8c01089e: and r4,r2
    cpu.r[2] &= cpu.r[4];
    // 0x8c0108a0: cmp/eq r4,r2
    cpu.set_t(cpu.r[2] == cpu.r[4]);
    // 0x8c0108a2: bf 0x8c0108bc
    if cpu.t() {
        // 0x8c0108a4: mov r15,r2
        cpu.r[2] = cpu.r[15];
        // 0x8c0108a6: mov.l @r2,r1
        cpu.r[1] = mem.read_u32(cpu.r[2]).unwrap_or(0);
        // 0x8c0108a8: mov.l 0x8c0108d4,r3
        cpu.r[3] = mem.read_u32(0x8c0108d4).unwrap_or(0); // 0x007fffff
        // 0x8c0108aa: tst r3,r1
        cpu.set_t(cpu.r[3] & cpu.r[1] == 0);
        // 0x8c0108ac: bt 0x8c0108b6
        if !cpu.t() {
            // 0x8c0108ae: mov.l 0x8c0108d8,r2
            cpu.r[2] = mem.read_u32(0x8c0108d8).unwrap_or(0); // 0x8c23d8b8
            // 0x8c0108b0: mov.w 0x8c0108c6,r0
            cpu.r[0] = mem.read_u16(0x8c0108c6).unwrap_or(0) as i16 as i32 as u32; // 0x44d
            // 0x8c0108b2: bra 0x8c0108bc ; 0x8c0108b4: mov.l r0,@r2 (delay)
            let _ = mem.write_u32(cpu.r[2], cpu.r[0]);
        } else {
            // 0x8c0108b6: mov.l 0x8c0108d8,r1
            cpu.r[1] = mem.read_u32(0x8c0108d8).unwrap_or(0); // 0x8c23d8b8
            // 0x8c0108b8: mov.w 0x8c0108c8,r2
            cpu.r[2] = mem.read_u16(0x8c0108c8).unwrap_or(0) as i16 as i32 as u32; // 0x44c
            // 0x8c0108ba: mov.l r2,@r1
            let _ = mem.write_u32(cpu.r[1], cpu.r[2]);
        }
    }
    // 0x8c0108bc: fmov @r15,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[15]).unwrap_or(0.0);
    // 0x8c0108be: add #4,r15
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c0108c0: lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c0108c2: rts ; 0x8c0108c4: nop (delay slot)
}
