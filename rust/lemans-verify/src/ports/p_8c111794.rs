//! 0x8c111794 — insertion sort over `[r4, r5)` stepping by 4.
//!
//! `r13` is the key. The inner loop shifts the preceding pointer right while
//! the compare result byte is nonzero, and the `bt.s` delay store places the
//! key on both continue and outer-loop exit. Outer test is `cmp/hi r14,r10`
//! (`T = r10 > r14` unsigned); inner test is `cmp/hi r9,r11`
//! (`T = r11 > r9` unsigned). The shift block clobbers `r0`.
//! Callee-saved `r9`–`r14` and `pr` are restored; final `T` is always 0.
//!
//! Compare element `[r12]` (loaded into `r5` in the `bf.s` delay slot) with
//! key `r13`, using the signed field at +24:
//! both `cmp/pl` → `0x8c085480` lexcmp of `r13+40` vs `r5+40` (shift while the
//! element is greater); else if both fields are `<= 0`, shift while `[r5] > [r13]`
//! signed; else the result is the key's own `cmp/pl` (a positive key shifts a
//! non-positive element right, and a non-positive key stops on a positive one).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 8c111794..8c11179e: push r14,r13,r12,r11,r10,r9
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[12]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[11]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[10]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[9]);
    // 8c1117a0: mov r5,r10 ; 8c1117a2: sts.l pr,@-r15
    cpu.r[10] = cpu.r[5];
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 8c1117a4: mov r4,r9 ; 8c1117a6: mov r9,r14 ; 8c1117a8: add #4,r14
    cpu.r[9] = cpu.r[4];
    cpu.r[14] = cpu.r[9];
    cpu.r[14] = cpu.r[14].wrapping_add(4);
    // 8c1117aa: cmp/hi r14,r10 ; 8c1117ac: bf 0x8c111808
    cpu.set_t(cpu.r[10] > cpu.r[14]);
    if cpu.t() {
        loop {
            // 8c1117ae: mov.l @r14,r13 ; b0: mov r14,r12 ; b2: mov r14,r11
            cpu.r[13] = mem.read_u32(cpu.r[14]).unwrap_or(0);
            cpu.r[12] = cpu.r[14];
            cpu.r[11] = cpu.r[14];
            // 8c1117b4: bra 0x8c1117c0 ; b6: add #-4,r12 (delay)
            cpu.r[12] = cpu.r[12].wrapping_sub(4);
            loop {
                // 8c1117c0: cmp/hi r9,r11 ; c2: bf 0x8c111800
                cpu.set_t(cpu.r[11] > cpu.r[9]);
                if !cpu.t() {
                    break;
                }
                // 8c1117c4: mov.l @(24,r13),r3 ; c6: cmp/pl r3
                cpu.r[3] = mem.read_u32(cpu.r[13].wrapping_add(24)).unwrap_or(0);
                cpu.set_t((cpu.r[3] as i32) > 0);
                let key_pos = cpu.t();
                // 8c1117c8: bf.s 0x8c1117e0 ; ca: mov.l @r12,r5 (delay, both sides)
                cpu.r[5] = mem.read_u32(cpu.r[12]).unwrap_or(0);
                let mut both_pos = false;
                if key_pos {
                    // 8c1117cc: mov.l @(24,r5),r0 ; ce: cmp/pl r0 ; d0: bf 0x8c1117e0
                    cpu.r[0] = mem.read_u32(cpu.r[5].wrapping_add(24)).unwrap_or(0);
                    cpu.set_t((cpu.r[0] as i32) > 0);
                    both_pos = cpu.t();
                }
                if both_pos {
                    // 8c1117d2: mov.l 0x8c111818,r0 — 0x8c085480
                    cpu.r[0] = mem.read_u32(0x8c111818).unwrap_or(0);
                    // 8c1117d4: mov r13,r4 ; d6: add #40,r5
                    // 8c1117d8: jsr @r0 ; da: add #40,r4 (delay)
                    cpu.r[4] = cpu.r[13];
                    cpu.r[5] = cpu.r[5].wrapping_add(40);
                    cpu.pr = 0x8c1117dc;
                    cpu.r[4] = cpu.r[4].wrapping_add(40);
                    crate::registry::p_8c085480::run(cpu, mem);
                    // 8c1117dc: bra 0x8c1117fa ; de: nop
                } else {
                    // 8c1117e0: mov #1,r1 ; e2: cmp/gt r3,r1 (T = 1 > r3)
                    cpu.r[1] = 1;
                    cpu.set_t((cpu.r[1] as i32) > (cpu.r[3] as i32));
                    if cpu.t() {
                        // 8c1117e6: mov.l @(24,r5),r0 ; e8: cmp/gt r0,r1
                        cpu.r[0] = mem.read_u32(cpu.r[5].wrapping_add(24)).unwrap_or(0);
                        cpu.set_t((cpu.r[1] as i32) > (cpu.r[0] as i32));
                        if cpu.t() {
                            // 8c1117ec: mov.l @r13,r1 ; ee: mov.l @r5,r0
                            // 8c1117f0: cmp/gt r1,r0 ; f2: bra ; f4: movt r0
                            cpu.r[1] = mem.read_u32(cpu.r[13]).unwrap_or(0);
                            cpu.r[0] = mem.read_u32(cpu.r[5]).unwrap_or(0);
                            cpu.set_t((cpu.r[0] as i32) > (cpu.r[1] as i32));
                            cpu.r[0] = cpu.t() as u32;
                        } else {
                            // 8c1117f6: cmp/pl r3 ; f8: movt r0
                            cpu.set_t((cpu.r[3] as i32) > 0);
                            cpu.r[0] = cpu.t() as u32;
                        }
                    } else {
                        // 8c1117f6: cmp/pl r3 ; f8: movt r0
                        cpu.set_t((cpu.r[3] as i32) > 0);
                        cpu.r[0] = cpu.t() as u32;
                    }
                }
                // 8c1117fa: extu.b r0,r0 ; fc: tst r0,r0 ; fe: bf 0x8c1117b8
                cpu.r[0] &= 0xff;
                cpu.set_t(cpu.r[0] == 0);
                if cpu.t() {
                    break;
                }
                // 8c1117b8: mov.l @r12,r0 ; ba: mov.l r0,@r11
                cpu.r[0] = mem.read_u32(cpu.r[12]).unwrap_or(0);
                let _ = mem.write_u32(cpu.r[11], cpu.r[0]);
                // 8c1117bc: add #-4,r12 ; be: add #-4,r11
                cpu.r[12] = cpu.r[12].wrapping_sub(4);
                cpu.r[11] = cpu.r[11].wrapping_sub(4);
            }
            // 8c111800: add #4,r14 ; 02: cmp/hi r14,r10
            cpu.r[14] = cpu.r[14].wrapping_add(4);
            cpu.set_t(cpu.r[10] > cpu.r[14]);
            // 8c111804: bt.s 0x8c1117ae ; 06: mov.l r13,@r11 (delay: both sides)
            let take = cpu.t();
            let _ = mem.write_u32(cpu.r[11], cpu.r[13]);
            if !take {
                break;
            }
        }
    }
    // 8c111808: lds.l @r15+,pr ; pop r9..r13 ; rts ; @r15+,r14
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[9] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[10] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[11] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[12] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
