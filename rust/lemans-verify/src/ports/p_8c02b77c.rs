//! 0x8c02b77c — derive a scale factor (and stamp it) from a per-object flags
//! word.
//!
//! Arguments: r4 = object base, r5 = index; the record is `r5 * 0xac` bytes
//! into r4. `r1 = [r4 + 0x6c]` is the flag word.
//!
//! ```text
//! r0=0xac ; mul.l r0,r5 ; sts macl,r5 ; r0=0x8c2b7578
//! r4 += r5 ; r4 += 88 ; r1=[r4+20] ; fr0=1.0 ; tst r1,r1
//! bf.s mul ; mov.l @r0,r3 (delay)          ; r3 = context pointer
//! zero: [r4+68]=0 ; r4+=72 ; rts ; fmov fr0,@r4 (delay)
//! mul:  tst #8  → r3+=48 ; rts ; fmov @r3,fr0 (delay)
//!       tst #4  → r3+=68 ; rts ; fmov @r3,fr0 (delay)
//!       tst #0x40 → fr0 *= [r3+0x80]
//!       tst #0x20 → fr0 *= [r3+0x6c]
//!       tst #0x10 → fr0 *= [r3+0x58]
//!       tst #0x100 → fr0 *= [r3+0x80]
//!       tst #0x200 → r3+=0x6c ; fr0 *= [r3]
//!       [r4+68]=r1 ; r4+=72 ; rts ; fmov fr0,@r4 (delay)
//! ```
//!
//! T on exit: zero path keeps the `tst r1,r1` result (1); both bit-8/bit-4
//! returns leave 0 from their own tst; the multiply path leaves the `tst` of
//! `r1 & 0x200` (0 when the bit is set, 1 when clear). The delay slot of
//! `bf.s` runs on both outcomes, so r3 is loaded even on the zero path.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c02b77c: mov.l 0x8c02b818,r0
    cpu.r[0] = mem.read_u32(0x8c02b818).unwrap_or(0); // 0xac
    // 0x8c02b77e: mul.l r0,r5
    let product = (cpu.r[0] as i32 as i64) * (cpu.r[5] as i32 as i64);
    cpu.mach = (product >> 32) as u32;
    cpu.macl = product as u32;
    // 0x8c02b780: sts macl,r5
    cpu.r[5] = cpu.macl;
    // 0x8c02b782: mov.l 0x8c02b81c,r0
    cpu.r[0] = mem.read_u32(0x8c02b81c).unwrap_or(0); // 0x8c2b7578
    // 0x8c02b784: add r5,r4
    cpu.r[4] = cpu.r[4].wrapping_add(cpu.r[5]);
    // 0x8c02b786: add #88,r4
    cpu.r[4] = cpu.r[4].wrapping_add(88);
    // 0x8c02b788: mov.l @(20,r4),r1
    cpu.r[1] = mem.read_u32(cpu.r[4].wrapping_add(20)).unwrap_or(0);
    // 0x8c02b78a: fldi1 fr0
    cpu.fr[0] = 1.0;
    // 0x8c02b78c: tst r1,r1
    cpu.set_t(cpu.r[1] == 0);
    // 0x8c02b78e: bf.s 0x8c02b79c
    let zero = cpu.t();
    // 0x8c02b790: [delay] mov.l @r0,r3
    cpu.r[3] = mem.read_u32(cpu.r[0]).unwrap_or(0);
    if zero {
        // 0x8c02b792: mov #68,r0
        cpu.r[0] = 68;
        // 0x8c02b794: mov.l r1,@(r0,r4)
        let addr = cpu.r[0].wrapping_add(cpu.r[4]);
        let _ = mem.write_u32(addr, cpu.r[1]);
        // 0x8c02b796: add #72,r4
        cpu.r[4] = cpu.r[4].wrapping_add(72);
        // 0x8c02b798: rts ; 0x8c02b79a: fmov fr0,@r4 (delay)
        let _ = mem.write_f32(cpu.r[4], cpu.fr[0]);
        return;
    }
    // 0x8c02b79c: mov r1,r0
    cpu.r[0] = cpu.r[1];
    // 0x8c02b79e: and #8,r0
    cpu.r[0] &= 8;
    // 0x8c02b7a0: tst r0,r0
    cpu.set_t(cpu.r[0] == 0);
    // 0x8c02b7a2: bt 0x8c02b7aa
    if !cpu.t() {
        // 0x8c02b7a4: add #48,r3
        cpu.r[3] = cpu.r[3].wrapping_add(48);
        // 0x8c02b7a6: rts ; 0x8c02b7a8: fmov @r3,fr0 (delay)
        cpu.fr[0] = mem.read_f32(cpu.r[3]).unwrap_or(0.0);
        return;
    }
    // 0x8c02b7aa: mov r1,r0
    cpu.r[0] = cpu.r[1];
    // 0x8c02b7ac: and #4,r0
    cpu.r[0] &= 4;
    // 0x8c02b7ae: tst r0,r0
    cpu.set_t(cpu.r[0] == 0);
    // 0x8c02b7b0: bt 0x8c02b7b8
    if !cpu.t() {
        // 0x8c02b7b2: add #68,r3
        cpu.r[3] = cpu.r[3].wrapping_add(68);
        // 0x8c02b7b4: rts ; 0x8c02b7b6: fmov @r3,fr0 (delay)
        cpu.fr[0] = mem.read_f32(cpu.r[3]).unwrap_or(0.0);
        return;
    }
    // 0x8c02b7b8: mov r1,r0
    cpu.r[0] = cpu.r[1];
    // 0x8c02b7ba: and #64,r0
    cpu.r[0] &= 64;
    // 0x8c02b7bc: tst r0,r0
    cpu.set_t(cpu.r[0] == 0);
    // 0x8c02b7be: bt 0x8c02b7ca
    if !cpu.t() {
        // 0x8c02b7c0: mov.l 0x8c02b820,r5
        cpu.r[5] = mem.read_u32(0x8c02b820).unwrap_or(0); // 0x80
        // 0x8c02b7c2: mov r3,r0
        cpu.r[0] = cpu.r[3];
        // 0x8c02b7c4: add r5,r0
        cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[5]);
        // 0x8c02b7c6: fmov @r0,fr1
        cpu.fr[1] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
        // 0x8c02b7c8: fmul fr1,fr0
        cpu.fr[0] *= cpu.fr[1];
    }
    // 0x8c02b7ca: mov r1,r0
    cpu.r[0] = cpu.r[1];
    // 0x8c02b7cc: and #32,r0
    cpu.r[0] &= 32;
    // 0x8c02b7ce: tst r0,r0
    cpu.set_t(cpu.r[0] == 0);
    // 0x8c02b7d0: bt 0x8c02b7da
    if !cpu.t() {
        // 0x8c02b7d2: mov r3,r0
        cpu.r[0] = cpu.r[3];
        // 0x8c02b7d4: add #108,r0
        cpu.r[0] = cpu.r[0].wrapping_add(108);
        // 0x8c02b7d6: fmov @r0,fr1
        cpu.fr[1] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
        // 0x8c02b7d8: fmul fr1,fr0
        cpu.fr[0] *= cpu.fr[1];
    }
    // 0x8c02b7da: mov r1,r0
    cpu.r[0] = cpu.r[1];
    // 0x8c02b7dc: and #16,r0
    cpu.r[0] &= 16;
    // 0x8c02b7de: tst r0,r0
    cpu.set_t(cpu.r[0] == 0);
    // 0x8c02b7e0: bt 0x8c02b7ea
    if !cpu.t() {
        // 0x8c02b7e2: mov r3,r0
        cpu.r[0] = cpu.r[3];
        // 0x8c02b7e4: add #88,r0
        cpu.r[0] = cpu.r[0].wrapping_add(88);
        // 0x8c02b7e6: fmov @r0,fr1
        cpu.fr[1] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
        // 0x8c02b7e8: fmul fr1,fr0
        cpu.fr[0] *= cpu.fr[1];
    }
    // 0x8c02b7ea: mov.l 0x8c02b824,r5
    cpu.r[5] = mem.read_u32(0x8c02b824).unwrap_or(0); // 0x100
    // 0x8c02b7ec: mov r1,r0
    cpu.r[0] = cpu.r[1];
    // 0x8c02b7ee: and r5,r0
    cpu.r[0] &= cpu.r[5];
    // 0x8c02b7f0: tst r0,r0
    cpu.set_t(cpu.r[0] == 0);
    // 0x8c02b7f2: bt 0x8c02b7fe
    if !cpu.t() {
        // 0x8c02b7f4: mov.l 0x8c02b820,r5
        cpu.r[5] = mem.read_u32(0x8c02b820).unwrap_or(0); // 0x80
        // 0x8c02b7f6: mov r3,r0
        cpu.r[0] = cpu.r[3];
        // 0x8c02b7f8: add r5,r0
        cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[5]);
        // 0x8c02b7fa: fmov @r0,fr1
        cpu.fr[1] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
        // 0x8c02b7fc: fmul fr1,fr0
        cpu.fr[0] *= cpu.fr[1];
    }
    // 0x8c02b7fe: mov.l 0x8c02b828,r5
    cpu.r[5] = mem.read_u32(0x8c02b828).unwrap_or(0); // 0x200
    // 0x8c02b800: mov r1,r0
    cpu.r[0] = cpu.r[1];
    // 0x8c02b802: and r5,r0
    cpu.r[0] &= cpu.r[5];
    // 0x8c02b804: tst r0,r0
    cpu.set_t(cpu.r[0] == 0);
    // 0x8c02b806: bt 0x8c02b80e
    if !cpu.t() {
        // 0x8c02b808: add #108,r3
        cpu.r[3] = cpu.r[3].wrapping_add(108);
        // 0x8c02b80a: fmov @r3,fr1
        cpu.fr[1] = mem.read_f32(cpu.r[3]).unwrap_or(0.0);
        // 0x8c02b80c: fmul fr1,fr0
        cpu.fr[0] *= cpu.fr[1];
    }
    // 0x8c02b80e: mov #68,r0
    cpu.r[0] = 68;
    // 0x8c02b810: mov.l r1,@(r0,r4)
    let addr = cpu.r[0].wrapping_add(cpu.r[4]);
    let _ = mem.write_u32(addr, cpu.r[1]);
    // 0x8c02b812: add #72,r4
    cpu.r[4] = cpu.r[4].wrapping_add(72);
    // 0x8c02b814: rts ; 0x8c02b816: fmov fr0,@r4 (delay)
    let _ = mem.write_f32(cpu.r[4], cpu.fr[0]);
}
