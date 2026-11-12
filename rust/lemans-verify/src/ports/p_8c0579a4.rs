//! 0x8c0579a4 — recentre a list of float (x, y) pairs on integer quarter units.
//!
//! `param_2` (`r10`) is an object with:
//! * `+0x44` — array of node pointers, `+0x48` — node count;
//! * `+0x38` — optional interface object (vtable methods at +0xc/+0x14/+0x38/+0x40).
//!
//! Pass 1 classifies every node whose `[node+4]` word is non-zero. The callee
//! 0x8c05f294 returns `[node+4] + 4`; a signed 16-bit load there, shifted
//! `<< 22` then arithmetic `>> 29`, yields the sign-extended 3-bit field in
//! bits 7..9. Two flags are cleared as the loop walks the nodes:
//! `r14` (clear on 2 or 3) and `r13` (clear on 1 or 3). If both end clear the
//! function returns without touching the interface.
//!
//! Interface protocol (all calls go through `[param_2+0x38]`'s vtable):
//! * `+0xc(0x40)` — gate; a zero low byte returns immediately;
//! * `+0x14()` — element count `n` (`r12`);
//! * `+0x38(2, 0, n-1, &desc)` — with `r4` = interface, `r5` = 2, `r6` = 0,
//!   `r7` = n-1 and the address of the two-word `{base, stride}` scratch
//!   descriptor pushed as the fifth argument;
//! * `+0x40()` — close, after the two arithmetic passes.
//!
//! The min/max pass walks `n` entries of `{f32 x, f32 y}` at
//! `base + i*stride`, tracking min/max per component in `fr3/fr4` (min) and
//! `fr1/fr2` (max), seeded from the literals ±3.4028235e37. It then computes
//! `r3 = (int)((minx+maxx)*0.25)` and `r1 = (int)((miny+maxy)*0.25)`, converts
//! those back to floats and doubles them (`fr1`/`fr2`), and the second pass
//! subtracts `fr1` from x when `r14` is set and `fr2` from y when `r13` is set.
//!
//! The four interface calls are `jsr @r0` through vtable slots that are only
//! known at run time, so they are modelled by reading the slot exactly like the
//! original does and running that address's registered port. The port of a
//! target is the behavioural model of the function the slot points at; a slot
//! pointing outside the ported set is reported with a panic instead of being
//! skipped. Every call sets `pr` to the return address after the `jsr` delay
//! slot, and the delay-slot register updates (`r5 = 0x40`, `r5 = 2`) are
//! applied before the callee runs. Literal-pool reads (0x8c057b2c..0x8c057b40,
//! the 0x8c05f294 pointer at 0x8c057b30) are real memory loads.

use sh4_oracle::{Cpu, Memory};

/// Model a `jsr @r0` whose target is a runtime vtable slot: the slot value is
/// the address of an already-ported routine.
fn call_slot(cpu: &mut Cpu, mem: &mut Memory, target: u32) {
    match crate::registry::lookup(target) {
        Some(port) => port(cpu, mem),
        None => panic!("0x8c0579a4: indirect call to unported target 0x{target:08x}"),
    }
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 8c0579a4: mov.l r14,@-r15 ; mov.l r13,@-r15 ; mov.l r12,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[12]);
    // 8c0579aa: mov.l r11,@-r15 ; mov.l r10,@-r15 ; sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[11]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[10]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 8c0579b0: add #-8,r15 ; mov r4,r10 ; mov.l 0x8c057b2c,r11
    cpu.r[15] = cpu.r[15].wrapping_sub(8);
    cpu.r[10] = cpu.r[4];
    cpu.r[11] = mem.read_u32(0x8c057b2c).unwrap_or(0); // 0xfffffffe
    // 8c0579b6: mov #1,r14 ; mov r14,r13 ; bra 0x8c0579fa ; mov #0,r12
    cpu.r[14] = 1;
    cpu.r[13] = 1;
    cpu.r[12] = 0;

    'epilogue: {
        // ---- pass 1: classify nodes -------------------------------------
        // 8c0579fa: mov r10,r0 ; add #72,r0 ; mov.l @r0,r0
        loop {
            cpu.r[0] = mem.read_u32(cpu.r[10].wrapping_add(72)).unwrap_or(0);
            // cmp/gt r12,r0 (signed), bt 0x8c0579be
            cpu.set_t((cpu.r[0] as i32) > (cpu.r[12] as i32));
            if !cpu.t() {
                break;
            }
            // 8c0579be: mov r10,r0 ; add #68,r0 ; mov.l @r0,r1
            cpu.r[1] = mem.read_u32(cpu.r[10].wrapping_add(68)).unwrap_or(0);
            // mov r12,r0 ; shll2 r0 ; mov.l @(r0,r1),r0 ; mov.l @(4,r0),r4
            cpu.r[0] = cpu.r[12] << 2; // oracle's shll2 leaves T alone
            cpu.r[0] = mem.read_u32(cpu.r[1].wrapping_add(cpu.r[0])).unwrap_or(0);
            cpu.r[4] = mem.read_u32(cpu.r[0].wrapping_add(4)).unwrap_or(0);
            // 8c0579cc: tst r4,r4 ; bt 0x8c0579f8
            cpu.set_t(cpu.r[4] == 0);
            if !cpu.t() {
                // 8c0579d0: mov.l 0x8c057b30,r0 (0x8c05f294) ; jsr @r0 ; nop
                cpu.r[0] = mem.read_u32(0x8c057b30).unwrap_or(0);
                cpu.pr = 0x8c0579d6;
                crate::registry::p_8c05f294::run(cpu, mem);
                // 8c0579d6: mov.w @r0,r3 (sign-extended)
                cpu.r[3] = mem.read_u16(cpu.r[0]).unwrap_or(0) as i16 as i32 as u32;
                // 8c0579d8: mov #22,r1 ; shld r1,r3
                cpu.r[1] = 22;
                cpu.r[3] <<= 22;
                // 8c0579dc: mov #-29,r0 ; shad r0,r3 (negative: arithmetic right)
                cpu.r[0] = (-29i32) as u32;
                let amount = cpu.r[0] as i32;
                cpu.r[3] = if amount >= 0 {
                    cpu.r[3] << ((amount as u32) & 31)
                } else {
                    ((cpu.r[3] as i32) >> (((-amount) as u32) & 31)) as u32
                };
                // 8c0579e0: exts.w r3,r0 ; mov r0,r3 ; add r11,r3
                cpu.r[0] = cpu.r[3] as i16 as i32 as u32;
                cpu.r[3] = cpu.r[0];
                cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[11]);
                // 8c0579e6: mov #1,r1 ; cmp/hs r3,r1 ; bf 0x8c0579ee
                cpu.r[1] = 1;
                cpu.set_t(cpu.r[1] >= cpu.r[3]);
                if cpu.t() {
                    cpu.r[14] = 0; // 8c0579ec: mov #0,r14 (type 2 or 3)
                }
                // 8c0579ee: cmp/eq #1,r0 ; bt 0x8c0579f6
                cpu.set_t(cpu.r[0] == 1);
                if cpu.t() {
                    cpu.r[13] = 0;
                } else {
                    // 8c0579f2: cmp/eq #3,r0 ; bf 0x8c0579f8
                    cpu.set_t(cpu.r[0] == 3);
                    if cpu.t() {
                        cpu.r[13] = 0; // 8c0579f6: mov #0,r13
                    }
                }
            }
            // 8c0579f8: add #1,r12
            cpu.r[12] = cpu.r[12].wrapping_add(1);
        }

        // ---- gate: at least one flag must survive -----------------------
        // 8c057a04: extu.b r14,r0 ; tst r0,r0 ; bf.s 0x8c057a14 ; mov #1,r1
        cpu.r[0] = cpu.r[14] & 0xff;
        cpu.set_t(cpu.r[0] == 0);
        cpu.r[1] = 1; // delay slot, runs whether or not the branch is taken
        if cpu.t() {
            // 8c057a0c: extu.b r13,r0 ; tst r0,r0 ; bf 0x8c057a14
            cpu.r[0] = cpu.r[13] & 0xff;
            cpu.set_t(cpu.r[0] == 0);
            if cpu.t() {
                cpu.r[1] = 0; // 8c057a12: mov #0,r1
            }
        }
        // 8c057a14: extu.b r1,r0 ; tst r0,r0 ; bt 0x8c057b1a
        cpu.r[0] = cpu.r[1] & 0xff;
        cpu.set_t(cpu.r[0] == 0);
        if cpu.t() {
            break 'epilogue;
        }

        // ---- interface gate --------------------------------------------
        // 8c057a1a: mov.l @(56,r10),r4 ; tst r4,r4 ; bt 0x8c057b1a
        cpu.r[4] = mem.read_u32(cpu.r[10].wrapping_add(56)).unwrap_or(0);
        cpu.set_t(cpu.r[4] == 0);
        if cpu.t() {
            break 'epilogue;
        }
        // 8c057a20: mov.l @r4,r0 ; mov.l @(12,r0),r0 ; jsr @r0 ; mov #64,r5
        cpu.r[0] = mem.read_u32(cpu.r[4]).unwrap_or(0);
        cpu.r[0] = mem.read_u32(cpu.r[0].wrapping_add(12)).unwrap_or(0);
        cpu.r[5] = 64;
        cpu.pr = 0x8c057a28;
        call_slot(cpu, mem, cpu.r[0]);
        // 8c057a28: extu.b r0,r0 ; tst r0,r0 ; bt 0x8c057b1a
        cpu.r[0] &= 0xff;
        cpu.set_t(cpu.r[0] == 0);
        if cpu.t() {
            break 'epilogue;
        }

        // ---- element count ---------------------------------------------
        // 8c057a2e: mov.l @(56,r10),r4 ; mov.l @r4,r0 ; mov.l @(20,r0),r0
        cpu.r[4] = mem.read_u32(cpu.r[10].wrapping_add(56)).unwrap_or(0);
        cpu.r[0] = mem.read_u32(cpu.r[4]).unwrap_or(0);
        cpu.r[0] = mem.read_u32(cpu.r[0].wrapping_add(20)).unwrap_or(0);
        cpu.pr = 0x8c057a38;
        call_slot(cpu, mem, cpu.r[0]); // delay slot: nop
        // 8c057a38: mov #0,r6 ; mov.l r6,@r15 ; mov r15,r1 ; mov.l r6,@(4,r15)
        // 8c057a40: mov r0,r12 ; mov.l r1,@-r15
        cpu.r[6] = 0;
        let _ = mem.write_u32(cpu.r[15], cpu.r[6]);
        cpu.r[1] = cpu.r[15];
        let _ = mem.write_u32(cpu.r[15].wrapping_add(4), cpu.r[6]);
        cpu.r[12] = cpu.r[0];
        cpu.r[15] = cpu.r[15].wrapping_sub(4);
        let _ = mem.write_u32(cpu.r[15], cpu.r[1]);
        // 8c057a44: mov.l @(56,r10),r4 ; mov r12,r7 ; add #-1,r7
        cpu.r[4] = mem.read_u32(cpu.r[10].wrapping_add(56)).unwrap_or(0);
        cpu.r[7] = cpu.r[12].wrapping_sub(1);
        // 8c057a4a: mov.l @r4,r0 ; mov.l @(56,r0),r0 ; jsr @r0 ; mov #2,r5
        cpu.r[0] = mem.read_u32(cpu.r[4]).unwrap_or(0);
        cpu.r[0] = mem.read_u32(cpu.r[0].wrapping_add(56)).unwrap_or(0);
        cpu.r[5] = 2;
        cpu.pr = 0x8c057a52;
        call_slot(cpu, mem, cpu.r[0]);
        // 8c057a52: add #4,r15
        cpu.r[15] = cpu.r[15].wrapping_add(4);

        // ---- min/max pass ----------------------------------------------
        // 8c057a54: mova 0x8c057b34,r0 ; fmov @r0,fr3 ; cmp/pl r12
        cpu.r[0] = 0x8c057b34;
        cpu.fr[3] = mem.read_f32(cpu.r[0]).unwrap_or(0.0); // +3.4028235e37
        cpu.set_t((cpu.r[12] as i32) > 0);
        // 8c057a5a: mov #0,r1 ; mova 0x8c057b38,r0 ; fmov @r0,fr1
        cpu.r[1] = 0;
        cpu.r[0] = 0x8c057b38;
        cpu.fr[1] = mem.read_f32(cpu.r[0]).unwrap_or(0.0); // -3.4028235e37
        // 8c057a60: fmov fr3,fr4 ; bf.s 0x8c057aaa ; fmov fr1,fr2
        cpu.fr[4] = cpu.fr[3];
        cpu.fr[2] = cpu.fr[1]; // delay slot, always executed
        if cpu.t() {
            loop {
                // 8c057a66: mov.l @(4,r15),r3 ; mov.l @r15,r4
                cpu.r[3] = mem.read_u32(cpu.r[15].wrapping_add(4)).unwrap_or(0);
                cpu.r[4] = mem.read_u32(cpu.r[15]).unwrap_or(0);
                // 8c057a6a: mov r3,r0 ; mul.l r1,r0 ; sts macl,r0 ; add r4,r0
                cpu.r[0] = cpu.r[3];
                let p = (cpu.r[0] as i32 as i64) * (cpu.r[1] as i32 as i64);
                cpu.mach = (p >> 32) as u32;
                cpu.macl = p as u32;
                cpu.r[0] = cpu.macl;
                cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[4]);
                // 8c057a72: fmov @r0,fr0 ; add #4,r0 ; fcmp/gt fr3,fr0
                cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
                cpu.r[0] = cpu.r[0].wrapping_add(4);
                cpu.set_t(cpu.fr[0] > cpu.fr[3]);
                // 8c057a78: bf.s 0x8c057a80 ; fmov @r0,fr5
                cpu.fr[5] = mem.read_f32(cpu.r[0]).unwrap_or(0.0); // delay slot
                if !cpu.t() {
                    cpu.fr[3] = cpu.fr[0]; // 8c057a80
                }
                // 8c057a82: fcmp/gt fr4,fr5 ; bf 0x8c057a8a
                cpu.set_t(cpu.fr[5] > cpu.fr[4]);
                if !cpu.t() {
                    cpu.fr[4] = cpu.fr[5]; // 8c057a8a
                }
                // 8c057a8c: fcmp/gt fr1,fr0 ; bf 0x8c057a94
                cpu.set_t(cpu.fr[0] > cpu.fr[1]);
                if !cpu.t() {
                    cpu.fr[0] = cpu.fr[1]; // 8c057a94
                }
                // 8c057a96: fcmp/gt fr2,fr5 ; bf.s 0x8c057aa0 ; fmov fr0,fr1
                cpu.set_t(cpu.fr[5] > cpu.fr[2]);
                cpu.fr[1] = cpu.fr[0]; // delay slot, always executed
                if !cpu.t() {
                    cpu.fr[5] = cpu.fr[2]; // 8c057aa0
                }
                // 8c057aa2: add #1,r1 ; cmp/gt r1,r12 ; bt.s 0x8c057a6a ;
                //           fmov fr5,fr2
                cpu.r[1] = cpu.r[1].wrapping_add(1);
                cpu.set_t((cpu.r[12] as i32) > (cpu.r[1] as i32));
                cpu.fr[2] = cpu.fr[5]; // delay slot, always executed
                if !cpu.t() {
                    break;
                }
            }
        }

        // ---- quarter-unit centres --------------------------------------
        // 8c057aaa: mova 0x8c057b3c,r0 ; fmov @r0,fr0 (0.5)
        cpu.r[0] = 0x8c057b3c;
        cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
        // 8c057aae: fadd fr1,fr3 ; fadd fr2,fr4
        cpu.fr[3] = cpu.fr[3] + cpu.fr[1];
        cpu.fr[4] = cpu.fr[4] + cpu.fr[2];
        // 8c057ab2: fmul fr0,fr3 ; fmul fr0,fr4 ; fmul fr0,fr3
        cpu.fr[3] = cpu.fr[3] * cpu.fr[0];
        cpu.fr[4] = cpu.fr[4] * cpu.fr[0];
        cpu.fr[3] = cpu.fr[3] * cpu.fr[0];
        // 8c057ab8: mova 0x8c057b40,r0 ; fmul fr0,fr4 (fr0 is still 0.5)
        cpu.r[0] = 0x8c057b40;
        cpu.fr[4] = cpu.fr[4] * cpu.fr[0];
        // 8c057abc: ftrc fr3,fpul ; sts fpul,r3
        cpu.fpul = cpu.fr[3] as i32 as u32;
        cpu.r[3] = cpu.fpul;
        // 8c057ac0: ftrc fr4,fpul
        cpu.fpul = cpu.fr[4] as i32 as u32;
        // 8c057ac2: fmov @r0,fr0 (2.0) ; sts fpul,r1
        cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
        cpu.r[1] = cpu.fpul;
        // 8c057ac6: lds r3,fpul ; float fpul,fr1
        cpu.fpul = cpu.r[3];
        cpu.fr[1] = cpu.fpul as i32 as f32;
        // 8c057aca: lds r1,fpul ; cmp/pl r12 ; fmul fr0,fr1 ; mov #0,r5
        cpu.fpul = cpu.r[1];
        cpu.set_t((cpu.r[12] as i32) > 0);
        cpu.fr[1] = cpu.fr[1] * cpu.fr[0];
        cpu.r[5] = 0;
        // 8c057ad2: float fpul,fr2 ; bf.s 0x8c057b0c ; fmul fr0,fr2
        cpu.fr[2] = cpu.fpul as i32 as f32;
        cpu.fr[2] = cpu.fr[2] * cpu.fr[0]; // delay slot, always executed
        if cpu.t() {
            // 8c057ad8: extu.b r13,r3 ; extu.b r14,r4 (hoisted out of the loop)
            cpu.r[3] = cpu.r[13] & 0xff;
            cpu.r[4] = cpu.r[14] & 0xff;
            loop {
                // 8c057adc: tst r4,r4 ; bt 0x8c057af0
                cpu.set_t(cpu.r[4] == 0);
                if !cpu.t() {
                    // 8c057ae0: mov.l @(4,r15),r1 ; mov.l @r15,r0
                    cpu.r[1] = mem.read_u32(cpu.r[15].wrapping_add(4)).unwrap_or(0);
                    cpu.r[0] = mem.read_u32(cpu.r[15]).unwrap_or(0);
                    // 8c057ae4: mul.l r5,r1 ; sts macl,r1 ; add r0,r1
                    let p = (cpu.r[1] as i32 as i64) * (cpu.r[5] as i32 as i64);
                    cpu.mach = (p >> 32) as u32;
                    cpu.macl = p as u32;
                    cpu.r[1] = cpu.macl;
                    cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[0]);
                    // 8c057aea: fmov @r1,fr0 ; fsub fr1,fr0 ; fmov fr0,@r1
                    cpu.fr[0] = mem.read_f32(cpu.r[1]).unwrap_or(0.0);
                    cpu.fr[0] = cpu.fr[0] - cpu.fr[1];
                    let _ = mem.write_f32(cpu.r[1], cpu.fr[0]);
                }
                // 8c057af0: tst r3,r3 ; bt 0x8c057b06
                cpu.set_t(cpu.r[3] == 0);
                if !cpu.t() {
                    // 8c057af4: mov.l @(4,r15),r1 ; mov.l @r15,r0
                    cpu.r[1] = mem.read_u32(cpu.r[15].wrapping_add(4)).unwrap_or(0);
                    cpu.r[0] = mem.read_u32(cpu.r[15]).unwrap_or(0);
                    // 8c057af8: mul.l r5,r1 ; sts macl,r1 ; add r0,r1 ; add #4,r1
                    let p = (cpu.r[1] as i32 as i64) * (cpu.r[5] as i32 as i64);
                    cpu.mach = (p >> 32) as u32;
                    cpu.macl = p as u32;
                    cpu.r[1] = cpu.macl;
                    cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[0]);
                    cpu.r[1] = cpu.r[1].wrapping_add(4);
                    // 8c057b00: fmov @r1,fr0 ; fsub fr2,fr0 ; fmov fr0,@r1
                    cpu.fr[0] = mem.read_f32(cpu.r[1]).unwrap_or(0.0);
                    cpu.fr[0] = cpu.fr[0] - cpu.fr[2];
                    let _ = mem.write_f32(cpu.r[1], cpu.fr[0]);
                }
                // 8c057b06: add #1,r5 ; cmp/gt r5,r12 ; bt 0x8c057adc
                cpu.r[5] = cpu.r[5].wrapping_add(1);
                cpu.set_t((cpu.r[12] as i32) > (cpu.r[5] as i32));
                if !cpu.t() {
                    break;
                }
            }
        }

        // ---- close the interface ---------------------------------------
        // 8c057b0c: mov.l @(56,r10),r4 ; mov #64,r2 ; mov.l @r4,r0
        //           add r0,r2 ; mov.l @r2,r0 ; jsr @r0 ; nop
        cpu.r[4] = mem.read_u32(cpu.r[10].wrapping_add(56)).unwrap_or(0);
        cpu.r[2] = 64;
        cpu.r[0] = mem.read_u32(cpu.r[4]).unwrap_or(0);
        cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[0]);
        cpu.r[0] = mem.read_u32(cpu.r[2]).unwrap_or(0);
        cpu.pr = 0x8c057b1a;
        call_slot(cpu, mem, cpu.r[0]);
    }

    // 8c057b1a: add #8,r15 ; lds.l @r15+,pr ; mov.l @r15+,r10 ... r13
    cpu.r[15] = cpu.r[15].wrapping_add(8);
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[10] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[11] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[12] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // rts delay slot: mov.l @r15+,r14
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
