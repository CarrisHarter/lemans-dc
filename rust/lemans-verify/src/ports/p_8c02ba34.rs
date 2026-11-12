//! 0x8c02ba34 — pick the strongest field of a per-entity record.
//!
//! `r4` points at a record table and `r5` is an index; the routine reads a
//! 32-bit flag word at `r4 + 108 + index*172`, where `index` is `r5` after a
//! `mul.l` by the pool constant 172. A global at `*(u32*)0x8c2b7578` holds the
//! record base `r6`. The result is returned in `fr0`, not `r0`.
//!
//! ```text
//! ba34 mov.l 0x8c02bae8,r0   r0 = 172
//! ba36 mul.l r0,r5           MACH:MACL = sext32(r5) * 172
//! ba38 sts macl,r5           r5 = low word (r5 = index)
//! ba3a mov.l 0x8c02baec,r0   r0 = 0x8c2b7578
//! ba3c add #108,r4           r4 += 108
//! ba3e mov.l @r0,r6          r6 = record base
//! ba40 mov r5,r0
//! ba42 mov.l @(r0,r4),r3     r3 = flags = [r4 + r5]
//! ba44 tst r3,r3
//! ba46 bf.s 0x8c02ba50
//! ba48 fldi0 fr0             delay slot: fr0 = +0.0 on both outcomes
//! ba4a add #16,r6            flags == 0: fr0 = [r6+16]
//! ba4c rts ; ba4e fmov @r6,fr0 (delay)
//! ba50 mov r3,r0 ; ba52 and #8,r0 ; ba54 tst r0,r0 ; ba56 bt 0x8c02ba5e
//! ba58 add #36,r6            bit 3: fr0 = [r6+36]
//! ba5a rts ; ba5c fmov @r6,fr0 (delay)
//! ba5e mov r3,r0 ; ba60 and #4,r0 ; ba62 tst ; ba64 bt 0x8c02ba6c
//! ba66 add #56,r6            bit 2: fr0 = [r6+56]
//! ba68 rts ; ba6a fmov @r6,fr0 (delay)
//! ba6c mov r3,r0 ; ba6e and #16,r0 ; ba70 tst ; ba72 bt 0x8c02ba84
//! ba74 mov r6,r0 ; ba76 add #76,r0 ; ba78 fmov @r0,fr1
//! ba7a fcmp/gt fr0,fr1 ; ba7c bt 0x8c02ba82
//! ba7e bra 0x8c02ba84 ; ba80 nop (delay) ; ba82 fmov fr1,fr0
//! ba84 mov r3,r0 ; ba86 and #64,r0 ; ba88 tst ; ba8a bt 0x8c02ba9c
//! ba8c mov r6,r0 ; ba8e add #116,r0 ; ba90 fmov @r0,fr1
//! ba92 fcmp/gt fr0,fr1 ; ba94 bt 0x8c02ba9a
//! ba96 bra 0x8c02ba9c ; ba98 nop ; ba9a fmov fr1,fr0
//! ba9c mov r3,r0 ; ba9e and #32,r0 ; baa0 tst ; baa2 bt 0x8c02bab4
//! baa4 mov r6,r0 ; baa6 add #96,r0 ; baa8 fmov @r0,fr1
//! baaa fcmp/gt fr0,fr1 ; baac bt 0x8c02bab2
//! baae bra 0x8c02bab4 ; bab0 nop ; bab2 fmov fr1,fr0
//! bab4 mov.l 0x8c02baf0,r1   r1 = 0x100
//! bab6 mov r3,r0 ; bab8 and r1,r0 ; baba tst ; babc bt 0x8c02bace
//! babe mov r6,r0 ; bac0 add #116,r0 ; bac2 fmov @r0,fr1
//! bac4 fcmp/gt fr0,fr1 ; bac6 bt 0x8c02bacc
//! bac8 bra 0x8c02bace ; baca nop ; bacc fmov fr1,fr0
//! bace mov.l 0x8c02baf4,r0   r0 = 0x200
//! bad0 and r0,r3             r3 = flags & 0x200
//! bad2 tst r3,r3 ; bad4 bt 0x8c02bae4
//! bad6 add #96,r6 ; bad8 fmov @r6,fr1
//! bada fcmp/gt fr0,fr1 ; badc bt 0x8c02bae2 ; bade rts ; bae0 nop
//! bae2 fmov fr1,fr0 ; bae4 rts ; bae6 nop
//! ```
//!
//! Every comparison keeps the larger value, so `fr0` ends as the maximum of
//! `[r6+16]`, `[r6+36]`, `[r6+56]`, `[r6+76]`, `[r6+116]`, `[r6+96]`,
//! `[r6+116]` and `[r6+96]` over the bits set in the flags word. The register
//! residue is path-dependent and is preserved here: `r0` is the last computed
//! address (or a masked flag), `r3` is the original flags word until the final
//! `and #0x200,r3` reduces it to 0 or 0x200, and T is the last `tst`/`fcmp/gt`
//! result. `fr1` is only written on comparison arms. No memory is stored.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c02ba34: mov.l 0x8c02bae8,r0
    cpu.r[0] = mem.read_u32(0x8c02bae8).unwrap_or(0); // 0xac = 172
    // 0x8c02ba36: mul.l r0,r5
    let p = (cpu.r[5] as i32 as i64) * (cpu.r[0] as i32 as i64);
    cpu.mach = (p >> 32) as u32;
    cpu.macl = p as u32;
    // 0x8c02ba38: sts macl,r5
    cpu.r[5] = cpu.macl;
    // 0x8c02ba3a: mov.l 0x8c02baec,r0
    cpu.r[0] = mem.read_u32(0x8c02baec).unwrap_or(0); // 0x8c2b7578
    // 0x8c02ba3c: add #108,r4
    cpu.r[4] = cpu.r[4].wrapping_add(108);
    // 0x8c02ba3e: mov.l @r0,r6
    cpu.r[6] = mem.read_u32(cpu.r[0]).unwrap_or(0);
    // 0x8c02ba40: mov r5,r0
    cpu.r[0] = cpu.r[5];
    // 0x8c02ba42: mov.l @(r0,r4),r3
    cpu.r[3] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[4])).unwrap_or(0);
    // 0x8c02ba44: tst r3,r3
    cpu.set_t(cpu.r[3] == 0);
    // 0x8c02ba46: bf.s 0x8c02ba50 ; 0x8c02ba48: fldi0 fr0 (delay, both outcomes)
    cpu.fr[0] = 0.0;
    if cpu.t() {
        // 0x8c02ba4a: add #16,r6
        cpu.r[6] = cpu.r[6].wrapping_add(16);
        // 0x8c02ba4e: fmov @r6,fr0 (rts delay slot)
        cpu.fr[0] = mem.read_f32(cpu.r[6]).unwrap_or(0.0);
        return;
    }
    // 0x8c02ba50: mov r3,r0
    cpu.r[0] = cpu.r[3];
    // 0x8c02ba52: and #8,r0
    cpu.r[0] &= 8;
    // 0x8c02ba54: tst r0,r0
    cpu.set_t(cpu.r[0] == 0);
    // 0x8c02ba56: bt 0x8c02ba5e
    if !cpu.t() {
        // 0x8c02ba58: add #36,r6
        cpu.r[6] = cpu.r[6].wrapping_add(36);
        // 0x8c02ba5c: fmov @r6,fr0 (rts delay slot)
        cpu.fr[0] = mem.read_f32(cpu.r[6]).unwrap_or(0.0);
        return;
    }
    // 0x8c02ba5e: mov r3,r0
    cpu.r[0] = cpu.r[3];
    // 0x8c02ba60: and #4,r0
    cpu.r[0] &= 4;
    // 0x8c02ba62: tst r0,r0
    cpu.set_t(cpu.r[0] == 0);
    // 0x8c02ba64: bt 0x8c02ba6c
    if !cpu.t() {
        // 0x8c02ba66: add #56,r6
        cpu.r[6] = cpu.r[6].wrapping_add(56);
        // 0x8c02ba6a: fmov @r6,fr0 (rts delay slot)
        cpu.fr[0] = mem.read_f32(cpu.r[6]).unwrap_or(0.0);
        return;
    }
    // 0x8c02ba6c: mov r3,r0
    cpu.r[0] = cpu.r[3];
    // 0x8c02ba6e: and #16,r0
    cpu.r[0] &= 16;
    // 0x8c02ba70: tst r0,r0
    cpu.set_t(cpu.r[0] == 0);
    // 0x8c02ba72: bt 0x8c02ba84
    if !cpu.t() {
        // 0x8c02ba74: mov r6,r0 ; 0x8c02ba76: add #76,r0
        cpu.r[0] = cpu.r[6].wrapping_add(76);
        // 0x8c02ba78: fmov @r0,fr1
        cpu.fr[1] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
        // 0x8c02ba7a: fcmp/gt fr0,fr1
        cpu.set_t(cpu.fr[1] > cpu.fr[0]);
        // 0x8c02ba7c: bt 0x8c02ba82 (fmov fr1,fr0); else bra 0x8c02ba84 (nop delay)
        if cpu.t() {
            cpu.fr[0] = cpu.fr[1];
        }
    }
    // 0x8c02ba84: mov r3,r0
    cpu.r[0] = cpu.r[3];
    // 0x8c02ba86: and #64,r0
    cpu.r[0] &= 64;
    // 0x8c02ba88: tst r0,r0
    cpu.set_t(cpu.r[0] == 0);
    // 0x8c02ba8a: bt 0x8c02ba9c
    if !cpu.t() {
        // 0x8c02ba8c: mov r6,r0 ; 0x8c02ba8e: add #116,r0
        cpu.r[0] = cpu.r[6].wrapping_add(116);
        // 0x8c02ba90: fmov @r0,fr1
        cpu.fr[1] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
        // 0x8c02ba92: fcmp/gt fr0,fr1
        cpu.set_t(cpu.fr[1] > cpu.fr[0]);
        // 0x8c02ba94: bt 0x8c02ba9a (fmov fr1,fr0); else bra 0x8c02ba9c (nop delay)
        if cpu.t() {
            cpu.fr[0] = cpu.fr[1];
        }
    }
    // 0x8c02ba9c: mov r3,r0
    cpu.r[0] = cpu.r[3];
    // 0x8c02ba9e: and #32,r0
    cpu.r[0] &= 32;
    // 0x8c02baa0: tst r0,r0
    cpu.set_t(cpu.r[0] == 0);
    // 0x8c02baa2: bt 0x8c02bab4
    if !cpu.t() {
        // 0x8c02baa4: mov r6,r0 ; 0x8c02baa6: add #96,r0
        cpu.r[0] = cpu.r[6].wrapping_add(96);
        // 0x8c02baa8: fmov @r0,fr1
        cpu.fr[1] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
        // 0x8c02baaa: fcmp/gt fr0,fr1
        cpu.set_t(cpu.fr[1] > cpu.fr[0]);
        // 0x8c02baac: bt 0x8c02bab2 (fmov fr1,fr0); else bra 0x8c02bab4 (nop delay)
        if cpu.t() {
            cpu.fr[0] = cpu.fr[1];
        }
    }
    // 0x8c02bab4: mov.l 0x8c02baf0,r1
    cpu.r[1] = mem.read_u32(0x8c02baf0).unwrap_or(0); // 0x100
    // 0x8c02bab6: mov r3,r0
    cpu.r[0] = cpu.r[3];
    // 0x8c02bab8: and r1,r0
    cpu.r[0] &= cpu.r[1];
    // 0x8c02baba: tst r0,r0
    cpu.set_t(cpu.r[0] == 0);
    // 0x8c02babc: bt 0x8c02bace
    if !cpu.t() {
        // 0x8c02babe: mov r6,r0 ; 0x8c02bac0: add #116,r0
        cpu.r[0] = cpu.r[6].wrapping_add(116);
        // 0x8c02bac2: fmov @r0,fr1
        cpu.fr[1] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
        // 0x8c02bac4: fcmp/gt fr0,fr1
        cpu.set_t(cpu.fr[1] > cpu.fr[0]);
        // 0x8c02bac6: bt 0x8c02bacc (fmov fr1,fr0); else bra 0x8c02bace (nop delay)
        if cpu.t() {
            cpu.fr[0] = cpu.fr[1];
        }
    }
    // 0x8c02bace: mov.l 0x8c02baf4,r0
    cpu.r[0] = mem.read_u32(0x8c02baf4).unwrap_or(0); // 0x200
    // 0x8c02bad0: and r0,r3
    cpu.r[3] &= cpu.r[0];
    // 0x8c02bad2: tst r3,r3
    cpu.set_t(cpu.r[3] == 0);
    // 0x8c02bad4: bt 0x8c02bae4
    if !cpu.t() {
        // 0x8c02bad6: add #96,r6
        cpu.r[6] = cpu.r[6].wrapping_add(96);
        // 0x8c02bad8: fmov @r6,fr1
        cpu.fr[1] = mem.read_f32(cpu.r[6]).unwrap_or(0.0);
        // 0x8c02bada: fcmp/gt fr0,fr1
        cpu.set_t(cpu.fr[1] > cpu.fr[0]);
        // 0x8c02badc: bt 0x8c02bae2
        if cpu.t() {
            // 0x8c02bae2: fmov fr1,fr0
            cpu.fr[0] = cpu.fr[1];
        }
    }
    // 0x8c02bae4: rts ; 0x8c02bae6: nop (delay slot)
}
