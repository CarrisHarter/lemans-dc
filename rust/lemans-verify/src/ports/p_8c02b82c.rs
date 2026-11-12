//! 0x8c02b82c — turn a flag word from an object into a component count and
//! cache it back, returning the accumulated value in fr0.
//!
//! Arguments: r4 = object base (or array base), r5 = element index (stride
//! 0xac). Two globals drive it: the stride and the floating field table
//! pointer at `*(u32*)0x8c2b7578`.
//!
//! CFG (no calls; every path ends in `rts` with the delay slot storing fr0):
//! ```text
//! P = r4 + 0xac*r5 + 0x58                    (multiplication also sets mach/macl)
//! g = *(u32*)0x8c2b7578
//! fl = *(u32*)(P + 0x14)                     (= object + 0x6c)
//! fr0 = 0.0                                  (fldi0, delay slot of bt.s)
//! fl & 0x01 -> [P+0x44] = fl; store fr0 at P+0x50; return 0.0
//! fl & 0x08 -> return *(f32*)(g+0x28)
//! fl & 0x04 -> return *(f32*)(g+0x3c)
//! s = 0.0
//! fl & 0x40 -> s += *(f32*)(g+0x78)
//! fl & 0x20 -> s += *(f32*)(g+0x64)
//! fl & 0x10 -> s += *(f32*)(g+0x50)
//! fl & 0x100 -> fr2 = *(f32*)(g+0x78); fr1 = 2.0; s += fr2*fr1
//! fl & 0x200 -> fr2 = *(f32*)(g+0x64); fr1 = 2.0; s += fr2*fr1
//! [P+0x44] = fl; store s at P+0x50; return s
//! ```
//!
//! Hidden side effects the decompiled C hides:
//! - `mul.l r0,r5` writes `mach` = high word and `macl` = low word of the
//!   signed 0xac*index product; `r5` then keeps `macl`, which is also added
//!   into r4.
//! - r4 ends at P+0x50 on every path (the early 0x01 arm reaches it with
//!   `add #80`), P itself on the two early returns.
//! - r6 ends at g+0x28 (0x08 arm), g+0x3c (0x04 arm), or g (+0x64 when the
//!   0x200 bit is set).
//! - fr1 keeps the last loaded field (or 2.0 on the 0x100/0x200 arms); fr2
//!   keeps the scaled field when either scale arm runs.
//! - T is left by the last `tst`: 0 on the three early returns and on the
//!   0x200 arm, 1 on the computing path when 0x200 is clear.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c02b82c: mov.l 0x8c02b8d4,r0 (0xac)
    cpu.r[0] = mem.read_u32(0x8C02B8D4).unwrap_or(0);
    // 0x8c02b82e: mul.l r0,r5
    let p = (cpu.r[0] as i32 as i64) * (cpu.r[5] as i32 as i64);
    cpu.mach = (p >> 32) as u32;
    cpu.macl = p as u32;
    // 0x8c02b830: sts macl,r5
    cpu.r[5] = cpu.macl;
    // 0x8c02b832: mov.l 0x8c02b8d8,r0 (0x8c2b7578)
    cpu.r[0] = mem.read_u32(0x8C02B8D8).unwrap_or(0);
    // 0x8c02b834: add r5,r4
    cpu.r[4] = cpu.r[4].wrapping_add(cpu.r[5]);
    // 0x8c02b836: add #88,r4
    cpu.r[4] = cpu.r[4].wrapping_add(88);
    // 0x8c02b838: mov.l @r0,r6
    cpu.r[6] = mem.read_u32(cpu.r[0]).unwrap_or(0);
    // 0x8c02b83a: mov.l @(20,r4),r3
    cpu.r[3] = mem.read_u32(cpu.r[4].wrapping_add(20)).unwrap_or(0);
    // 0x8c02b83c: mov r3,r0 ; 0x8c02b83e: and #1,r0 ; 0x8c02b840: tst r0,r0
    cpu.r[0] = cpu.r[3] & 1;
    cpu.set_t(cpu.r[0] == 0);
    // 0x8c02b842: bt.s 0x8c02b850 ; 0x8c02b844: fldi0 fr0 (delay slot)
    cpu.fr[0] = 0.0;
    if !cpu.t() {
        // 0x8c02b846: mov #68,r0
        cpu.r[0] = 68;
        // 0x8c02b848: mov.l r3,@(r0,r4)
        let _ = mem.write_u32(cpu.r[0].wrapping_add(cpu.r[4]), cpu.r[3]);
        // 0x8c02b84a: add #80,r4
        cpu.r[4] = cpu.r[4].wrapping_add(80);
        // 0x8c02b84c: rts ; 0x8c02b84e: fmov fr0,@r4 (delay slot)
        let _ = mem.write_f32(cpu.r[4], cpu.fr[0]);
        return;
    }

    // 0x8c02b850: mov r3,r0 ; and #8,r0 ; tst r0,r0 ; 0x8c02b856: bt 0x8c02b85e
    cpu.r[0] = cpu.r[3] & 8;
    cpu.set_t(cpu.r[0] == 0);
    if !cpu.t() {
        // 0x8c02b858: add #40,r6
        cpu.r[6] = cpu.r[6].wrapping_add(40);
        // 0x8c02b85a: rts ; 0x8c02b85c: fmov @r6,fr0 (delay slot)
        cpu.fr[0] = mem.read_f32(cpu.r[6]).unwrap_or(0.0);
        return;
    }

    // 0x8c02b85e: mov r3,r0 ; and #4,r0 ; tst r0,r0 ; 0x8c02b864: bt 0x8c02b86c
    cpu.r[0] = cpu.r[3] & 4;
    cpu.set_t(cpu.r[0] == 0);
    if !cpu.t() {
        // 0x8c02b866: add #60,r6
        cpu.r[6] = cpu.r[6].wrapping_add(60);
        // 0x8c02b868: rts ; 0x8c02b86a: fmov @r6,fr0 (delay slot)
        cpu.fr[0] = mem.read_f32(cpu.r[6]).unwrap_or(0.0);
        return;
    }

    // 0x8c02b86c: and #64: fr0 += *(g+0x78)
    cpu.r[0] = cpu.r[3] & 64;
    cpu.set_t(cpu.r[0] == 0);
    if !cpu.t() {
        cpu.r[0] = cpu.r[6].wrapping_add(120);
        cpu.fr[1] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
        cpu.fr[0] = cpu.fr[0] + cpu.fr[1];
    }

    // 0x8c02b87c: and #32: fr0 += *(g+0x64)
    cpu.r[0] = cpu.r[3] & 32;
    cpu.set_t(cpu.r[0] == 0);
    if !cpu.t() {
        cpu.r[0] = cpu.r[6].wrapping_add(100);
        cpu.fr[1] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
        cpu.fr[0] = cpu.fr[0] + cpu.fr[1];
    }

    // 0x8c02b88c: and #16: fr0 += *(g+0x50)
    cpu.r[0] = cpu.r[3] & 16;
    cpu.set_t(cpu.r[0] == 0);
    if !cpu.t() {
        cpu.r[0] = cpu.r[6].wrapping_add(80);
        cpu.fr[1] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
        cpu.fr[0] = cpu.fr[0] + cpu.fr[1];
    }

    // 0x8c02b89c: mov.l 0x8c02b8dc,r1 (0x100) ; r0 = r3 ; and r1,r0 ; tst
    cpu.r[1] = mem.read_u32(0x8C02B8DC).unwrap_or(0);
    cpu.r[0] = cpu.r[3] & cpu.r[1];
    cpu.set_t(cpu.r[0] == 0);
    if !cpu.t() {
        // 0x8c02b8a6: mov r6,r1 ; add #120,r1
        cpu.r[1] = cpu.r[6].wrapping_add(120);
        // 0x8c02b8aa: mova 0x8c02b8e0,r0 (2.0f)
        cpu.r[0] = 0x8C02B8E0;
        // 0x8c02b8ac: fmov @r1,fr2 ; 0x8c02b8ae: fmov @r0,fr1
        cpu.fr[2] = mem.read_f32(cpu.r[1]).unwrap_or(0.0);
        cpu.fr[1] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
        // 0x8c02b8b0: fmul fr1,fr2 ; 0x8c02b8b2: fadd fr2,fr0
        cpu.fr[2] = cpu.fr[2] * cpu.fr[1];
        cpu.fr[0] = cpu.fr[0] + cpu.fr[2];
    }

    // 0x8c02b8b4: mov.l 0x8c02b8e4,r1 (0x200) ; r0 = r3 ; and r1,r0 ; tst
    cpu.r[1] = mem.read_u32(0x8C02B8E4).unwrap_or(0);
    cpu.r[0] = cpu.r[3] & cpu.r[1];
    cpu.set_t(cpu.r[0] == 0);
    if !cpu.t() {
        // 0x8c02b8be: add #100,r6 ; 0x8c02b8c0: mova 0x8c02b8e0,r0
        cpu.r[6] = cpu.r[6].wrapping_add(100);
        cpu.r[0] = 0x8C02B8E0;
        // 0x8c02b8c2: fmov @r6,fr2 ; 0x8c02b8c4: fmov @r0,fr1
        cpu.fr[2] = mem.read_f32(cpu.r[6]).unwrap_or(0.0);
        cpu.fr[1] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
        // 0x8c02b8c6: fmul fr1,fr2 ; 0x8c02b8c8: fadd fr2,fr0
        cpu.fr[2] = cpu.fr[2] * cpu.fr[1];
        cpu.fr[0] = cpu.fr[0] + cpu.fr[2];
    }

    // 0x8c02b8ca: mov #68,r0
    cpu.r[0] = 68;
    // 0x8c02b8cc: mov.l r3,@(r0,r4)
    let _ = mem.write_u32(cpu.r[0].wrapping_add(cpu.r[4]), cpu.r[3]);
    // 0x8c02b8ce: add #80,r4
    cpu.r[4] = cpu.r[4].wrapping_add(80);
    // 0x8c02b8d0: rts ; 0x8c02b8d2: fmov fr0,@r4 (delay slot)
    let _ = mem.write_f32(cpu.r[4], cpu.fr[0]);
}
