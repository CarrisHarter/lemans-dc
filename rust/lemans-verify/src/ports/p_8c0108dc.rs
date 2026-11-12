//! 0x8c0108dc — call asin (0x8c01438c) on fr4, then classify fr0:
//! NaN (exponent all-set, mantissa nonzero) writes 0x44d to 0x8c23d8b8,
//! Inf (mantissa zero) writes 0x44c, finite writes nothing. Returns fr0.
//!
//! The result is stashed at [r15] as raw single-precision bits and reloaded on
//! exit, so fr0 comes back bit-exact even when the callee produced a NaN.
//! asin's domain is |x| <= 1 plus NaN early-outs, so the Inf arm below is
//! transcribed but unreachable through this caller; only finite (0x8c0108f2
//! bf taken) and NaN (bf not taken, 0x8c0108fc bt not taken) occur in practice.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c0108dc: sts.l pr,@-r15 ; 0x8c0108de: add #-4,r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    // 0x8c0108e0: mov.l 0x8c01091c,r3 (0x8c01438c)
    cpu.r[3] = mem.read_u32(0x8c01091c).unwrap_or(0);
    // 0x8c0108e2: jsr @r3 ; 0x8c0108e4: nop (delay slot)
    cpu.pr = 0x8c0108e6;
    crate::registry::p_8c01438c::run(cpu, mem);
    // 0x8c0108e6: mov r15,r3
    cpu.r[3] = cpu.r[15];
    // 0x8c0108e8: mov.l 0x8c010920,r4 (0x7f800000)
    cpu.r[4] = mem.read_u32(0x8c010920).unwrap_or(0);
    // 0x8c0108ea: fmov fr0,@r15
    let _ = mem.write_f32(cpu.r[15], cpu.fr[0]);
    // 0x8c0108ec: mov.l @r3,r2 ; 0x8c0108ee: and r4,r2 ; 0x8c0108f0: cmp/eq r4,r2
    cpu.r[2] = mem.read_u32(cpu.r[3]).unwrap_or(0);
    cpu.r[2] &= cpu.r[4];
    cpu.set_t(cpu.r[2] == cpu.r[4]);
    // 0x8c0108f2: bf 0x8c01090c
    if cpu.t() {
        // 0x8c0108f4: mov r15,r2 ; 0x8c0108f6: mov.l @r2,r1
        cpu.r[2] = cpu.r[15];
        cpu.r[1] = mem.read_u32(cpu.r[2]).unwrap_or(0);
        // 0x8c0108f8: mov.l 0x8c010924,r3 (0x007fffff)
        cpu.r[3] = mem.read_u32(0x8c010924).unwrap_or(0);
        // 0x8c0108fa: tst r3,r1 ; 0x8c0108fc: bt 0x8c010906
        cpu.set_t(cpu.r[3] & cpu.r[1] == 0);
        if !cpu.t() {
            // 0x8c0108fe: mov.l 0x8c010928,r2 (0x8c23d8b8)
            cpu.r[2] = mem.read_u32(0x8c010928).unwrap_or(0);
            // 0x8c010900: mov.w 0x8c010916,r0 (0x44d)
            cpu.r[0] = mem.read_u16(0x8c010916).unwrap_or(0) as i16 as i32 as u32;
            // 0x8c010902: bra 0x8c01090c ; 0x8c010904: mov.l r0,@r2 (delay)
            let _ = mem.write_u32(cpu.r[2], cpu.r[0]);
        } else {
            // 0x8c010906: mov.l 0x8c010928,r1 (0x8c23d8b8)
            cpu.r[1] = mem.read_u32(0x8c010928).unwrap_or(0);
            // 0x8c010908: mov.w 0x8c010918,r2 (0x44c)
            cpu.r[2] = mem.read_u16(0x8c010918).unwrap_or(0) as i16 as i32 as u32;
            // 0x8c01090a: mov.l r2,@r1
            let _ = mem.write_u32(cpu.r[1], cpu.r[2]);
        }
    }
    // 0x8c01090c: fmov @r15,fr0 ; 0x8c01090e: add #4,r15
    cpu.fr[0] = mem.read_f32(cpu.r[15]).unwrap_or(0.0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c010910: lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c010912: rts ; 0x8c010914: nop (delay)
}
